
use super::*;
use crate::editor::shooting::config::ShootingConfig;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn roughness_measure_matches_the_fixture_default() {
    let snapshot = crate::schema::default_snapshot();
    let labels = shooting_play_labels(&ShootingConfig::default());
    match measure(&snapshot, labels) {
        WindowMeasure::Slider { value, .. } => assert_eq!(value, 1.0),
        other => panic!("roughness measure must be a slider, got {other:?}"),
    }
}
