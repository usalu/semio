//! 📷️ Icon-window option — the active-shot select.
//! Its command handler lives in `🎮️commands/📷️shot::set_active_shot`.

use crate::editor::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    let shot = crate::artifacts::shooting::schema::active_shot(snapshot);
    WindowMeasure::Select {
        id: "shooting.measure.shot".into(),
        label: Some(labels.shot.into()),
        value: shot.map(|entry| entry.id.clone()).unwrap_or_default(),
        items: snapshot.shots.iter().map(|entry| MeasureSelectItem { id: format!("shooting.measure.shot.{}", entry.id), value: entry.id.clone(), label: entry.label.clone() }).collect(),
        on_change: crate::editor::shooting::shooting_action("setActiveShot", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::config::ShootingConfig;
    use crate::editor::shooting::terminology::shooting_play_labels;

    #[test]
    fn shot_measure_lists_every_shot() {
        let snapshot = crate::artifacts::shooting::schema::default_snapshot();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&snapshot, labels) {
            WindowMeasure::Select { items, .. } => assert_eq!(items.len(), snapshot.shots.len()),
            other => panic!("shot measure must be a select, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
