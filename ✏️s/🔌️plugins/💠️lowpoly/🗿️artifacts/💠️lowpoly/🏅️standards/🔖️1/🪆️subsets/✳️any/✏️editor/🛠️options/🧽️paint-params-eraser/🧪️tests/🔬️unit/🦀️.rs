use super::*;

#[semio_framework_async_macros::async_test]
async fn measure_is_tagged_for_the_eraser_utility() {
    let m = measure(&LowpolyConfig::default(), semio_framework_plugin::resolve_labels::<LowpolyLabels>(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)));
    match m {
        WindowMeasure::Group { active_utility_id, .. } => assert_eq!(active_utility_id, Some("eraser".to_string())),
        other => panic!("expected Group, got {other:?}"),
    }
}
