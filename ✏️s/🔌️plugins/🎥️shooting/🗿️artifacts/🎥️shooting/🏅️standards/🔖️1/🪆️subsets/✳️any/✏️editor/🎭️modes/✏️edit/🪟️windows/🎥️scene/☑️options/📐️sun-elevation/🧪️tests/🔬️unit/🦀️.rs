use super::*;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn sun_elevation_measure_allows_below_horizon() {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let labels = shooting_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    match measure(&snapshot, labels) {
        WindowMeasure::Slider { min, max, .. } => assert_eq!((min, max), (-10.0, 90.0)),
        other => panic!("sun-elevation measure must be a slider, got {other:?}"),
    }
}
