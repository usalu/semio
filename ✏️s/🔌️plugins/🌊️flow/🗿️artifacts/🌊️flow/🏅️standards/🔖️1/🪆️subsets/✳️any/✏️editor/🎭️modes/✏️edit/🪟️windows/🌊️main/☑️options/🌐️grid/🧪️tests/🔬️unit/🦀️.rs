use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use crate::editor::flow::terminology::FlowPlayLabels;
#[allow(unused_imports)]
use crate::editor::flow::base::FLOW_PLAY_APP_ID;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDescriptor;
use crate::editor::flow::modes::edit::windows::main::options::grid::*;
use crate::editor::flow::terminology::flow_play_labels;

/// 🔳️ The factor slider's `min`/`max` are the contract `🎮️commands/👁️set-grid-visible`'s handler clamps to —
/// pinned here so the two can't drift apart.
#[semio_framework_async_macros::async_test]
async fn the_factor_slider_range_matches_the_command_handler_clamp() {
    let config = FlowMainWindowConfig::default();
    match measure(&config, flow_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) {
        WindowMeasure::Group { children, .. } => {
            let slider = children.iter().find(|child| matches!(child, WindowMeasure::Slider { id, .. } if id == "flow-play-measures.grid-factor")).expect("factor slider");
            assert!(matches!(slider, WindowMeasure::Slider { min, max, .. } if *min == 0.5 && *max == 50.0));
            assert_eq!(children.len(), 3, "visibility, snap, factor");
        }
        other => panic!("grid measure must be a group, got {other:?}"),
    }
}
