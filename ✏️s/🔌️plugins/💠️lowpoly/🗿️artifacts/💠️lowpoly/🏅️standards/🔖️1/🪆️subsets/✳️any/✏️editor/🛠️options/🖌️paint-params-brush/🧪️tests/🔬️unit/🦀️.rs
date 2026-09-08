
use super::*;

#[semio_framework_async_macros::async_test]
async fn measure_is_tagged_for_the_brush_utility() {
    let m = measure(&LowpolyConfig::default(), semio_framework_plugin::resolve_labels_for_locale::<LowpolyLabels>("en-US"));
    match m {
        WindowMeasure::Group { active_utility_id, .. } => assert_eq!(active_utility_id, Some("brush".to_string())),
        other => panic!("expected Group, got {other:?}"),
    }
}
