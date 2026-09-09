use super::*;
use crate::editor::animate::commands::engagement_input;
use crate::editor::animate::testkit::{dispatch, presentation_app};
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
    use semio_framework_plugin::testkit::meta;
    let mut app = presentation_app().await;
    dispatch(&mut app, PresentationCommand::EngagementSubmit(EngagementSubmit { value: "add".into() })).await;
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len(), 1);

    dispatch(&mut app, PresentationCommand::EngagementSubmit(EngagementSubmit { value: "clear".into() })).await;
    assert!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.is_empty());

    app.dispatch_typed(PresentationCommand::AddTile(crate::editor::animate::commands::add_tile::AddTile { crop: None }), &meta("local")).await.expect("seed for copy");
    let copy_result = app.dispatch_typed(PresentationCommand::EngagementSubmit(EngagementSubmit { value: "copy prompt".into() }), &meta("local")).await.expect("copy keyword");
    assert!(matches!(copy_result.requested_effects.as_slice(), [Effect::DownloadMediaExport { .. }]));
}

#[semio_framework_async_macros::async_test]
async fn engagement_submit_unrecognized_input_is_a_no_op() {
    use semio_framework_plugin::testkit::meta;
    let mut app = presentation_app().await;
    let result = app.dispatch_typed(PresentationCommand::EngagementSubmit(EngagementSubmit { value: "gibberish".into() }), &meta("local")).await.expect("unrecognized");
    assert!(result.mutations.is_empty());
    assert!(result.requested_effects.is_empty());
}
