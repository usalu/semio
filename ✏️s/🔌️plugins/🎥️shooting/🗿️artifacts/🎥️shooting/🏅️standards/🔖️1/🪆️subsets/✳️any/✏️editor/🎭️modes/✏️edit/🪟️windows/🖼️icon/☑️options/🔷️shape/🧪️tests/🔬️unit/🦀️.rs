
use super::*;
use crate::editor::shooting::config::ShootingConfig;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn shape_measure_offers_rectangle_and_ellipse() {
    let snapshot = crate::schema::default_snapshot();
    let labels = shooting_play_labels(&ShootingConfig::default());
    match measure(&snapshot, labels) {
        WindowMeasure::Select { items, value, .. } => {
            assert_eq!(items.len(), 2);
            assert_eq!(value, "rectangle");
        }
        other => panic!("shape measure must be a select, got {other:?}"),
    }
}
