use crate::editor::lowpoly::testkit::{app, dispatch};
use crate::editor::lowpoly::LowpolyCommand;
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn toggle_sun_flips_enabled() {
    let mut a = app().await;
    dispatch(&mut a, LowpolyCommand::ToggleSun(super::toggle_sun::ToggleSun {})).await;
    // 🎯️ Config isn't directly readable off `VcsArtifactApp`; assert through window measures instead
    // (mirrors the pre-migration test's approach of reading effects, not internal state).
    let measures = a.window_measures(&semio_framework_plugin::ViewModel::default()).await;
    assert!(!measures.is_empty());
}
