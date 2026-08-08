//! 🔷️ Icon-window option — the active shot's clip-shape select.
//! Its command handler lives in `🎮️commands/📷️shot::set_active_shot_shape`.

use crate::apps::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    let shot = crate::artifacts::shooting::engine::active_shot(snapshot);
    WindowMeasure::Select {
        id: "shooting.measure.shape".into(),
        label: Some(labels.shape_select_label.into()),
        value: shot.map_or_else(|| "rectangle".into(), |entry| entry.shape.clone()),
        items: vec![
            MeasureSelectItem { id: "shooting.measure.shape.rectangle".into(), value: "rectangle".into(), label: labels.shape_rectangle.into() },
            MeasureSelectItem { id: "shooting.measure.shape.ellipse".into(), value: "ellipse".into(), label: labels.shape_ellipse.into() },
        ],
        on_change: crate::apps::shooting::shooting_action("setActiveShotShape", None),
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
    fn shape_measure_offers_rectangle_and_ellipse() {
        let snapshot = crate::artifacts::shooting::engine::default_snapshot();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&snapshot, labels) {
            WindowMeasure::Select { items, value, .. } => {
                assert_eq!(items.len(), 2);
                assert_eq!(value, "rectangle");
            }
            other => panic!("shape measure must be a select, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
