//! 🪚️ Process3d artifact — document entities (workshop machines/capabilities, stock, process steps)
//! plus this artifact's `ArtifactKindSpec`.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `Process3dSnapshot` composes
//! stdio's `brep` subset (`SemioBrepSnapshot`, real analytic B-Rep topology) for the stock/tool
//! solid geometry — killing this plugin's old `SolidSpec` DSL-enum, which duplicated brep content
//! with a lighter parametric shape — and stdio's `flow` subset (`SemioFlowSnapshot`) for the
//! ordered step timeline. See the `🔖️WorkingScene` region below for the ephemeral bridge type
//! (`WorkingSolid`/`ProcessStep`/`ProcessMeasure`/`ProcessWorkingScene`) that replaces `SolidSpec`'s
//! old role as the plugin's own editable in-memory geometry vocabulary.

#![allow(async_fn_in_trait)]

extern crate semio_framework_schema as framework_schema;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_process3d_demo_tests;

use protocol::{Identified, Patchable};
use semio_framework_dispatch_macros::dyn_enum;
use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::{
    BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioBrepSnapshot, STDIO_SEMIOBREP_DOCUMENT_SCHEMA,
};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, PortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};

pub use crate::schema::mutations::Process3dMutation;

pub use crate::schema::diff::Process3dDiff;

use crate::schema::diff::Process3dToolSolidChildList;

pub const PROCESS_3D_SCHEMA: &str = "process.3d";

/// 🪪️ ARTIFACT-LEVEL dialect constant (contract §1 grammar) — lives here, not under `editor`/
/// `viewer`, specifically so the sibling `viewer` module can read it without ever importing through
/// the `editor` module. `artifact_kind` matches this schema's own `#[artifact_schema(id = "…")]`
/// (`🧬️schema/🦀️component.rs:17`, `"s.process.process3d"`); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the canonical surface id is
/// `s.process.process3d@1/*#editor` / `s.process.process3d@1/*#viewer`.
pub const PROCESS3D_DIALECT: Dialect = Dialect { artifact_kind: "s.process.process3d", standard: StandardId("1"), subset: SubsetId::ANY };

//#region 🔖️Workshop
/// 📏️ A stock dimension a capability rule checks against a capability's own parameter value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, dsl::DslScalar)]
pub enum StockQuantity {
    #[default]
    Width,
    Depth,
    Height,
    MaxDimension,
    MinDimension,
}

/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]`'s enum path only supports
/// internally-tagged (`#[value(tag = "…")]`) representations, but `StockQuantity` is a plain
/// unit-only "string enum" — serde's own default (untagged bare-string) representation for an
/// enum with no `#[serde(...)]` attribute at all — so the wire shape here is just the bare variant
/// name, matching what `Serialize`/`Deserialize` already produce for this type today.
impl semio_framework_os_kernel::ToValue for StockQuantity {
    fn to_value(&self) -> semio_framework_os_kernel::DslValue {
        let name = match self {
            StockQuantity::Width => "width",
            StockQuantity::Depth => "depth",
            StockQuantity::Height => "height",
            StockQuantity::MaxDimension => "maxDimension",
            StockQuantity::MinDimension => "minDimension",
        };
        semio_framework_os_kernel::DslValue::String(name.to_string())
    }
}
impl semio_framework_os_kernel::FromValue for StockQuantity {
    fn from_value(value: semio_framework_os_kernel::DslValue) -> Result<Self, semio_framework_os_kernel::ValueError> {
        match value {
            semio_framework_os_kernel::DslValue::String(s) => match s.as_str() {
                "width" => Ok(StockQuantity::Width),
                "depth" => Ok(StockQuantity::Depth),
                "height" => Ok(StockQuantity::Height),
                "maxDimension" => Ok(StockQuantity::MaxDimension),
                "minDimension" => Ok(StockQuantity::MinDimension),
                other => Err(semio_framework_os_kernel::ValueError::new(format!("unknown StockQuantity variant `{other}`"))),
            },
            other => Err(semio_framework_os_kernel::ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🪚️ Which kernel geometry effect a capability produces — `ProcessMeasure`'s three shapes are the
/// fixed, small vocabulary every machine capability ultimately maps onto via its `MeasureRecipe`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MeasureKind {
    Cut,
    Drill,
    Attach,
}

/// ✅️ "the named stock quantity must be at least/at most the named capability parameter's value (±
/// margin)" — a capability's rules are ANDed together, e.g. a crosscut capability needs stock width
/// AND height above the blade diameter.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum CapabilityRule {
    Min {
        quantity: StockQuantity,
        parameter: String,
        #[dsl(unit = "m")]
        margin: f64,
    },
    Max {
        quantity: StockQuantity,
        parameter: String,
        #[dsl(unit = "m")]
        margin: f64,
    },
}

/// 🔧️ One named numeric parameter of a capability (e.g. blade diameter) — workshop-editable, and
/// referenced by id from the capability's own `MeasureRecipe`/`CapabilityRule`s.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct CapabilityParameter {
    pub id: String,
    pub label: String,
    #[dsl(unit = "m")]
    pub value: f64,
}

/// 🪚️ How a capability's parameters build a kernel `ProcessMeasure` — every field names a
/// `Capability::parameters` entry by id, resolved at measure-build time; `measure_kind()` derives the
/// fixed Cut/Drill/Attach effect so it never needs to be stored redundantly alongside the recipe.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum)]
#[value(tag = "recipe", rename_all = "camelCase")]
pub enum MeasureRecipe {
    /// ✂️ A disc-shaped cut tool sized from a blade `diameter` and `kerf` (tool thickness).
    DiscCut { diameter: String, kerf: String },
    /// ✂️ A blade-shaped cut tool sized from `kerf` (width), cut `length` (depth), and cut `depth` (height).
    BladeCut { kerf: String, length: String, depth: String },
    /// ✂️ A square pocket cut tool sized from a `diameter` (width/depth) and `depth` (height).
    PocketCut { diameter: String, depth: String },
    /// 🕳️ A cylindrical bore sized from `radius` and `depth`.
    BoreDrill { radius: String, depth: String },
    /// 🔩️ A cylindrical additive component sized from `radius` and `length` (height).
    CylinderAttach { radius: String, length: String },
    /// 🔩️ A box-shaped additive component sized from `width`, `depth`, and `height`.
    BoxAttach { width: String, depth: String, height: String },
}

impl MeasureRecipe {
    pub fn measure_kind(&self) -> MeasureKind {
        match self {
            MeasureRecipe::DiscCut { .. } | MeasureRecipe::BladeCut { .. } | MeasureRecipe::PocketCut { .. } => MeasureKind::Cut,
            MeasureRecipe::BoreDrill { .. } => MeasureKind::Drill,
            MeasureRecipe::CylinderAttach { .. } | MeasureRecipe::BoxAttach { .. } => MeasureKind::Attach,
        }
    }
}

/// 🌉️ Hand `dsl::DslField` impl — `MeasureRecipe` is a `DslEnum` (`DslVariants` only), and
/// `Capability::recipe` is a REQUIRED, never-optional field that must stay a bare `MeasureRecipe`.
impl dsl::DslField for MeasureRecipe {
    fn shape() -> dsl::Shape {
        dsl::Shape::Statements(<MeasureRecipe as dsl::DslVariants>::variants())
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Statements(vec![<MeasureRecipe as dsl::DslVariants>::to_named_record(self)])
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Statements(items) if items.len() == 1 => <MeasureRecipe as dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|e| e.message),
            other => Err(format!("expected exactly 1 tagged recipe value, found {other:?}")),
        }
    }
}

/// 🪚️ One thing a machine can do; every capability turns into a step: `recipe` fixes the geometric
/// effect and how it's sized, `parameters` size the tool, `rules` gate legality against the stock.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Capability {
    pub id: String,
    pub label: String,
    pub icon_id: String,
    pub recipe: MeasureRecipe,
    #[value(default)]
    pub parameters: Vec<CapabilityParameter>,
    #[value(default)]
    #[dsl(statements, block)]
    pub rules: Vec<CapabilityRule>,
}

/// 🛠️ A machine in the document's workshop — an embedded snapshot, never a reference; consistent with
/// `StepOrigin`'s never-resolve invariant (see its doc comment), and robust to catalog drift: editing
/// or removing an installed catalog can never retroactively change an already-configured workshop.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct WorkshopMachine {
    pub id: String,
    pub label: String,
    pub icon_id: String,
    /// 🏷️ Which installed catalog this snapshot was seeded from — informational only, never resolved
    /// (a machine stays fully usable after its source catalog is uninstalled).
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[value(default)]
    pub capabilities: Vec<Capability>,
}

impl Identified<String> for WorkshopMachine {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹️ Sparse edit for a `WorkshopMachine` — `None` fields are left untouched.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct WorkshopMachinePatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<Capability>>,
}

impl Patchable<WorkshopMachinePatch> for WorkshopMachine {
    fn apply_patch(&mut self, patch: &WorkshopMachinePatch) {
        if let Some(label) = &patch.label {
            self.label = label.clone();
        }
        if let Some(icon_id) = &patch.icon_id {
            self.icon_id = icon_id.clone();
        }
        if let Some(capabilities) = &patch.capabilities {
            self.capabilities = capabilities.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<WorkshopMachinePatch> {
        let patch = WorkshopMachinePatch {
            label: (self.label != other.label).then(|| other.label.clone()),
            icon_id: (self.icon_id != other.icon_id).then(|| other.icon_id.clone()),
            capabilities: (self.capabilities != other.capabilities).then(|| other.capabilities.clone()),
        };
        (patch != WorkshopMachinePatch::default()).then_some(patch)
    }
}

/// 🏭️ The document's configured workshop: the machines available to build steps from.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Workshop {
    #[value(default)]
    pub machines: Vec<WorkshopMachine>,
}

impl Default for Workshop {
    fn default() -> Self {
        Self { machines: generic_machines() }
    }
}

/// 📦️ The three built-in generic machines (saw/drill/attacher), reproducing the same default tool
/// sizes `ProcessMeasure` used before capabilities existed — pure data with no catalog dependency, so
/// every document (including ones deserialized without a `workshop` field) always has a working
/// workshop and the utility bar's click-to-place cut/drill/attach never dead-ends.
pub fn generic_machines() -> Vec<WorkshopMachine> {
    vec![
        WorkshopMachine {
            id: "saw".into(),
            label: "Generic Saw".into(),
            icon_id: "scissors".into(),
            catalog_id: None,
            capabilities: vec![Capability {
                id: "cut".into(),
                label: "Cut".into(),
                icon_id: "scissors".into(),
                recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "length".into(), depth: "depth".into() },
                parameters: vec![
                    CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.05 },
                    CapabilityParameter { id: "length".into(), label: "Length".into(), value: 0.5 },
                    CapabilityParameter { id: "depth".into(), label: "Depth".into(), value: 0.5 },
                ],
                rules: Vec::new(),
            }],
        },
        WorkshopMachine {
            id: "drill".into(),
            label: "Generic Drill".into(),
            icon_id: "circle-dot".into(),
            catalog_id: None,
            capabilities: vec![Capability {
                id: "drill".into(),
                label: "Drill".into(),
                icon_id: "circle-dot".into(),
                recipe: MeasureRecipe::BoreDrill { radius: "radius".into(), depth: "depth".into() },
                parameters: vec![CapabilityParameter { id: "radius".into(), label: "Radius".into(), value: 0.05 }, CapabilityParameter { id: "depth".into(), label: "Depth".into(), value: 0.3 }],
                rules: Vec::new(),
            }],
        },
        WorkshopMachine {
            id: "attacher".into(),
            label: "Generic Attacher".into(),
            icon_id: "plus".into(),
            catalog_id: None,
            capabilities: vec![Capability {
                id: "attach".into(),
                label: "Attach".into(),
                icon_id: "plus".into(),
                recipe: MeasureRecipe::CylinderAttach { radius: "radius".into(), length: "length".into() },
                parameters: vec![CapabilityParameter { id: "radius".into(), label: "Radius".into(), value: 0.03 }, CapabilityParameter { id: "length".into(), label: "Length".into(), value: 0.2 }],
                rules: Vec::new(),
            }],
        },
    ]
}

/// 🧩️ A machine catalog: contributes machines (with capabilities) the workshop configurator can
/// install into a document's workshop. Implemented by the built-in generic/domain catalogs
/// (`crate::schema::{GenericCatalog, MetalCatalog, WoodCatalog, RoboticCatalog,
/// ConcreteCatalog}`) and by the app's runtime-contributed `ContributedMachineCatalog`
/// (`crate::editor::process3d::ContributedMachineCatalog`) — closed into `MachineCatalogs` below
/// (O1: dyn dispatch is banned from trait-method return position; R11: closed implementor set ⇒
/// `dyn_enum_close!`, never a box).
// 🚫️async: E1 pure — every implementor (5 in `schema`, `editor`'s `ContributedMachineCatalog`, plus
// each `process-extension-*` crate's local catalog) is a zero-suspension struct-literal/field
// accessor; every caller (`editor::installed_catalogs`/`catalog_machine`, each extension's `bundle()`)
// already consumes these unawaited — see R9.
#[dyn_enum]
pub trait MachineCatalog {
    fn catalog_id(&self) -> &str;
    fn label(&self) -> &str;
    fn icon_id(&self) -> &str;
    fn machines(&self) -> Vec<WorkshopMachine>;
}

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The closed set of `MachineCatalog` implementors. Closed HERE, in the same module as `#[dyn_enum]`
    /// (not at the `editor::installed_catalogs` call site that gathers them): the generated
    /// `__semio_dispatch_MachineCatalog!` is `#[macro_export]`ed, and rustc rejects any reference to a
    /// `macro_export`ed macro produced by expansion IN THE SAME CRATE via an absolute/`crate::`-qualified
    /// path (rust-lang/rust#52234) — verified directly against real rustc: even the documented
    /// `use crate::__semio_dispatch_MachineCatalog;` cross-module recipe (`📓️terra-dyn-enum-macro-report.md`
    /// finding 1) still hits this error when the closing site is a genuine sibling module tree (`editor` is
    /// a top-level sibling of `artifacts`, not a descendant of `process3d`), because that `use` is itself an
    /// absolute path. Only a BARE invocation in the trait's OWN literal module resolves it, via ordinary
    /// `macro_rules!` textual scoping — so the enum lives here, and `editor::installed_catalogs` imports
    /// `MachineCatalogs` like any other type.
    pub enum MachineCatalogs: MachineCatalog {
        Generic(schema::GenericCatalog),
        Metal(schema::MetalCatalog),
        Wood(schema::WoodCatalog),
        Robotic(schema::RoboticCatalog),
        Concrete(schema::ConcreteCatalog),
        Contributed(editor::process3d::ContributedMachineCatalog),
    }
}
//#endregion 🔖️Workshop

//#region 🔖️Document
fn default_axis_z() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

fn default_true() -> bool {
    true
}

/// 🧭️ Position + axis-angle rotation applied via the brep kernel's `rotate_sync`/`translate_sync`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct StepOrigin {
    pub machine_id: String,
    pub capability_id: String,
}

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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum WorkingSolid {
    Box {
        width: f64,
        depth: f64,
        height: f64,
    },
    Cylinder {
        radius: f64,
        height: f64,
    },
    Sphere {
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
}

impl Default for WorkingSolid {
    fn default() -> Self {
        WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 }
    }
}

/// 🪵️ The raw workpiece the process starts from — ephemeral working-scene counterpart of the
/// persisted `stock_id`/`stock_label`/`stock_pose`/`stock_solid` fields on `Process3dSnapshot`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "measure", rename_all = "camelCase")]
pub enum ProcessMeasure {
    /// ✂️ Subtractive: subtracts an arbitrary tool solid (e.g. a thin box as a saw blade).
    Cut { tool: WorkingSolid, pose: Pose },
    /// 🕳️ Subtractive: a cylinder of `radius`×`depth` subtracted at `pose` (axis = drill direction).
    Drill { radius: f64, depth: f64, pose: Pose },
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
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

    fn diff_patch(&self, other: &Self) -> Option<ProcessStepPatch> {
        let patch = ProcessStepPatch {
            label: (self.label != other.label).then(|| other.label.clone()),
            enabled: (self.enabled != other.enabled).then_some(other.enabled),
            measure: (self.measure != other.measure).then(|| other.measure.clone()),
            origin: (self.origin != other.origin).then(|| other.origin.clone()),
        };
        (patch != ProcessStepPatch::default()).then_some(patch)
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

//#region 🔖️BrepConverters
/// 🌉️ WRITE direction, real (not a stub): analytic B-Rep topology for each `WorkingSolid` variant.
/// `Box`/`Cylinder`/`Sphere` are exact (no tessellation/approximation — `BrepSurface::Cylinder`/
/// `Sphere` are themselves analytic surface primitives, and a `BrepLoop` with zero edges is the
/// correct "untrimmed, spans the whole surface" shape for the sphere's single closed face and the
/// cylinder's lateral face). `ImportedMesh`/`ImportedSolid` reference content the app's own kernel
/// session must resolve (mesh URL / kernel-session-local handle) — this plugin-only migration cannot
/// tessellate/import that here, so it mints an honest EMPTY-topology placeholder rather than
/// fabricating fake geometry; the doc comment on `SolidSpec`'s old `ImportedMesh`/`ImportedSolid`
/// variants already flagged this as kernel-session-resolved, never persisted-content-resolved.
pub fn brep_snapshot_for_working_solid(solid: &WorkingSolid) -> SemioBrepSnapshot {
    match solid {
        WorkingSolid::Box { width, depth, height } => brep_snapshot_for_box(*width, *depth, *height),
        WorkingSolid::Cylinder { radius, height } => brep_snapshot_for_cylinder(*radius, *height),
        WorkingSolid::Sphere { radius } => brep_snapshot_for_sphere(*radius),
        WorkingSolid::ImportedMesh { .. } | WorkingSolid::ImportedSolid { .. } => empty_brep_snapshot(),
    }
}

fn empty_brep_snapshot() -> SemioBrepSnapshot {
    SemioBrepSnapshot { schema: STDIO_SEMIOBREP_DOCUMENT_SCHEMA.into(), vertices: Vec::new(), edges: Vec::new(), loops: Vec::new(), faces: Vec::new(), shells: Vec::new(), solids: Vec::new(), coedges: Vec::new(), next_label: 0 }
}

fn point3(p: [f64; 3]) -> semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint3 {
    semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint3 { x: p[0], y: p[1], z: p[2] }
}

/// 📦️ Corner-at-local-origin box, spanning `[0,w]×[0,d]×[0,h]` — 8 vertices, 12 straight edges, 6
/// planar faces, 1 shell, 1 solid.
fn brep_snapshot_for_box(width: f64, depth: f64, height: f64) -> SemioBrepSnapshot {
    let (w, d, h) = (width, depth, height);
    let corners = [[0.0, 0.0, 0.0], [w, 0.0, 0.0], [w, d, 0.0], [0.0, d, 0.0], [0.0, 0.0, h], [w, 0.0, h], [w, d, h], [0.0, d, h]];
    let vertices: Vec<BrepVertex> = corners.iter().enumerate().map(|(i, c)| BrepVertex { id: format!("v{i}"), point: point3(*c), tol: 0.0 }).collect();
    let line_edge = |id: &str, a: usize, b: usize| -> BrepEdge {
        let origin = corners[a];
        let direction = [corners[b][0] - corners[a][0], corners[b][1] - corners[a][1], corners[b][2] - corners[a][2]];
        BrepEdge { id: id.into(), start_vertex: format!("v{a}"), end_vertex: format!("v{b}"), curve: BrepCurve::Line { origin: point3(origin), direction: point3(direction) }, tol: 0.0 }
    };
    let edges = vec![
        line_edge("e0", 0, 1),
        line_edge("e1", 1, 2),
        line_edge("e2", 2, 3),
        line_edge("e3", 3, 0),
        line_edge("e4", 4, 5),
        line_edge("e5", 5, 6),
        line_edge("e6", 6, 7),
        line_edge("e7", 7, 4),
        line_edge("e8", 0, 4),
        line_edge("e9", 1, 5),
        line_edge("e10", 2, 6),
        line_edge("e11", 3, 7),
    ];
    let loop_of = |id: &str, edges: &[(&str, bool)]| -> BrepLoop { BrepLoop { id: id.into(), edges: edges.iter().map(|(e, o)| BrepLoopEdge { edge: (*e).into(), orientation: *o }).collect() } };
    let loops = vec![
        loop_of("l0", &[("e0", true), ("e1", true), ("e2", true), ("e3", true)]),
        loop_of("l1", &[("e4", true), ("e5", true), ("e6", true), ("e7", true)]),
        loop_of("l2", &[("e0", true), ("e9", true), ("e4", false), ("e8", false)]),
        loop_of("l3", &[("e1", true), ("e10", true), ("e5", false), ("e9", false)]),
        loop_of("l4", &[("e2", true), ("e11", true), ("e6", false), ("e10", false)]),
        loop_of("l5", &[("e3", true), ("e8", true), ("e7", false), ("e11", false)]),
    ];
    let plane_face = |id: &str, loop_id: &str, origin: [f64; 3], normal: [f64; 3], orientation: bool| -> BrepFace {
        BrepFace { id: id.into(), outer_loop: loop_id.into(), inner_loops: Vec::new(), surface: BrepSurface::Plane { origin: point3(origin), normal: point3(normal) }, orientation, tol: 0.0 }
    };
    let faces = vec![
        plane_face("f0", "l0", [0.0, 0.0, 0.0], [0.0, 0.0, -1.0], true),
        plane_face("f1", "l1", [0.0, 0.0, h], [0.0, 0.0, 1.0], true),
        plane_face("f2", "l2", [0.0, 0.0, 0.0], [0.0, -1.0, 0.0], true),
        plane_face("f3", "l3", [w, 0.0, 0.0], [1.0, 0.0, 0.0], true),
        plane_face("f4", "l4", [0.0, d, 0.0], [0.0, 1.0, 0.0], true),
        plane_face("f5", "l5", [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], true),
    ];
    let shells = vec![BrepShell { id: "s0".into(), faces: (0..6).map(|i| BrepShellFace { face: format!("f{i}"), orientation: true }).collect() }];
    let solids = vec![BrepSolid { id: "so0".into(), shells: vec![BrepSolidShell { shell: "s0".into(), is_void: false }] }];
    SemioBrepSnapshot { schema: STDIO_SEMIOBREP_DOCUMENT_SCHEMA.into(), vertices, edges, loops, faces, shells, solids, coedges: Vec::new(), next_label: 0 }
}

/// 🥫️ Axis-aligned cylinder, base centered at local origin, axis along +Z, spanning `[0,height]`.
/// Two circular cap edges + two planar cap faces + one analytic `BrepSurface::Cylinder` lateral
/// face (untrimmed — its loop has no bounding edges, since the lateral surface is naturally closed
/// around the axis and only bounded top/bottom by the shared circular edges via the loop's implicit
/// parametric domain, matching the same "loop may be empty" convention the sphere face uses).
fn brep_snapshot_for_cylinder(radius: f64, height: f64) -> SemioBrepSnapshot {
    let (r, h) = (radius, height);
    let vertices = vec![BrepVertex { id: "v0".into(), point: point3([r, 0.0, 0.0]), tol: 0.0 }, BrepVertex { id: "v1".into(), point: point3([r, 0.0, h]), tol: 0.0 }];
    let edges = vec![
        BrepEdge { id: "e0".into(), start_vertex: "v0".into(), end_vertex: "v0".into(), curve: BrepCurve::Circle { center: point3([0.0, 0.0, 0.0]), axis: point3([0.0, 0.0, 1.0]), radius: r }, tol: 0.0 },
        BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Circle { center: point3([0.0, 0.0, h]), axis: point3([0.0, 0.0, 1.0]), radius: r }, tol: 0.0 },
    ];
    let loops = vec![
        BrepLoop { id: "l0".into(), edges: vec![BrepLoopEdge { edge: "e0".into(), orientation: true }] },
        BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] },
        BrepLoop { id: "l2".into(), edges: Vec::new() },
    ];
    let faces = vec![
        BrepFace { id: "f0".into(), outer_loop: "l0".into(), inner_loops: Vec::new(), surface: BrepSurface::Plane { origin: point3([0.0, 0.0, 0.0]), normal: point3([0.0, 0.0, -1.0]) }, orientation: true, tol: 0.0 },
        BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: Vec::new(), surface: BrepSurface::Plane { origin: point3([0.0, 0.0, h]), normal: point3([0.0, 0.0, 1.0]) }, orientation: true, tol: 0.0 },
        BrepFace { id: "f2".into(), outer_loop: "l2".into(), inner_loops: Vec::new(), surface: BrepSurface::Cylinder { origin: point3([0.0, 0.0, 0.0]), axis: point3([0.0, 0.0, 1.0]), radius: r }, orientation: true, tol: 0.0 },
    ];
    let shells = vec![BrepShell { id: "s0".into(), faces: vec![BrepShellFace { face: "f0".into(), orientation: true }, BrepShellFace { face: "f1".into(), orientation: true }, BrepShellFace { face: "f2".into(), orientation: true }] }];
    let solids = vec![BrepSolid { id: "so0".into(), shells: vec![BrepSolidShell { shell: "s0".into(), is_void: false }] }];
    SemioBrepSnapshot { schema: STDIO_SEMIOBREP_DOCUMENT_SCHEMA.into(), vertices, edges, loops, faces, shells, solids, coedges: Vec::new(), next_label: 0 }
}

/// 🔮️ Sphere centered at local origin — one closed, untrimmed analytic `BrepSurface::Sphere` face
/// (no boundary curves at all: zero vertices/edges, one loop with zero edges, matching the shared
/// "a `BrepLoop` with no edges means the whole surface" convention).
fn brep_snapshot_for_sphere(radius: f64) -> SemioBrepSnapshot {
    let loops = vec![BrepLoop { id: "l0".into(), edges: Vec::new() }];
    let faces = vec![BrepFace { id: "f0".into(), outer_loop: "l0".into(), inner_loops: Vec::new(), surface: BrepSurface::Sphere { center: point3([0.0, 0.0, 0.0]), radius }, orientation: true, tol: 0.0 }];
    let shells = vec![BrepShell { id: "s0".into(), faces: vec![BrepShellFace { face: "f0".into(), orientation: true }] }];
    let solids = vec![BrepSolid { id: "so0".into(), shells: vec![BrepSolidShell { shell: "s0".into(), is_void: false }] }];
    SemioBrepSnapshot { schema: STDIO_SEMIOBREP_DOCUMENT_SCHEMA.into(), vertices: Vec::new(), edges: Vec::new(), loops, faces, shells, solids, coedges: Vec::new(), next_label: 0 }
}

/// 🌉️ READ direction, documented gap: recovering a `WorkingSolid` (a small parametric vocabulary)
/// from an arbitrary resolved `SemioBrepSnapshot` (general topology) is not generally invertible —
/// nothing tags a brep as "this originated from a box/cylinder/sphere". Mirrors `📐️cad`'s own
/// documented read-side gap for its per-pane object lists. Callers that only need SOME extent
/// (rendering/bounds) should read `stock_bounding_box`, which works directly off the brep's own
/// vertex list instead of trying to recover a `WorkingSolid`.
pub fn working_solid_from_brep_snapshot(_brep: &SemioBrepSnapshot) -> WorkingSolid {
    WorkingSolid::default()
}

/// 🪪️ Mint a deterministic, content-addressed `s.stdio.semio.brep` CHILD HANDLE from `content`
/// (mirrors `📐️cad`'s `cad_model_child_handle`/`💠️lowpoly`'s `mesh_child_handle` — same
/// `store::ArtifactChild::new`/`ArtifactDialect` shape). Two callers with byte-identical content
/// mint the same handle.
pub fn brep_child_handle(slug: &str, content: &SemioBrepSnapshot) -> store::ArtifactChild<SemioBrepSnapshot> {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    serde_json::to_string(&semio_framework_os_kernel::ToValue::to_value(content)).unwrap_or_default().hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("{slug}-brep-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "brep".into() };
    let target = store::os_io::ArtifactRef { artifact_id: format!("process-{slug}-brep"), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️BrepConverters

//#region 🔖️FlowConverters
/// 🌉️ WRITE direction, real: one `FlowNode` per `ProcessStep`, laid out left-to-right in timeline
/// order. `enabled`/`origin`/measure-specific scalars round-trip exactly via string params;
/// `Cut`/`Attach`'s `WorkingSolid` tool/component is addressed indirectly by a `toolChildId` param
/// naming its entry in `Process3dSnapshot::tool_solids` (minted alongside by the caller — see
/// `process_working_scene_to_snapshot`), since a `FlowParam` value is a plain string, never a child
/// handle itself.
pub fn flow_node_from_process_step(step: &ProcessStep, index: usize, tool_child_id: Option<&str>) -> FlowNode {
    let mut params = vec![FlowParam { key: "enabled".into(), value: step.enabled.to_string() }];
    if let Some(origin) = &step.origin {
        params.push(FlowParam { key: "originMachineId".into(), value: origin.machine_id.clone() });
        params.push(FlowParam { key: "originCapabilityId".into(), value: origin.capability_id.clone() });
    }
    let pose = step.measure.pose();
    params.push(FlowParam { key: "posePositionX".into(), value: pose.position[0].to_string() });
    params.push(FlowParam { key: "posePositionY".into(), value: pose.position[1].to_string() });
    params.push(FlowParam { key: "posePositionZ".into(), value: pose.position[2].to_string() });
    params.push(FlowParam { key: "poseAxisX".into(), value: pose.axis[0].to_string() });
    params.push(FlowParam { key: "poseAxisY".into(), value: pose.axis[1].to_string() });
    params.push(FlowParam { key: "poseAxisZ".into(), value: pose.axis[2].to_string() });
    params.push(FlowParam { key: "poseAngle".into(), value: pose.angle.to_string() });
    match &step.measure {
        ProcessMeasure::Drill { radius, depth, .. } => {
            params.push(FlowParam { key: "radius".into(), value: radius.to_string() });
            params.push(FlowParam { key: "depth".into(), value: depth.to_string() });
        }
        ProcessMeasure::Cut { .. } | ProcessMeasure::Attach { .. } => {
            if let Some(child_id) = tool_child_id {
                params.push(FlowParam { key: "toolChildId".into(), value: child_id.to_string() });
            }
        }
    }
    FlowNode { id: step.id.clone(), kind: step.measure.kind_slug().into(), label: step.label.clone(), params, position: SemioPoint2 { x: index as f64 * 200.0, y: 0.0 } }
}

/// 🌉️ READ direction, real for everything reachable from the node's own params (`enabled`/`origin`/
/// pose/radius/depth); `Cut`/`Attach`'s tool/component solid is a documented gap
/// (`working_solid_from_brep_snapshot`'s own doc comment — a `toolChildId` param names WHICH child
/// holds the resolved geometry, but resolving that handle to real content needs a `LinkResolver`
/// this ticket doesn't add).
pub fn process_step_from_flow_node(node: &FlowNode) -> ProcessStep {
    let param = |key: &str| node.params.iter().find(|p| p.key == key).map(|p| p.value.as_str());
    let f = |key: &str| -> f64 { param(key).and_then(|v| v.parse().ok()).unwrap_or(0.0) };
    let enabled = param("enabled").map_or(true, |v| v == "true");
    let origin = match (param("originMachineId"), param("originCapabilityId")) {
        (Some(machine_id), Some(capability_id)) => Some(StepOrigin { machine_id: machine_id.into(), capability_id: capability_id.into() }),
        _ => None,
    };
    let pose = Pose { position: [f("posePositionX"), f("posePositionY"), f("posePositionZ")], axis: [f("poseAxisX"), f("poseAxisY"), f("poseAxisZ")], angle: f("poseAngle") };
    let measure = match node.kind.as_str() {
        "drill" => ProcessMeasure::Drill { radius: f("radius"), depth: f("depth"), pose },
        "attach" => ProcessMeasure::Attach { component: WorkingSolid::default(), pose },
        _ => ProcessMeasure::Cut { tool: WorkingSolid::default(), pose },
    };
    ProcessStep { id: node.id.clone(), label: node.label.clone(), enabled, origin, measure }
}

/// 🪪️ Mint a deterministic, content-addressed `s.stdio.semio.flow` CHILD HANDLE from `content`.
pub fn flow_child_handle(content: &SemioFlowSnapshot) -> store::ArtifactChild<SemioFlowSnapshot> {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    serde_json::to_string(&semio_framework_os_kernel::ToValue::to_value(content)).unwrap_or_default().hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("steps-flow-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "process-steps-flow".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🌉️ WRITE direction, real: the full flow graph for a step timeline — one node per step (see
/// `flow_node_from_process_step`) plus a linear sequence edge between consecutive steps (this
/// plugin's timeline has no branching, so a simple chain is an honest, lossless topology — not an
/// approximation of a richer graph the old `Vec<ProcessStep>` never had either).
pub fn flow_snapshot_for_steps(steps: &[ProcessStep], tool_child_ids: &std::collections::BTreeMap<String, String>) -> SemioFlowSnapshot {
    let nodes: Vec<FlowNode> = steps.iter().enumerate().map(|(i, step)| flow_node_from_process_step(step, i, tool_child_ids.get(&step.id).map(|s| s.as_str()))).collect();
    let edges: Vec<FlowEdge> = steps
        .windows(2)
        .map(|pair| FlowEdge { id: format!("e-{}-{}", pair[0].id, pair[1].id), from: PortRef { node: pair[0].id.clone(), port: "out".into() }, to: PortRef { node: pair[1].id.clone(), port: "in".into() }, kind: "sequence".into() })
        .collect();
    SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes, edges }
}

/// 🌉️ READ direction: node order in `SemioFlowSnapshot.nodes` IS the step order (matches
/// `flow_snapshot_for_steps`'s construction — the `position.x` synthetic layout is display-only,
/// never re-derived from).
pub fn process_steps_from_flow_snapshot(flow: &SemioFlowSnapshot) -> Vec<ProcessStep> {
    flow.nodes.iter().map(process_step_from_flow_node).collect()
}
//#endregion 🔖️FlowConverters

//#region 🔖️SceneConverters
// 🧠️ Same-process working-scene cache, keyed by the composed child's own content-addressed
// `child_id` — populated at mint time (`process_working_scene_to_snapshot`, the only place literal
// content is in hand), read at scene-reconstruction time
// (`process_working_scene_from_snapshot`). Mirrors `🌊️flow`'s own `FLOW_SCRATCH` pattern (this
// ticket's wave 4, `📓️wave4-reports/flow-report.md`) rather than lowpoly/cad/writer's plain
// "always empty" fallback: within the SAME process, a document round-tripped only through this
// module (never serialized out and a fresh process spun up to read it back) gets its real content
// back, not a fabricated empty scene. Still an `EngineRep`-shaped bridge, not a real resolver —
// crossing a process boundary (a fresh process loading a saved document, or an undo/redo that
// bypasses `ArtifactApp::handle`) still degrades to the honest empty fallback, matching every
// other exemplar's documented staleness gap.
/// 🌉️ WRITE direction, real: builds the full persisted `Process3dSnapshot` from a literal
/// `ProcessWorkingScene` — the only place this migration can mint real composed-child CONTENT
/// (fixture construction, `empty_process3d_snapshot`, and any host-level "mint then dispatch"
/// gesture), since there is no `LinkResolver` to pull literal content back out of an
/// already-persisted child handle later. Mints one `stock_solid` brep child, one `steps` flow
/// child, and one `tool_solids` brep child per `Cut`/`Attach` step (skipped for `Drill`, which
/// carries no `WorkingSolid`) — and caches the literal `Stock`/`Vec<ProcessStep>` behind each
/// snapshot-owned payload records so reopen and worker migration reconstruct the same scene.
pub fn process_working_scene_to_snapshot(scene: &ProcessWorkingScene, workshop: Workshop, resolved_up_to: Option<usize>) -> Process3dSnapshot {
    let mut tool_solids = Vec::new();
    let mut tool_child_ids = std::collections::BTreeMap::new();
    for step in &scene.steps {
        let solid = match &step.measure {
            ProcessMeasure::Cut { tool, .. } => Some(tool),
            ProcessMeasure::Attach { component, .. } => Some(component),
            ProcessMeasure::Drill { .. } => None,
        };
        if let Some(solid) = solid {
            let content = brep_snapshot_for_working_solid(solid);
            let handle = brep_child_handle(&format!("tool-{}", step.id), &content);
            tool_child_ids.insert(step.id.clone(), handle.child_id.clone());
            tool_solids.push(handle);
        }
    }
    let stock_content = brep_snapshot_for_working_solid(&scene.stock.solid);
    let stock_solid = brep_child_handle("stock", &stock_content);
    let flow_content = flow_snapshot_for_steps(&scene.steps, &tool_child_ids);
    let steps = flow_child_handle(&flow_content);
    Process3dSnapshot {
        workshop,
        stock_id: scene.stock.id.clone(),
        stock_label: scene.stock.label.clone(),
        stock_pose: scene.stock.pose.clone(),
        stock_payload: scene.stock.clone(),
        stock_solid,
        steps,
        step_payloads: scene.steps.clone(),
        tool_solids,
        resolved_up_to,
    }
}

/// 🌉️ READ direction: `Process3dSnapshot` composes its `stock_solid`/`steps`/`tool_solids` fields
/// as HANDLES only (`child_id`+`target`, never resolved content — see `🏪️store/🦀️.rs`'s
/// `🔖️Composition` region), and no `LinkResolver`/`ChildStoreFactory` seam reaches
/// `ArtifactApp::handle` yet (confirmed directly against `🔌️plugin/🦀️.rs`, W1-owned,
/// read-only for this wave). The durable stock/step records are authoritative while the child
/// handles retain composition identity.
pub fn process_working_scene_from_snapshot(snapshot: &Process3dSnapshot) -> ProcessWorkingScene {
    let mut stock = snapshot.stock_payload.clone();
    stock.id = snapshot.stock_id.clone();
    stock.label = snapshot.stock_label.clone();
    stock.pose = snapshot.stock_pose.clone();
    ProcessWorkingScene { stock, steps: snapshot.step_payloads.clone() }
}

/// 🔁 Shared re-mint for every step-scoped mutation (`create`/`delete`/`rename`/`change-step-
/// enabled`/`change-step-origin`/`replace-step-measure`/`reorder-steps`): given `base` and an
/// already-edited step list, rebuilds `steps`/`step_payloads`/`tool_solids` by delegating to
/// `process_working_scene_to_snapshot` — the one place real composed-child content is minted —
/// so no mutation duplicates that minting logic. `stock`/`workshop`/`resolved_up_to` are carried
/// through from `base` untouched; callers only ever splice the returned diff's `steps`/
/// `step_payloads`/`tool_solids` fields into their own `Process3dDiff`.
pub fn process3d_step_timeline_diff(base: &Process3dSnapshot, new_steps: Vec<ProcessStep>) -> Process3dDiff {
    let mut scene = process_working_scene_from_snapshot(base);
    scene.steps = new_steps;
    let minted = process_working_scene_to_snapshot(&scene, base.workshop.clone(), base.resolved_up_to);
    Process3dDiff { steps: Some(minted.steps), step_payloads: Some(minted.step_payloads), tool_solids: Some(Process3dToolSolidChildList { values: minted.tool_solids }), ..Default::default() }
}
//#endregion 🔖️SceneConverters
//#endregion 🔖️WorkingScene

/// 🪚️ Process 3d projection: workshop + stock + composed step timeline + tool solids + timeline cursor.
/// 📸️ Persisted process3d snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::schema::snapshot::Process3dSnapshot;

/// 🗄️ Empty process3d snapshot (default workshop + stock, no steps) — real composed children minted
/// from the default `ProcessWorkingScene` via `process_working_scene_to_snapshot`, never bare/unset
/// handles.
pub fn empty_process3d_snapshot() -> Process3dSnapshot {
    process_working_scene_to_snapshot(&ProcessWorkingScene::default(), Workshop::default(), None)
}
//#endregion 🔖️Document

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::process3d::create_process3d_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.process".into(),
        name: "3D Process".into(),
        source_format: PROCESS_3D_SCHEMA.into(),
        component_kind: "process3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::Brep,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
        schema: PROCESS_3D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.ifc".into(), "stdio.json".into(), "stdio.obj".into(), "stdio.png".into(), "stdio.step".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.ifc".into(), "stdio.json".into(), "stdio.obj".into(), "stdio.png".into(), "stdio.step".into(), "stdio.stl".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.process.process3d")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.process.process3d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.process.process3d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.process.process3d.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.process.process3d.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.process.process3d@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.process.process3d@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.composer.ifc")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.ifc@4/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.ifc@4/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.composer.step")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.step@ap214/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.step@ap214/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.composer.png")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.png@1.2/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.png@1.2/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.composer.dwg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.dwg@ac1018/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dwg@ac1018/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.composer.stl")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.stl@ascii/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.stl@ascii/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.composer.gltf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.gltf@2.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.gltf@2.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.composer.obj")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.obj@3.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.obj@3.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"process.3d:process3d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "process.3d")?)?
                .claim(ArtifactIdentityClaim::codec_extension("process.3d", "process3d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Process 3D")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Process 3D")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Process 3D")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Process 3D")?)?,
        )
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(schema::process3d_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::process3d_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<editor::process3d::Process3dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod bounds {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                        pub use text::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod create_step {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🧪️tests/🪚️accepts-a-rip-f93d45/🦀️.rs"]
                            mod tests_accepts_a_rip_cut_step_and_changes_nothing;
                        }
                        #[path = "."]
                        pub mod delete_step {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🧪️tests/🚫️accepts-a-step-e6cfa8/🦀️.rs"]
                            mod tests_accepts_a_step_id_and_changes_nothing;
                        }
                        #[path = "."]
                        pub mod rename_step {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-step/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-step/🧪️tests/🔤️accepts-a-new-7492c5/🦀️.rs"]
                            mod tests_accepts_a_new_label_and_changes_nothing;
                        }
                        #[path = "."]
                        pub mod change_step_enabled {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘change-step-enabled/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘change-step-enabled/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘change-step-enabled/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘change-step-enabled/🧪️tests/⏸️accepts-a-disable-e2f494/🦀️.rs"]
                            mod tests_accepts_a_disable_flag_and_changes_nothing;
                        }
                        #[path = "."]
                        pub mod change_step_origin {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-step-origin/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-step-origin/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-step-origin/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-step-origin/🧪️tests/🏭️accepts-a-2b8ada/🦀️.rs"]
                            mod tests_accepts_a_machine_provenance_and_changes_nothing;
                        }
                        #[path = "."]
                        pub mod replace_step_measure {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐replace-step-measure/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐replace-step-measure/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐replace-step-measure/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐replace-step-measure/🧪️tests/🕳️accepts-a-bore-e4ce0f/🦀️.rs"]
                            mod tests_accepts_a_bore_measure_and_changes_nothing;
                        }
                        #[path = "."]
                        pub mod reorder_steps {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🧪️tests/🔀️accepts-a-target-f657e1/🦀️.rs"]
                            mod tests_accepts_a_target_index_and_changes_nothing;
                        }
                        #[path = "."]
                        pub mod create_machine {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭create-machine/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭create-machine/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭create-machine/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭create-machine/🧪️tests/🪛️adds-a-drill-724e18/🦀️.rs"]
                            mod tests_adds_a_drill_press_to_the_workshop;
                        }
                        #[path = "."]
                        pub mod delete_machine {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-machine/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-machine/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-machine/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-machine/🧪️tests/➖️empties-the-796a2e/🦀️.rs"]
                            mod tests_empties_the_workshop_of_the_saw;
                        }
                        #[path = "."]
                        pub mod rename_machine {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖rename-machine/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖rename-machine/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖rename-machine/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖rename-machine/🧪️tests/🏷️retitles-the-saw/🦀️.rs"]
                            mod tests_retitles_the_saw;
                        }
                        #[path = "."]
                        pub mod change_machine_icon {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-machine-icon/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-machine-icon/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-machine-icon/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-machine-icon/🧪️tests/🪚️swaps-the-saw-9debd1/🦀️.rs"]
                            mod tests_swaps_the_saw_icon;
                        }
                        #[path = "."]
                        pub mod replace_machine_capabilities {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-machine-capabilities/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-machine-capabilities/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-machine-capabilities/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-machine-capabilities/🧪️tests/🔬️t009/🦀️.rs"]
                            mod tests_trades_the_blade_cut_for_a_gated_pocket_cut;
                        }
                        #[path = "."]
                        pub mod move_stock {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-stock/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-stock/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-stock/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-stock/🧪️tests/🎈️lifts-and-tilts-the-stock/🦀️.rs"]
                            mod tests_lifts_and_tilts_the_stock;
                        }
                        #[path = "."]
                        pub mod change_stock_label {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-stock-label/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-stock-label/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-stock-label/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-stock-label/🧪️tests/🔤️relabels-the-oak-2adb5b/🦀️.rs"]
                            mod tests_relabels_the_oak_beam_as_planed;
                        }
                        #[path = "."]
                        pub mod replace_stock_solid {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-stock-solid/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-stock-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-stock-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-stock-solid/🧪️tests/🧊️reissues-the-d10326/🦀️.rs"]
                            mod tests_reissues_the_stock_brep_child_handle;
                        }
                        #[path = "."]
                        pub mod change_cursor {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-cursor/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-cursor/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-cursor/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-cursor/🧪️tests/⏯️pins-the-replay-7c60ce/🦀️.rs"]
                            mod tests_pins_the_replay_cursor_to_two_steps;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod ifc {
                                    #[path = "."]
                                    pub mod v4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🏗️ifc/🔖️4/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod step {
                                    #[path = "."]
                                    pub mod v_ap214 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod stl {
                                    #[path = "."]
                                    pub mod v_ascii {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod gltf {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod obj {
                                    #[path = "."]
                                    pub mod v3_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod ifc {
                                    #[path = "."]
                                    pub mod v4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🏗️ifc/🔖️4/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod step {
                                    #[path = "."]
                                    pub mod v_ap214 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod stl {
                                    #[path = "."]
                                    pub mod v_ascii {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod gltf {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod obj {
                                    #[path = "."]
                                    pub mod v3_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod io {
    pub use super::standards::v1::subsets::any::io::*;
}
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
}
pub mod document_dsl {
    pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
}
pub mod spr {
    pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::diff::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
    }
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::mutations::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
    }
}
pub mod snapshot {
    pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
    }
}

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod process3d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️camera/🦀️.rs"]
            pub mod camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️contribution/🦀️.rs"]
            pub mod contribution;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️cursor/🦀️.rs"]
            pub mod cursor;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️engagement/🦀️.rs"]
            pub mod engagement;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️inspector/🦀️.rs"]
            pub mod inspector;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️media/🦀️.rs"]
            pub mod media;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️step/🦀️.rs"]
            pub mod step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪵️stock/🦀️.rs"]
            pub mod stock;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☀️sun/🦀️.rs"]
            pub mod sun;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛠️workshop/🦀️.rs"]
            pub mod workshop;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌍️world/🦀️.rs"]
            pub mod world;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod workpiece {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/☑️options/☀️sun/🦀️.rs"]
                            pub mod sun;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛠️workshop/🦀️.rs"]
            pub mod workshop;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod process3d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod workpiece {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪚️workpiece/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
