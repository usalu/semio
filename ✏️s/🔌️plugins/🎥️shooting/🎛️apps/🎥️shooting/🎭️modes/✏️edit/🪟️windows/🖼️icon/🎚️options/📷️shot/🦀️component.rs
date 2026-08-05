//! 📷️ Icon-window option — the active-shot select.
//! Its command handler lives in `🎮️commands/📷️shot::set_active_shot`.

use crate::apps::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingFixture;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Measure
pub fn measure(fixture: &ShootingFixture, labels: &ShootingLabels) -> WindowMeasure {
    let shot = crate::artifacts::shooting::engine::active_shot(fixture);
    WindowMeasure::Select {
        id: "shooting.measure.shot".into(),
        label: Some(labels.shot.into()),
        value: shot.map(|entry| entry.id.clone()).unwrap_or_default(),
        items: fixture.shots.iter().map(|entry| MeasureSelectItem { id: format!("shooting.measure.shot.{}", entry.id), value: entry.id.clone(), label: entry.label.clone() }).collect(),
        on_change: crate::apps::shooting::shooting_action("setActiveShot", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::config::ShootingConfig;
    use crate::apps::shooting::terminology::shooting_play_labels;

    #[test]
    fn shot_measure_lists_every_shot() {
        let fixture = crate::artifacts::shooting::engine::default_fixture();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&fixture, labels) {
            WindowMeasure::Select { items, .. } => assert_eq!(items.len(), fixture.shots.len()),
            other => panic!("shot measure must be a select, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
