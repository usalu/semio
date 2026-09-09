use super::*;

#[semio_framework_async_macros::async_test]
async fn measure_builds_a_group() {
    let config = LowpolyConfig::default();
    assert!(matches!(measure(&config, semio_framework_plugin::resolve_labels::<LowpolyLabels>(&semio_framework_plugin::ViewModel::default())), WindowMeasure::Group { .. }));
}
