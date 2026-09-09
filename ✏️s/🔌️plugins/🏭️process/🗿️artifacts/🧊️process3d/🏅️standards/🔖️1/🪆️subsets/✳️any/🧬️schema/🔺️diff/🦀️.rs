//! 🧬️ Process3d diff schema — sparse field delta over the artifact.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `steps` is no longer a
//! `Process3dStepsDelta` (added/removed/patched/reordered) collection diff — the whole timeline is
//! ONE composed `s.stdio.semio.flow` child now, so its diff is a single-`Option` handle swap (the
//! "always-present slot" convention from the migration recipe §8: `Option<ArtifactChild<S>>`, not
//! `Option<Option<...>>` — `steps` is never absent, only ever replaced wholesale). `tool_solids`
//! uses the sibling "collection of children" convention (`📐️cad`'s `CadDrawingChildList` precedent):
//! a whole-list wrapper behind a single `Option`.

use crate::{Pose, ProcessStep, Stock, Workshop};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the process3d artifact.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.process.process3d")]
pub struct Process3dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::Process3dArtifact>>,
    #[state(artifact)]
    pub workshop: Option<Workshop>,
    #[state(artifact)]
    pub stock_id: Option<String>,
    #[state(artifact)]
    pub stock_label: Option<String>,
    #[state(artifact)]
    pub stock_pose: Option<Pose>,
    #[state(artifact)]
    pub stock_payload: Option<Stock>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.brep")]
    pub stock_solid: Option<store::ArtifactChild<SemioBrepSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub steps: Option<store::ArtifactChild<SemioFlowSnapshot>>,
    #[state(artifact)]
    pub step_payloads: Option<Vec<ProcessStep>>,
    #[state(artifact)]
    pub tool_solids: Option<Process3dToolSolidChildList>,
    #[state(artifact)]
    pub resolved_up_to: Option<Option<usize>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧩️ Whole-list wrapper for the `tool_solids` composed CHILD COLLECTION diff field — same
/// `RunList` shape `✳️text`/`✳️kit`/`📐️cad`'s `CadDrawingChildList` use for their own
/// `Vec<ArtifactChild<S>>` diff fields.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dToolSolidChildList {
    pub values: Vec<store::ArtifactChild<SemioBrepSnapshot>>,
}
//#endregion 🔖️DeltaHelpers
