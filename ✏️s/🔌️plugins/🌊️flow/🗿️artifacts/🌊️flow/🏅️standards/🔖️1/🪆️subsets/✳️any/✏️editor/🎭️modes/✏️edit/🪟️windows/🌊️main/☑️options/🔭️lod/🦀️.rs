//! 🔭️ Main-window option — the level-of-detail select.
//! Its command handler lives in `🎮️commands/🔬️set-lod-mode`.

use crate::editor::flow::config::FlowConfig;
use crate::editor::flow::terminology::FlowPlayLabels;
use crate::editor::flow::FLOW_PLAY_APP_ID;
use flow::{dag::dag_lod_scale_json, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework::optional_json_to_dsl;
use semio_framework_plugin::{ActionDescriptor, MeasureSelectItem, WindowMeasure};
use serde_json::{json, Value};

//#region 🔖️Measure
pub fn measure(config: &FlowConfig, labels: &FlowPlayLabels) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: FLOW_LOD_MODE_AUTOMATIC.into(), value: FLOW_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() }];
    items.extend(serde_json::from_str::<Vec<Value>>(&dag_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|lod| {
        let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
        let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    let on_change = ActionDescriptor { controller_id: FLOW_PLAY_APP_ID.into(), action: "setLodMode".into(), args: optional_json_to_dsl(Some(json!({ "value": config.lod_mode }))) };
    WindowMeasure::Select { id: "flow-play-measures.lod".into(), label: Some(labels.lod_mode.into()), value: config.lod_mode.clone(), items, on_change }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
