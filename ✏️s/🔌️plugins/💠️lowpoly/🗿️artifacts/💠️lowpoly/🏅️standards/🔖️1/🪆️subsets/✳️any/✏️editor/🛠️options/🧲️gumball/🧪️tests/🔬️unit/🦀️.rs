use super::*;
use crate::editor::lowpoly::unit_tests::context::{action_meta, app, dispatch};
use crate::editor::lowpoly::LowpolyCommand;
use semio_framework_plugin::PluginApp;

/// 🎛️ Every handle group is on by default, and a `setUtilityParam` toggle switches exactly one off —
/// which the Model scene's `gumballConfig` and the window toggle both report.
#[semio_framework_async_macros::async_test]
async fn gumball_handles_default_on_and_toggle_off_through_the_utility_param() {
    let mut a = app().await;
    assert_eq!(GumballHandles::from_config(&LowpolyConfig::default()), GumballHandles { r#move: true, rotate: true, scale: true });
    dispatch(&mut a, LowpolyCommand::SetUtilityParam(crate::editor::lowpoly::commands::utility::set_utility_param::SetUtilityParam { key: GUMBALL_ROTATE_PARAM.into(), value_json: "false".into() })).await;
    let view_state = action_meta().view_state.expect("view state");
    let measures = a.window_measures(&view_state).await;
    let json = serde_json::to_string(&measures).expect("measures json");
    assert!(json.contains("lowpoly-gumball-rotate"), "the gumball group is a window measure: {json}");
    let main = crate::editor::lowpoly::unit_tests::context::render(&mut a, crate::editor::lowpoly::LOWPOLY_PLAY_BODY_MAIN).await;
    assert!(main.contains("\\\"rotate\\\":false") || main.contains("\"rotate\":false"), "the scene's gumballConfig turns the rotate handles off: {}", &main[..main.len().min(600)]);
    assert!(main.contains("\\\"moveAxes\\\":true") || main.contains("\"moveAxes\":true"), "move handles stay on");
}
