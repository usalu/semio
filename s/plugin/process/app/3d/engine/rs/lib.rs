//! ⚙️ Process 3d app — headless compute (constitutional: engine).

use base64::Engine;
use kernel_3d_brepkit::{BrepkitKernel, ObjSolidExporter, ObjSolidImporter, SolidExporter, SolidImporter, StepSolidExporter, StepSolidImporter, StlSolidExporter, StlSolidImporter};
use kernel_3d_engine::{BrepKernel, GeometryHandle};
use process_3d::{Pose, Process3dDocument, ProcessMeasure, ProcessStep, SolidSpec, Stock};
use semio_framework_plugin::{MeshData, MeshExporter, MeshImporter};
use serde::Serialize;
use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};
use store::DocumentDsl;

/// 🕳️ Tessellation tolerance for kernel replay/export.
const PROCESS3D_TESSELLATION_TOLERANCE: f64 = 0.05;
/// 🧠 Kernel replay memo capacity (prefix signatures kept per session).
const PROCESS3D_KERNEL_MEMO_CAP: usize = 128;

//#region 🔖ExampleFixtures
pub use process_3d_dsl::{PROCESS_3D_PLATE_EXAMPLE_TEXT as PLATE_EXAMPLE_DSL, PROCESS_3D_TIMBER_EXAMPLE_TEXT as TIMBER_EXAMPLE_DSL};

pub fn default_document() -> Process3dDocument {
    Process3dDocument::parse_dsl(TIMBER_EXAMPLE_DSL).unwrap_or_default()
}

pub fn plate_document() -> Process3dDocument {
    Process3dDocument::parse_dsl(PLATE_EXAMPLE_DSL).unwrap_or_else(|_| default_document())
}
//#endregion 🔖ExampleFixtures

//#region 🔖IdGeneration
static PROCESS3D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn next_step_id() -> String {
    format!("step-{}", PROCESS3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
}
//#endregion 🔖IdGeneration

//#region 🔖Modules
fn default_cut_measure() -> ProcessMeasure {
    ProcessMeasure::Cut { tool: SolidSpec::Box { width: 0.05, depth: 0.5, height: 0.5 }, pose: Pose::default() }
}

fn default_drill_measure() -> ProcessMeasure {
    ProcessMeasure::Drill { radius: 0.05, depth: 0.3, pose: Pose::default() }
}

fn default_attach_measure() -> ProcessMeasure {
    ProcessMeasure::Attach { component: SolidSpec::Cylinder { radius: 0.03, height: 0.2 }, pose: Pose::default() }
}

/// 🔧 A named numeric machine parameter (e.g. blade diameter) — sizes the tool geometry a
/// modification kind builds and gates which modifications are legal against the current stock.
pub struct Capability {
    pub id: &'static str,
    pub label: &'static str,
    pub value: f64,
}

/// 🪚 Which kernel-level geometry operation a modification kind produces — `ProcessMeasure`'s three
/// existing shapes are the fixed, small vocabulary every machine ultimately maps onto.
#[derive(Clone, Copy, PartialEq)]
pub enum MeasureKind {
    Cut,
    Drill,
    Attach,
}

/// 📏 A stock dimension a validation rule checks against a capability value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TargetQuantity {
    StockWidth,
    StockDepth,
    StockHeight,
}

/// ✅ "quantity must be at least/at most the named capability's value (± margin)" — a modification
/// kind's rules are ANDed together, e.g. crosscut needs stock width AND height above the blade diameter.
pub enum ValidationRule {
    MinAgainstCapability { quantity: TargetQuantity, capability: &'static str, margin: f64 },
    MaxAgainstCapability { quantity: TargetQuantity, capability: &'static str, margin: f64 },
}

/// 📐 The stock dimensions a validation rule is checked against.
#[derive(Clone, Copy)]
pub struct ValidationContext {
    pub stock_width: f64,
    pub stock_depth: f64,
    pub stock_height: f64,
}

/// 🚫 One failed validation rule, with the actual vs. required value for a human-readable reason.
#[derive(Debug)]
pub struct ValidationFailure {
    pub quantity: TargetQuantity,
    pub actual: f64,
    pub required: f64,
    pub is_min: bool,
}

/// 🪚 One thing a machine can do (e.g. "crosscut"), producing `measure_kind` geometry sized from
/// the machine's capabilities, gated by `rules`.
pub struct ModificationKind {
    pub id: &'static str,
    pub label: &'static str,
    pub icon_id: &'static str,
    pub measure_kind: MeasureKind,
    pub rules: &'static [ValidationRule],
}

/// 🛠️ A tool (e.g. a circular saw) with capabilities and the modification kinds it offers.
pub struct Machine {
    pub id: &'static str,
    pub label: &'static str,
    pub icon_id: &'static str,
    pub capabilities: &'static [Capability],
    pub modification_kinds: &'static [ModificationKind],
}

/// 📦 A domain-specific bundle of machines (e.g. "wood", "concrete"); `geometry` is the generic default.
pub struct Module {
    pub id: &'static str,
    pub label: &'static str,
    pub machines: &'static [Machine],
}

pub const GEOMETRY_SAW: Machine = Machine { id: "saw", label: "Generic Saw", icon_id: "scissors", capabilities: &[], modification_kinds: &[ModificationKind { id: "cut", label: "Cut", icon_id: "scissors", measure_kind: MeasureKind::Cut, rules: &[] }] };
pub const GEOMETRY_DRILL: Machine =
    Machine { id: "drill", label: "Generic Drill", icon_id: "circle-dot", capabilities: &[], modification_kinds: &[ModificationKind { id: "drill", label: "Drill", icon_id: "circle-dot", measure_kind: MeasureKind::Drill, rules: &[] }] };
pub const GEOMETRY_ATTACHER: Machine =
    Machine { id: "attacher", label: "Generic Attacher", icon_id: "plus", capabilities: &[], modification_kinds: &[ModificationKind { id: "attach", label: "Attach", icon_id: "plus", measure_kind: MeasureKind::Attach, rules: &[] }] };
pub const GEOMETRY_MODULE: Module = Module { id: "geometry", label: "Geometry", machines: &[GEOMETRY_SAW, GEOMETRY_DRILL, GEOMETRY_ATTACHER] };

pub const CROSSCUT_RULES: &[ValidationRule] =
    &[ValidationRule::MinAgainstCapability { quantity: TargetQuantity::StockWidth, capability: "diameter", margin: 0.0 }, ValidationRule::MinAgainstCapability { quantity: TargetQuantity::StockHeight, capability: "diameter", margin: 0.0 }];

pub const WOOD_CIRCULAR_SAW: Machine = Machine {
    id: "circularSaw",
    label: "Circular Saw",
    icon_id: "scissors",
    capabilities: &[Capability { id: "diameter", label: "Diameter", value: 0.184 }],
    modification_kinds: &[ModificationKind { id: "crosscut", label: "Crosscut", icon_id: "scissors", measure_kind: MeasureKind::Cut, rules: CROSSCUT_RULES }],
};
pub const WOOD_TABLE_SAW: Machine = Machine {
    id: "tableSaw",
    label: "Table Saw",
    icon_id: "scissors",
    capabilities: &[Capability { id: "diameter", label: "Diameter", value: 0.315 }],
    modification_kinds: &[ModificationKind { id: "crosscut", label: "Crosscut", icon_id: "scissors", measure_kind: MeasureKind::Cut, rules: CROSSCUT_RULES }],
};
pub const WOOD_MODULE: Module = Module { id: "wood", label: "Wood", machines: &[WOOD_CIRCULAR_SAW, WOOD_TABLE_SAW] };

pub const CONCRETE_DIAMOND_SAW: Machine = Machine {
    id: "diamondSaw",
    label: "Diamond Saw",
    icon_id: "scissors",
    capabilities: &[Capability { id: "diameter", label: "Diameter", value: 0.35 }],
    modification_kinds: &[ModificationKind { id: "crosscut", label: "Crosscut", icon_id: "scissors", measure_kind: MeasureKind::Cut, rules: CROSSCUT_RULES }],
};
pub const CONCRETE_MODULE: Module = Module { id: "concrete", label: "Concrete", machines: &[CONCRETE_DIAMOND_SAW] };

pub const ALL_MODULES: &[Module] = &[GEOMETRY_MODULE, WOOD_MODULE, CONCRETE_MODULE];

/// 🕳️ Kerf/thickness of a machine-built disc cut tool (crosscut etc.) — the tool's extent along its own normal.
const CROSSCUT_KERF: f64 = 0.05;

pub fn find_modification(module_id: &str, machine_id: &str, modification_kind_id: &str) -> Option<(&'static Module, &'static Machine, &'static ModificationKind)> {
    let module = ALL_MODULES.iter().find(|module| module.id == module_id)?;
    let machine = module.machines.iter().find(|machine| machine.id == machine_id)?;
    let kind = machine.modification_kinds.iter().find(|kind| kind.id == modification_kind_id)?;
    Some((module, machine, kind))
}

/// 🔎 Finds the geometry module's machine offering a given `measure` kind ("cut"/"drill"/"attach")
/// — the routing target for the utility bar, click/drag placement, and module-less `addStep` callers.
pub fn geometry_machine_for_measure(measure_kind: MeasureKind) -> (&'static Machine, &'static ModificationKind) {
    for machine in GEOMETRY_MODULE.machines {
        for kind in machine.modification_kinds {
            if kind.measure_kind == measure_kind {
                return (machine, kind);
            }
        }
    }
    unreachable!("every MeasureKind has a generic geometry machine")
}

fn capability_value(machine: &Machine, capability_id: &str) -> Option<f64> {
    machine.capabilities.iter().find(|capability| capability.id == capability_id).map(|capability| capability.value)
}

pub fn validate_modification(machine: &Machine, kind: &ModificationKind, ctx: &ValidationContext) -> Vec<ValidationFailure> {
    kind.rules
        .iter()
        .filter_map(|rule| {
            let (quantity, capability, margin, is_min) = match rule {
                ValidationRule::MinAgainstCapability { quantity, capability, margin } => (*quantity, *capability, *margin, true),
                ValidationRule::MaxAgainstCapability { quantity, capability, margin } => (*quantity, *capability, *margin, false),
            };
            let actual = match quantity {
                TargetQuantity::StockWidth => ctx.stock_width,
                TargetQuantity::StockDepth => ctx.stock_depth,
                TargetQuantity::StockHeight => ctx.stock_height,
            };
            let value = capability_value(machine, capability)?;
            let required = if is_min { value + margin } else { value - margin };
            let ok = if is_min { actual >= required } else { actual <= required };
            if ok {
                None
            } else {
                Some(ValidationFailure { quantity, actual, required, is_min })
            }
        })
        .collect()
}

pub fn validation_reason(failures: &[ValidationFailure]) -> String {
    failures
        .iter()
        .map(|failure| {
            let axis = match failure.quantity {
                TargetQuantity::StockWidth => "width",
                TargetQuantity::StockDepth => "depth",
                TargetQuantity::StockHeight => "height",
            };
            let comparator = if failure.is_min { "≥" } else { "≤" };
            format!("needs stock {axis} {comparator} {:.0}mm (have {:.0}mm)", failure.required * 1000.0, failure.actual * 1000.0)
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// 📐 Imported specs carry no persisted bounding box, so validation falls back to a 1m³ approximation
/// until the kernel is consulted (matches `cad`'s extent-less fallback for handle-only objects).
pub fn stock_extent(solid: &SolidSpec) -> [f64; 3] {
    match solid {
        SolidSpec::Box { width, depth, height } => [*width, *depth, *height],
        SolidSpec::Cylinder { radius, height } => [*radius * 2.0, *radius * 2.0, *height],
        SolidSpec::Sphere { radius } => [*radius * 2.0, *radius * 2.0, *radius * 2.0],
        SolidSpec::ImportedMesh { .. } | SolidSpec::ImportedSolid { .. } => [1.0, 1.0, 1.0],
    }
}

pub fn validation_context_for_stock(stock: &Stock) -> ValidationContext {
    let [width, depth, height] = stock_extent(&stock.solid);
    ValidationContext { stock_width: width, stock_depth: depth, stock_height: height }
}

/// 🪚 Builds the `ProcessMeasure` a machine's modification kind produces — capability-parameterized
/// where the machine has one (e.g. a saw's `diameter` capability sizes a disc cut tool), otherwise
/// falling back to the generic geometry-module defaults.
pub fn measure_for_modification(machine: &Machine, kind: &ModificationKind, position: Option<[f64; 3]>) -> ProcessMeasure {
    let mut measure = match kind.measure_kind {
        MeasureKind::Cut => match capability_value(machine, "diameter") {
            Some(diameter) => ProcessMeasure::Cut { tool: SolidSpec::Cylinder { radius: diameter / 2.0, height: CROSSCUT_KERF }, pose: Pose::default() },
            None => default_cut_measure(),
        },
        MeasureKind::Drill => default_drill_measure(),
        MeasureKind::Attach => default_attach_measure(),
    };
    if let Some(position) = position {
        let pose = match &mut measure {
            ProcessMeasure::Cut { pose, .. } | ProcessMeasure::Drill { pose, .. } | ProcessMeasure::Attach { pose, .. } => pose,
        };
        pose.position = position;
    }
    measure
}
//#endregion 🔖Modules

//#region 🔖KernelReplay
fn hash_value<T: Serialize>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    if let Ok(json) = serde_json::to_string(value) {
        json.hash(&mut hasher);
    }
    hasher.finish()
}

/// 🧠 Kernel + prefix memo: `hash(stock, enabled steps[0..i])` → solid handle, so cursor scrubbing and
/// step edits only recompute the suffix that actually changed.
/// 🧊 Concrete (not boxed-trait) so `SolidExporter`/`SolidImporter` (STEP/OBJ/STL/GLB import+export)
/// can borrow `&BrepkitKernel`/`&mut BrepkitKernel` directly; `&mut BrepkitKernel` still coerces to
/// `&mut dyn BrepKernel` at every existing call site below, so the CSG replay path is unaffected.
struct ProcessKernelSession {
    kernel: BrepkitKernel,
    memo: HashMap<u64, GeometryHandle>,
    stock_signature: u64,
}

impl ProcessKernelSession {
    fn new() -> Self {
        Self { kernel: BrepkitKernel::new(), memo: HashMap::new(), stock_signature: 0 }
    }
}

static PROCESS_BREP_KERNEL: OnceLock<Mutex<ProcessKernelSession>> = OnceLock::new();

fn process_kernel_session() -> &'static Mutex<ProcessKernelSession> {
    PROCESS_BREP_KERNEL.get_or_init(|| Mutex::new(ProcessKernelSession::new()))
}

fn prefix_signature(stock_signature: u64, steps: &[&ProcessStep]) -> u64 {
    let mut hasher = DefaultHasher::new();
    stock_signature.hash(&mut hasher);
    if let Ok(json) = serde_json::to_string(steps) {
        json.hash(&mut hasher);
    }
    hasher.finish()
}

/// 📦 Builds a posed kernel solid for a spec via `*_prim_sync` → `rotate_sync` → `translate_sync`.
fn solid_for_spec(kernel: &mut dyn BrepKernel, spec: &SolidSpec, pose: &Pose) -> Option<GeometryHandle> {
    let base = match spec {
        SolidSpec::Box { width, depth, height } => kernel_3d_engine::block_on(kernel.box_prim(*width, *depth, *height)).ok()?,
        SolidSpec::Cylinder { radius, height } => kernel_3d_engine::block_on(kernel.cylinder_prim(*radius, *height)).ok()?,
        SolidSpec::Sphere { radius } => kernel_3d_engine::block_on(kernel.sphere_prim(*radius)).ok()?,
        SolidSpec::ImportedSolid { solid_handle } => {
            let handle = GeometryHandle(solid_handle.clone());
            kernel_3d_engine::block_on(kernel.kind(&handle)).ok()?;
            handle
        }
        // 🖼️ A GLB-imported reference mesh has no real B-Rep topology in the kernel, so it cannot
        // serve as a CSG operand (stock or tool); the stock-level fallback handles display instead.
        SolidSpec::ImportedMesh { .. } => return None,
    };
    let rotated = if pose.angle != 0.0 { kernel_3d_engine::block_on(kernel.rotate(&base, pose.axis, pose.angle)).ok()? } else { base };
    if pose.position != [0.0, 0.0, 0.0] {
        kernel_3d_engine::block_on(kernel.translate(&rotated, pose.position)).ok()
    } else {
        Some(rotated)
    }
}

/// 🧭 Axis-angle rotation that maps world-up `[0,0,1]` onto an arbitrary unit `normal`, so a box
/// primitive's local Z axis (its `height` dimension) ends up flush with a picked face's normal.
pub fn axis_angle_from_up_to(normal: [f64; 3]) -> ([f64; 3], f64) {
    const UP: [f64; 3] = [0.0, 0.0, 1.0];
    let dot = (UP[0] * normal[0] + UP[1] * normal[1] + UP[2] * normal[2]).clamp(-1.0, 1.0);
    if dot > 1.0 - 1e-9 {
        return ([0.0, 0.0, 1.0], 0.0);
    }
    if dot < -1.0 + 1e-9 {
        return ([1.0, 0.0, 0.0], std::f64::consts::PI);
    }
    let cross = [UP[1] * normal[2] - UP[2] * normal[1], UP[2] * normal[0] - UP[0] * normal[2], UP[0] * normal[1] - UP[1] * normal[0]];
    let len = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
    let axis = if len > 1e-9 { [cross[0] / len, cross[1] / len, cross[2] / len] } else { [0.0, 0.0, 1.0] };
    (axis, dot.acos())
}

fn tool_solid_for_measure(kernel: &mut dyn BrepKernel, measure: &ProcessMeasure) -> Option<GeometryHandle> {
    match measure {
        ProcessMeasure::Cut { tool, pose } => solid_for_spec(kernel, tool, pose),
        ProcessMeasure::Drill { radius, depth, pose } => solid_for_spec(kernel, &SolidSpec::Cylinder { radius: *radius, height: *depth }, pose),
        ProcessMeasure::Attach { component, pose } => solid_for_spec(kernel, component, pose),
    }
}

/// 🧠 Replays enabled steps up to the cursor, reusing the longest memoized prefix.
fn replay_process(session: &mut ProcessKernelSession, doc: &Process3dDocument) -> Option<GeometryHandle> {
    let stock_signature = hash_value(&doc.stock);
    if stock_signature != session.stock_signature {
        session.memo.clear();
        session.stock_signature = stock_signature;
    }
    let limit = doc.resolved_up_to.unwrap_or(doc.steps.len()).min(doc.steps.len());
    let enabled_steps: Vec<&ProcessStep> = doc.steps[..limit].iter().filter(|step| step.enabled).collect();

    let mut start = enabled_steps.len();
    let mut current: Option<GeometryHandle> = loop {
        let signature = prefix_signature(stock_signature, &enabled_steps[..start]);
        if let Some(handle) = session.memo.get(&signature) {
            break Some(handle.clone());
        }
        if start == 0 {
            break None;
        }
        start -= 1;
    };
    if current.is_none() {
        current = solid_for_spec(&mut session.kernel, &doc.stock.solid, &doc.stock.pose);
        if let Some(handle) = &current {
            session.memo.insert(prefix_signature(stock_signature, &[]), handle.clone());
        }
    }
    let mut handle = current?;
    for (index, step) in enabled_steps.iter().enumerate().skip(start) {
        let tool = tool_solid_for_measure(&mut session.kernel, &step.measure)?;
        handle = match step.measure {
            ProcessMeasure::Attach { .. } => kernel_3d_engine::block_on(session.kernel.fuse(&handle, &tool)).ok()?,
            _ => kernel_3d_engine::block_on(session.kernel.cut(&handle, &tool)).ok()?,
        };
        session.memo.insert(prefix_signature(stock_signature, &enabled_steps[..=index]), handle.clone());
    }
    if session.memo.len() > PROCESS3D_KERNEL_MEMO_CAP {
        if let Some(key) = session.memo.keys().next().copied() {
            session.memo.remove(&key);
        }
    }
    Some(handle)
}

pub fn processed_mesh(doc: &Process3dDocument) -> Option<MeshData> {
    let mut session = process_kernel_session().lock().ok()?;
    let handle = replay_process(&mut session, doc)?;
    let mesh = kernel_3d_engine::block_on(session.kernel.tessellate(&handle, PROCESS3D_TESSELLATION_TOLERANCE)).ok()?;
    let face_groups: Vec<(u32, u32, u32)> = mesh.face_groups.iter().map(|group| (group.entity_id.parse().unwrap_or(0), group.start, group.count)).collect();
    Some(semio_framework_plugin::mesh_from_indexed_with_face_groups(&mesh.position, &mesh.normal, &mesh.index, &face_groups))
}

pub fn processed_volume(doc: &Process3dDocument) -> Option<f64> {
    let mut session = process_kernel_session().lock().ok()?;
    let handle = replay_process(&mut session, doc)?;
    kernel_3d_engine::block_on(session.kernel.volume(&handle)).ok()
}
//#endregion 🔖KernelReplay

//#region 🔖MediaImportExport
/// 📤 A pending native-geometry export ready to become a `HostEffect::DownloadMediaExport`.
pub struct Process3dModelExport {
    pub filename: String,
    pub data: Value,
    pub mime_type: String,
    pub encoding: Option<String>,
}

/// 📤 Encodes the replayed stock through `format`'s codec. STEP/OBJ/STL go through the
/// `SolidExporter` trait objects (real B-Rep, exact where the format allows it); GLB goes through
/// the mesh tessellation bridge (`processed_mesh` → `GlbExporter`), matching how it is already
/// rendered/exported elsewhere in this app.
pub fn export_process3d_model(fixture: &Process3dDocument, format: &str) -> Option<Process3dModelExport> {
    if format == "glb" {
        let mesh = processed_mesh(fixture)?;
        let bytes = semio_framework_plugin::GlbExporter.export(&mesh).ok()?;
        let media_format = semio_framework_plugin::OsMediaFormat::Glb;
        return Some(Process3dModelExport {
            filename: format!("process3d.{}", media_format.as_str()),
            data: Value::String(base64::engine::general_purpose::STANDARD.encode(bytes)),
            mime_type: media_format.mime_type().into(),
            encoding: Some("base64".into()),
        });
    }
    let exporter: Box<dyn SolidExporter> = match format {
        "obj" => Box::new(ObjSolidExporter),
        "stl" => Box::new(StlSolidExporter),
        _ => Box::new(StepSolidExporter),
    };
    let mut session = process_kernel_session().lock().ok()?;
    let handle = replay_process(&mut session, fixture)?;
    let bytes = exporter.export(&session.kernel, &[handle], PROCESS3D_TESSELLATION_TOLERANCE).ok()?;
    let media_format = exporter.format();
    let binary = media_format.is_binary();
    let data = if binary { Value::String(base64::engine::general_purpose::STANDARD.encode(&bytes)) } else { Value::String(String::from_utf8(bytes).ok()?) };
    Some(Process3dModelExport { filename: format!("process3d.{}", media_format.as_str()), data, mime_type: media_format.mime_type().into(), encoding: if binary { Some("base64".into()) } else { None } })
}

/// 📦 Decodes a `requestFileOpen(readAs: "dataUrl")` payload into raw bytes.
fn process3d_bytes_from_data_url(data_url: &str) -> Option<Vec<u8>> {
    if let Some((header, encoded)) = data_url.split_once(',') {
        if header.starts_with("data:") {
            return base64::engine::general_purpose::STANDARD.decode(encoded).ok();
        }
    }
    Some(data_url.as_bytes().to_vec())
}

/// 📥 Imports a picked file into a brand-new stock-only fixture (steps cleared): STEP/OBJ/STL go
/// through the `SolidImporter` trait objects and land as `SolidSpec::ImportedSolid` (real B-Rep,
/// reusable as a Cut/Drill/Attach operand); GLB is decoded once (via the mesh tessellation bridge,
/// `GlbImporter`) purely to validate it, then kept as `SolidSpec::ImportedMesh` referencing the
/// original data url directly — it carries no exact B-Rep, so it is never re-imported into the kernel.
pub fn import_process3d_model(name: &str, data_url: &str) -> Option<Process3dDocument> {
    let bytes = process3d_bytes_from_data_url(data_url)?;
    let mut fixture = Process3dDocument::default();
    if name.ends_with(".glb") {
        semio_framework_plugin::GlbImporter.import(&bytes).ok()?;
        fixture.stock = Stock { id: "stock".into(), label: "Imported GLB".into(), solid: SolidSpec::ImportedMesh { mesh_url: data_url.into() }, pose: Pose::default() };
        return Some(fixture);
    }
    let (importer, label): (Box<dyn SolidImporter>, &str) = if name.ends_with(".stp") || name.ends_with(".step") {
        (Box::new(StepSolidImporter), "Imported STEP")
    } else if name.ends_with(".obj") {
        (Box::new(ObjSolidImporter), "Imported OBJ")
    } else if name.ends_with(".stl") {
        (Box::new(StlSolidImporter), "Imported STL")
    } else {
        return None;
    };
    let mut session = process_kernel_session().lock().ok()?;
    let handle = importer.import(&mut session.kernel, &bytes, PROCESS3D_TESSELLATION_TOLERANCE).ok()?.into_iter().next()?;
    session.memo.clear();
    session.stock_signature = 0;
    fixture.stock = Stock { id: "stock".into(), label: label.into(), solid: SolidSpec::ImportedSolid { solid_handle: handle.0 }, pose: Pose::default() };
    Some(fixture)
}
//#endregion 🔖MediaImportExport

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_document_parses_timber_example() {
        let document = default_document();
        assert_eq!(document.steps.len(), 4);
        assert!(document.resolved_up_to.is_none());
    }

    #[test]
    fn plate_document_parses_and_opens_mid_timeline() {
        let document = plate_document();
        assert_eq!(document.steps.len(), 3);
        assert_eq!(document.resolved_up_to, Some(2));
    }

    #[test]
    fn drill_reduces_volume_below_stock() {
        let mut fixture = Process3dDocument::default();
        fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
        let stock_volume = processed_volume(&fixture).expect("stock volume");
        fixture.steps.push(ProcessStep {
            id: "drill-1".into(),
            label: "Drill".into(),
            enabled: true,
            origin: None,
            measure: ProcessMeasure::Drill { radius: 0.2, depth: 1.0, pose: Pose { position: [0.0, 0.0, 0.5], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
        });
        let drilled_volume = processed_volume(&fixture).expect("drilled volume");
        assert!(drilled_volume < stock_volume, "drilled volume {drilled_volume} should be less than stock volume {stock_volume}");
    }

    #[test]
    fn attach_increases_volume_above_stock() {
        let mut fixture = Process3dDocument::default();
        fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
        let stock_volume = processed_volume(&fixture).expect("stock volume");
        fixture.steps.push(ProcessStep {
            id: "attach-1".into(),
            label: "Attach".into(),
            enabled: true,
            origin: None,
            measure: ProcessMeasure::Attach { component: SolidSpec::Sphere { radius: 0.3 }, pose: Pose { position: [1.0, 0.0, 0.5], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
        });
        let attached_volume = processed_volume(&fixture).expect("attached volume");
        assert!(attached_volume > stock_volume, "attached volume {attached_volume} should exceed stock volume {stock_volume}");
    }

    #[test]
    fn disabled_step_is_skipped_on_replay() {
        let mut fixture = Process3dDocument::default();
        fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
        let stock_volume = processed_volume(&fixture).expect("stock volume");
        fixture.steps.push(ProcessStep { id: "drill-1".into(), label: "Drill".into(), enabled: false, origin: None, measure: ProcessMeasure::Drill { radius: 0.2, depth: 1.0, pose: Pose::default() } });
        let volume_with_disabled_step = processed_volume(&fixture).expect("volume");
        assert!((volume_with_disabled_step - stock_volume).abs() < 1e-6);
    }

    #[test]
    fn cursor_zero_yields_stock_volume() {
        let mut fixture = Process3dDocument::default();
        fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
        let stock_volume = processed_volume(&fixture).expect("stock volume");
        fixture.steps.push(ProcessStep { id: "drill-1".into(), label: "Drill".into(), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.2, depth: 1.0, pose: Pose::default() } });
        fixture.resolved_up_to = Some(0);
        let volume_at_cursor_zero = processed_volume(&fixture).expect("volume");
        assert!((volume_at_cursor_zero - stock_volume).abs() < 1e-6);
    }

    #[test]
    fn face_drag_orients_box_along_normal() {
        let (axis, angle) = axis_angle_from_up_to([0.0, 1.0, 0.0]);
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
        assert!((axis[0] - (-1.0)).abs() < 1e-9 && axis[1].abs() < 1e-9 && axis[2].abs() < 1e-9);
    }

    #[test]
    fn face_drag_degenerate_antiparallel_normal_does_not_panic() {
        let (_, angle) = axis_angle_from_up_to([0.0, 0.0, -1.0]);
        assert!((angle - std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn box_primitive_spans_from_local_origin_corner() {
        let mut kernel = BrepkitKernel::new();
        let handle = kernel_3d_engine::block_on(kernel.box_prim(2.0, 3.0, 4.0)).expect("box prim");
        let mesh = kernel_3d_engine::block_on(kernel.tessellate(&handle, 0.1)).expect("tessellate");
        let axis_bounds = |offset: usize| -> (f32, f32) {
            let values: Vec<f32> = mesh.position.iter().skip(offset).step_by(3).copied().collect();
            (values.iter().cloned().fold(f32::INFINITY, f32::min), values.iter().cloned().fold(f32::NEG_INFINITY, f32::max))
        };
        let (min_x, max_x) = axis_bounds(0);
        let (min_y, max_y) = axis_bounds(1);
        let (min_z, max_z) = axis_bounds(2);
        assert!(min_x.abs() < 1e-4 && (max_x - 2.0).abs() < 1e-4, "box x should span [0, width] from the local origin corner, got [{min_x}, {max_x}]");
        assert!(min_y.abs() < 1e-4 && (max_y - 3.0).abs() < 1e-4, "box y should span [0, depth], got [{min_y}, {max_y}]");
        assert!(min_z.abs() < 1e-4 && (max_z - 4.0).abs() < 1e-4, "box z should span [0, height], got [{min_z}, {max_z}]");
    }

    #[test]
    fn kernel_replay_memoizes_prefixes_across_cursor_scrub() {
        let mut fixture = Process3dDocument::default();
        fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
        fixture.steps.push(ProcessStep { id: "drill-1".into(), label: "Drill".into(), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.1, depth: 1.0, pose: Pose::default() } });
        fixture.resolved_up_to = Some(1);
        processed_volume(&fixture).expect("volume at cursor 1");
        let session = process_kernel_session().lock().expect("kernel session lock");
        assert!(session.memo.len() >= 2, "expected stock + drilled prefixes memoized, got {}", session.memo.len());
    }
}
//#endregion 🧪Tests
