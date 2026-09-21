use super::*;
use crate::editor::animate::commands::engagement_input;
use crate::editor::animate::unit_tests::context::{dispatch, presentation_app};
use crate::editor::animate::PresentationCommand;
use semio_framework_plugin::Effect;

#[semio_framework_async_macros::async_test]
async fn engagement_input_stores_draft_and_submit_parses_grid_pattern() {
    let mut app = presentation_app().await;
    dispatch(&mut app, PresentationCommand::EngagementInput(engagement_input::EngagementInput { value: "2x3".into() })).await;
    dispatch(&mut app, PresentationCommand::EngagementSubmit(EngagementSubmit { value: "2x3".into() })).await;
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len(), 6, "2x3 grid pattern seeds 6 tiles");
}

#[semio_framework_async_macros::async_test]
async fn engagement_submit_add_clear_and_copy_keywords() {
    use semio_framework_plugin::artifact_app_laws::meta;
    let mut app = presentation_app().await;
    dispatch(&mut app, PresentationCommand::EngagementSubmit(EngagementSubmit { value: "add".into() })).await;
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len(), 1);

    dispatch(&mut app, PresentationCommand::EngagementSubmit(EngagementSubmit { value: "clear".into() })).await;
    assert!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.is_empty());

    crate::editor::animate::unit_tests::context::dispatch(&mut app, PresentationCommand::AddTile(crate::editor::animate::commands::add_tile::AddTile { crop: None })).await;
    let copy_result = crate::editor::animate::unit_tests::context::dispatch(&mut app, PresentationCommand::EngagementSubmit(EngagementSubmit { value: "copy prompt".into() })).await;
    assert!(matches!(copy_result.requested_effects.as_slice(), [Effect::DownloadMediaExport { .. }]));
}

#[semio_framework_async_macros::async_test]
async fn engagement_submit_unrecognized_input_is_a_no_op() {
    use semio_framework_plugin::artifact_app_laws::meta;
    let mut app = presentation_app().await;
    let result = crate::editor::animate::unit_tests::context::dispatch(&mut app, PresentationCommand::EngagementSubmit(EngagementSubmit { value: "gibberish".into() })).await;
    assert!(result.mutations.is_empty());
    assert!(result.requested_effects.is_empty());
}
