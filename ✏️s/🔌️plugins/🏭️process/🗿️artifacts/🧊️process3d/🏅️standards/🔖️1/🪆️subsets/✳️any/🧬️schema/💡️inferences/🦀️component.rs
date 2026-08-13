//! 💡️ Process3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `stock`/`steps` compose real
//! `s.stdio.semio.brep`/`s.stdio.semio.flow` CHILD HANDLES on `Process3dSnapshot` now, not inline
//! content — `Process3dInference::infer(&Process3dSnapshot)` can only see the stock's `stock_pose`
//! (real, no resolver needed) and cannot recover `stock_bounds`'s full extent or `step_count`
//! without resolving those children, which no `LinkResolver` seam exists to do yet (checked directly
//! against `🔌️plugin/🦀️component.rs`, W1-owned — see `ProcessWorkingScene`'s own doc comment in the
//! artifact root file). `stock_bounds` degrades to the honest single-point-at-`stock_pose.position`
//! bound (real, not fabricated); `step_count` degrades to 0 with the gap documented on the field.
//! The KERNEL REPLAY pipeline (`ProcessKernelReplay`/`replay_process`/`processed_mesh`/
//! `processed_volume`) still does real, unmodified CSG work — it now reads a `ProcessWorkingScene`
//! (the ephemeral, real-content bridge type) instead of `Process3dSnapshot` directly, exactly the
//! "one accessor every render/export/inference call site funnels through" pattern the migration
//! recipe's §3 prescribes.

use crate::artifacts::process3d::{
    Capability, MeasureKind, MeasureRecipe, Pose, Process3dSnapshot, ProcessMeasure, ProcessStep, ProcessWorkingScene, Stock, StockQuantity, WorkingSolid, Workshop, WorkshopMachine,
};
use protocol::Inference;
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{Brep, BrepKernel, GeometryHandle};
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::bounds::{brep_bounding_box, BoundingBox};

/// 🕳️ Tessellation tolerance for kernel replay/export.
const PROCESS3D_TESSELLATION_TOLERANCE: f64 = 0.05;
/// 🧠️ Kernel replay memo capacity (prefix signatures kept per session).
const PROCESS3D_KERNEL_MEMO_CAP: usize = 128;

//#region 🔖️Inference
/// 💡️ Everything inferable from a process3d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `stockBounds`/`stepCount`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
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

impl protocol::Inference<Process3dSnapshot> for Process3dInference {
    fn infer(snapshot: &Process3dSnapshot) -> Self {
        Self { stock_bounds: BoundingBox { min: snapshot.stock_pose.position, max: snapshot.stock_pose.position }, step_count: 0 }
    }
}

/// 🌉️ Hand impl (not derived): a naive `#[derive(Default)]` would give `stock_bounds` an
/// all-zero box, which disagrees with `infer(&Process3dSnapshot::default())`. Defining default as
/// "infer the default snapshot" makes the two definitionally equal.
impl Default for Process3dInference {
    fn default() -> Self {
        Self::infer(&Process3dSnapshot::default())
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
        &[
            protocol::InferenceFieldSpec { id: "s.process.process3d.inference.bounds.stockBounds", reads: &["stockPose"] },
            protocol::InferenceFieldSpec { id: "s.process.process3d.inference.bounds.stepCount", reads: &["steps"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::process3d::standards::v1::subsets::any::schema::Process3dBuilder {
    type Snapshot = Process3dSnapshot;
    type Inference = Process3dInference;

    /// 🎯️ Whole-snapshot scalars — nothing here is per-entity, so the cache/session are unused
    /// (same "plain `Inference`" shape the family doc calls out as correct for `dimensions`/
    /// `outline`/`bounds`-style facets).
    fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = (cache, session);
        <Process3dInference as protocol::Inference<Process3dSnapshot>>::infer(snapshot)
    }
}
//#endregion 🔖️ArtifactInferrer

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
/// 🔓️ `pub` (not private): `🚪️io`'s `export_process3d_model`/`import_process3d_model` need the
/// exact kernel + replayed handle (not just the tessellated `processed_mesh`/`processed_volume`
/// projections) to drive the real `SolidExporter`/`SolidImporter` trait objects — a crate-internal
/// seam, never re-exported past this crate.
/// 🌱 `kernel: Brep` is owned directly, never behind `BrepEngineHost`/`Mutex` (ticket
/// 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave G4): every caller already
/// constructs a fresh `ProcessKernelReplay::new()` per call (verified — no call site anywhere in this
/// plugin holds one across calls), so the deleted host's cross-call registry was never load-bearing;
/// it only ever added a lock nobody contended on.
pub struct ProcessKernelReplay {
    kernel: Brep,
    tables: ProcessKernelMemo,
    stock_signature: u64,
}

struct ProcessKernelMemo {
    memo: HashMap<u64, GeometryHandle>,
}

impl ProcessKernelReplay {
    pub fn new() -> Self {
        Self {
            kernel: Brep::new(),
            tables: ProcessKernelMemo { memo: HashMap::new() },
            stock_signature: 0,
        }
    }

    /// 🔩 Immutable kernel access — `tessellate`/`volume`/`kind` take `&self`.
    pub fn kernel(&self) -> &Brep {
        &self.kernel
    }

    /// 🔩 Mutable kernel access — every CSG-producing `BrepKernel` method takes `&mut self`.
    pub fn kernel_mut(&mut self) -> &mut Brep {
        &mut self.kernel
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
fn solid_for_spec(kernel: &mut dyn BrepKernel, spec: &WorkingSolid, pose: &Pose) -> Option<GeometryHandle> {
    let base = match spec {
        WorkingSolid::Box { width, depth, height } => semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(kernel.box_prim(*width, *depth, *height)).ok()?,
        WorkingSolid::Cylinder { radius, height } => semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(kernel.cylinder_prim(*radius, *height)).ok()?,
        WorkingSolid::Sphere { radius } => semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(kernel.sphere_prim(*radius)).ok()?,
        WorkingSolid::ImportedSolid { solid_handle } => {
            let handle = GeometryHandle(solid_handle.clone());
            semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(kernel.kind(&handle)).ok()?;
            handle
        }
        // 🖼️ A GLB-imported reference mesh has no real B-Rep topology in the kernel, so it cannot
        // serve as a CSG operand (stock or tool); the stock-level fallback handles display instead.
        WorkingSolid::ImportedMesh { .. } => return None,
    };
    let rotated = if pose.angle != 0.0 { semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(kernel.rotate(&base, pose.axis, pose.angle)).ok()? } else { base };
    if pose.position != [0.0, 0.0, 0.0] {
        semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(kernel.translate(&rotated, pose.position)).ok()
    } else {
        Some(rotated)
    }
}

fn tool_solid_for_measure(kernel: &mut dyn BrepKernel, measure: &ProcessMeasure) -> Option<GeometryHandle> {
    match measure {
        ProcessMeasure::Cut { tool, pose } => solid_for_spec(kernel, tool, pose),
        ProcessMeasure::Drill { radius, depth, pose } => solid_for_spec(kernel, &WorkingSolid::Cylinder { radius: *radius, height: *depth }, pose),
        ProcessMeasure::Attach { component, pose } => solid_for_spec(kernel, component, pose),
    }
}

/// 🧠️ Replays enabled steps up to the cursor, reusing the longest memoized prefix. Reads a real,
/// literal `ProcessWorkingScene` (never a bare `Process3dSnapshot` — see file doc comment).
pub fn replay_process(session: &mut ProcessKernelReplay, scene: &ProcessWorkingScene, resolved_up_to: Option<usize>) -> Option<GeometryHandle> {
    let stock_signature = hash_value(&scene.stock);
    if stock_signature != session.stock_signature {
        session.tables.memo.clear();
        session.stock_signature = stock_signature;
    }
    let limit = resolved_up_to.unwrap_or(scene.steps.len()).min(scene.steps.len());
    let enabled_steps: Vec<&ProcessStep> = scene.steps[..limit].iter().filter(|step| step.enabled).collect();

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
        let stock = solid_for_spec(session.kernel_mut(), &scene.stock.solid, &scene.stock.pose)?;
        session.tables.memo.insert(prefix_signature(stock_signature, &[]), stock.clone());
        current = Some(stock);
    }

    let mut handle = current?;
    for (index, step) in enabled_steps.iter().enumerate().skip(start) {
        let tool = tool_solid_for_measure(session.kernel_mut(), &step.measure)?;
        let next = match step.measure {
            ProcessMeasure::Attach { .. } => semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(session.kernel_mut().fuse(&handle, &tool)).ok()?,
            _ => semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(session.kernel_mut().cut(&handle, &tool)).ok()?,
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

pub fn processed_mesh(scene: &ProcessWorkingScene, resolved_up_to: Option<usize>) -> Option<semio_framework_plugin::MeshData> {
    let mut session = ProcessKernelReplay::new();
    let handle = replay_process(&mut session, scene, resolved_up_to)?;
    let mesh = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(session.kernel().tessellate(&handle, PROCESS3D_TESSELLATION_TOLERANCE)).ok()?;
    let face_groups: Vec<(u32, u32, u32)> = mesh.face_groups.iter().map(|group| (group.entity_id.parse().unwrap_or(0), group.start, group.count)).collect();
    Some(semio_framework_plugin::mesh_from_indexed_with_face_groups(&mesh.position, &mesh.normal, &mesh.index, &face_groups))
}

pub fn processed_volume(scene: &ProcessWorkingScene, resolved_up_to: Option<usize>) -> Option<f64> {
    let mut session = ProcessKernelReplay::new();
    let handle = replay_process(&mut session, scene, resolved_up_to)?;
    semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(session.kernel().volume(&handle)).ok()
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
    for machine in crate::artifacts::process3d::generic_machines() {
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
pub fn stock_extent(solid: &WorkingSolid) -> [f64; 3] {
    match solid {
        WorkingSolid::Box { width, depth, height } => [*width, *depth, *height],
        WorkingSolid::Cylinder { radius, height } => [*radius * 2.0, *radius * 2.0, *height],
        WorkingSolid::Sphere { radius } => [*radius * 2.0, *radius * 2.0, *radius * 2.0],
        WorkingSolid::ImportedMesh { .. } | WorkingSolid::ImportedSolid { .. } => [1.0, 1.0, 1.0],
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
pub fn process3d_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.process.process3d.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::process3d::{ProcessStep, StepOrigin};

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = Process3dSnapshot::default();
        assert_eq!(Process3dInference::infer(&snapshot), Process3dInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Process3dInference::infer(&Process3dSnapshot::default()), Process3dInference::default());
    }

    /// 🌉️ Documented gap (see file doc comment): a plain snapshot can't see its composed `steps`
    /// child's content, so `step_count` is always 0 regardless of the working scene's real steps.
    #[test]
    fn step_count_is_zero_pending_a_resolver() {
        let snapshot = Process3dSnapshot::default();
        assert_eq!(Process3dInference::infer(&snapshot).step_count, 0);
    }
    //#endregion 🧪️InferenceLaws

    //#region 🧪️KernelReplay
    fn drill_step(id: &str, radius: f64, depth: f64, pose: Pose) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Drill".into(), enabled: true, origin: Some(StepOrigin { machine_id: "drill".into(), capability_id: "drill".into() }), measure: ProcessMeasure::Drill { radius, depth, pose } }
    }

    fn session_volume(session: &mut ProcessKernelReplay, scene: &ProcessWorkingScene, resolved_up_to: Option<usize>) -> f64 {
        let handle = replay_process(session, scene, resolved_up_to).expect("replayed handle");
        semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(session.kernel().volume(&handle)).expect("replayed volume")
    }

    #[test]
    fn drill_reduces_volume_below_stock() {
        let mut scene = ProcessWorkingScene { stock: Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }, steps: Vec::new() };
        let stock_volume = processed_volume(&scene, None).expect("stock volume");
        scene.steps.push(ProcessStep {
            id: "drill-1".into(),
            label: "Drill".into(),
            enabled: true,
            origin: None,
            measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.4, depth: 0.4, height: 1.2 }, pose: Pose { position: [0.3, 0.3, -0.1], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
        });
        let drilled_volume = processed_volume(&scene, None).expect("drilled volume");
        assert!(drilled_volume < stock_volume, "drilled volume {drilled_volume} should be less than stock volume {stock_volume}");
    }

    #[test]
    fn attach_increases_volume_above_stock() {
        for _ in 0..32 {
            let mut scene = ProcessWorkingScene { stock: Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }, steps: Vec::new() };
            let stock_volume = processed_volume(&scene, None).expect("stock volume");
            scene.steps.push(ProcessStep {
                id: "attach-1".into(),
                label: "Attach".into(),
                enabled: true,
                origin: None,
                measure: ProcessMeasure::Attach { component: WorkingSolid::Box { width: 0.4, depth: 0.4, height: 0.4 }, pose: Pose { position: [0.3, 0.3, 1.0], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
            });
            let attached_volume = processed_volume(&scene, None).expect("attached volume");
            assert!(attached_volume > stock_volume, "attached volume {attached_volume} should exceed stock volume {stock_volume}");
        }
    }

    #[test]
    fn disabled_step_is_skipped_on_replay() {
        let mut session = ProcessKernelReplay::new();
        let mut scene = ProcessWorkingScene { stock: Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }, steps: Vec::new() };
        let stock_volume = session_volume(&mut session, &scene, None);
        scene.steps.push(ProcessStep { id: "drill-1".into(), label: "Drill".into(), enabled: false, origin: None, measure: ProcessMeasure::Drill { radius: 0.2, depth: 1.0, pose: Pose::default() } });
        let volume_with_disabled_step = session_volume(&mut session, &scene, None);
        assert!((volume_with_disabled_step - stock_volume).abs() < 1e-6);
    }

    #[test]
    fn cursor_zero_yields_stock_volume() {
        let mut session = ProcessKernelReplay::new();
        let mut scene = ProcessWorkingScene { stock: Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }, steps: Vec::new() };
        let stock_volume = session_volume(&mut session, &scene, None);
        scene.steps.push(drill_step("drill-1", 0.2, 1.0, Pose::default()));
        let volume_at_cursor_zero = session_volume(&mut session, &scene, Some(0));
        assert!((volume_at_cursor_zero - stock_volume).abs() < 1e-6);
    }

    #[test]
    fn box_primitive_spans_from_local_origin_corner() {
        let mut kernel = Brep::new();
        let handle = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(kernel.box_prim(2.0, 3.0, 4.0)).expect("box prim");
        let mesh = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(kernel.tessellate(&handle, 0.1)).expect("tessellate");
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
    //#endregion 🧪️KernelReplay
}
//#endregion 🧪️Tests
