//! 📐️ Scene-window option — the sun elevation slider.
//! Its command handler lives in `🎮️commands/☀️scene::set_sun_elevation`.

use crate::editor::shooting::terminology::ShootingLabels;
use crate::ShootingSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "shooting.measure.sun-elevation".into(),
        label: Some(labels.measure_sun_elevation.into()),
        value: snapshot.scene.sun.elevation,
        min: -10.0,
        max: 90.0,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: crate::editor::shooting::shooting_window_action("setSunElevation", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
