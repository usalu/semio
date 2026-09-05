//! 📏️ Main-window option — the proximity-select distance slider.
//! Its command handler lives in `🎮️commands/🔬️set-lod-mode`.

use crate::editor::flow::config::FlowConfig;
use crate::editor::flow::terminology::FlowPlayLabels;
use crate::editor::flow::FLOW_PLAY_APP_ID;
use semio_framework_plugin::{ActionDescriptor, WindowMeasure};

//#region 🔖️Measure
/// 🎯️ An `ActionDescriptor` addressed at the flow play app for this measure's distance slider —
/// `WindowMeasure` stays on the un-migrated chrome-level action type (mirrors the sibling `🌐️grid`
/// option's identical `grid_action`; `flow_action`'s new `ActionId`/`UiValue` result no longer fits
/// `WindowMeasure::Slider.on_change`'s `ActionDescriptor` field).
fn proximity_action(action: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: FLOW_PLAY_APP_ID.into(), action: action.into(), args: None }
}

pub fn measure(config: &FlowConfig, labels: &FlowPlayLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "flow-play-measures.proximity".into(),
        label: Some(labels.proximity_distance.into()),
        value: config.proximity_distance,
        min: 0.0,
        max: 240.0,
        step: Some(4.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: proximity_action("setProximityDistance"),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::flow::schema::FLOW_DEFAULT_PROXIMITY_DISTANCE;
    use crate::editor::flow::terminology::flow_play_labels;

    #[semio_framework_async_macros::async_test]
    async fn the_slider_range_brackets_the_default_distance() {
        let config = FlowConfig::default();
        match measure(&config, flow_play_labels(&config)) {
            WindowMeasure::Slider { value, min, max, .. } => {
                assert_eq!(value, FLOW_DEFAULT_PROXIMITY_DISTANCE);
                assert!(min <= value && value <= max, "default {value} must sit inside {min}..={max}");
            }
            other => panic!("proximity measure must be a slider, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
