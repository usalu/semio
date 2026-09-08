//! ✨️ Scene-window option — the material-roughness slider.
//! Its command handler lives in `🎮️commands/☀️scene::set_material_roughness`.

use crate::ShootingSnapshot;
use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "shooting.measure.roughness".into(),
        label: Some(labels.measure_roughness.into()),
        value: snapshot.scene.material.roughness,
        min: 0.0,
        max: 1.0,
        step: Some(0.05),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: crate::editor::shooting::shooting_window_action("setMaterialRoughness", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
