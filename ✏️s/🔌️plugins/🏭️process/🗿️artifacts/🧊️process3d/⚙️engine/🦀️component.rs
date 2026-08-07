//! ⚙️ Process3d artifact — headless compute (constitutional: engine): kernel replay, the built-in
//! machine catalog registry, capability validation, media import/export, and this app's plugin `register()`.

use base64::Engine;
use crate::artifacts::process3d::{
    Capability, MachineCatalog, MeasureKind, MeasureRecipe, Pose, Process3dDocument, ProcessMeasure, ProcessStep, SolidSpec, Stock, StockQuantity, Workshop, WorkshopMachine,
};
use semio_s_3d::brep::kernel::{Brep, ObjSolidExporter, ObjSolidImporter, SolidExporter, SolidImporter, StepSolidExporter, StepSolidImporter, StlSolidExporter, StlSolidImporter};
use semio_s_3d::brep::engine::{BrepEngineHost, BrepKernel, GeometryHandle};
use semio_framework_plugin::{MeshData, MeshExporter, MeshImporter};
use serde::Serialize;
use serde_json::Value;
use semio_framework::{parse_contributions, Contribution};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;
use store::DocumentDsl;

/// 🕳️ Tessellation tolerance for kernel replay/export.
const PROCESS3D_TESSELLATION_TOLERANCE: f64 = 0.05;
/// 🧠️ Kernel replay memo capacity (prefix signatures kept per session).
const PROCESS3D_KERNEL_MEMO_CAP: usize = 128;

//#region 🔖️Plugin
/// 🔌️ Registers this app's document exporters/import handlers and codec with the OS runtime — the
/// `setup` hook `📦️glue.rs`'s `semio_plugin!{}` invocation calls.
pub fn register() {
    register_pilot_languages();
    fn process3d_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
        let document: Process3dDocument = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
        processed_mesh(&document).ok_or_else(|| "process3d: kernel replay failed".to_string())
    }

    fn process3d_document_from_mesh(_mesh: &MeshData) -> Result<Value, String> {
        Err("process3d: mesh import not supported".into())
    }

    semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.process", "process", process3d_mesh_from_document);
    semio_framework_os::register_mesh_dwg_import_handler("3d.process", process3d_document_from_mesh);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::process3d::Process3dPlayApp>(crate::artifacts::process3d::PROCESS_3D_SCHEMA);
}
//#endregion 🔖️Plugin

//#region 🔖️ExampleFixtures
pub use crate::artifacts::process3d::dsl::{PROCESS_3D_PLATE_EXAMPLE_TEXT as PLATE_EXAMPLE_DSL, PROCESS_3D_TIMBER_EXAMPLE_TEXT as TIMBER_EXAMPLE_DSL};

pub fn default_document() -> Process3dDocument {
    Process3dDocument::parse_dsl(TIMBER_EXAMPLE_DSL).unwrap_or_default()
}

pub fn plate_document() -> Process3dDocument {
    Process3dDocument::parse_dsl(PLATE_EXAMPLE_DSL).unwrap_or_else(|_| default_document())
}
//#endregion 🔖️ExampleFixtures

//#region 🔖️IdGeneration

pub fn next_step_id() -> String {
    format!("step-{}", &blake3::hash(concat!(file!(), line!(), "step-{}").as_bytes()).to_hex()[..12])
}
//#endregion 🔖️IdGeneration

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `crate::artifacts::process3d::
/// artifact_kind()`'s literal for `"3d.process"` (schema/media type/export+import formats/presentation
/// fields copied verbatim), plus the two workflow ports: `geometry:in` (Many, unrequired — accepts
/// upstream geometry producers, e.g. cad/lowpoly) and `brep:out` (Many, unrequired, `kind_id:
/// "3d.process"` — reusing the artifact kind already declared, never a second `.artifact_kind(...)` call).
pub fn process3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: crate::artifacts::process3d::PROCESS_3D_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Brep },
        ports: vec![
            semio_framework_plugin::MediaPortSpec {
                id: "geometry:in".into(),
                label: "Geometry".into(),
                direction: semio_framework_plugin::MediaPortDirection::In,
                media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Any },
                kind_id: None,
                required: false,
                multiplicity: semio_framework_plugin::PortMultiplicity::Many,
            },
            semio_framework_plugin::MediaPortSpec {
                id: "brep:out".into(),
                label: "Brep".into(),
                direction: semio_framework_plugin::MediaPortDirection::Out,
                media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Brep },
                kind_id: Some("3d.process".into()),
                required: false,
                multiplicity: semio_framework_plugin::PortMultiplicity::Many,
            },
        ],
        export_formats: vec![semio_framework_plugin::OsMediaFormat::Step, semio_framework_plugin::OsMediaFormat::Obj, semio_framework_plugin::OsMediaFormat::Stl, semio_framework_plugin::OsMediaFormat::Glb],
        import_formats: vec![semio_framework_plugin::OsMediaFormat::Step, semio_framework_plugin::OsMediaFormat::Obj, semio_framework_plugin::OsMediaFormat::Stl],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.process".into(), name: "3D Process".into(), dimension: "3d".into(), component_kind: "process3d".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Catalog
fn leak_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

/// 🧩️ One hot-installed machine catalog deserialized from `Contribution::ProcessMachines`.
#[derive(Clone)]
struct ContributedMachineCatalog {
    catalog_id: &'static str,
    label: &'static str,
    icon_id: &'static str,
    machines: Vec<WorkshopMachine>,
}

impl MachineCatalog for ContributedMachineCatalog {
    fn catalog_id(&self) -> &'static str {
        self.catalog_id
    }

    fn label(&self) -> &'static str {
        self.label
    }

    fn icon_id(&self) -> &'static str {
        self.icon_id
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        self.machines.clone()
    }
}

static CONTRIBUTED_MACHINE_CATALOGS: Mutex<Vec<ContributedMachineCatalog>> = Mutex::new(Vec::new());
static LAST_PROCESS_CONTRIBUTIONS_JSON: Mutex<String> = Mutex::new(String::new());

const PROCESS3D_PLAY_APP_ID: &str = "process3d-play";

/// 🔌️ Refreshes contributed `process.machines` catalogs when the host pushes a new catalogue.
pub fn sync_process_machine_contributions(contributions_json: &str) {
    let mut last = LAST_PROCESS_CONTRIBUTIONS_JSON.lock().expect("process contributions lock");
    if *last == contributions_json {
        return;
    }
    let mut catalogs = Vec::new();
    for entry in parse_contributions(contributions_json) {
        let Contribution::ProcessMachines { app_id, module_id, label, icon_id, machines_json, .. } = entry.contribution else {
            continue;
        };
        if app_id != PROCESS3D_PLAY_APP_ID {
            continue;
        }
        let machines: Vec<WorkshopMachine> = serde_json::from_str(&machines_json).unwrap_or_default();
        catalogs.push(ContributedMachineCatalog {
            catalog_id: leak_str(module_id),
            label: leak_str(label),
            icon_id: leak_str(icon_id.to_string()),
            machines,
        });
    }
    *CONTRIBUTED_MACHINE_CATALOGS.lock().expect("process contributed catalogs lock") = catalogs;
    *last = contributions_json.to_string();
}

fn builtin_installed_catalogs() -> Vec<Box<dyn MachineCatalog>> {
    vec![Box::new(GenericCatalog)]
}

/// 📦️ The built-in generic catalog — wraps `crate::artifacts::process3d::generic_machines()`, the same
/// fallback used to seed a document's default workshop, exposed here as an installable `MachineCatalog`
/// so it appears alongside domain catalogs in the workshop configurator's "installed catalogs" list.
pub struct GenericCatalog;

impl MachineCatalog for GenericCatalog {
    fn catalog_id(&self) -> &'static str {
        "geometry"
    }

    fn label(&self) -> &'static str {
        "Geometry"
    }

    fn icon_id(&self) -> &'static str {
        "shapes"
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        crate::artifacts::process3d::generic_machines()
    }
}

/// 🧩️ Every machine catalog installed in this build, in stable display order — the built-in generic
/// catalog first (so it renders as the default-open section), then every `process.machines` contribution
/// merged via `sync_process_machine_contributions` from runtime-installable extensions under
/// `🏭️process/🧩️extensions/`.
pub fn installed_catalogs() -> Vec<Box<dyn MachineCatalog>> {
    let mut catalogs = builtin_installed_catalogs();
    let contributed = CONTRIBUTED_MACHINE_CATALOGS.lock().expect("process contributed catalogs lock");
    catalogs.extend(contributed.iter().map(|catalog| Box::new(catalog.clone()) as Box<dyn MachineCatalog>));
    catalogs
}

/// 🔎️ One machine, by catalog + machine id, with `catalog_id` stamped onto the snapshot — the
/// "install into workshop" lookup for the workshop configurator's add-machine action.
pub fn catalog_machine(catalog_id: &str, machine_id: &str) -> Option<WorkshopMachine> {
    let catalog = installed_catalogs().into_iter().find(|catalog| catalog.catalog_id() == catalog_id)?;
    let mut machine = catalog.machines().into_iter().find(|machine| machine.id == machine_id)?;
    machine.catalog_id = Some(catalog_id.to_string());
    Some(machine)
}

/// 🔎️ One workshop machine's capability, by id — the resolution target for `AddStep`'s
/// `(machine_id, capability_id)` and for re-validating a step's `StepOrigin` provenance.
pub fn find_capability<'a>(workshop: &'a Workshop, machine_id: &str, capability_id: &str) -> Option<(&'a WorkshopMachine, &'a Capability)> {
    let machine = workshop.machines.iter().find(|machine| machine.id == machine_id)?;
    let capability = machine.capabilities.iter().find(|capability| capability.id == capability_id)?;
    Some((machine, capability))
}

/// 🔎️ First workshop capability producing `kind` — the routing target for the utility bar,
/// click/drag placement, and machine-less `addStep` callers. Falls back to a fresh generic machine if
/// the workshop's generics were removed, so click-to-place utilities never dead-end.
pub fn capability_for_measure_kind(workshop: &Workshop, kind: MeasureKind) -> (WorkshopMachine, Capability) {
    for machine in &workshop.machines {
        for capability in &machine.capabilities {
            if capability.recipe.measure_kind() == kind {
                return (machine.clone(), capability.clone());
            }
        }
    }
    for machine in crate::artifacts::process3d::generic_machines() {
        for capability in machine.capabilities.iter() {
            if capability.recipe.measure_kind() == kind {
                return (machine.clone(), capability.clone());
            }
        }
    }
    unreachable!("every MeasureKind has a generic fallback machine")
}

/// 📐️ The stock dimensions a capability rule is checked against.
#[derive(Clone, Copy)]
pub struct ValidationContext {
    pub stock_width: f64,
    pub stock_depth: f64,
    pub stock_height: f64,
}

/// 🚫️ One failed capability rule, with the actual vs. required value for a human-readable reason.
#[derive(Debug)]
pub struct ValidationFailure {
    pub quantity: StockQuantity,
    pub actual: f64,
    pub required: f64,
    pub is_min: bool,
}

fn parameter_value(capability: &Capability, parameter_id: &str) -> Option<f64> {
    capability.parameters.iter().find(|parameter| parameter.id == parameter_id).map(|parameter| parameter.value)
}

fn quantity_value(ctx: &ValidationContext, quantity: StockQuantity) -> f64 {
    match quantity {
        StockQuantity::Width => ctx.stock_width,
        StockQuantity::Depth => ctx.stock_depth,
        StockQuantity::Height => ctx.stock_height,
        StockQuantity::MaxDimension => ctx.stock_width.max(ctx.stock_depth).max(ctx.stock_height),
        StockQuantity::MinDimension => ctx.stock_width.min(ctx.stock_depth).min(ctx.stock_height),
    }
}

fn quantity_label(quantity: StockQuantity) -> &'static str {
    match quantity {
        StockQuantity::Width => "width",
        StockQuantity::Depth => "depth",
        StockQuantity::Height => "height",
        StockQuantity::MaxDimension => "max dimension",
        StockQuantity::MinDimension => "min dimension",
    }
}

/// ✅️ Checks a capability's rules against the current stock — a rule whose parameter is missing from
/// the capability is skipped (lenient, matches the pre-workshop behavior), never a hard error.
pub fn validate_capability(capability: &Capability, ctx: &ValidationContext) -> Vec<ValidationFailure> {
    capability
        .rules
        .iter()
        .filter_map(|rule| {
            let (quantity, parameter_id, margin, is_min) = match rule {
                crate::artifacts::process3d::CapabilityRule::Min { quantity, parameter, margin } => (*quantity, parameter.as_str(), *margin, true),
                crate::artifacts::process3d::CapabilityRule::Max { quantity, parameter, margin } => (*quantity, parameter.as_str(), *margin, false),
            };
            let value = parameter_value(capability, parameter_id)?;
            let actual = quantity_value(ctx, quantity);
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
            let comparator = if failure.is_min { "≥" } else { "≤" };
            format!("needs stock {} {comparator} {:.0}mm (have {:.0}mm)", quantity_label(failure.quantity), failure.required * 1000.0, failure.actual * 1000.0)
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// 📐️ Imported specs carry no persisted bounding box, so validation falls back to a 1m³ approximation
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

/// 🪚️ Builds the `ProcessMeasure` a capability's recipe produces, sized from the capability's own
/// parameters (a missing parameter resolves to `0.0`, matching `validate_capability`'s lenient lookup).
pub fn measure_for_capability(capability: &Capability, position: Option<[f64; 3]>) -> ProcessMeasure {
    let value = |id: &str| parameter_value(capability, id).unwrap_or(0.0);
    let mut measure = match &capability.recipe {
        MeasureRecipe::DiscCut { diameter, kerf } => ProcessMeasure::Cut { tool: SolidSpec::Cylinder { radius: value(diameter) / 2.0, height: value(kerf) }, pose: Pose::default() },
        MeasureRecipe::BladeCut { kerf, length, depth } => ProcessMeasure::Cut { tool: SolidSpec::Box { width: value(kerf), depth: value(length), height: value(depth) }, pose: Pose::default() },
        MeasureRecipe::PocketCut { diameter, depth } => {
            let side = value(diameter);
            ProcessMeasure::Cut { tool: SolidSpec::Box { width: side, depth: side, height: value(depth) }, pose: Pose::default() }
        }
        MeasureRecipe::BoreDrill { radius, depth } => ProcessMeasure::Drill { radius: value(radius), depth: value(depth), pose: Pose::default() },
        MeasureRecipe::CylinderAttach { radius, length } => ProcessMeasure::Attach { component: SolidSpec::Cylinder { radius: value(radius), height: value(length) }, pose: Pose::default() },
        MeasureRecipe::BoxAttach { width, depth, height } => ProcessMeasure::Attach { component: SolidSpec::Box { width: value(width), depth: value(depth), height: value(height) }, pose: Pose::default() },
    };
    if let Some(position) = position {
        let pose = match &mut measure {
            ProcessMeasure::Cut { pose, .. } | ProcessMeasure::Drill { pose, .. } | ProcessMeasure::Attach { pose, .. } => pose,
        };
        pose.position = position;
    }
    measure
}
//#endregion 🔖️Catalog

//#region 🔖️KernelReplay
fn hash_value<T: Serialize>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    if let Ok(json) = serde_json::to_string(value) {
        json.hash(&mut hasher);
    }
    hasher.finish()
}

/// 🧠️ Kernel + prefix memo: `hash(stock, enabled steps[0..i])` → solid handle, so cursor scrubbing and
/// step edits only recompute the suffix that actually changed.
/// 🧊️ Concrete (not boxed-trait) so `SolidExporter`/`SolidImporter` (STEP/OBJ/STL/GLB import+export)
/// can borrow `&Brep`/`&mut Brep` directly; `&mut Brep` still coerces to
/// `&mut dyn BrepKernel` at every existing call site below, so the CSG replay path is unaffected.
struct ProcessKernelReplay {
    host: BrepEngineHost,
    tables: ProcessKernelMemo,
    stock_signature: u64,
}

struct ProcessKernelMemo {
    memo: HashMap<u64, GeometryHandle>,
}

impl ProcessKernelReplay {
    fn new() -> Self {
        Self {
            host: BrepEngineHost::new(64 * 1024 * 1024),
            tables: ProcessKernelMemo { memo: HashMap::new() },
            stock_signature: 0,
        }
    }

    fn kernel(&self) -> &std::sync::Mutex<Brep> {
        self.host.kernel()
    }
}

fn prefix_signature(stock_signature: u64, steps: &[&ProcessStep]) -> u64 {
    let mut hasher = DefaultHasher::new();
    stock_signature.hash(&mut hasher);
    if let Ok(json) = serde_json::to_string(steps) {
        json.hash(&mut hasher);
    }
    hasher.finish()
}

/// 📦️ Builds a posed kernel solid for a spec via `*_prim_sync` → `rotate_sync` → `translate_sync`.
fn solid_for_spec(kernel: &mut dyn BrepKernel, spec: &SolidSpec, pose: &Pose) -> Option<GeometryHandle> {
    let base = match spec {
        SolidSpec::Box { width, depth, height } => semio_s_3d::brep::engine::block_on(kernel.box_prim(*width, *depth, *height)).ok()?,
        SolidSpec::Cylinder { radius, height } => semio_s_3d::brep::engine::block_on(kernel.cylinder_prim(*radius, *height)).ok()?,
        SolidSpec::Sphere { radius } => semio_s_3d::brep::engine::block_on(kernel.sphere_prim(*radius)).ok()?,
        SolidSpec::ImportedSolid { solid_handle } => {
            let handle = GeometryHandle(solid_handle.clone());
            semio_s_3d::brep::engine::block_on(kernel.kind(&handle)).ok()?;
            handle
        }
        // 🖼️ A GLB-imported reference mesh has no real B-Rep topology in the kernel, so it cannot
        // serve as a CSG operand (stock or tool); the stock-level fallback handles display instead.
        SolidSpec::ImportedMesh { .. } => return None,
    };
    let rotated = if pose.angle != 0.0 { semio_s_3d::brep::engine::block_on(kernel.rotate(&base, pose.axis, pose.angle)).ok()? } else { base };
    if pose.position != [0.0, 0.0, 0.0] {
        semio_s_3d::brep::engine::block_on(kernel.translate(&rotated, pose.position)).ok()
    } else {
        Some(rotated)
    }
}

/// 🧭️ Axis-angle rotation that maps world-up `[0,0,1]` onto an arbitrary unit `normal`, so a box
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

/// 🧠️ Replays enabled steps up to the cursor, reusing the longest memoized prefix.
fn replay_process(session: &mut ProcessKernelReplay, doc: &Process3dDocument) -> Option<GeometryHandle> {
    let stock_signature = hash_value(&doc.stock);
    if stock_signature != session.stock_signature {
        session.tables.memo.clear();
        session.stock_signature = stock_signature;
    }
    let limit = doc.resolved_up_to.unwrap_or(doc.steps.len()).min(doc.steps.len());
    let enabled_steps: Vec<&ProcessStep> = doc.steps[..limit].iter().filter(|step| step.enabled).collect();

    let mut start = enabled_steps.len();
    let mut current: Option<GeometryHandle> = loop {
        let signature = prefix_signature(stock_signature, &enabled_steps[..start]);
        if let Some(handle) = session.tables.memo.get(&signature) {
            break Some(handle.clone());
        }
        if start == 0 {
            break None;
        }
        start -= 1;
    };

    if current.is_none() {
        let stock = {
            let mut kernel = session.kernel().lock().ok()?;
            solid_for_spec(&mut *kernel, &doc.stock.solid, &doc.stock.pose)
        }?;
        session.tables.memo.insert(prefix_signature(stock_signature, &[]), stock.clone());
        current = Some(stock);
    }

    let mut handle = current?;
    for (index, step) in enabled_steps.iter().enumerate().skip(start) {
        let next = {
            let mut kernel = session.kernel().lock().ok()?;
            let tool = tool_solid_for_measure(&mut *kernel, &step.measure)?;
            match step.measure {
                ProcessMeasure::Attach { .. } => semio_s_3d::brep::engine::block_on(kernel.fuse(&handle, &tool)).ok()?,
                _ => semio_s_3d::brep::engine::block_on(kernel.cut(&handle, &tool)).ok()?,
            }
        };
        handle = next;
        session.tables.memo.insert(prefix_signature(stock_signature, &enabled_steps[..=index]), handle.clone());
    }
    if session.tables.memo.len() > PROCESS3D_KERNEL_MEMO_CAP {
        if let Some(key) = session.tables.memo.keys().next().copied() {
            session.tables.memo.remove(&key);
        }
    }
    Some(handle)
}

pub fn processed_mesh(doc: &Process3dDocument) -> Option<MeshData> {
    let mut session = ProcessKernelReplay::new();
    let handle = replay_process(&mut session, doc)?;
    let mesh = {
        let kernel = session.kernel().lock().ok()?;
        semio_s_3d::brep::engine::block_on(kernel.tessellate(&handle, PROCESS3D_TESSELLATION_TOLERANCE))
    }
    .ok()?;
    let face_groups: Vec<(u32, u32, u32)> = mesh.face_groups.iter().map(|group| (group.entity_id.parse().unwrap_or(0), group.start, group.count)).collect();
    Some(semio_framework_plugin::mesh_from_indexed_with_face_groups(&mesh.position, &mesh.normal, &mesh.index, &face_groups))
}

pub fn processed_volume(doc: &Process3dDocument) -> Option<f64> {
    let mut session = ProcessKernelReplay::new();
    let handle = replay_process(&mut session, doc)?;
    let volume = {
        let kernel = session.kernel().lock().ok()?;
        semio_s_3d::brep::engine::block_on(kernel.volume(&handle))
    };
    volume.ok()
}
//#endregion 🔖️KernelReplay

//#region 🔖️MediaImportExport
/// 📤️ A pending native-geometry export ready to become a `HostEffect::DownloadMediaExport`.
pub struct Process3dModelExport {
    pub filename: String,
    pub data: Value,
    pub mime_type: String,
    pub encoding: Option<String>,
}

/// 📤️ Encodes the replayed stock through `format`'s codec. STEP/OBJ/STL go through the
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
    let mut session = ProcessKernelReplay::new();
    let handle = replay_process(&mut session, fixture)?;
    let bytes = exporter.export(&*session.kernel().lock().ok()?, &[handle], PROCESS3D_TESSELLATION_TOLERANCE).ok()?;
    let media_format = exporter.format();
    let binary = media_format.is_binary();
    let data = if binary { Value::String(base64::engine::general_purpose::STANDARD.encode(&bytes)) } else { Value::String(String::from_utf8(bytes).ok()?) };
    Some(Process3dModelExport { filename: format!("process3d.{}", media_format.as_str()), data, mime_type: media_format.mime_type().into(), encoding: if binary { Some("base64".into()) } else { None } })
}

/// 📦️ Decodes a `requestFileOpen(readAs: "dataUrl")` payload into raw bytes.
fn process3d_bytes_from_data_url(data_url: &str) -> Option<Vec<u8>> {
    if let Some((header, encoded)) = data_url.split_once(',') {
        if header.starts_with("data:") {
            return base64::engine::general_purpose::STANDARD.decode(encoded).ok();
        }
    }
    Some(data_url.as_bytes().to_vec())
}

/// 📥️ Imports a picked file into a brand-new stock-only fixture (steps cleared): STEP/OBJ/STL go
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
    let mut session = ProcessKernelReplay::new();
    let handle = importer.import(&mut *session.kernel().lock().ok()?, &bytes, PROCESS3D_TESSELLATION_TOLERANCE).ok()?.into_iter().next()?;
    session.tables.memo.clear();
    session.stock_signature = 0;
    fixture.stock = Stock { id: "stock".into(), label: label.into(), solid: SolidSpec::ImportedSolid { solid_handle: handle.0 }, pose: Pose::default() };
    Some(fixture)
}
//#endregion 🔖️MediaImportExport

//#region 🔖️DocumentHelpers
/// ✂️➕️ Read-only operation builders for the two structural collection edits every mutating command
/// needs: inserting a step at the resolved-up-to cursor (and advancing it), and removing a step by id
/// (and pulling the cursor back if it sat past the removed step). Shared by the `🎮️commands/🪜️step` and
/// `🎮️commands/🌍️world` command modules — building `Process3dOperation`s from an immutable
/// `&Process3dDocument` keeps every handler free of manual mutation, since the VCS store applies them.
pub fn insert_step_operations(fixture: &Process3dDocument, step: ProcessStep) -> Vec<crate::artifacts::process3d::op::Process3dOperation> {
    use crate::artifacts::process3d::op::Process3dOperation;
    use protocol::CollectionOperation;
    let cursor = fixture.resolved_up_to.unwrap_or(fixture.steps.len()).min(fixture.steps.len());
    vec![Process3dOperation::Steps { collection: CollectionOperation::Add { index: cursor, item: step } }, Process3dOperation::SetCursor { resolved_up_to: Some(cursor + 1) }]
}

pub fn remove_step_operations(fixture: &Process3dDocument, id: &str) -> Option<Vec<crate::artifacts::process3d::op::Process3dOperation>> {
    use crate::artifacts::process3d::op::Process3dOperation;
    use protocol::CollectionOperation;
    let index = fixture.steps.iter().position(|step| step.id == id)?;
    let mut operations = vec![Process3dOperation::Steps { collection: CollectionOperation::Remove { id: id.to_string() } }];
    if let Some(cursor) = fixture.resolved_up_to {
        if cursor > index {
            operations.push(Process3dOperation::SetCursor { resolved_up_to: Some(cursor - 1) });
        }
    }
    Some(operations)
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn session_volume(session: &mut ProcessKernelReplay, fixture: &Process3dDocument) -> f64 {
        let handle = replay_process(session, fixture).expect("replayed handle");
        semio_s_3d::brep::engine::block_on(session.kernel().lock().expect("kernel lock").volume(&handle)).expect("replayed volume")
    }

    //#region 🔖️ConfigCoverage
    #[test]
    fn process3d_io_mirrors_the_declared_artifact_kind() {
        let io = process3d_io();
        assert_eq!(io.document_schema, crate::artifacts::process3d::PROCESS_3D_SCHEMA);
        assert_eq!(io.artifact.id, "3d.process");
        assert_eq!(io.export_formats.len(), 4);
        assert_eq!(io.import_formats.len(), 3);
    }

    /// 🔌️ WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe:
    /// `geometry:in` and `brep:out` are declared with the right direction/kind/multiplicity.
    #[test]
    fn process3d_io_declares_geometry_in_and_brep_out_ports() {
        let io = process3d_io();
        let geometry_in = io.ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
        assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert!(geometry_in.kind_id.is_none());
        assert!(!geometry_in.required);
        assert_eq!(geometry_in.multiplicity, semio_framework_plugin::PortMultiplicity::Many);

        let brep_out = io.ports.iter().find(|port| port.id == "brep:out").expect("brep:out declared");
        assert_eq!(brep_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(brep_out.kind_id.as_deref(), Some("3d.process"));
        assert!(!brep_out.required);
        assert_eq!(brep_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
        assert_eq!(brep_out.media_type.class, semio_framework_plugin::MediaClass::ThreeD);
        assert_eq!(brep_out.media_type.form, semio_framework_plugin::MediaForm::Brep);
    }
    //#endregion 🔖️ConfigCoverage

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
        let mut session = ProcessKernelReplay::new();
        let mut fixture = Process3dDocument::default();
        fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
        let stock_volume = session_volume(&mut session, &fixture);
        fixture.steps.push(ProcessStep {
            id: "drill-1".into(),
            label: "Drill".into(),
            enabled: true,
            origin: None,
            measure: ProcessMeasure::Drill { radius: 0.2, depth: 1.0, pose: Pose { position: [0.0, 0.0, 0.5], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
        });
        let drilled_volume = session_volume(&mut session, &fixture);
        assert!(drilled_volume < stock_volume, "drilled volume {drilled_volume} should be less than stock volume {stock_volume}");
    }

    #[test]
    fn attach_increases_volume_above_stock() {
        for _ in 0..32 {
            let mut session = ProcessKernelReplay::new();
            let mut fixture = Process3dDocument::default();
            fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
            let stock_volume = session_volume(&mut session, &fixture);
            fixture.steps.push(ProcessStep {
                id: "attach-1".into(),
                label: "Attach".into(),
                enabled: true,
                origin: None,
                measure: ProcessMeasure::Attach { component: SolidSpec::Sphere { radius: 0.3 }, pose: Pose { position: [1.0, 0.0, 0.5], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
            });
            let attached_volume = session_volume(&mut session, &fixture);
            assert!(attached_volume > stock_volume, "attached volume {attached_volume} should exceed stock volume {stock_volume}");
        }
    }

    #[test]
    fn disabled_step_is_skipped_on_replay() {
        let mut session = ProcessKernelReplay::new();
        let mut fixture = Process3dDocument::default();
        fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
        let stock_volume = session_volume(&mut session, &fixture);
        fixture.steps.push(ProcessStep { id: "drill-1".into(), label: "Drill".into(), enabled: false, origin: None, measure: ProcessMeasure::Drill { radius: 0.2, depth: 1.0, pose: Pose::default() } });
        let volume_with_disabled_step = session_volume(&mut session, &fixture);
        assert!((volume_with_disabled_step - stock_volume).abs() < 1e-6);
    }

    #[test]
    fn cursor_zero_yields_stock_volume() {
        let mut session = ProcessKernelReplay::new();
        let mut fixture = Process3dDocument::default();
        fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
        let stock_volume = session_volume(&mut session, &fixture);
        fixture.steps.push(ProcessStep { id: "drill-1".into(), label: "Drill".into(), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.2, depth: 1.0, pose: Pose::default() } });
        fixture.resolved_up_to = Some(0);
        let volume_at_cursor_zero = session_volume(&mut session, &fixture);
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
        let mut kernel = Brep::new();
        let handle = semio_s_3d::brep::engine::block_on(kernel.box_prim(2.0, 3.0, 4.0)).expect("box prim");
        let mesh = semio_s_3d::brep::engine::block_on(kernel.tessellate(&handle, 0.1)).expect("tessellate");
        let axis_bounds = |offset: usize| -> (f32, f32) {
            let values: Vec<f32> = mesh.position.iter().skip(offset).step_by(3).copied().collect();
            (values.iter().copied().fold(f32::INFINITY, f32::min), values.iter().copied().fold(f32::NEG_INFINITY, f32::max))
        };
        let (min_x, max_x) = axis_bounds(0);
        let (min_y, max_y) = axis_bounds(1);
        let (min_z, max_z) = axis_bounds(2);
        assert!(min_x.abs() < 1e-4 && (max_x - 2.0).abs() < 1e-4, "box x should span [0, width] from the local origin corner, got [{min_x}, {max_x}]");
        assert!(min_y.abs() < 1e-4 && (max_y - 3.0).abs() < 1e-4, "box y should span [0, depth], got [{min_y}, {max_y}]");
        assert!(min_z.abs() < 1e-4 && (max_z - 4.0).abs() < 1e-4, "box z should span [0, height], got [{min_z}, {max_z}]");
    }

    #[test]
    fn sync_process_machine_contributions_merges_hot_installed_catalogs() {
        use semio_framework::{Contribution, ProgramContributionEntry};
        let machine = WorkshopMachine {
            id: "hot-saw".into(),
            label: "Hot Saw".into(),
            icon_id: "scissors".into(),
            catalog_id: None,
            capabilities: vec![],
        };
        let entry = ProgramContributionEntry {
            plugin_id: "process-module-test".into(),
            contribution: Contribution::ProcessMachines {
                app_id: "process3d-play".into(),
                module_id: "hot-catalog".into(),
                label: "Hot Catalog".into(),
                icon_id: "wrench".into(),
                machines_json: serde_json::to_string(&vec![machine]).unwrap(),
            },
        };
        let json = serde_json::to_string(&vec![entry]).unwrap();
        sync_process_machine_contributions(&json);
        assert!(installed_catalogs().iter().any(|catalog| catalog.catalog_id() == "hot-catalog"));
        sync_process_machine_contributions("[]");
    }
}
//#endregion 🧪️Tests


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "process.process3d",
        extension: Some("process3d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::process3d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::process3d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::process3d::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::process3d::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("process.process3d"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "process.process3d.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::process3d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::process3d::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::process3d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::process3d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("process.process3d.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "process.process3d.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::process3d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::process3d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("process.process3d.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "process3d.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::process3d::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::process3d::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("process3d.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "process3d.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::process3d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::process3d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("process3d.spr"),
    });
}
