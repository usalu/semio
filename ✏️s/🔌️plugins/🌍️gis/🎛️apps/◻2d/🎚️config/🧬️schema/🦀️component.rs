//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gis2d.config")]
pub struct Gis2dConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub layer_visibility: BTreeMap<String, bool>,
    #[state(local_ui)] pub camera_json: String,
    #[state(local_ui)] pub render_mode: String,
    #[state(local_ui)] pub vector_style: String,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub feature_selection_json: String,
    #[state(local_ui)] pub hover_json: String,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub selection_mode: String,
    #[state(local_ui)] pub layer_stroke_scale: BTreeMap<String, f64>,
    #[state(local_ui)] pub locale: String,
}

