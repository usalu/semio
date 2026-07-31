//! 🪚️ Process 3d app — document entities (constitutional: general).

use protocol::{Identified, Patchable};
use serde::{Deserialize, Serialize};

pub const PROCESS_3D_SCHEMA: &str = "process.3d";

//#region 🔖️Document
fn default_axis_z() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

fn default_true() -> bool {
    true
}

/// 🧭️ Position + axis-angle rotation applied via the brep kernel's `rotate_sync`/`translate_sync`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

/// 📦️ Primitive solid spec resolvable via `BrepkitKernel::*_prim_sync`, or a non-parametric imported
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
    ImportedMesh {
        mesh_url: String,
    },
    /// 🧊️ STEP/OBJ/STL-imported solid with real B-Rep topology, resolved through the app's kernel
    /// session by handle id (mirrors `cad`'s `solidHandle` pattern); ephemeral to that session.
    ImportedSolid {
        solid_handle: String,
    },
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
    Cut { tool: SolidSpec, #[dsl(block)] pose: Pose },
    /// 🕳️ Subtractive: a cylinder of `radius`×`depth` subtracted at `pose` (axis = drill direction).
    Drill { #[dsl(unit = "m")] radius: f64, #[dsl(unit = "m")] depth: f64, #[dsl(block)] pose: Pose },
    /// 🔩️ Additive: fuses another component solid at `pose`.
    Attach { component: SolidSpec, #[dsl(block)] pose: Pose },
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

/// 🏭️ Provenance: which module/machine/modification-kind produced a step (display + future re-validation).
/// Purely informational — kernel replay only ever reads `ProcessMeasure`, never resolves this back to a
/// catalog entry, so an older/renamed catalog can never retroactively change already-authored geometry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct StepOrigin {
    pub module_id: String,
    pub machine_id: String,
    pub modification_kind_id: String,
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

/// 🪚️ Process 3d projection: stock + ordered steps + timeline cursor.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "process3d", layout = "lines")]
pub struct Process3dDocument {
    #[serde(default)]
    #[dsl(block)]
    pub stock: Stock,
    #[serde(default)]
    pub steps: Vec<ProcessStep>,
    /// ⏱️ Number of enabled steps replayed (0..=steps.len()); `None` applies all.
    #[serde(default)]
    pub resolved_up_to: Option<usize>,
}

pub fn empty_process3d_projection() -> Process3dDocument {
    Process3dDocument::default()
}
//#endregion 🔖️Document

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
}
//#endregion 🧪️Tests
