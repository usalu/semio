//! 🧾 Directed port graph scene descriptors and fixture JSON helpers.

use serde::{Deserialize, Serialize};

pub use mathematical_graph::{CameraJson, NodeDescJson};
pub use mathematical_graph_port::HandleDescJson;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeDescJson {
    pub id: String,
    pub source: String,
    pub target: String,
    /// @emoji 🧩 Semantic edge-kind id for compatibility at `edge` specificity.
    #[serde(default)]
    pub edge_kind: Option<String>,
    /// @emoji 🔺 Per-instance source tip id from the edge tip registry (`none` disables).
    #[serde(default)]
    pub source_tip: Option<String>,
    /// @emoji 🔺 Per-instance target tip id from the edge tip registry (`none` disables).
    #[serde(default)]
    pub target_tip: Option<String>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default)]
    pub user_data: Option<serde_json::Value>,
    #[serde(default)]
    pub visible: Option<bool>,
}

/// @emoji 🧵 Transient cubic link from a handle to another handle or a free world point (descriptor + link gesture).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WireDescJson {
    pub id: String,
    pub source: String,
    /// @emoji 🧩 Semantic wire-kind id (defaults from catalog when omitted in fixtures).
    #[serde(default)]
    pub wire_kind: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub end_x: Option<f64>,
    #[serde(default)]
    pub end_y: Option<f64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default)]
    pub user_data: Option<serde_json::Value>,
    #[serde(default)]
    pub visible: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneDescriptorJson {
    pub nodes: Vec<NodeDescJson>,
    pub handles: Vec<HandleDescJson>,
    pub edges: Vec<EdgeDescJson>,
    #[serde(default)]
    pub wires: Vec<WireDescJson>,
    /// @emoji 💠 JS‑authored ids to paint with secondary “left selection” chrome (not in current `selected` flags).
    #[serde(default)]
    pub selection_exit_highlight_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FixtureV1Json {
    pub schema: String,
    pub camera: CameraJson,
    pub nodes: Vec<serde_json::Value>,
    pub edges: Vec<serde_json::Value>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
}

/// 🧾 Reads fixture edge endpoint handle ids from `source` and `target` string fields only.
pub fn fixture_edge_handle_ids_from_object(eo: &serde_json::Map<String, serde_json::Value>) -> Option<(&str, &str)> {
    let source = eo.get("source").and_then(|v| v.as_str())?;
    let target = eo.get("target").and_then(|v| v.as_str())?;
    Some((source, target))
}

fn board_json_hidden_flag(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
    obj.get("hidden").and_then(|v| v.as_bool())
}

pub fn board_json_visible_option(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
    match board_json_hidden_flag(obj) {
        Some(hidden) => Some(!hidden),
        None => obj.get("visible").and_then(|v| v.as_bool()),
    }
}

pub fn board_json_visible_or_true(obj: &serde_json::Map<String, serde_json::Value>) -> bool {
    board_json_visible_option(obj).unwrap_or(true)
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn normalize_board_descriptor_hidden_to_visible(value: &mut serde_json::Value) {
    let Some(root) = value.as_object_mut() else {
        return;
    };
    for key in ["nodes", "handles", "edges", "wires"] {
        let Some(rows) = root.get_mut(key).and_then(|v| v.as_array_mut()) else {
            continue;
        };
        for row in rows {
            let Some(obj) = row.as_object_mut() else {
                continue;
            };
            if let Some(visible) = board_json_visible_option(obj) {
                obj.insert("visible".into(), serde_json::json!(visible));
            }
        }
    }
}
