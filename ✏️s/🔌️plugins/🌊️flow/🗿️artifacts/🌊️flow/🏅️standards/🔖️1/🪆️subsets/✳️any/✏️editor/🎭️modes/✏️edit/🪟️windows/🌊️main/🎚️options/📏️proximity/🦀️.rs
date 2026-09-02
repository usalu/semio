//! 📏️ Main-window option — the proximity-select distance slider.
//! Its command handler lives in `🎮️commands/🔭️set-lod-mode`.

use crate::editor::flow::config::FlowConfig;
use crate::editor::flow::flow_action;
use crate::editor::flow::terminology::FlowPlayLabels;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
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
        on_change: flow_action("setProximityDistance", None),
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
