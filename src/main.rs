use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{prelude::*, backends::raycast::RaycastPickable};
use bevy_prototype_lyon::prelude::*;

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup)
    ;
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    server: Res<AssetServer>,
){
    commands.spawn(
        Camera2dBundle::default()
    );


    commands.spawn((
        MaterialMesh2dBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            mesh: meshes.add(shape::Circle::new(50.0).into()).into(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        PickableBundle::default(),
    )).with_children(|parent| {
        parent.spawn((
            MaterialMesh2dBundle {
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                mesh: meshes.add(shape::Circle::new(25.0).into()).into(),
                transform: Transform::from_translation(Vec3::new(0.0, 50.0, -10.0)),
                ..default()
            },
            PickableBundle::default(),
        ));
    });

    commands.spawn((
        SpriteBundle {
            texture: server.load("0000.png"),
            transform: Transform {
                translation: Vec3::new(200.0, 0.0, 0.0),
                scale: Vec3::new(0.1, 0.1, 1.0),
                ..default()
            },
            ..default()
        },
        PickableBundle::default(),
    )).with_children(|parent| {
        parent.spawn((
            MaterialMesh2dBundle {
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                mesh: meshes.add(shape::Circle::new(250.0).into()).into(),
                transform: Transform::from_translation(Vec3::new(0.0, 500.0, -10.0)),
                ..default()
            },
            PickableBundle::default(),
        ));
    });


    commands.spawn((
        SpriteBundle {
            texture: server.load("0000.png"),
            transform: Transform {
                translation: Vec3::new(-200.0, 0.0, 0.0),
                scale: Vec3::new(0.1, 0.1, 1.0),
                ..default()
            },
            ..default()
        },
        PickableBundle::default(),
    )).with_children(|parent| {

        let outline_shape = shapes::Circle {
            radius: 60.0,
            center: Vec2::from((0.0, 0.0)),
        };
    
        let outline_path = GeometryBuilder::build_as(&outline_shape);

        parent.spawn((
            ShapeBundle {
                path: outline_path,
                ..default()
            },
            Stroke::new(
                Color::ORANGE, 100.0
            ),
            RaycastPickable::default(),
        ));
    });

}