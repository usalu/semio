//! 🗂️ Icon-window option — the active shot's export-format select.
//! Its command handler lives in `🎮️commands/📷️shot::set_active_shot_format`.

use crate::ShootingSnapshot;
use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    let shot = crate::schema::active_shot(snapshot);
    WindowMeasure::Select {
        id: "shooting.measure.format".into(),
        label: Some(labels.format_select_label.into()),
        value: shot.map_or_else(|| "svg".into(), |entry| entry.format.clone()),
        items: vec![
            MeasureSelectItem { id: "shooting.measure.format.svg".into(), value: "svg".into(), label: labels.format_svg.into() },
            MeasureSelectItem { id: "shooting.measure.format.png".into(), value: "png".into(), label: labels.format_png.into() },
        ],
        on_change: crate::editor::shooting::shooting_window_action("setActiveShotFormat", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
