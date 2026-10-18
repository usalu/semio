//! 🧬️ Process3d snapshot schema — artifact-lane fields only.
//!
//! 🌉️ `stock_solid`/`steps`/`tool_solids` compose real `s.stdio.semio.brep`/`s.stdio.semio.flow`
//! CHILD HANDLES. The snapshot is a `dsl::DslRecord`: text, pack and the pack-schema identity all
//! come from the one derived `__dsl_spec()`, and mounted envelope ingress decodes that same canonical
//! pack through the framework retained cursors into typed owners (`🔖️MountedTypedSnapshotOwner`).
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;

//#region 🔖️Snapshot
/// 📸️ Persisted process3d document snapshot (persistent fields of the artifact). `stock_solid`/
/// `steps`/`tool_solids` are composed CHILD slots — `#[child(...)]` drives
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Children must sit directly
/// on this struct (not nested inside a helper record) for the derive to see them.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
    pub stock_solid: store::os_store::artifact_child::ArtifactChild<SemioBrepSnapshot>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub steps: store::os_store::artifact_child::ArtifactChild<SemioFlowSnapshot>,
    #[state(artifact)]
    #[value(default)]
    pub step_payloads: Vec<ProcessStep>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default)]
    pub tool_solids: Vec<store::os_store::artifact_child::ArtifactChild<SemioBrepSnapshot>>,
}

impl Default for Process3dSnapshot {
    fn default() -> Self {
        crate::snapshot::empty_process3d_snapshot()
    }
}

//#endregion 🔖️Snapshot

pub const PROCESS_3D_SCHEMA: &str = "process.3d";
/// 🪪️ ARTIFACT-LEVEL dialect constant (contract §1 grammar) — lives here, not under `editor`/
/// `viewer`, specifically so the sibling `viewer` module can read it without ever importing through
/// the `editor` module. `artifact_kind` matches this schema's own `#[artifact_schema(id = "…")]`
/// (`🧬️schema/🦀️component.rs:17`, `"s.process.process3d"`); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the canonical surface id is
/// `s.process.process3d@1/*#editor` / `s.process.process3d@1/*#viewer`.
pub const PROCESS3D_DIALECT: Dialect = Dialect { artifact_kind: "s.process.process3d", standard: StandardId("1"), subset: SubsetId::ANY };
//#region 🔖️Workshop

#[path = "🪚️machines-and-catalogs/🦀️.rs"]
mod machines_and_catalogs;
pub use machines_and_catalogs::{
    generic_machines, Capability, CapabilityParameter, CapabilityRule, ContributedMachineCatalog, MachineCatalog, MachineCatalogs, MeasureKind, MeasureRecipe, ProcessMachinesTopicPayload, StockQuantity, Workshop, WorkshopMachine,
    WorkshopMachinePatch,
};

#[path = "📐️document-records/🦀️.rs"]
mod document_records;
pub use document_records::{Pose, ProcessMeasure, ProcessStep, ProcessStepPatch, ProcessWorkingScene, StepOrigin, Stock, WorkingSolid};

#[path = "🧱️brep-converters/🦀️.rs"]
mod brep_converters;
pub use brep_converters::{brep_child_handle, brep_snapshot_for_working_solid, reference_solid, working_solid_child_handle, working_solid_from_brep_snapshot, ReferenceSolid, REFERENCE_SOLIDS, REFERENCE_SOLID_CONCRETE_FOREST_LEFT};

#[path = "🔀️flow-and-scene-converters/🦀️.rs"]
mod flow_and_scene_converters;
pub use flow_and_scene_converters::{
    empty_process3d_snapshot, flow_child_handle, flow_node_from_process_step, flow_snapshot_for_steps, genesis_process3d_child_pack, process3d_child_restore_projection, process_step_from_flow_node, process_steps_from_flow_snapshot,
    process_working_scene_from_snapshot, process_working_scene_to_snapshot,
};

//#endregion 🔖️Workshop
//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::process3d::create_process3d_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.process".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("3D Process", "3D-Prozess"),
        source_format: PROCESS_3D_SCHEMA.into(),
        component_kind: "process3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::Brep,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
        schema: PROCESS_3D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into()],
        import_stdio_kinds: vec!["stdio.json".into()],
    }
}
//#endregion 🔖️ArtifactKind

/// 📸️ The snapshot facet's schema leaves (rust, typescript, graphql, json schema, proto).
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: concat!(include_str!("🦀️.rs"), include_str!("🪚️machines-and-catalogs/🦀️.rs"), include_str!("📐️document-records/🦀️.rs"), include_str!("🧱️brep-converters/🦀️.rs"), include_str!("🔀️flow-and-scene-converters/🦀️.rs")), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto"), };
