extern crate semio_framework_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use semio_s_artifact_lowpoly_lowpoly_options::gumball::*;
use semio_s_artifact_lowpoly_lowpoly_config::LowpolyConfig;
use semio_s_artifact_lowpoly_lowpoly_kit::lowpoly_window_action;
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::terminology::LowpolyLabels;
use semio_s_artifact_lowpoly_lowpoly_view::{utility_param_bool, utility_params_value, GUMBALL_MOVE_PARAM, GUMBALL_ROTATE_PARAM, GUMBALL_SCALE_PARAM};
use semio_framework_ui_locale::LabelText;
use semio_framework_plugin::WindowMeasure;
use crate::context::{action_meta, app, dispatch};
use semio_s_artifact_lowpoly_lowpoly_editor::LowpolyCommand;
use semio_framework_plugin::PluginApp;

/// 🎛️ Every handle group is on by default, and a `setUtilityParam` toggle switches exactly one off —
/// which the Model scene's `gumballConfig` and the window toggle both report.
#[semio_framework_async_macros::async_test]
async fn gumball_handles_default_on_and_toggle_off_through_the_utility_param() {
    let mut a = app().await;
    assert_eq!(GumballHandles::from_config(&LowpolyConfig::default()), GumballHandles { r#move: true, rotate: true, scale: true });
    dispatch(&mut a, LowpolyCommand::SetUtilityParam(semio_s_artifact_lowpoly_lowpoly_commands::utility::set_utility_param::SetUtilityParam { key: GUMBALL_ROTATE_PARAM.into(), value_json: "false".into() })).await;
    let view_state = action_meta().view_state.expect("view state");
    let measures = a.window_measures(&view_state).await;
    let json = serde_json::to_string(&measures).expect("measures json");
    assert!(json.contains("lowpoly-gumball-rotate"), "the gumball group is a window measure: {json}");
    let main = crate::context::render(&mut a, semio_s_artifact_lowpoly_lowpoly_modes::windows::model::LOWPOLY_PLAY_BODY_MAIN).await;
    assert!(main.contains("\\\"rotate\\\":false") || main.contains("\"rotate\":false"), "the scene's gumballConfig turns the rotate handles off: {}", &main[..main.len().min(600)]);
    assert!(main.contains("\\\"moveAxes\\\":true") || main.contains("\"moveAxes\":true"), "move handles stay on");
}
