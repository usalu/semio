//! 🔭️ Main-window option — the level-of-detail select.
//! Its command handler lives in `🎮️commands/🔭️set-lod-mode`.

use crate::editor::flow::config::FlowConfig;
use crate::editor::flow::flow_action;
use crate::editor::flow::terminology::FlowPlayLabels;
use flow::{dag::dag_lod_scale_json, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};
use serde_json::{json, Value};

//#region 🔖️Measure
pub fn measure(config: &FlowConfig, labels: &FlowPlayLabels) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: FLOW_LOD_MODE_AUTOMATIC.into(), value: FLOW_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() }];
    items.extend(serde_json::from_str::<Vec<Value>>(&dag_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|lod| {
        let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
        let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select { id: "flow-play-measures.lod".into(), label: Some(labels.lod_mode.into()), value: config.lod_mode.clone(), items, on_change: flow_action("setLodMode", Some(json!({ "value": config.lod_mode }))) }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::terminology::flow_play_labels;

    #[semio_framework_async_macros::async_test]
    async fn the_select_always_offers_the_automatic_entry_first() {
        let config = FlowConfig::default();
        match measure(&config, flow_play_labels(&config)) {
            WindowMeasure::Select { items, value, .. } => {
                assert_eq!(items.first().expect("automatic entry").id, FLOW_LOD_MODE_AUTOMATIC);
                assert_eq!(value, FLOW_LOD_MODE_AUTOMATIC);
                assert!(items.len() > 1, "the real lod scale must be appended: {items:?}");
            }
            other => panic!("lod measure must be a select, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
