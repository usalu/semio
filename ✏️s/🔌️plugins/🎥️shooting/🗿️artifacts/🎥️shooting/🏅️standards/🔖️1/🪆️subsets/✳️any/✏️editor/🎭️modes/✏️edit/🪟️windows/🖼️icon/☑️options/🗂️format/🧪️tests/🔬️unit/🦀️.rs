use super::*;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn format_measure_offers_svg_and_png() {
    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let labels = shooting_play_labels(&semio_framework_plugin::ViewModel::default());
    match measure(&snapshot, labels) {
        WindowMeasure::Select { items, value, .. } => {
            assert_eq!(items.len(), 2);
            assert_eq!(value, "svg");
        }
        other => panic!("format measure must be a select, got {other:?}"),
    }
}
