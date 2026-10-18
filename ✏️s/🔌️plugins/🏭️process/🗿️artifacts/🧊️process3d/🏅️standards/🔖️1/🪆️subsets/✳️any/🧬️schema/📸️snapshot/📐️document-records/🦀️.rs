//! 📐️ Document records of the process3d snapshot: the axis and flag defaults, the pose and step origin, the working solid, the stock, the process measure and step with their patches and the working scene. See [`super`].

use protocol::{Identified, Patchable};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Document
fn default_axis_z() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

fn default_true() -> bool {
    true
}

/// 🧭️ Position + axis-angle rotation applied via the brep kernel's `rotate_sync`/`translate_sync`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct Pose {
    #[value(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[value(default = "default_axis_z")]
    #[dsl(dir)]
    pub axis: [f64; 3],
    #[value(default)]
    #[dsl(angle = "rad")]
    pub angle: f64,
}

impl Default for Pose {
    fn default() -> Self {
        Self { position: [0.0, 0.0, 0.0], axis: default_axis_z(), angle: 0.0 }
    }
}

/// 🏭️ Provenance: which workshop machine/capability produced a step (display + future re-validation).
/// Purely informational — kernel replay only ever reads `ProcessMeasure`, never resolves this back to a
/// workshop entry, so editing or removing the machine/capability can never retroactively change
/// already-authored geometry.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct StepOrigin {
    pub machine_id: String,
    pub capability_id: String,
}
//#endregion 🔖️Document

//#region 🔖️Document
//#region 🔖️WorkingScene
/// 🧱️ EPHEMERAL, per-invocation working representation of a process3d document's editable geometry
/// and step timeline — never persisted, never a `Process3dSnapshot` field (ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4, following the `EngineRep` contract in
/// `⚙️engine/🦀️.rs`: wholly derived, dropped at the end of the call that built it).
///
/// `WorkingSolid` is the direct successor of the old, now-DELETED persisted `SolidSpec` DSL-enum —
/// same five variants, same role (a parametric/imported solid the kernel resolves into real
/// geometry), but demoted from "part of the document's own content model" (duplicating what stdio's
/// `brep` subset already expresses) to "the plugin's own ephemeral editing vocabulary", exactly
/// mirroring `📐️cad`'s `CadObject`/`CadGeometry` (ephemeral bridge types kept beside a composed
/// child, never re-persisted themselves). `Process3dSnapshot` composes `SemioBrepSnapshot` CHILD
/// HANDLES for `stock_solid`/`tool_solids`; this is what the app derives a `WorkingSolid` from (or
/// builds fresh input for) before it can call the kernel — see `brep_snapshot_for_working_solid`
/// (WRITE, real) below for the analytic converter that turns a `WorkingSolid` into real,
/// content-addressable `SemioBrepSnapshot` topology.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum WorkingSolid {
    Box {
        #[dsl(unit = "m")]
        width: f64,
        #[dsl(unit = "m")]
        depth: f64,
        #[dsl(unit = "m")]
        height: f64,
    },
    Cylinder {
        #[dsl(unit = "m")]
        radius: f64,
        #[dsl(unit = "m")]
        height: f64,
    },
    Sphere {
        #[dsl(unit = "m")]
        radius: f64,
    },
    /// 🖼️ Non-parametric GLB-imported reference mesh — tessellation-only, no real B-Rep topology.
    ImportedMesh {
        mesh_url: String,
    },
    /// 🧊️ STEP/OBJ/STL-imported solid with real B-Rep topology, resolved through the app's kernel
    /// session by handle id; ephemeral to that session.
    ImportedSolid {
        solid_handle: String,
    },
    /// 🏛️ A reference solid this artifact ships as exact STEP B-Rep (`reference_solid`), addressed by
    /// its stable id and rebuilt into real kernel topology at every replay — the durable, cross-session
    /// counterpart of `ImportedSolid` for the `♻️mit-bestand` reuse inventory (the hexagonal-cut
    /// concrete forest pieces).
    Reference {
        reference_id: String,
    },
}

impl Default for WorkingSolid {
    fn default() -> Self {
        WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 }
    }
}

/// 🪵️ The raw workpiece the process starts from — ephemeral working-scene counterpart of the
/// persisted `stock_id`/`stock_label`/`stock_pose`/`stock_solid` fields on `Process3dSnapshot`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct Stock {
    pub id: String,
    pub label: String,
    pub solid: WorkingSolid,
    pub pose: Pose,
}

impl Default for Stock {
    fn default() -> Self {
        Self { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::default(), pose: Pose::default() }
    }
}

/// 🪚️ One processing measure: subtractive (cut/drill via `cut_sync`) or additive (attach via `fuse_sync`).
/// Ephemeral working-scene counterpart of a `flow` node's `kind`/`params` — see
/// `flow_node_from_process_step`/`process_step_from_flow_node` below for the real bidirectional
/// converter.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "measure", rename_all = "camelCase")]
pub enum ProcessMeasure {
    /// ✂️ Subtractive: subtracts an arbitrary tool solid (e.g. a thin box as a saw blade).
    Cut { tool: WorkingSolid, pose: Pose },
    /// 🕳️ Subtractive: a cylinder of `radius`×`depth` subtracted at `pose` (axis = drill direction).
    Drill {
        #[dsl(unit = "m")]
        radius: f64,
        #[dsl(unit = "m")]
        depth: f64,
        pose: Pose,
    },
    /// 🔩️ Additive: fuses another component solid at `pose`.
    Attach { component: WorkingSolid, pose: Pose },
}

impl ProcessMeasure {
    pub fn kind_slug(&self) -> &'static str {
        match self {
            ProcessMeasure::Cut { .. } => "cut",
            ProcessMeasure::Drill { .. } => "drill",
            ProcessMeasure::Attach { .. } => "attach",
        }
    }

    pub fn pose(&self) -> &Pose {
        match self {
            ProcessMeasure::Cut { pose, .. } | ProcessMeasure::Drill { pose, .. } | ProcessMeasure::Attach { pose, .. } => pose,
        }
    }
}

/// 🎞️ One ordered step of the process timeline — ephemeral working-scene counterpart of one
/// `SemioFlowSnapshot` `FlowNode` (see `flow_node_from_process_step`/`process_step_from_flow_node`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct ProcessStep {
    pub id: String,
    pub label: String,
    #[value(default = "default_true")]
    pub enabled: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<StepOrigin>,
    pub measure: ProcessMeasure,
}

impl Identified<String> for ProcessStep {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹️ Sparse edit for a `ProcessStep` — `None` fields are left untouched.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ProcessStepPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub measure: Option<ProcessMeasure>,
    /// 🏭️ Outer `Option` = "this patch touches origin"; inner `Option` = the new value (`None` clears it).
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Option<StepOrigin>>,
}

impl Patchable<ProcessStepPatch> for ProcessStep {
    fn apply_patch(&mut self, patch: &ProcessStepPatch) {
        if let Some(label) = &patch.label {
            self.label = label.clone();
        }
        if let Some(enabled) = patch.enabled {
            self.enabled = enabled;
        }
        if let Some(measure) = &patch.measure {
            self.measure = measure.clone();
        }
        if let Some(origin) = &patch.origin {
            self.origin = origin.clone();
        }
    }
}

/// 🧱️ The full ephemeral working scene: current stock + ordered step timeline, exactly the shape
/// `Process3dSnapshot` used to carry inline before this migration. Lives beside the persisted
/// document — populated at mutation-diff-build time (`process_working_scene_to_snapshot`, the WRITE
/// direction — the only place literal content is in hand) or at fixture-construction time; the READ
/// direction (`process_working_scene_from_snapshot`, resolving a persisted document's composed
/// children back into a working scene) is a documented gap until a real `LinkResolver`/
/// `ChildStoreFactory` seam reaches `ArtifactApp::handle` (checked directly against
/// `🔌️plugin/🦀️.rs`, W1-owned, confirmed still absent as of this wave — see
/// `process_working_scene_from_snapshot`'s own doc comment).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ProcessWorkingScene {
    pub stock: Stock,
    pub steps: Vec<ProcessStep>,
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Document
