use super::*;

#[semio_framework_async_macros::async_test]
async fn measure_builds_a_group() {
    let config = LowpolyConfig::default();
    assert!(matches!(measure(&config, semio_plugin::resolve_labels::<LowpolyLabels>(&semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native))), WindowMeasure::Group { .. }));
}
