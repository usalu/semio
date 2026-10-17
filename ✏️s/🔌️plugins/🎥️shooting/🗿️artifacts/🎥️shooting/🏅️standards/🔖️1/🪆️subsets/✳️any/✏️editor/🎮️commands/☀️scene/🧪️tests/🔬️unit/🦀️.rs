#[allow(unused_imports)]
use crate::{editor::shooting::config::ShootingConfig, editor::shooting::config::ShootingConfigMutation};
#[allow(unused_imports)]
use crate::editor::shooting::ui_helpers_and_io::ShootingDispatchCtx;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;
#[allow(unused_imports)]
use crate::ShootingSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::shooting::commands::scene::*;
use crate::editor::shooting::unit_tests::context::{dispatch, shooting_app};
use crate::editor::shooting::ShootingCommand;

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
