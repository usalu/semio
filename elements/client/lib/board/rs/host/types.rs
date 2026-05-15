//! 🧾 Serde shapes mirroring the React `BoardSceneDescriptor` + fixture v1 payloads.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CameraJson {
	pub x: f64,
	pub y: f64,
	pub zoom: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDescJson {
	pub id: String,
	pub x: f64,
	pub y: f64,
	#[serde(default)]
	pub draggable: Option<bool>,
	#[serde(default)]
	pub selected: Option<bool>,
	#[serde(default)]
	pub style: Option<String>,
	#[serde(default)]
	pub text: Option<String>,
	#[serde(default)]
	pub user_data: Option<serde_json::Value>,
	#[serde(default)]
	pub visible: Option<bool>,
	pub shape: Option<String>,
	#[serde(default)]
	pub radius: Option<f64>,
	#[serde(default)]
	pub width: Option<f64>,
	#[serde(default)]
	pub height: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleDescJson {
	pub id: String,
	pub node_id: String,
	pub angle: f64,
	#[serde(default)]
	pub radius: Option<f64>,
	#[serde(default)]
	pub selected: Option<bool>,
	#[serde(default)]
	pub style: Option<String>,
	#[serde(default)]
	pub user_data: Option<serde_json::Value>,
	#[serde(default)]
	pub visible: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeDescJson {
	pub id: String,
	pub from: String,
	pub to: String,
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
pub struct SceneDescriptorJson {
	pub nodes: Vec<NodeDescJson>,
	pub handles: Vec<HandleDescJson>,
	pub edges: Vec<EdgeDescJson>,
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
