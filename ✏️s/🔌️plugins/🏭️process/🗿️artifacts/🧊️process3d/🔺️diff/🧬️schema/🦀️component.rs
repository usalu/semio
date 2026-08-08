//! 🧬️ Process3d diff schema — sparse field delta over the artifact.

use crate::artifacts::process3d::{ProcessStep, ProcessStepPatch, Stock, Workshop};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the process3d artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.process.process3d")]
pub struct Process3dDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::process3d::schema::Process3dArtifact>>,
    #[state(persistent)] pub workshop: Option<Workshop>,
    #[state(persistent)] pub stock: Option<Stock>,
    #[state(persistent)] pub steps: Option<Process3dStepsDelta>,
    #[state(persistent)] pub resolved_up_to: Option<Option<usize>>,
    #[state(shared_ui)] pub selected_id: Option<Option<String>>,
    #[state(shared_ui)] pub selected_face_id: Option<Option<usize>>,
    #[state(shared_ui)] pub active_utility_id: Option<String>,
    #[state(local_ui)] pub selection_method: Option<String>,
    #[state(local_ui)] pub engagement_input: Option<String>,
    #[state(local_ui)] pub camera_position_x: Option<f64>,
    #[state(local_ui)] pub camera_position_y: Option<f64>,
    #[state(local_ui)] pub camera_position_z: Option<f64>,
    #[state(local_ui)] pub camera_target_x: Option<f64>,
    #[state(local_ui)] pub camera_target_y: Option<f64>,
    #[state(local_ui)] pub camera_target_z: Option<f64>,
    #[state(local_ui)] pub camera_fov: Option<f64>,
    #[state(local_ui)] pub sun_enabled: Option<bool>,
    #[state(local_ui)] pub sun_azimuth: Option<f64>,
    #[state(local_ui)] pub sun_elevation: Option<f64>,
    #[state(local_ui)] pub sun_intensity: Option<f64>,
    #[state(local_ui)] pub sun_color: Option<String>,
    #[state(local_ui)] pub locale: Option<String>,
    #[state(local_ui)] pub contributions_json: Option<String>,
    #[state(preview)] pub hovered_id: Option<Option<String>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧩 Identified-collection delta for `steps`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Process3dStepsDelta {
    pub added: Vec<ProcessStep>,
    pub removed: Vec<String>,
    pub patched: Vec<Process3dStepPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `steps` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process3dStepPatchEntry {
    pub id: String,
    pub patch: ProcessStepPatch,
}
//#endregion 🔖️DeltaHelpers
