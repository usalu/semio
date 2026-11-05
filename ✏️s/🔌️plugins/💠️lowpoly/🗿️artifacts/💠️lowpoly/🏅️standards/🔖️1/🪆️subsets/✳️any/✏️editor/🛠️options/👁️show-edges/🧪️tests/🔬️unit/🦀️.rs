use super::*;

#[semio_framework_async_macros::async_test]
async fn measure_reflects_config_state() {
    let config = LowpolyConfig { show_edges: false, ..LowpolyConfig::default() };
    let m = measure(&config, semio_plugin::resolve_labels::<LowpolyLabels>(&semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)));
    assert!(matches!(m, WindowMeasure::Toggle { pressed: false, .. }));
}
