
use super::*;
use crate::editor::shooting::config::ShootingConfig;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn center_model_toggle_starts_pressed() {
    let labels = shooting_play_labels(&ShootingConfig::default());
    match measure(labels) {
        WindowMeasure::Toggle { pressed, .. } => assert!(pressed),
        other => panic!("center-model measure must be a toggle, got {other:?}"),
    }
}
