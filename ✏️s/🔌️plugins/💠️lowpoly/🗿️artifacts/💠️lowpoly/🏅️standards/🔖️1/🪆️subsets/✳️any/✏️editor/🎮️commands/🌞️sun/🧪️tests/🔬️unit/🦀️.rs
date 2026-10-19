extern crate semio_framework_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use crate::context::{action_meta, app, dispatch};
use semio_s_artifact_lowpoly_lowpoly_editor::LowpolyCommand;
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn toggle_sun_flips_enabled() {
    let mut a = app().await;
    dispatch(&mut a, LowpolyCommand::ToggleSun(semio_s_artifact_lowpoly_lowpoly_commands::sun::toggle_sun::ToggleSun {})).await;
    let view_state = action_meta().view_state.expect("the Model window's view state");
    let measures = a.window_measures(&view_state).await;
    assert!(!measures.is_empty(), "the Model window publishes its measures: {measures:?}");
}
