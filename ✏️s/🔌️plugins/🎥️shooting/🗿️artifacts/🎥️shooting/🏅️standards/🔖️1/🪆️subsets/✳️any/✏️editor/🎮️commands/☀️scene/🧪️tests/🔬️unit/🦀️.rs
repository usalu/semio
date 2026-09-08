
use super::*;
use crate::editor::shooting::ShootingCommand;
use crate::editor::shooting::testkit::{dispatch, shooting_app};

#[semio_framework_async_macros::async_test]
async fn scene_setters_mutate_lighting() {
    let mut app = shooting_app().await;
    dispatch(&mut app, ShootingCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 })).await;
    dispatch(&mut app, ShootingCommand::SetShadowEnabled(set_shadow_enabled::SetShadowEnabled { value: false })).await;
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.scene.sun.azimuth, 90.0);
    assert!(!snapshot.scene.shadow.enabled);
}

#[semio_framework_async_macros::async_test]
async fn toggle_sun_round_trips_through_ops_and_defaults_off() {
    let mut app = shooting_app().await;
    assert!(!app.snapshot().expect("snapshot").scene.sun.enabled);
    dispatch(&mut app, ShootingCommand::ToggleSun(toggle_sun::ToggleSun { value: true })).await;
    assert!(app.snapshot().expect("snapshot").scene.sun.enabled);
}
