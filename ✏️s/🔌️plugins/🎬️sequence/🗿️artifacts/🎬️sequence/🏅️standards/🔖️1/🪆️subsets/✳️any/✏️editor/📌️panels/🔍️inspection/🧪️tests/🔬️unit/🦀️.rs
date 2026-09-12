use super::*;
use crate::editor::sequence::terminology::sequence_play_labels;
use crate::editor::sequence::unit_tests::context::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn inspection_shows_prompt_when_nothing_selected() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_INSPECTOR).await.contains("Select a step"));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactApp::render` carries no
/// `InteractionView` (only `handle`/`copy_fragment`/`cut_operations` gained one — see this
/// ticket's `w3b-summary.md`), so the live app can never feed this panel a real selection today —
/// `SequencePlayApp::render` always calls this with an empty slice, a documented framework gap
/// (the same one `space`'s node-graph canvas rendering and context menu carry). This exercises
/// `render`'s own selected-step branch directly instead of through the app's dispatch/render loop.
#[semio_framework_async_macros::async_test]
async fn inspection_shows_selected_step_kind() {
    let app = new_app().await;
    let fixture = app.snapshot().expect("projection").to_fixture();
    let labels = sequence_play_labels(&semio_framework_plugin::ViewModel::default());
    let node = render(&fixture, &["step-1".to_string()], labels).expect("inspector");
    let node = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire inspector");
    assert!(serde_json::to_string(&node).unwrap().contains("state.set"));
}
