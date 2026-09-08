
use super::*;
use crate::editor::flow::terminology::flow_play_labels;

/// 🔳️ The factor slider's `min`/`max` are the contract `🎮️commands/👁️set-grid-visible`'s handler clamps to —
/// pinned here so the two can't drift apart.
#[semio_framework_async_macros::async_test]
async fn the_factor_slider_range_matches_the_command_handler_clamp() {
    let config = FlowConfig::default();
    match measure(&config, flow_play_labels(&semio_framework_plugin::ViewModel::default())) {
        WindowMeasure::Group { children, .. } => {
            let slider = children.iter().find(|child| matches!(child, WindowMeasure::Slider { id, .. } if id == "flow-play-measures.grid-factor")).expect("factor slider");
            assert!(matches!(slider, WindowMeasure::Slider { min, max, .. } if *min == 0.5 && *max == 50.0));
            assert_eq!(children.len(), 3, "visibility, snap, factor");
        }
        other => panic!("grid measure must be a group, got {other:?}"),
    }
}
