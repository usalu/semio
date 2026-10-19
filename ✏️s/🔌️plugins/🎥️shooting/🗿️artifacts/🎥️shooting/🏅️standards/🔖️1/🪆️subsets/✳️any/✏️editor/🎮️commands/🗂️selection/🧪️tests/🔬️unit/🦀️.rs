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
use crate::editor::shooting::commands::selection::*;
use crate::editor::shooting::unit_tests::context::{dispatch, shooting_app};
use crate::editor::shooting::ShootingCommand;

#[semio_framework_async_macros::async_test]
async fn set_shot_selection_is_config_only_and_selects_the_shot_in_the_inspector() {
    use crate::editor::shooting::unit_tests::context::render;
    use crate::editor::shooting::SHOOTING_PLAY_BODY_INSPECTION;

    let mut app = shooting_app().await;
    let shot_id = app.snapshot().expect("snapshot").shots.first().expect("fixture shot").id.clone();
    let result = dispatch(&mut app, ShootingCommand::SetShotSelection(set_shot_selection::SetShotSelection { shot_ids: vec![shot_id] })).await;
    assert!(!result.edited_document(), "shot selection is config-only");
    assert!(render(&mut app, SHOOTING_PLAY_BODY_INSPECTION).await.contains("shooting-play-inspector.shot"), "inspector renders the shot group for the selected shot");
}

#[semio_framework_async_macros::async_test]
async fn center_model_toggle_bumps_fit_revision_only_on_the_off_to_on_edge() {
    let mut app = shooting_app().await;
    dispatch(&mut app, ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: Some(false) })).await;
    dispatch(&mut app, ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: Some(true) })).await;
    let result = dispatch(&mut app, ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: None })).await;
    assert!(!result.edited_document(), "center-model is config-only");
}
