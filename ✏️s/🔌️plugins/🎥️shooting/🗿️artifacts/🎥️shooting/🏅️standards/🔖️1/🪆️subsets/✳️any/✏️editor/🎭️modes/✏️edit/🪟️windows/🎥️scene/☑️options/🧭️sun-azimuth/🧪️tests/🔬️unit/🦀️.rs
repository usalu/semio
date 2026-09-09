use super::*;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn sun_azimuth_measure_spans_a_full_turn() {
    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let labels = shooting_play_labels(&semio_framework_plugin::ViewModel::default());
    match measure(&snapshot, labels) {
        WindowMeasure::Slider { min, max, .. } => assert_eq!((min, max), (0.0, 360.0)),
        other => panic!("sun-azimuth measure must be a slider, got {other:?}"),
    }
}
