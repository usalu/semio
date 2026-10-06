//! 🧬️ Process3d snapshot schema — artifact-lane fields only.
//!
//! 🌉️ `stock_solid`/`steps`/`tool_solids` compose real `s.stdio.semio.brep`/`s.stdio.semio.flow`
//! CHILD HANDLES. The snapshot is a `dsl::DslRecord`: text, pack and the pack-schema identity all
//! come from the one derived `__dsl_spec()`, and mounted envelope ingress decodes that same canonical
//! pack through the framework retained cursors into typed owners (`🔖️MountedTypedSnapshotOwner`).

use crate::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, ProcessStep, Stock, WorkingSolid, Workshop, WorkshopMachine};
use framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use store::mounted_pack_rt as mounted;


//#region 🔖️Snapshot
/// 📸️ Persisted process3d document snapshot (persistent fields of the artifact). `stock_solid`/
/// `steps`/`tool_solids` are composed CHILD slots — `#[child(...)]` drives
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Children must sit directly
/// on this struct (not nested inside a helper record) for the derive to see them.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "process3d")]
#[artifact_schema(id = "s.process.process3d")]
pub struct Process3dSnapshot {
    #[state(artifact)]
    pub workshop: Workshop,
    #[state(artifact)]
    pub stock_id: String,
    #[state(artifact)]
    pub stock_label: String,
    #[state(artifact)]
    pub stock_pose: Pose,
    #[state(artifact)]
    pub stock_payload: Stock,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub stock_solid: store::ArtifactChild<SemioBrepSnapshot>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub steps: store::ArtifactChild<SemioFlowSnapshot>,
    #[state(artifact)]
    #[value(default)]
    pub step_payloads: Vec<ProcessStep>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default)]
    pub tool_solids: Vec<store::ArtifactChild<SemioBrepSnapshot>>,
}

impl Default for Process3dSnapshot {
    fn default() -> Self {
        crate::empty_process3d_snapshot()
    }
}



//#region 🔖️DerivedArtifactCodecs



//#endregion 🔖️DerivedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🌉️IdentityBridge

//#endregion 🌉️IdentityBridge


