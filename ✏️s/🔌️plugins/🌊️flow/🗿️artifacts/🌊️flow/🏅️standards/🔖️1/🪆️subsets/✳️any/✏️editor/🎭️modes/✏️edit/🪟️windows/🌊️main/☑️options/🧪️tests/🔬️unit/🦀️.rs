//! 🧪️ The measure laws of the main window options: the grid factor slider, the proximity slider and the level-of-detail select, one module per option.

mod grid {
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
}

mod lod {
    use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
    use flow::FLOW_LOD_MODE_AUTOMATIC;
    use semio_framework_plugin::WindowMeasure;
    #[allow(unused_imports)]
    use crate::editor::flow::terminology::FlowPlayLabels;
    #[allow(unused_imports)]
    use crate::editor::flow::base::FLOW_PLAY_APP_ID;
    #[allow(unused_imports)]
    use flow::dag::dag_lod_scale_json;
    #[allow(unused_imports)]
    use semio_framework_plugin::{ActionDescriptor, MeasureSelectItem};
    #[allow(unused_imports)]
    use serde_json::Value;
    use crate::editor::flow::modes::edit::windows::main::options::lod::*;
    use crate::editor::flow::terminology::flow_play_labels;

    #[semio_framework_async_macros::async_test]
    async fn the_select_always_offers_the_automatic_entry_first() {
        let config = FlowMainWindowConfig::default();
        match measure(&config, flow_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) {
            WindowMeasure::Select { items, value, .. } => {
                assert_eq!(items.first().expect("automatic entry").id, FLOW_LOD_MODE_AUTOMATIC);
                assert_eq!(value, FLOW_LOD_MODE_AUTOMATIC);
                assert!(items.len() > 1, "the real lod scale must be appended: {items:?}");
            }
            other => panic!("lod measure must be a select, got {other:?}"),
        }
    }
}

mod proximity {
    use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
    use semio_framework_plugin::WindowMeasure;
    #[allow(unused_imports)]
    use crate::editor::flow::terminology::FlowPlayLabels;
    #[allow(unused_imports)]
    use crate::editor::flow::base::FLOW_PLAY_APP_ID;
    #[allow(unused_imports)]
    use semio_framework_plugin::ActionDescriptor;
    use crate::editor::flow::modes::edit::windows::main::options::proximity::*;
    use crate::editor::flow::terminology::flow_play_labels;
    use crate::schema::FLOW_DEFAULT_PROXIMITY_DISTANCE;

    #[semio_framework_async_macros::async_test]
    async fn the_slider_range_brackets_the_default_distance() {
        let config = FlowMainWindowConfig::default();
        match measure(&config, flow_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) {
            WindowMeasure::Slider { value, min, max, .. } => {
                assert_eq!(value, FLOW_DEFAULT_PROXIMITY_DISTANCE);
                assert!(min <= value && value <= max, "default {value} must sit inside {min}..={max}");
            }
            other => panic!("proximity measure must be a slider, got {other:?}"),
        }
    }
}
