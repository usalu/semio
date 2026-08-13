//! 🧬️ Mathematical diff schema — sparse field delta over the artifact.

use crate::artifacts::mathematical::{MathematicalComputedChild, MathematicalNotationChild, MathematicalResultsChild};
use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the mathematical artifact. `notation`/`results`/`computed`/`equation`
/// are always-present slots (never absent, only ever replaced) — single-`Option`, matching writer's
/// `document: Option<WriterDocumentChild>` diff shape, not lowpoly's optional-slot double-`Option`.
/// The former `artifact: Option<Box<MathematicalArtifact>>` whole-snapshot-replace slot is REMOVED:
/// it was dead code (never constructed by any app command — `SetArtifact` already routes through
/// the granular `ReplaceGraph`/`ReplacePoints` mutations) and would otherwise be exactly the banned
/// `SetSnapshot` whole-document-replace vocabulary this ticket's `📌️important.md` forbids. `equation`
/// (wave M3a, 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS) is a WHOLE-node
/// replace too — sparse WITHIN the tree happens via label-addressed mutation payloads
/// (`change-coefficient`'s `EquationNodeLabel`), never by diffing two `EquationNode` trees.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.mathematical.mathematical")]
pub struct MathematicalDiff {
    #[state(persistent)]
    pub notation: Option<MathematicalNotationChild>,
    #[state(persistent)]
    pub results: Option<MathematicalResultsChild>,
    #[state(persistent)]
    pub computed: Option<MathematicalComputedChild>,
    #[state(persistent)]
    pub equation: Option<EquationSnapshot>,
    #[state(local_ui)]
    pub camera_x: Option<f64>,
    #[state(local_ui)]
    pub camera_y: Option<f64>,
    #[state(local_ui)]
    pub camera_zoom: Option<f64>,
    #[state(local_ui)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff
