//! ☀️ Scene-window option — the sun-enabled toggle.
//! Its command handler lives in `🎮️commands/☀️scene::toggle_sun`.

use crate::editor::shooting::terminology::ShootingLabels;
use crate::ShootingSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Toggle {
        id: "shooting.measure.sun-enabled".into(),
        icon_id: "sun".into(),
        label: Some(labels.measure_sun.into()),
        pressed: snapshot.scene.sun.enabled,
        text: None,
        on_change: crate::editor::shooting::shooting_window_action("toggleSun", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
