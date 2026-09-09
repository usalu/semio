use super::*;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn sun_enabled_measure_mirrors_the_fixture_default_off() {
    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let labels = shooting_play_labels(&semio_framework_plugin::ViewModel::default());
    match measure(&snapshot, labels) {
        WindowMeasure::Toggle { pressed, .. } => assert!(!pressed),
        other => panic!("sun-enabled measure must be a toggle, got {other:?}"),
    }
}
