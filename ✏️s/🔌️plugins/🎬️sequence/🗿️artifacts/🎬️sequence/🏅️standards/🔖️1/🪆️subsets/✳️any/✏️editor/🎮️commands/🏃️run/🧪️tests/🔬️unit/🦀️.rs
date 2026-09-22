use crate::editor::sequence::unit_tests::context::{dispatch_in_script, new_app, render_in, script_window_meta, SequenceApp};
use crate::editor::sequence::SequenceCommand;

use super::run_command::Run;
use super::stop_command::Stop;

/// 📦️ The script window is a `SurfaceKind::TextEditor`, so its buffer rides inside the surface
/// node's PACKED `TextEditorScene` (`SurfaceProps.doc` bytes) and never appears as text in the
/// projected tree — a `render_in(..).contains("run result")` reads the byte array, not the buffer.
/// The last run result is therefore read the same way the window-ownership law reads it:
/// `project_and_retire_fixture_tree` (inside `render_in`) → `decode_fixture_scene::<TextEditorScene>`.
async fn script_buffer(app: &mut SequenceApp, view: &semio_framework_plugin::ViewModel) -> String {
    let projection = render_in(app, crate::editor::sequence::modes::edit::windows::script::SEQUENCE_PLAY_BODY_SCRIPT, view).await;
    let scene: semio_framework_plugin::TextEditorScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&projection).expect("packed script scene");
    scene.buffer
}

/// 🏃️ `run`/`stop` publish the last run result as the SCRIPT window's own window transient, so both
/// the dispatch and the read have to be addressed at that window instance — an unaddressed dispatch
/// is refused with `sequence-script-window-view-required`, and an unaddressed render sees no
/// transient at all.
#[semio_framework_async_macros::async_test]
async fn run_stores_result_and_renders_in_script() {
    let mut app = new_app().await;
    dispatch_in_script(&mut app, SequenceCommand::Run(Run {})).await;
    let view = script_window_meta().view_state.expect("script window view");
    assert!(script_buffer(&mut app, &view).await.contains("run result"));
}

#[semio_framework_async_macros::async_test]
async fn stop_command_clears_last_run_result() {
    let mut app = new_app().await;
    dispatch_in_script(&mut app, SequenceCommand::Run(Run {})).await;
    dispatch_in_script(&mut app, SequenceCommand::Stop(Stop {})).await;
    let view = script_window_meta().view_state.expect("script window view");
    assert!(!script_buffer(&mut app, &view).await.contains("run result"));
}
