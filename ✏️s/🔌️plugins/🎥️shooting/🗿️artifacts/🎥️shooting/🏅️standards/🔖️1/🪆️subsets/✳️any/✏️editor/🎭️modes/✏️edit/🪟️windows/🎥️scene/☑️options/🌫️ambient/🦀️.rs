//! 🌫️ Scene-window option — the ambient-intensity slider.
//! Its command handler lives in `🎮️commands/☀️scene::set_ambient_intensity`.

use crate::editor::shooting::terminology::ShootingLabels;
use crate::ShootingSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "shooting.measure.ambient".into(),
        label: Some(labels.measure_ambient.into()),
        value: snapshot.scene.ambient.intensity,
        min: 0.0,
        max: 3.0,
        step: Some(0.05),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: crate::editor::shooting::shooting_window_action("setAmbientIntensity", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
