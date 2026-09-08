
use super::*;
use crate::editor::flow::terminology::flow_play_labels;

#[semio_framework_async_macros::async_test]
async fn the_select_always_offers_the_automatic_entry_first() {
    let config = FlowConfig::default();
    match measure(&config, flow_play_labels(&semio_framework_plugin::ViewModel::default())) {
        WindowMeasure::Select { items, value, .. } => {
            assert_eq!(items.first().expect("automatic entry").id, FLOW_LOD_MODE_AUTOMATIC);
            assert_eq!(value, FLOW_LOD_MODE_AUTOMATIC);
            assert!(items.len() > 1, "the real lod scale must be appended: {items:?}");
        }
        other => panic!("lod measure must be a select, got {other:?}"),
    }
}
