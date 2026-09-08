//! 🔳️ Main-window option — the canvas grid group (visibility toggle, snap toggle, factor slider).
//! Its command handlers live in `🎮️commands/👁️set-grid-visible`.

use crate::editor::flow::config::FlowConfig;
use crate::editor::flow::terminology::FlowPlayLabels;
use crate::editor::flow::FLOW_PLAY_APP_ID;
use semio_framework_plugin::{ActionDescriptor, WindowMeasure};

//#region 🔖️Measure
/// 🎯️ An `ActionDescriptor` addressed at the flow play app for one of this group's grid measures.
fn grid_action(action: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: FLOW_PLAY_APP_ID.into(), action: action.into(), args: None }
}

pub fn measure(config: &FlowConfig, labels: &FlowPlayLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "flow-play-measures.grid".into(),
        label: labels.grid.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Toggle { id: "flow-play-measures.grid-visible".into(), icon_id: "layout-grid".into(), label: Some(labels.grid_visible.into()), pressed: config.grid_visible, text: None, on_change: grid_action("setGridVisible") },
            WindowMeasure::Toggle { id: "flow-play-measures.grid-snap".into(), icon_id: "magnet".into(), label: Some(labels.grid_snap.into()), pressed: config.grid_snap_enabled, text: None, on_change: grid_action("setGridSnapEnabled") },
            WindowMeasure::Slider {
                id: "flow-play-measures.grid-factor".into(),
                label: Some(format!("{} {:.1}", labels.grid_factor.as_str(), config.grid_factor)),
                value: config.grid_factor,
                min: 0.5,
                max: 50.0,
                step: Some(0.5),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: grid_action("setGridFactor"),
            },
        ],
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
