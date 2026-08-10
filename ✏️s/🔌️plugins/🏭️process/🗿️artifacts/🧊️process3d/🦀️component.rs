//! 🪚️ Process3d artifact — document entities (workshop machines/capabilities, stock, process steps)
//! plus this artifact's `ArtifactKindSpec`.

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, };
use serde::{Deserialize, Serialize};

pub use crate::artifacts::process3d::schema::mutations::Process3dMutation;

pub use crate::artifacts::process3d::schema::diff::Process3dDiff;

pub const PROCESS_3D_SCHEMA: &str = "process.3d";

//#region 🔖️Workshop
/// 📏️ A stock dimension a capability rule checks against a capability's own parameter value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum StockQuantity {
    #[default]
    Width,
    Depth,
    Height,
    MaxDimension,
    MinDimension,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityParameter {
    pub id: String,
    pub label: String,
    #[dsl(unit = "m")]
    pub value: f64,
}

/// 🪚️ How a capability's parameters build a kernel `ProcessMeasure` — every field names a
/// `Capability::parameters` entry by id, resolved at measure-build time; `measure_kind()` derives the
/// fixed Cut/Drill/Attach effect so it never needs to be stored redundantly alongside the recipe.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "recipe", rename_all = "camelCase", rename_all_fields = "camelCase")]
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

/// 🌉️ Same reasoning/idiom as `SolidSpec`'s and `ProcessMeasure`'s hand `dsl::DslField` impls (see
/// `SolidSpec`'s doc comment) — `MeasureRecipe` is a `DslEnum` (`DslVariants` only), and
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Capability {
    pub id: String,
    pub label: String,
    pub icon_id: String,
    pub recipe: MeasureRecipe,
    #[serde(default)]
    pub parameters: Vec<CapabilityParameter>,
    #[serde(default)]
    #[dsl(statements, block)]
    pub rules: Vec<CapabilityRule>,
}

/// 🛠️ A machine in the document's workshop — an embedded snapshot, never a reference; consistent with
/// `StepOrigin`'s never-resolve invariant (see its doc comment), and robust to catalog drift: editing
/// or removing an installed catalog can never retroactively change an already-configured workshop.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorkshopMachine {
    pub id: String,
    pub label: String,
    pub icon_id: String,
    /// 🏷️ Which installed catalog this snapshot was seeded from — informational only, never resolved
    /// (a machine stays fully usable after its source catalog is uninstalled).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(default)]
    pub capabilities: Vec<Capability>,
}

impl Identified<String> for WorkshopMachine {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹️ Sparse edit for a `WorkshopMachine` — `None` fields are left untouched.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorkshopMachinePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Workshop {
    #[serde(default)]
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
/// install into a document's workshop. Implemented by the built-in generic catalog
/// (`crate::artifacts::process3d::engine`) and by each built-in domain catalog module
/// (`crate::artifacts::process3d::engine::catalogs::{wood,concrete,metal,robotic}`).
pub trait MachineCatalog {
    fn catalog_id(&self) -> &'static str;
    fn label(&self) -> &'static str;
    fn icon_id(&self) -> &'static str;
    fn machines(&self) -> Vec<WorkshopMachine>;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Pose {
    #[serde(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[serde(default = "default_axis_z")]
    #[dsl(dir)]
    pub axis: [f64; 3],
    #[serde(default)]
    #[dsl(angle = "rad")]
    pub angle: f64,
}

impl Default for Pose {
    fn default() -> Self {
        Self { position: [0.0, 0.0, 0.0], axis: default_axis_z(), angle: 0.0 }
    }
}

/// 📦️ Primitive solid spec resolvable via `Brep::*_prim_sync`, or a non-parametric imported
/// reference (mesh or real B-Rep solid) resolved by the app's own kernel session instead of a primitive.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SolidSpec {
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
    /// 🖼️ Non-parametric GLB-imported reference mesh — tessellation-only, no real B-Rep topology
    /// (mirrors `cad`'s `meshUrl` pattern); cannot serve as a Cut/Drill/Attach tool.
    ImportedMesh { mesh_url: String },
    /// 🧊️ STEP/OBJ/STL-imported solid with real B-Rep topology, resolved through the app's kernel
    /// session by handle id (mirrors `cad`'s `solidHandle` pattern); ephemeral to that session.
    ImportedSolid { solid_handle: String },
}

/// 🌉️ `#[derive(dsl::DslEnum)]` only gives `SolidSpec` a `dsl::DslVariants` binding (a tagged-record
/// table), not `dsl::DslField` — so it can't sit directly in a plain (non-`Option`/`Vec`) field on
/// its own. Every real usage site (`Stock::solid`, `ProcessMeasure::Cut::tool`,
/// `ProcessMeasure::Attach::component`) is a REQUIRED, never-optional, never-collection single value,
/// which the derive macro would normally solve via `#[dsl(statements)] Box<SolidSpec>` — but boxing
/// would change the field's Rust-visible type and break `process/plugin`'s existing pattern
/// matches/struct literals against a bare `SolidSpec`. This hand impl reuses the exact same "exactly
/// one tagged statement" idiom the derive's `Box<T>`-required-statements codegen uses internally,
/// applied directly to `SolidSpec` so every real field stays unboxed.
impl dsl::DslField for SolidSpec {
    fn shape() -> dsl::Shape {
        dsl::Shape::Statements(<SolidSpec as dsl::DslVariants>::variants())
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Statements(vec![<SolidSpec as dsl::DslVariants>::to_named_record(self)])
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Statements(items) if items.len() == 1 => <SolidSpec as dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|e| e.message),
            other => Err(format!("expected exactly 1 tagged solid value, found {other:?}")),
        }
    }
}

/// 🪵️ The raw workpiece the process starts from.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Stock {
    pub id: String,
    pub label: String,
    pub solid: SolidSpec,
    #[serde(default)]
    #[dsl(block)]
    pub pose: Pose,
}

impl Default for Stock {
    fn default() -> Self {
        Self { id: "stock".into(), label: "Stock".into(), solid: SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }
    }
}

/// 🪚️ One processing measure: subtractive (cut/drill via `cut_sync`) or additive (attach via `fuse_sync`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "measure", rename_all = "camelCase")]
pub enum ProcessMeasure {
    /// ✂️ Subtractive: subtracts an arbitrary tool solid (e.g. a thin box as a saw blade).
    Cut {
        tool: SolidSpec,
        #[dsl(block)]
        pose: Pose,
    },
    /// 🕳️ Subtractive: a cylinder of `radius`×`depth` subtracted at `pose` (axis = drill direction).
    Drill {
        #[dsl(unit = "m")]
        radius: f64,
        #[dsl(unit = "m")]
        depth: f64,
        #[dsl(block)]
        pose: Pose,
    },
    /// 🔩️ Additive: fuses another component solid at `pose`.
    Attach {
        component: SolidSpec,
        #[dsl(block)]
        pose: Pose,
    },
}

/// 🌉️ Same reasoning/idiom as `SolidSpec`'s hand `dsl::DslField` impl — `ProcessMeasure` is a
/// `DslEnum` (`DslVariants` only), and `ProcessStep::measure` is a REQUIRED, never-optional field
/// that must stay a bare `ProcessMeasure` (not `Box<ProcessMeasure>`) for `process/plugin`'s existing
/// `match &mut step.measure { ProcessMeasure::Cut { .. } => .. }` usage to keep compiling untouched.
impl dsl::DslField for ProcessMeasure {
    fn shape() -> dsl::Shape {
        dsl::Shape::Statements(<ProcessMeasure as dsl::DslVariants>::variants())
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Statements(vec![<ProcessMeasure as dsl::DslVariants>::to_named_record(self)])
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Statements(items) if items.len() == 1 => <ProcessMeasure as dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|e| e.message),
            other => Err(format!("expected exactly 1 tagged measure value, found {other:?}")),
        }
    }
}

/// 🏭️ Provenance: which workshop machine/capability produced a step (display + future re-validation).
/// Purely informational — kernel replay only ever reads `ProcessMeasure`, never resolves this back to a
/// workshop entry, so editing or removing the machine/capability can never retroactively change
/// already-authored geometry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct StepOrigin {
    pub machine_id: String,
    pub capability_id: String,
}

/// 🎞️ One ordered step of the process timeline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStep {
    pub id: String,
    pub label: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub origin: Option<StepOrigin>,
    /// 🌉️ `#[serde(flatten)]` is a JSON-only concern (`dsl(flatten)` is a dead/unimplemented derive
    /// flag) — the DSL grammar just gives `measure` its own ordinary tagged shape via `ProcessMeasure`'s
    /// hand `dsl::DslField` impl (see its doc comment), printed as its own `cut|drill|attach ...`
    /// statement on the step's line.
    #[serde(flatten)]
    pub measure: ProcessMeasure,
}

impl Identified<String> for ProcessStep {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹️ Sparse edit for a `ProcessStep` — `None` fields are left untouched.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStepPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measure: Option<ProcessMeasure>,
    /// 🏭️ Outer `Option` = "this patch touches origin"; inner `Option` = the new value (`None` clears it).
    #[serde(default, skip_serializing_if = "Option::is_none")]
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

/// 🪚️ Process 3d projection: workshop + stock + ordered steps + timeline cursor.
/// 📸️ Persisted process3d snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::artifacts::process3d::schema::snapshot::Process3dSnapshot;

/// 🗄️ Empty process3d snapshot (default workshop + stock, no steps).
pub fn empty_process3d_snapshot() -> Process3dSnapshot {
    Process3dSnapshot::default()
}
//#endregion 🔖️Document

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::process3d::create_process3d_app`'s `🔖️Manifest` region.
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
            export_stdio_kinds: vec!["stdio.dwg", "stdio.glb", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.glb", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_step_json_without_origin_deserializes_with_none() {
        let legacy_json = r#"{"id":"cut-1","label":"Cut","enabled":true,"measure":"cut","tool":{"kind":"box","width":0.1,"depth":0.1,"height":0.1},"pose":{"position":[0.0,0.0,0.0],"axis":[0.0,0.0,1.0],"angle":0.0}}"#;
        let step: ProcessStep = serde_json::from_str(legacy_json).expect("legacy step json");
        assert!(step.origin.is_none());
        assert_eq!(step.id, "cut-1");
    }

    #[test]
    fn imported_mesh_solid_spec_round_trips_json() {
        let solid = SolidSpec::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() };
        let json = serde_json::to_value(&solid).expect("serialize");
        assert_eq!(json["kind"], "importedMesh");
        assert_eq!(json["meshUrl"], "data:model/gltf-binary;base64,AAAA");
        let parsed: SolidSpec = serde_json::from_value(json).expect("deserialize");
        assert_eq!(parsed, solid);
    }

    #[test]
    fn imported_solid_solid_spec_round_trips_json() {
        let solid = SolidSpec::ImportedSolid { solid_handle: "solid-42".into() };
        let json = serde_json::to_value(&solid).expect("serialize");
        assert_eq!(json["kind"], "importedSolid");
        assert_eq!(json["solidHandle"], "solid-42");
        let parsed: SolidSpec = serde_json::from_value(json).expect("deserialize");
        assert_eq!(parsed, solid);
    }

    #[test]
    fn artifact_kind_declares_the_expected_media_surface() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "3d.process");
        assert_eq!(kind.schema, PROCESS_3D_SCHEMA);
        assert_eq!(kind.export_formats.len(), 4);
        assert_eq!(kind.import_formats.len(), 3);
    }

    //#region 🔖️WorkshopTests
    fn sample_capability() -> Capability {
        Capability {
            id: "crosscut".into(),
            label: "Crosscut".into(),
            icon_id: "scissors".into(),
            recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
            parameters: vec![CapabilityParameter { id: "bladeDiameter".into(), label: "Blade Diameter".into(), value: 0.184 }, CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.002 }],
            rules: vec![CapabilityRule::Min { quantity: StockQuantity::Width, parameter: "bladeDiameter".into(), margin: 0.0 }, CapabilityRule::Max { quantity: StockQuantity::Height, parameter: "bladeDiameter".into(), margin: 0.05 }],
        }
    }

    fn sample_workshop() -> Workshop {
        Workshop { machines: vec![WorkshopMachine { id: "circularSaw".into(), label: "Circular Saw".into(), icon_id: "scissors".into(), catalog_id: Some("wood".into()), capabilities: vec![sample_capability()] }] }
    }

    /// 📜️ The document's deepest new nesting (workshop → machines → capabilities → parameters/rules,
    /// 3 `Vec` levels deep) must round-trip through the DSL text codec — the riskiest new grammar surface.
    #[test]
    fn workshop_dsl_round_trips_through_document() {
        let snapshot = Process3dSnapshot { workshop: sample_workshop(), ..Process3dSnapshot::default() };
        store::os_store::test_support::assert_dsl_round_trip(&snapshot);
    }

    #[test]
    fn default_workshop_has_the_three_generic_machines() {
        let workshop = Workshop::default();
        let ids: Vec<&str> = workshop.machines.iter().map(|machine| machine.id.as_str()).collect();
        assert_eq!(ids, ["saw", "drill", "attacher"]);
        assert!(workshop.machines.iter().all(|machine| machine.catalog_id.is_none()));
    }

    #[test]
    fn document_without_workshop_field_deserializes_to_generic_workshop() {
        let legacy_json = r#"{"stock":{"id":"stock","label":"Stock","solid":{"kind":"box","width":1.0,"depth":1.0,"height":1.0},"pose":{"position":[0.0,0.0,0.0],"axis":[0.0,0.0,1.0],"angle":0.0}},"steps":[],"resolvedUpTo":null}"#;
        let snapshot: Process3dSnapshot = serde_json::from_str(legacy_json).expect("legacy document json");
        assert_eq!(snapshot.workshop, Workshop::default());
    }

    #[test]
    fn workshop_machine_patch_apply_and_diff_round_trip() {
        let mut machine = WorkshopMachine { id: "circularSaw".into(), label: "Circular Saw".into(), icon_id: "scissors".into(), catalog_id: Some("wood".into()), capabilities: vec![sample_capability()] };
        let original = machine.clone();
        let patch = WorkshopMachinePatch { label: Some("Big Saw".into()), icon_id: None, capabilities: None };
        machine.apply_patch(&patch);
        assert_eq!(machine.label, "Big Saw");
        assert_eq!(machine.capabilities, original.capabilities);
        let diff = original.diff_patch(&machine).expect("diff");
        assert_eq!(diff, patch);
    }

    #[test]
    fn workshop_machine_patch_diff_is_none_for_identical_machines() {
        let machine = WorkshopMachine { id: "circularSaw".into(), label: "Circular Saw".into(), icon_id: "scissors".into(), catalog_id: None, capabilities: vec![] };
        assert!(machine.diff_patch(&machine).is_none());
    }
    //#endregion 🔖️WorkshopTests
}
//#endregion 🧪️Tests
