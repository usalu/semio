//! 🗂️ Icon-window option — the active shot's export-format select.
//! Its command handler lives in `🎮️commands/📷️shot::set_active_shot_format`.

use crate::editor::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Measure
pub async fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    let shot = crate::artifacts::shooting::schema::active_shot(snapshot);
    WindowMeasure::Select {
        id: "shooting.measure.format".into(),
        label: Some(labels.format_select_label.into()),
        value: shot.map_or_else(|| "svg".into(), |entry| entry.format.clone()),
        items: vec![
            MeasureSelectItem { id: "shooting.measure.format.svg".into(), value: "svg".into(), label: labels.format_svg.into() },
            MeasureSelectItem { id: "shooting.measure.format.png".into(), value: "png".into(), label: labels.format_png.into() },
        ],
        on_change: crate::editor::shooting::shooting_action("setActiveShotFormat", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::config::ShootingConfig;
    use crate::editor::shooting::terminology::shooting_play_labels;

    #[test]
    async fn format_measure_offers_svg_and_png() {
        let snapshot = crate::artifacts::shooting::schema::default_snapshot();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&snapshot, labels) {
            WindowMeasure::Select { items, value, .. } => {
                assert_eq!(items.len(), 2);
                assert_eq!(value, "svg");
            }
            other => panic!("format measure must be a select, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
