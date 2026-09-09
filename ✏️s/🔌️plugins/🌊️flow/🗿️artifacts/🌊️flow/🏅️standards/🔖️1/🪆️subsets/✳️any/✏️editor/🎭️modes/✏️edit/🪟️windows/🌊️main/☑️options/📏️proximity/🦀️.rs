//! 📏️ Main-window option — the proximity-select distance slider.
//! Its command handler lives in `🎮️commands/🔬️set-lod-mode`.

use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
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

pub fn measure(config: &FlowMainWindowConfig, labels: &FlowPlayLabels) -> WindowMeasure {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
