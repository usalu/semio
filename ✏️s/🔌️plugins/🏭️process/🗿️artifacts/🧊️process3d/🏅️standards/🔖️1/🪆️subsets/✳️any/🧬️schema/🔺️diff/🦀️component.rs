//! 🧬️ Process3d diff schema — sparse field delta over the artifact.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `steps` is no longer a
//! `Process3dStepsDelta` (added/removed/patched/reordered) collection diff — the whole timeline is
//! ONE composed `s.stdio.semio.flow` child now, so its diff is a single-`Option` handle swap (the
//! "always-present slot" convention from the migration recipe §8: `Option<ArtifactChild<S>>`, not
//! `Option<Option<...>>` — `steps` is never absent, only ever replaced wholesale). `tool_solids`
//! uses the sibling "collection of children" convention (`📐️cad`'s `CadDrawingChildList` precedent):
//! a whole-list wrapper behind a single `Option`.

use crate::artifacts::process3d::{Pose, Workshop};
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the process3d artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.process.process3d")]
pub struct Process3dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::process3d::schema::Process3dArtifact>>,
    #[state(artifact)]
    pub workshop: Option<Workshop>,
    #[state(artifact)]
    pub stock_id: Option<String>,
    #[state(artifact)]
    pub stock_label: Option<String>,
    #[state(artifact)]
    pub stock_pose: Option<Pose>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.brep")]
    pub stock_solid: Option<store::ArtifactChild<SemioBrepSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub steps: Option<store::ArtifactChild<SemioFlowSnapshot>>,
    #[state(artifact)]
    pub tool_solids: Option<Process3dToolSolidChildList>,
    #[state(artifact)]
    pub resolved_up_to: Option<Option<usize>>,
    #[state(presence)]
    pub selected_id: Option<Option<String>>,
    #[state(presence)]
    pub selected_face_id: Option<Option<usize>>,
    #[state(presence)]
    pub active_utility_id: Option<String>,
    #[state(config)]
    pub selection_method: Option<String>,
    #[state(config)]
    pub engagement_input: Option<String>,
    #[state(config)]
    pub camera_position_x: Option<f64>,
    #[state(config)]
    pub camera_position_y: Option<f64>,
    #[state(config)]
    pub camera_position_z: Option<f64>,
    #[state(config)]
    pub camera_target_x: Option<f64>,
    #[state(config)]
    pub camera_target_y: Option<f64>,
    #[state(config)]
    pub camera_target_z: Option<f64>,
    #[state(config)]
    pub camera_fov: Option<f64>,
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
    pub locale: Option<String>,
    #[state(config)]
    pub contributions_json: Option<String>,
    #[state(artifact)]
    pub hovered_id: Option<Option<String>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧩️ Whole-list wrapper for the `tool_solids` composed CHILD COLLECTION diff field — same
/// `RunList` shape `✳️text`/`✳️kit`/`📐️cad`'s `CadDrawingChildList` use for their own
/// `Vec<ArtifactChild<S>>` diff fields.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Process3dToolSolidChildList {
    pub values: Vec<store::ArtifactChild<SemioBrepSnapshot>>,
}
//#endregion 🔖️DeltaHelpers
