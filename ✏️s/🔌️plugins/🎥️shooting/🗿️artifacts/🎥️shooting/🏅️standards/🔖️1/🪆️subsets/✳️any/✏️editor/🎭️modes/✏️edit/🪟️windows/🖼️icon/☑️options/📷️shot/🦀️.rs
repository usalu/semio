//! 📷️ Icon-window option — the active-shot select.
//! Its command handler lives in `🎮️commands/📷️shot::set_active_shot`.

use crate::ShootingSnapshot;
use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    let shot = crate::standards::v1::subsets::any::schema::active_shot(snapshot);
    WindowMeasure::Select {
        id: "shooting.measure.shot".into(),
        label: Some(labels.shot.into()),
        value: shot.map(|entry| entry.id.clone()).unwrap_or_default(),
        items: snapshot.shots.iter().map(|entry| MeasureSelectItem { id: format!("shooting.measure.shot.{}", entry.id), value: entry.id.clone(), label: entry.label.clone() }).collect(),
        on_change: crate::editor::shooting::shooting_window_action("setActiveShot", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
