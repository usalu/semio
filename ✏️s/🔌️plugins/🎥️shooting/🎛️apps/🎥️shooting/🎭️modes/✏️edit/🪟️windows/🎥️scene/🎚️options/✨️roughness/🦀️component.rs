//! ✨️ Scene-window option — the material-roughness slider.
//! Its command handler lives in `🎮️commands/☀️scene::set_material_roughness`.

use crate::apps::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingSnapshot;
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
        on_change: crate::apps::shooting::shooting_action("setMaterialRoughness", None),
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
    fn roughness_measure_matches_the_fixture_default() {
        let snapshot = crate::artifacts::shooting::engine::default_snapshot();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&snapshot, labels) {
            WindowMeasure::Slider { value, .. } => assert_eq!(value, 1.0),
            other => panic!("roughness measure must be a slider, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
