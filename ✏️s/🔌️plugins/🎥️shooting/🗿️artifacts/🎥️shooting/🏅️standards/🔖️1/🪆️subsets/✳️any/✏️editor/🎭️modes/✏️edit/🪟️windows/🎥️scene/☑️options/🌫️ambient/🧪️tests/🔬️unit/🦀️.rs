use super::*;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn ambient_measure_matches_the_fixture_default() {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let labels = shooting_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    match measure(&snapshot, labels) {
        WindowMeasure::Slider { value, .. } => assert_eq!(value, 1.15),
        other => panic!("ambient measure must be a slider, got {other:?}"),
    }
}
