use super::*;

#[semio_framework_async_macros::async_test]
async fn measure_builds_the_snap_grid_slider() {
    let config = LowpolyConfig::default();
    let m = measure(&config, semio_framework_plugin::resolve_labels::<LowpolyLabels>(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)));
    match m {
        WindowMeasure::Group { children, .. } => assert_eq!(children.len(), 1),
        other => panic!("expected Group, got {other:?}"),
    }
}
