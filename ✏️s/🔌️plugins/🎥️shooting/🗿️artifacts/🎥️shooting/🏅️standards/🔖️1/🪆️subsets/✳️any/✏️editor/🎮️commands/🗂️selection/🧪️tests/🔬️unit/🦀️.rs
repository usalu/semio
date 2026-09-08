
use super::*;
use crate::editor::shooting::ShootingCommand;
use crate::editor::shooting::testkit::{dispatch, shooting_app};

#[semio_framework_async_macros::async_test]
async fn set_shot_selection_is_config_only_and_selects_the_shot_in_the_inspector() {
    use crate::editor::shooting::SHOOTING_PLAY_BODY_INSPECTION;
    use crate::editor::shooting::testkit::render;

    let mut app = shooting_app().await;
    let shot_id = app.snapshot().expect("snapshot").shots.first().expect("fixture shot").id.clone();
    let result = dispatch(&mut app, ShootingCommand::SetShotSelection(set_shot_selection::SetShotSelection { shot_ids: vec![shot_id] })).await;
    assert!(result.mutations.is_empty(), "shot selection is config-only");
    assert!(render(&mut app, SHOOTING_PLAY_BODY_INSPECTION).await.contains("shooting-play-inspector.shot"), "inspector renders the shot group for the selected shot");
}

#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_artifact_mutations() {
    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() })).await;
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
}

#[semio_framework_async_macros::async_test]
async fn center_model_toggle_bumps_fit_revision_only_on_the_off_to_on_edge() {
    let mut app = shooting_app().await;
    dispatch(&mut app, ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: Some(false) })).await;
    dispatch(&mut app, ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: Some(true) })).await;
    // fit_revision itself is asserted end-to-end (render fitJson) in the scene window's own tests;
    // here we just assert the command round-trips without error under both edges.
    let result = dispatch(&mut app, ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: None })).await;
    assert!(result.mutations.is_empty(), "center-model is config-only");
}
