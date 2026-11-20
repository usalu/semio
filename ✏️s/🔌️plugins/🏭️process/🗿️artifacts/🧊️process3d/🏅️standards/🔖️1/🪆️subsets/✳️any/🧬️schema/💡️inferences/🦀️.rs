//! 💡️ Process3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `stock`/`steps` compose real
//! `s.stdio.semio.brep`/`s.stdio.semio.flow` CHILD HANDLES on `Process3dSnapshot` now, not inline
//! content — `Process3dInference::infer(&Process3dSnapshot)` can only see the stock's `stock_pose`
//! (real, no resolver needed) and cannot recover `stock_bounds`'s full extent or `step_count`
//! without resolving those children, which no `LinkResolver` seam exists to do yet (checked directly
//! against `🔌️plugin/🦀️.rs`, W1-owned — see `ProcessWorkingScene`'s own doc comment in the
//! artifact root file). `stock_bounds` degrades to the honest single-point-at-`stock_pose.position`
//! bound (real, not fabricated); `step_count` degrades to 0 with the gap documented on the field.
//! The KERNEL REPLAY pipeline (`ProcessKernelReplay`/`replay_process`/`processed_mesh`/
//! `processed_volume`) still does real, unmodified CSG work — it now reads a `ProcessWorkingScene`
//! (the ephemeral, real-content bridge type) instead of `Process3dSnapshot` directly, exactly the
//! "one accessor every render/export/inference call site funnels through" pattern the migration
//! recipe's §3 prescribes.

extern crate semio_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;

//#region 🧩️Modules
#[path = "."]
pub mod bounds {
    #[path = "📦bounds/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod component {
                        #[path = "."]
                        pub mod binary {
                            #[path = "../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                            pub mod inferences;
                        }
                        #[path = "."]
                        pub mod text {
                            #[path = "../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                            pub mod inferences;
                        }
                    }
                    pub use component::*;
                }
            }
        }
    }
}
//#endregion 🧩️Modules

use semio_s_artifact_process_process3d_model::{snapshot::Capability, snapshot::MeasureKind, snapshot::MeasureRecipe, snapshot::Pose, snapshot::Process3dSnapshot, snapshot::ProcessMeasure, snapshot::ProcessStep, snapshot::ProcessWorkingScene, snapshot::Stock, snapshot::StockQuantity, snapshot::WorkingSolid, snapshot::Workshop, snapshot::WorkshopMachine};
use framework_schema::ArtifactSchema;
use protocol::Inference;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_3d_brep_engine::{Brep, BrepError, BrepKernel, GeometryHandle};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
/// 🕳️ Tessellation tolerance for kernel replay/export.
const PROCESS3D_TESSELLATION_TOLERANCE: f64 = 0.05;
/// 🧠️ Kernel replay memo capacity (prefix signatures kept per session).
const PROCESS3D_KERNEL_MEMO_CAP: usize = 128;

//#region 🔖️Inference
/// 💡️ Everything inferable from a process3d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `stockBounds`/`stepCount`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.process.process3d.inference")]
pub struct Process3dInference {
    #[derived]
    pub stock_bounds: BoundingBox,
    /// 🌉️ Documented gap (see file doc comment): a plain `Process3dSnapshot` cannot see its
    /// composed `steps` child's content without a resolver, so this is always 0. Use
    /// `ProcessWorkingScene::steps.len()` for the real count when a working scene is in hand.
    #[derived]
    pub step_count: u64,
}

impl Inference<Process3dSnapshot> for Process3dInference {
    fn infer(snapshot: &Process3dSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { stock_bounds: BoundingBox { min: snapshot.stock_pose.position, max: snapshot.stock_pose.position }, step_count: 0 }
    
        })
    }
}

/// 🌉️ Hand impl (not derived): a naive `#[derive(Default)]` would give `stock_bounds` an
/// all-zero box, which disagrees with `infer(&Process3dSnapshot::default())`. Defining default as
/// "infer the default snapshot" makes the two definitionally equal.
impl Default for Process3dInference {
    fn default() -> Self {
        let snapshot = &Process3dSnapshot::default();

        Self { stock_bounds: BoundingBox { min: snapshot.stock_pose.position, max: snapshot.stock_pose.position }, step_count: 0 }
    }
}

impl protocol::InferenceSpec<Process3dSnapshot> for Process3dInference {
    fn inference_schema_id() -> &'static str {
        "s.process.process3d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.process.process3d.inference.bounds.stockBounds", reads: &["stockPose"] }, protocol::InferenceFieldSpec { id: "s.process.process3d.inference.bounds.stepCount", reads: &["steps"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️KernelReplay


/// 🧠️ Owns the kernel and memoizes the solid or empty result at recent enabled prefixes.
/// Exact export codecs can borrow the kernel and its live handles for the span of an export.
pub struct ProcessKernelReplay {
    kernel: Brep,
    tables: ProcessKernelMemo,
    stock_signature: u64,
}

#[derive(Default)]
struct ProcessKernelMemo {
    memo: HashMap<u64, Option<GeometryHandle>>,
    order: VecDeque<u64>,
}

impl ProcessKernelMemo {
    fn insert(&mut self, signature: u64, handle: Option<GeometryHandle>) {
        if self.memo.insert(signature, handle).is_none() {
            self.order.push_back(signature);
            if self.memo.len() > PROCESS3D_KERNEL_MEMO_CAP {
                let oldest = self.order.pop_front().expect("new entry has a queue position");
                self.memo.remove(&oldest);
            }
        }
    }
}

impl Default for ProcessKernelReplay {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessKernelReplay {
    pub fn new() -> Self {
        Self { kernel: Brep::new(), tables: ProcessKernelMemo::default(), stock_signature: 0 }
    }

    /// 🔩 Immutable kernel access — `tessellate`/`🟫️volume`/`kind` take `&self`.
    pub fn kernel(&self) -> &Brep {
        &self.kernel
    }

    /// 🔩 Mutable kernel access — every CSG-producing `BrepKernel` method takes `&mut self`.
    pub fn kernel_mut(&mut self) -> &mut Brep {
        &mut self.kernel
    }
}



/// 📦️ Builds a posed kernel solid for a spec via `*_prim_sync` → centre → `rotate_sync` → `translate_sync`.
///
/// 🧭️ A `Pose.position` is the solid's CENTRE and its rotation turns the solid about that centre —
/// the convention every authored document speaks (the timber default rests its 0.3 m stock on the
/// ground at `z = 0.15`, the plate example drills at `x = −0.45` of a 1.2 m plate). The kernel's
/// primitives are anchored elsewhere (`box_prim` spans `[0,w]×[0,d]×[0,h]` from its corner,
/// `cylinder_prim` runs `z ∈ [0,h]`, `sphere_prim` is already centred), so each is first shifted
/// by minus its own half extents ([`primitive_centre_offset`]). Left corner-anchored, the crosscut
/// and lap-joint tools of the default document sat flush on the stock's own faces and the replay
/// answered the unit-box fallback in the workpiece window (ticket 26/09/15/DEV-PROCESS-REACT-E2E).
/// 🚫️ `kernel: &mut Brep` (not `&mut dyn BrepKernel`): `Brep` is the only `BrepKernel` implementor this
/// crate ever constructs (`ProcessKernelReplay::kernel_mut`, the sole caller path below) — R11's
/// "exactly one impl ⇒ delete the trait object, use the concrete type" — and `BrepKernel` is declared
/// in `🗄️stdio`, a crate outside this packet's path scope, so it could not be `#[dyn_enum]`-closed here
/// even if a second implementor existed.
fn solid_for_spec(kernel: &mut Brep, spec: &WorkingSolid, pose: &Pose) -> Result<GeometryHandle, BrepError> {
    let anchored = match spec {
        WorkingSolid::Box { width, depth, height } => kernel.box_prim(*width, *depth, *height)?,
        WorkingSolid::Cylinder { radius, height } => kernel.cylinder_prim(*radius, *height)?,
        WorkingSolid::Sphere { radius } => kernel.sphere_prim(*radius)?,
        WorkingSolid::ImportedSolid { brep_pack } => semio_s_artifact_process_process3d_model::snapshot::restore_imported_brep(kernel, brep_pack)
            .ok_or_else(|| BrepError::InvalidInput("imported Process3d solid cannot be restored".into()))?,
        WorkingSolid::Reference { reference_id } => semio_s_artifact_process_process3d_model::snapshot::reference_solid(reference_id)
            .and_then(|reference| reference.import(kernel))
            .ok_or_else(|| BrepError::InvalidInput(format!("process reference {reference_id} cannot be resolved")))?,
        WorkingSolid::ImportedMesh { .. } => return Err(BrepError::InvalidInput("process replay requires solid geometry".into())),
    };
    let base = match primitive_centre_offset(spec) {
        Some(offset) => kernel.translate(&anchored, offset)?,
        None => anchored,
    };
    let rotated = if pose.angle != 0.0 { kernel.rotate(&base, pose.axis, pose.angle)? } else { base };
    if pose.position != [0.0, 0.0, 0.0] {
        kernel.translate(&rotated, pose.position)
    } else {
        Ok(rotated)
    }
}

/// 🧭️ The shift that moves a freshly built kernel primitive's own anchor onto its centre — `None`
/// when the primitive is already centred or is not a kernel primitive at all.
pub fn primitive_centre_offset(spec: &WorkingSolid) -> Option<[f64; 3]> {
    match spec {
        WorkingSolid::Box { width, depth, height } => Some([-width / 2.0, -depth / 2.0, -height / 2.0]),
        WorkingSolid::Cylinder { height, .. } => Some([0.0, 0.0, -height / 2.0]),
        WorkingSolid::Sphere { .. } | WorkingSolid::ImportedSolid { .. } | WorkingSolid::ImportedMesh { .. } | WorkingSolid::Reference { .. } => None,
    }
}

fn tool_solid_for_measure(kernel: &mut Brep, measure: &ProcessMeasure) -> Result<GeometryHandle, BrepError> {
    match measure {
        ProcessMeasure::Cut { tool, pose } => solid_for_spec(kernel, tool, pose),
        ProcessMeasure::Drill { radius, depth, pose } => solid_for_spec(kernel, &WorkingSolid::Cylinder { radius: *radius, height: *depth }, pose),
        ProcessMeasure::Attach { component, pose } => solid_for_spec(kernel, component, pose),
    }
}

/// 🧠️ Replays enabled steps through the cursor, memoizing both solids and empty prefixes.
pub fn replay_process(session: &mut ProcessKernelReplay, scene: &ProcessWorkingScene, resolved_up_to: Option<usize>) -> Result<Option<GeometryHandle>, BrepError> {
    let stock_signature = hash_value(&scene.stock);
    if stock_signature != session.stock_signature {
        session.tables = ProcessKernelMemo::default();
        session.stock_signature = stock_signature;
    }
    let limit = resolved_up_to.unwrap_or(scene.steps.len()).min(scene.steps.len());
    let enabled_steps: Vec<&ProcessStep> = scene.steps[..limit].iter().filter(|step| step.enabled).collect();
    let signatures = prefix_signatures(stock_signature, &enabled_steps);

    let mut start = enabled_steps.len();
    let mut handle = loop {
        let signature = signatures[start];
        if let Some(cached) = session.tables.memo.get(&signature) {
            break cached.clone();
        }
        if start == 0 {
            let stock = Some(solid_for_spec(session.kernel_mut(), &scene.stock.solid, &scene.stock.pose)?);
            session.tables.insert(signature, stock.clone());
            break stock;
        }
        start -= 1;
    };

    for (index, step) in enabled_steps.iter().enumerate().skip(start) {
        let attaches = matches!(&step.measure, ProcessMeasure::Attach { .. });
        handle = match (&handle, attaches) {
            (None, false) => None,
            (target, _) => {
                let kernel = session.kernel_mut();
                let tool = tool_solid_for_measure(kernel, &step.measure)?;
                match target {
                    None => Some(tool),
                    Some(target) if attaches => Some(kernel.fuse(target, &tool)?),
                    Some(target) => match kernel.cut(target, &tool) {
                        Ok(solid) => Some(solid),
                        Err(BrepError::EmptyResult) => None,
                        Err(error) => return Err(error),
                    },
                }
            }
        };
        session.tables.insert(signatures[index + 1], handle.clone());
    }
    Ok(handle)
}

fn mesh_from_handle(kernel: &Brep, handle: &GeometryHandle) -> Result<semio_plugin::MeshData, BrepError> {
    let mesh = kernel.tessellate(handle, PROCESS3D_TESSELLATION_TOLERANCE)?;
    if mesh.index.is_empty() {
        return Err(BrepError::Operation("process solid tessellated without faces".into()));
    }
    let face_groups: Vec<(u32, u32, u32)> = mesh.face_groups.iter().map(|group| (group.entity_id.parse().unwrap_or(0), group.start, group.count)).collect();
    Ok(semio_plugin::mesh_from_indexed_with_face_groups(&mesh.position, &mesh.normal, &mesh.index, &face_groups))
}

/// 🖼️ Tessellates the solid present at the cursor.
pub fn processed_mesh(scene: &ProcessWorkingScene, resolved_up_to: Option<usize>) -> Result<Option<semio_plugin::MeshData>, BrepError> {
    let mut session = ProcessKernelReplay::new();
    replay_process(&mut session, scene, resolved_up_to)?.as_ref().map(|handle| mesh_from_handle(session.kernel(), handle)).transpose()
}

/// 📐️ Returns the material volume at the cursor, including zero for a valid empty workpiece.
pub fn processed_volume(scene: &ProcessWorkingScene, resolved_up_to: Option<usize>) -> Result<f64, BrepError> {
    let mut session = ProcessKernelReplay::new();
    match replay_process(&mut session, scene, resolved_up_to)? {
        Some(handle) => session.kernel().volume(&handle),
        None => Ok(0.0),
    }
}

/// 📦️ Evaluates editor previews without exposing temporary kernel geometry.
#[derive(Default)]
pub struct ProcessPreviewReplay {
    replay: ProcessKernelReplay,
}

impl ProcessPreviewReplay {
    /// 🖼️ Reuses recent prefixes and retires transient operands after projecting mesh and volume.
    pub fn evaluate(&mut self, scene: &ProcessWorkingScene, resolved_up_to: Option<usize>) -> Result<(Option<semio_plugin::MeshData>, f64), BrepError> {
        let result = replay_process(&mut self.replay, scene, resolved_up_to).and_then(|handle| {
            let Some(handle) = handle else { return Ok((None, 0.0)) };
            let kernel = self.replay.kernel();
            Ok((Some(mesh_from_handle(kernel, &handle)?), kernel.volume(&handle)?))
        });
        let live: HashSet<String> = self.replay.tables.memo.values().flatten().map(|handle| handle.as_str().to_owned()).collect();
        if self.replay.kernel.registry_len() > live.len() {
            self.replay.kernel.retain(&live);
        }
        result
    }
}
//#endregion 🔖️KernelReplay

//#region 🔖️CapabilityValidation
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
    for machine in semio_s_artifact_process_process3d_model::snapshot::generic_machines() {
        for capability in machine.capabilities.iter() {
            if capability.recipe.measure_kind() == kind {
                return (machine.clone(), capability.clone());
            }
        }
    }
    unreachable!("every MeasureKind has a generic fallback machine")
}

/// 🔎️ One workshop machine's capability, by id — the resolution target for `AddStep`'s
/// `(machine_id, capability_id)` and for re-validating a step's `StepOrigin` provenance.
pub fn find_capability<'a>(workshop: &'a Workshop, machine_id: &str, capability_id: &str) -> Option<(&'a WorkshopMachine, &'a Capability)> {
    let machine = workshop.machines.iter().find(|machine| machine.id == machine_id)?;
    let capability = machine.capabilities.iter().find(|capability| capability.id == capability_id)?;
    Some((machine, capability))
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

/// ✅️ Checks a capability's rules against the current stock — a rule whose parameter is missing from
/// the capability is skipped (lenient, matches the pre-workshop behavior), never a hard error.
pub fn validate_capability(capability: &Capability, ctx: &ValidationContext) -> Vec<ValidationFailure> {
    capability
        .rules
        .iter()
        .filter_map(|rule| {
            let (quantity, parameter_id, margin, is_min) = match rule {
                semio_s_artifact_process_process3d_model::snapshot::CapabilityRule::Min { quantity, parameter, margin } => (*quantity, parameter.as_str(), *margin, true),
                semio_s_artifact_process_process3d_model::snapshot::CapabilityRule::Max { quantity, parameter, margin } => (*quantity, parameter.as_str(), *margin, false),
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

/// 📐️ Returns local stock dimensions for capability checks.
pub fn stock_extent(solid: &WorkingSolid) -> Option<[f64; 3]> {
    Some(match solid {
        WorkingSolid::Box { width, depth, height } => [*width, *depth, *height],
        WorkingSolid::Cylinder { radius, height } => [*radius * 2.0, *radius * 2.0, *height],
        WorkingSolid::Sphere { radius } => [*radius * 2.0, *radius * 2.0, *radius * 2.0],
        WorkingSolid::Reference { reference_id } => semio_s_artifact_process_process3d_model::snapshot::reference_solid(reference_id)?.extent,
        WorkingSolid::ImportedSolid { brep_pack } => imported_stock_extent(brep_pack)?,
        WorkingSolid::ImportedMesh { .. } => return None,
    })
}

fn imported_stock_extent(brep_pack: &str) -> Option<[f64; 3]> {
    use std::sync::Mutex;
    static EXTENT: Mutex<Option<(String, Option<[f64; 3]>)>> = Mutex::new(None);
    if let Some((pack, extent)) = EXTENT.lock().ok().as_deref().and_then(Option::as_ref) {
        if pack == brep_pack {
            return *extent;
        }
    }
    let mut kernel = Brep::new();
    let solid = semio_s_artifact_process_process3d_model::snapshot::restore_imported_brep(&mut kernel, brep_pack)?;
    let bounds = kernel.bounds_sync(&solid).ok()?;
    let dimensions = std::array::from_fn(|axis| bounds.max[axis] - bounds.min[axis]);
    let extent = dimensions.iter().all(|dimension| dimension.is_finite() && *dimension > 0.0).then_some(dimensions);
    if let Ok(mut cached) = EXTENT.lock() {
        *cached = Some((brep_pack.into(), extent));
    }
    extent
}

pub fn validation_context_for_stock(stock: &Stock) -> Option<ValidationContext> {
    let [width, depth, height] = stock_extent(&stock.solid)?;
    Some(ValidationContext { stock_width: width, stock_depth: depth, stock_height: height })
}

/// 🪚️ Builds the `ProcessMeasure` a capability's recipe produces, sized from the capability's own
/// parameters (a missing parameter resolves to `0.0`, matching `validate_capability`'s lenient lookup).
pub fn measure_for_capability(capability: &Capability, position: Option<[f64; 3]>) -> ProcessMeasure {
    let value = |id: &str| parameter_value(capability, id).unwrap_or(0.0);
    let mut measure = match &capability.recipe {
        MeasureRecipe::DiscCut { diameter, kerf } => ProcessMeasure::Cut { tool: WorkingSolid::Cylinder { radius: value(diameter) / 2.0, height: value(kerf) }, pose: Pose::default() },
        MeasureRecipe::BladeCut { kerf, length, depth } => ProcessMeasure::Cut { tool: WorkingSolid::Box { width: value(kerf), depth: value(length), height: value(depth) }, pose: Pose::default() },
        MeasureRecipe::PocketCut { diameter, depth } => {
            let side = value(diameter);
            ProcessMeasure::Cut { tool: WorkingSolid::Box { width: side, depth: side, height: value(depth) }, pose: Pose::default() }
        }
        MeasureRecipe::BoreDrill { radius, depth } => ProcessMeasure::Drill { radius: value(radius), depth: value(depth), pose: Pose::default() },
        MeasureRecipe::CylinderAttach { radius, length } => ProcessMeasure::Attach { component: WorkingSolid::Cylinder { radius: value(radius), height: value(length) }, pose: Pose::default() },
        MeasureRecipe::BoxAttach { width, depth, height } => ProcessMeasure::Attach { component: WorkingSolid::Box { width: value(width), depth: value(depth), height: value(height) }, pose: Pose::default() },
    };
    if let Some(position) = position {
        let pose = match &mut measure {
            ProcessMeasure::Cut { pose, .. } | ProcessMeasure::Drill { pose, .. } | ProcessMeasure::Attach { pose, .. } => pose,
        };
        pose.position = position;
    }
    measure
}
//#endregion 🔖️CapabilityValidation

//#region 🔖️Descriptor
/// 💡️ Registers `s.process.process3d.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `process3d_artifact_schema_descriptor`'s registration.
pub fn process3d_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.process.process3d.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::bounds::BoundingBox;
//#endregion 🔁️Re-exports

/// 🧮️ Hashes typed semantic values directly, preserving numeric variants and IEEE words.
fn hash_intrinsic(value: &semio_framework_value::DslValue, hasher: &mut impl Hasher) {
    use semio_framework_value::{DslValue, Number};
    std::mem::discriminant(value).hash(hasher);
    match value {
        DslValue::Null => {},
        DslValue::Bool(value) => value.hash(hasher),
        DslValue::Number(value) => {
            std::mem::discriminant(value).hash(hasher);
            match value { Number::UInt(value) => value.hash(hasher), Number::Int(value) => value.hash(hasher), Number::Float(value) => value.to_bits().hash(hasher) }
        },
        DslValue::String(value) => value.hash(hasher),
        DslValue::Bytes(value) => value.hash(hasher),
        DslValue::Array(values) => { values.len().hash(hasher); for value in values { hash_intrinsic(value, hasher); } },
        DslValue::Object(values) => { values.len().hash(hasher); for (key, value) in values { key.hash(hasher); hash_intrinsic(value, hasher); } },
    }
}

pub fn hash_value<T: ToValue>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    let value = semio_framework_value::DecodedValue::new(value.to_value(), <semio_framework_value::DslValue as FromValue>::retire_decoded);
    hash_intrinsic(value.get(), &mut hasher);
    hasher.finish()
}

fn prefix_signatures(stock_signature: u64, steps: &[&ProcessStep]) -> Vec<u64> {
    let mut hasher = DefaultHasher::new();
    stock_signature.hash(&mut hasher);
    let mut signatures = Vec::with_capacity(steps.len() + 1);
    signatures.push(hasher.finish());
    for step in steps {
        let value = semio_framework_value::DecodedValue::new(step.measure.to_value(), <semio_framework_value::DslValue as FromValue>::retire_decoded);
        hash_intrinsic(value.get(), &mut hasher);
        signatures.push(hasher.finish());
    }
    signatures
}

#[cfg(test)]
#[path = "🧪️tests/📦️prefix-memo/🦀️.rs"]
mod prefix_memo_tests;
