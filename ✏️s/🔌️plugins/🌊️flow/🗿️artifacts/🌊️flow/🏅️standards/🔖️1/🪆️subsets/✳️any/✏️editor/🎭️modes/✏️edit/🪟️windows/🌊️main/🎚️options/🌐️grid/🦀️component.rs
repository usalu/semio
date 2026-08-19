//! 🔳️ Main-window option — the canvas grid group (visibility toggle, snap toggle, factor slider).
//! Its command handlers live in `🎮️commands/🌐️set-grid-visible`.

use crate::editor::flow::config::FlowConfig;
use crate::editor::flow::flow_action;
use crate::editor::flow::terminology::FlowPlayLabels;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub async fn measure(config: &FlowConfig, labels: &FlowPlayLabels) -> WindowMeasure {
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
            WindowMeasure::Toggle { id: "flow-play-measures.grid-visible".into(), icon_id: "layout-grid".into(), label: Some(labels.grid_visible.into()), pressed: config.grid_visible, text: None, on_change: flow_action("setGridVisible", None) },
            WindowMeasure::Toggle { id: "flow-play-measures.grid-snap".into(), icon_id: "magnet".into(), label: Some(labels.grid_snap.into()), pressed: config.grid_snap_enabled, text: None, on_change: flow_action("setGridSnapEnabled", None) },
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
                on_change: flow_action("setGridFactor", None),
            },
        ],
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::terminology::flow_play_labels;

    /// 🔳️ The factor slider's `min`/`max` are the contract `🎮️commands/🌐️set-grid-visible`'s handler clamps to —
    /// pinned here so the two can't drift apart.
    #[semio_framework_async_macros::async_test]
    async fn the_factor_slider_range_matches_the_command_handler_clamp() {
        let config = FlowConfig::default();
        match measure(&config, flow_play_labels(&config)) {
            WindowMeasure::Group { children, .. } => {
                let slider = children.iter().find(|child| matches!(child, WindowMeasure::Slider { id, .. } if id == "flow-play-measures.grid-factor")).expect("factor slider");
                assert!(matches!(slider, WindowMeasure::Slider { min, max, .. } if *min == 0.5 && *max == 50.0));
                assert_eq!(children.len(), 3, "visibility, snap, factor");
            }
            other => panic!("grid measure must be a group, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
