//! ✨️ Scene-window option — the material-roughness slider.
//! Its command handler lives in `🎮️commands/☀️scene::set_material_roughness`.

use crate::artifacts::shooting::ShootingSnapshot;
use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub async fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
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
        on_change: crate::editor::shooting::shooting_action("setMaterialRoughness", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::config::ShootingConfig;
    use crate::editor::shooting::terminology::shooting_play_labels;

    #[semio_framework_async_macros::async_test]
    async fn roughness_measure_matches_the_fixture_default() {
        let snapshot = crate::artifacts::shooting::schema::default_snapshot();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&snapshot, labels) {
            WindowMeasure::Slider { value, .. } => assert_eq!(value, 1.0),
            other => panic!("roughness measure must be a slider, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
