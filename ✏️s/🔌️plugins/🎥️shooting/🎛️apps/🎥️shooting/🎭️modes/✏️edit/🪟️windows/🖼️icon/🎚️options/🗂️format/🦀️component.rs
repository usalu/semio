//! 🗂️ Icon-window option — the active shot's export-format select.
//! Its command handler lives in `🎮️commands/📷️shot::set_active_shot_format`.

use crate::apps::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingFixture;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Measure
pub fn measure(fixture: &ShootingFixture, labels: &ShootingLabels) -> WindowMeasure {
    let shot = crate::artifacts::shooting::engine::active_shot(fixture);
    WindowMeasure::Select {
        id: "shooting.measure.format".into(),
        label: Some(labels.format_select_label.into()),
        value: shot.map_or_else(|| "svg".into(), |entry| entry.format.clone()),
        items: vec![
            MeasureSelectItem { id: "shooting.measure.format.svg".into(), value: "svg".into(), label: labels.format_svg.into() },
            MeasureSelectItem { id: "shooting.measure.format.png".into(), value: "png".into(), label: labels.format_png.into() },
        ],
        on_change: crate::apps::shooting::shooting_action("setActiveShotFormat", None),
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
    fn format_measure_offers_svg_and_png() {
        let fixture = crate::artifacts::shooting::engine::default_fixture();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&fixture, labels) {
            WindowMeasure::Select { items, value, .. } => {
                assert_eq!(items.len(), 2);
                assert_eq!(value, "svg");
            }
            other => panic!("format measure must be a select, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
