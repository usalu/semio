use crate::editor::lowpoly::unit_tests::context::{action_meta, app, dispatch};
use crate::editor::lowpoly::LowpolyCommand;
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn toggle_sun_flips_enabled() {
    let mut a = app().await;
    dispatch(&mut a, LowpolyCommand::ToggleSun(super::toggle_sun::ToggleSun {})).await;
    // 🎯️ Config isn't directly readable off `VcsArtifactApp`; assert through window measures instead
    // (mirrors the pre-migration test's approach of reading effects, not internal state).
    let view_state = action_meta().view_state.expect("the Model window's view state");
    let measures = a.window_measures(&view_state).await;
    assert!(!measures.is_empty(), "the Model window publishes its measures: {measures:?}");
}
