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
