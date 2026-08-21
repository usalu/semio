//! 🧬️ Cad diff schema — sparse field delta over the artifact.

use crate::artifacts::cad::mutations::CadNodePatch;
use crate::artifacts::cad::schema::{CadComponentSelection, CadDislocateOptions};
use crate::artifacts::cad::{CadCamera, CadDrawingChild, CadModelChild, CadNode, CadReferenceList};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the cad artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::cad::schema::CadArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub shape_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub building_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub energy_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub structure_classic_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub drawings: Option<CadDrawingChildList>,
    #[state(artifact)]
    pub references_by_model_definition_id: Option<BTreeMap<String, CadReferenceList>>,
    #[state(artifact)]
    pub nodes: Option<CadNodesDelta>,
    #[state(artifact)]
    pub active_model_definition_id: Option<String>,
    #[state(presence)]
    pub selected_object_ids: Option<CadStringList>,
    #[state(presence)]
    pub selected_node_ids: Option<CadStringList>,
    #[state(presence)]
    pub active_object_id: Option<Option<String>>,
    #[state(presence)]
    pub component_selection: Option<CadComponentSelection>,
    #[state(presence)]
    pub selected_reference_model_definition_id: Option<Option<String>>,
    #[state(presence)]
    pub selected_reference_id: Option<Option<String>>,
    #[state(presence)]
    pub selected_primitive_id: Option<Option<String>>,
    #[state(presence)]
    pub selected_primitive_kind: Option<Option<String>>,
    #[state(presence)]
    pub active_utility_id: Option<String>,
    #[state(presence)]
    pub active_example_id: Option<Option<String>>,
    #[state(config)]
    pub selection_method: Option<String>,
    #[state(config)]
    pub engagement_input: Option<String>,
    #[state(config)]
    pub engagement_step: Option<String>,
    #[state(config)]
    pub engagement_pane: Option<Option<String>>,
    #[state(config)]
    pub engagement_session_json: Option<Option<String>>,
    #[state(config)]
    pub last_finalized_interaction_id: Option<Option<String>>,
    #[state(config)]
    pub sun_enabled: Option<bool>,
    #[state(config)]
    pub sun_azimuth: Option<f64>,
    #[state(config)]
    pub sun_elevation: Option<f64>,
    #[state(config)]
    pub sun_intensity: Option<f64>,
    #[state(config)]
    pub sun_color: Option<String>,
    #[state(config)]
    pub camera: Option<CadCamera>,
    #[state(config)]
    pub camera_building: Option<CadCamera>,
    #[state(config)]
    pub camera_energy: Option<CadCamera>,
    #[state(config)]
    pub camera_structure_classic: Option<CadCamera>,
    #[state(config)]
    pub dislocate_shape: Option<CadDislocateOptions>,
    #[state(config)]
    pub dislocate_building: Option<CadDislocateOptions>,
    #[state(config)]
    pub dislocate_energy: Option<CadDislocateOptions>,
    #[state(config)]
    pub dislocate_structure_classic: Option<CadDislocateOptions>,
    #[state(config)]
    pub locale: Option<String>,
    #[state(config)]
    pub terminology: Option<String>,
    #[state(config)]
    pub contributions_json: Option<String>,
    #[state(artifact)]
    pub hovered_object_id: Option<Option<String>>,
    #[state(artifact)]
    pub hovered_target_object_id: Option<Option<String>>,
    #[state(artifact)]
    pub hovered_target_mode: Option<Option<String>>,
    #[state(artifact)]
    pub hovered_target_id: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CadStringList {
    pub values: Vec<String>,
}

/// 🧩️ Whole-list wrapper for the `drawings` composed CHILD COLLECTION diff field — same `RunList`
/// shape `✳️text`/`✳️kit` use for their own Vec-of-child diff fields (kit's own
/// `SemioKitModelChildList` is the direct precedent for a `Vec<ArtifactChild<S>>` diff wrapper).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CadDrawingChildList {
    pub values: Vec<CadDrawingChild>,
}

/// 🧩 Identified-collection delta for nodes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CadNodesDelta {
    pub added: Vec<CadNode>,
    pub removed: Vec<String>,
    pub patched: Vec<CadNodePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched node entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadNodePatchEntry {
    pub id: String,
    pub patch: CadNodePatch,
}
//#endregion 🔖️DeltaHelpers
