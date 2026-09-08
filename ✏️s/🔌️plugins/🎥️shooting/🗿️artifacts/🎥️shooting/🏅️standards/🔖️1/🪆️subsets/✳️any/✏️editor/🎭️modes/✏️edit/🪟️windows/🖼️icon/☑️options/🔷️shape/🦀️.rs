//! 🔷️ Icon-window option — the active shot's clip-shape select.
//! Its command handler lives in `🎮️commands/📷️shot::set_active_shot_shape`.

use crate::ShootingSnapshot;
use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    let shot = crate::standards::v1::subsets::any::schema::active_shot(snapshot);
    WindowMeasure::Select {
        id: "shooting.measure.shape".into(),
        label: Some(labels.shape_select_label.into()),
        value: shot.map_or_else(|| "rectangle".into(), |entry| entry.shape.clone()),
        items: vec![
            MeasureSelectItem { id: "shooting.measure.shape.rectangle".into(), value: "rectangle".into(), label: labels.shape_rectangle.into() },
            MeasureSelectItem { id: "shooting.measure.shape.ellipse".into(), value: "ellipse".into(), label: labels.shape_ellipse.into() },
        ],
        on_change: crate::editor::shooting::shooting_window_action("setActiveShotShape", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
