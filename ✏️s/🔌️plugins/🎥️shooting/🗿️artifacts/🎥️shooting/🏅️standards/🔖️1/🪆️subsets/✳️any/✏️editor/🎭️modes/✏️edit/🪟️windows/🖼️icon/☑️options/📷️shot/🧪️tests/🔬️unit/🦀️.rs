
use super::*;
use crate::editor::shooting::config::ShootingConfig;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn shot_measure_lists_every_shot() {
    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let labels = shooting_play_labels(&ShootingConfig::default());
    match measure(&snapshot, labels) {
        WindowMeasure::Select { items, .. } => assert_eq!(items.len(), snapshot.shots.len()),
        other => panic!("shot measure must be a select, got {other:?}"),
    }
}
