
use super::*;
use crate::editor::shooting::config::ShootingConfig;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn shadow_measure_starts_pressed_by_default() {
    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let labels = shooting_play_labels(&ShootingConfig::default());
    match measure(&snapshot, labels) {
        WindowMeasure::Toggle { pressed, .. } => assert!(pressed),
        other => panic!("shadow measure must be a toggle, got {other:?}"),
    }
}
