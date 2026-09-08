//! 🧭️ Scene-window option — the sun azimuth slider.
//! Its command handler lives in `🎮️commands/☀️scene::set_sun_azimuth`.

use crate::ShootingSnapshot;
use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "shooting.measure.sun-azimuth".into(),
        label: Some(labels.measure_sun_azimuth.into()),
        value: snapshot.scene.sun.azimuth,
        min: 0.0,
        max: 360.0,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: crate::editor::shooting::shooting_window_action("setSunAzimuth", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
