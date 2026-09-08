
use super::*;
use crate::editor::shooting::ShootingCommand;
use crate::editor::shooting::testkit::{dispatch, shooting_app};

#[semio_framework_async_macros::async_test]
async fn set_active_shot_label_patches_active_shot() {
    let mut app = shooting_app().await;
    dispatch(&mut app, ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Hero Shot".into() })).await;
    assert_eq!(crate::schema::active_shot(&app.snapshot().expect("snapshot")).unwrap().label, "Hero Shot");
}

#[semio_framework_async_macros::async_test]
async fn add_shot_action_appends_shot() {
    let mut app = shooting_app().await;
    dispatch(&mut app, ShootingCommand::AddShot(add_shot::AddShot { format: "svg".into(), shape: "ellipse".into() })).await;
    assert!(app.snapshot().expect("snapshot").shots.iter().any(|shot| shot.format == "svg" && shot.shape == "ellipse"));
}

#[semio_framework_async_macros::async_test]
async fn set_active_shot_updates_fixture() {
    let mut app = shooting_app().await;
    let second_id = app.snapshot().expect("snapshot").shots.get(1).map(|shot| shot.id.clone()).expect("second shot");
    dispatch(&mut app, ShootingCommand::SetActiveShot(set_active_shot::SetActiveShot { shot_id: Some(second_id.clone()) })).await;
    assert_eq!(app.snapshot().expect("snapshot").active_shot_id, second_id);
}
