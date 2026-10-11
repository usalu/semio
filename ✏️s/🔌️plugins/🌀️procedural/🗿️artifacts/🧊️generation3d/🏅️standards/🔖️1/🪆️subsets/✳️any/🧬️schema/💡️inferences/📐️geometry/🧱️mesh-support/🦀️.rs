//! 🧱️ The plumbing every `mesh.*` compute shares: typed mesh inputs, selection validation against the input mesh, localized mesh faults and the stepped machines that run kernel mesh jobs under a widget fuel grant.
//!
//! A mesh compute describes its work as a closure that runs inside the first step, so cloning the input mesh and building the kernel job are charged to the grant like every other unit. A machine advances in batches of [`BATCH`] kernel units per fuel, reports monotone progress, turns a kernel panic into a fault and drops its kernel job on cancel.

use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::mesh::{HalfedgeMesh, MeshKernelError, MeshModelingJob, MeshModelingProgress, MeshModelingStep, MeshSurfaceJob, MeshTessellationJob, MeshTessellationStep, Vec3};
use semio_framework_mesh_engine::PolygonMeshSource;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use std::panic::{catch_unwind, AssertUnwindSafe};

#[cfg(test)]
#[path = "🧪️tests/🧰️mesh-harness/🦀️.rs"]
pub mod harness;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

/// ⚖️ Kernel units of work one unit of widget fuel stands for: one mesh batch.
pub const BATCH: usize = 256;

//#region 🔖️Faults
pub const CANCELLED: &str = "generation3d.geometry.cancelled";
pub const SELECTION_STALE: &str = "generation3d.geometry.selection-stale";
pub const SELECTION_COUNT: &str = "generation3d.geometry.selection-count";
pub const MESH_KERNEL: &str = "generation3d.geometry.mesh-kernel";
pub const MESH_HANDLE: &str = "generation3d.geometry.mesh-handle";
pub const MESH_NON_MANIFOLD: &str = "generation3d.geometry.mesh-non-manifold";
pub const MESH_DEGENERATE: &str = "generation3d.geometry.mesh-degenerate";
pub const MESH_EMPTY_SELECTION: &str = "generation3d.geometry.mesh-empty-selection";
pub const MESH_IMPORT: &str = "generation3d.geometry.mesh-import";
pub const MESH_CAPACITY: &str = "generation3d.geometry.mesh-capacity";
pub const MESH_EXPORT: &str = "generation3d.geometry.mesh-export";
pub const KERNEL_STOPPED: &str = "generation3d.geometry.kernel";

/// 🛑️ The fault of a job that was cancelled before it finished.
pub fn cancelled_fault() -> WidgetFault {
    WidgetFault::new(CANCELLED, "The computation was cancelled.", "Die Berechnung wurde abgebrochen.")
}

fn stopped_fault() -> WidgetFault {
    WidgetFault::new(KERNEL_STOPPED, "The mesh kernel stopped unexpectedly while computing this widget.", "Der Netzkern wurde beim Berechnen dieses Widgets unerwartet beendet.")
}

/// 🛠️ The localized fault of a refused mesh kernel call.
pub fn mesh_fault(error: &MeshKernelError) -> WidgetFault {
    match error {
        MeshKernelError::InvalidHandle => WidgetFault::new(MESH_HANDLE, "The operation refers to a mesh element that does not exist.", "Die Operation verweist auf ein Netzelement, das nicht existiert."),
        MeshKernelError::NonManifold => WidgetFault::new(MESH_NON_MANIFOLD, "The mesh is not manifold where the operation works, so it cannot continue.", "Das Netz ist dort, wo die Operation arbeitet, nicht mannigfaltig; sie kann nicht fortgesetzt werden."),
        MeshKernelError::DegenerateOperation => WidgetFault::new(MESH_DEGENERATE, "The operation degenerates: an input is zero, collinear, not finite or leaves nothing to do.", "Die Operation entartet: Eine Eingabe ist null, kollinear, nicht endlich oder lässt nichts zu tun übrig."),
        MeshKernelError::EmptySelection => WidgetFault::new(MESH_EMPTY_SELECTION, "The selection is empty.", "Die Auswahl ist leer."),
        MeshKernelError::InvalidInput(detail) => WidgetFault::new(MESH_KERNEL, format!("The mesh kernel refused the input: {detail}."), format!("Der Netzkern hat die Eingabe abgelehnt: {detail}.")),
        MeshKernelError::Retained(detail) => WidgetFault::new(MESH_KERNEL, format!("The mesh kernel refused the retained work: {detail}."), format!("Der Netzkern hat die gehaltene Arbeit abgelehnt: {detail}.")),
    }
}

/// 🔁️ Turns a mesh kernel result into a widget result.
pub trait KernelResult<T> {
    fn kernel(self) -> Result<T, WidgetFault>;
}

impl<T> KernelResult<T> for Result<T, MeshKernelError> {
    fn kernel(self) -> Result<T, WidgetFault> {
        self.map_err(|error| mesh_fault(&error))
    }
}

/// 📥️ The fault of a mesh file or text that cannot be read.
pub fn import_fault(detail: impl std::fmt::Display) -> WidgetFault {
    WidgetFault::new(MESH_IMPORT, format!("The mesh data cannot be read: {detail}."), format!("Die Netzdaten können nicht gelesen werden: {detail}."))
}

/// 📏️ The fault of a mesh that would exceed what the kernel holds.
pub fn capacity_fault(what: (&str, &str), count: usize, limit: usize, port: &str) -> WidgetFault {
    WidgetFault::new(MESH_CAPACITY, format!("{} would have {count} faces, but a mesh holds at most {limit}.", what.0), format!("{} hätte {count} Flächen, ein Netz fasst aber höchstens {limit}.", what.1)).at(port)
}

/// 📤️ The fault of a mesh that cannot be written.
pub fn export_fault(detail: impl std::fmt::Display) -> WidgetFault {
    WidgetFault::new(MESH_EXPORT, format!("The mesh cannot be written: {detail}."), format!("Das Netz kann nicht geschrieben werden: {detail}."))
}
//#endregion 🔖️Faults

//#region 🔖️Inputs
/// 🧲️ The mesh element a selection refers to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Element {
    Vertex,
    Edge,
    Face,
}

impl Element {
    /// 🧲️ The mesh element of a selection kind.
    pub fn of(kind: SelectionKind) -> Self {
        match kind {
            SelectionKind::Vertex => Self::Vertex,
            SelectionKind::Edge => Self::Edge,
            SelectionKind::Face => Self::Face,
        }
    }

    /// 🔢️ How many ids of this element the mesh addresses; edge ids are half-edge handles.
    pub fn bound(self, mesh: &HalfedgeMesh) -> usize {
        match self {
            Self::Vertex => mesh.vertex_count(),
            Self::Edge => mesh.halfedge_count(),
            Self::Face => mesh.face_count(),
        }
    }

    fn names(self) -> (&'static str, &'static str) {
        match self {
            Self::Vertex => ("vertex", "Eckpunkt"),
            Self::Edge => ("edge", "Kante"),
            Self::Face => ("face", "Fläche"),
        }
    }
}

/// 🧲️ The ids a selection port names, ascending and without repeats, each validated against `mesh`: an id the mesh no longer has is `selection-stale`.
pub fn picked(inputs: &WidgetInputs, port: &str, mesh: &HalfedgeMesh, element: Element) -> Result<Vec<u32>, WidgetFault> {
    let selection = inputs.selection(port)?;
    let bound = element.bound(mesh);
    let mut ids = Vec::with_capacity(selection.ids.len());
    for id in &selection.ids {
        if *id >= bound as u64 {
            let (en, de) = element.names();
            let last = bound.saturating_sub(1);
            return Err(WidgetFault::new(
                SELECTION_STALE,
                format!("The selected {en} {id} does not exist on the input mesh (valid ids are 0 to {last}); the mesh changed upstream, so select again."),
                format!("{de} {id} der Auswahl gibt es am Eingabenetz nicht (gültig sind 0 bis {last}); das Netz hat sich davor geändert, wähle neu."),
            )
            .at(port));
        }
        ids.push(*id as u32);
    }
    ids.sort_unstable();
    ids.dedup();
    Ok(ids)
}

/// 🧲️ The ids of the single element a selection port names; anything else than exactly one is a `selection-count` fault.
pub fn picked_one(inputs: &WidgetInputs, port: &str, mesh: &HalfedgeMesh, element: Element) -> Result<u32, WidgetFault> {
    let ids = picked(inputs, port, mesh, element)?;
    match ids.as_slice() {
        [id] => Ok(*id),
        _ => Err(WidgetFault::new(SELECTION_COUNT, format!("This input takes exactly one element, but {} are selected.", ids.len()), format!("Dieser Eingang nimmt genau ein Element, es sind aber {} gewählt.", ids.len())).at(port)),
    }
}

/// 🧲️ The element a `mode` enum input names.
pub fn mode_element(inputs: &WidgetInputs, port: &str) -> Result<Element, WidgetFault> {
    match inputs.text(port)? {
        "vertex" => Ok(Element::Vertex),
        "edge" => Ok(Element::Edge),
        "face" => Ok(Element::Face),
        other => Err(WidgetFault::new("generation3d.geometry.input-option", format!("The mode \u{201c}{other}\u{201d} is not vertex, edge or face."), format!("Der Modus \u{201c}{other}\u{201d} ist weder vertex, edge noch face.")).at(port)),
    }
}

/// 🕸️ An owned copy of the input mesh, ready for a consuming kernel job.
pub fn owned_mesh(inputs: &WidgetInputs, port: &str) -> Result<HalfedgeMesh, WidgetFault> {
    Ok((**inputs.mesh(port)?).clone())
}

fn narrowed(value: f64, port: &str) -> Result<f32, WidgetFault> {
    let narrowed = value as f32;
    if narrowed.is_finite() {
        Ok(narrowed)
    } else {
        Err(WidgetFault::new("generation3d.geometry.input-range", format!("The value {value} is too large for the mesh kernel."), format!("Der Wert {value} ist für den Netzkern zu gross.")).at(port))
    }
}

/// 🔢️ A number input as the kernel's `f32`.
pub fn scalar(inputs: &WidgetInputs, port: &str) -> Result<f32, WidgetFault> {
    narrowed(inputs.number(port)?, port)
}

/// 🔢️ A whole-number input as a count.
pub fn count(inputs: &WidgetInputs, port: &str) -> Result<u32, WidgetFault> {
    let value = inputs.integer(port)?;
    u32::try_from(value).map_err(|_| WidgetFault::new("generation3d.geometry.input-range", format!("The value {value} is outside the supported count range."), format!("Der Wert {value} liegt ausserhalb des unterstützten Zählbereichs.")).at(port))
}

/// ➡️ A vector input as the kernel's vector.
pub fn vector3(inputs: &WidgetInputs, port: &str) -> Result<Vec3, WidgetFault> {
    let axes = inputs.vector(port)?;
    Ok(Vec3([narrowed(axes[0], port)?, narrowed(axes[1], port)?, narrowed(axes[2], port)?]))
}

/// 📍️ A point input as the kernel's vector.
pub fn point3(inputs: &WidgetInputs, port: &str) -> Result<Vec3, WidgetFault> {
    let axes = inputs.point(port)?;
    Ok(Vec3([narrowed(axes[0], port)?, narrowed(axes[1], port)?, narrowed(axes[2], port)?]))
}

/// 📦️ The single `mesh` output.
pub fn mesh_output(mesh: HalfedgeMesh) -> Outputs {
    outputs([("mesh", GeometryValue::mesh(mesh))])
}
//#endregion 🔖️Inputs

//#region 🔖️Machines
/// 🪜️ One slice of a machine: it needs more calls (with a 0..=1 progress) or it produced the outputs.
pub enum Flow {
    Working(f32),
    Done(Outputs),
}

/// 🧵️ A started piece of mesh work that advances by widget fuel.
pub trait Machine: Send {
    /// ⏱️ Does at most `fuel` units of work.
    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault>;
    /// 🛑️ Drops whatever the machine holds.
    fn cancel(&mut self);
}

type Build = Box<dyn FnOnce() -> Result<Box<dyn Machine>, WidgetFault> + Send>;

struct MachineJob {
    quality: Quality,
    build: Option<Build>,
    machine: Option<Box<dyn Machine>>,
    finished: bool,
}

impl MachineJob {
    fn fault(&mut self, fault: WidgetFault) -> WidgetStep {
        self.finished = true;
        self.build = None;
        self.machine = None;
        WidgetStep::Done(WidgetEvaluation::faulted(fault, self.quality))
    }
}

impl WidgetJob for MachineJob {
    fn step(&mut self, fuel: usize) -> WidgetStep {
        if self.finished {
            return self.fault(cancelled_fault());
        }
        let fuel = fuel.max(1);
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            if let Some(build) = self.build.take() {
                self.machine = Some(build()?);
            }
            match self.machine.as_mut() {
                Some(machine) => machine.advance(fuel),
                None => Err(cancelled_fault()),
            }
        }));
        match outcome {
            Ok(Ok(Flow::Working(progress))) => WidgetStep::Working { progress: progress.clamp(0.0, 0.999) },
            Ok(Ok(Flow::Done(outputs))) => {
                self.finished = true;
                self.machine = None;
                WidgetStep::Done(WidgetEvaluation::ok(outputs, self.quality))
            }
            Ok(Err(fault)) => self.fault(fault),
            Err(_) => self.fault(stopped_fault()),
        }
    }

    fn cancel(&mut self) {
        self.finished = true;
        self.build = None;
        if let Some(machine) = self.machine.as_mut() {
            machine.cancel();
        }
        self.machine = None;
    }
}

/// 🚀️ Starts a widget job whose machine is built inside the first step.
pub fn run(kind: &Kind, build: impl FnOnce() -> Result<Box<dyn Machine>, WidgetFault> + Send + 'static) -> Box<dyn WidgetJob> {
    Box::new(MachineJob { quality: kind.quality, build: Some(Box::new(build)), machine: None, finished: false })
}

/// 🧱️ A kernel job that turns a mesh into a mesh in budgeted units.
pub trait Stepper: Send {
    fn step(&mut self, budget: usize) -> Result<MeshModelingStep, MeshKernelError>;
    fn cancel(&mut self);
}

/// 🎟️ Advances a kernel modeling job by up to `budget` units under the same funding the kernel's own synchronous drivers grant it.
pub fn modeling_slice(job: &mut MeshModelingJob, budget: usize) -> Result<MeshModelingStep, MeshKernelError> {
    let grant = RetainedCloneGrant { maximum_items: budget.max(1), maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };
    job.step(budget, grant, &mut RetainedCloneProgress::default())
}

/// 🎟️ Advances a kernel tessellation by up to `events` producer events, each under the exact grant its own quote asks for.
pub fn tessellation_slice(job: &mut MeshTessellationJob, events: usize) -> Result<MeshTessellationStep, MeshKernelError> {
    let mut last = MeshTessellationStep::Working(job.progress());
    for _ in 0..events.max(1) {
        let copy = job.next_normal_copy_byte_demand().map_err(MeshKernelError::Retained)?.max(128);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: job.next_normal_capacity_byte_demand(copy).map_err(MeshKernelError::Retained)?, maximum_release_bytes: job.next_normal_release_byte_demand().map_err(MeshKernelError::Retained)?, maximum_depth: job.next_normal_depth_demand().map_err(MeshKernelError::Retained)? };
        last = job.step(grant).map_err(MeshKernelError::Retained)?.0;
        if !matches!(last, MeshTessellationStep::Working(_)) {
            break;
        }
    }
    Ok(last)
}

impl Stepper for MeshModelingJob {
    fn step(&mut self, budget: usize) -> Result<MeshModelingStep, MeshKernelError> {
        modeling_slice(self, budget)
    }

    fn cancel(&mut self) {
        MeshModelingJob::cancel(self);
    }
}

impl Stepper for MeshSurfaceJob {
    fn step(&mut self, budget: usize) -> Result<MeshModelingStep, MeshKernelError> {
        MeshSurfaceJob::step(self, budget)
    }

    fn cancel(&mut self) {
        MeshSurfaceJob::cancel(self);
    }
}

/// 📊 The progress ratio of a kernel job.
pub fn ratio(progress: MeshModelingProgress) -> f32 {
    progress.units_done as f32 / progress.units_total.max(1) as f32
}

struct Modeling<J: Stepper>(J);

impl<J: Stepper> Machine for Modeling<J> {
    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault> {
        match self.0.step(fuel.saturating_mul(BATCH)).kernel()? {
            MeshModelingStep::Working(progress) => Ok(Flow::Working(ratio(progress))),
            MeshModelingStep::Done(mesh) => Ok(Flow::Done(mesh_output(mesh))),
            MeshModelingStep::Cancelled(_) => Err(cancelled_fault()),
        }
    }

    fn cancel(&mut self) {
        self.0.cancel();
    }
}

/// 🛠️ Starts a widget job over a kernel mesh job that `make` builds inside the first step; the finished mesh is the `mesh` output.
pub fn modeling<J: Stepper + 'static>(kind: &Kind, make: impl FnOnce() -> Result<J, WidgetFault> + Send + 'static) -> Box<dyn WidgetJob> {
    run(kind, move || Ok(Box::new(Modeling(make()?)) as Box<dyn Machine>))
}
/// 🔎️ Reads a polygon source in bounded slices: it answers `None` until the whole text is read.
pub type Parse = Box<dyn FnMut(usize) -> Result<Option<PolygonMeshSource>, WidgetFault> + Send>;

struct FromSource {
    parse: Parse,
    job: Option<MeshModelingJob>,
}

impl Machine for FromSource {
    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault> {
        if self.job.is_none() {
            match (self.parse)(fuel)? {
                Some(source) => self.job = Some(HalfedgeMesh::polygon_source_job(source).kernel()?),
                None => return Ok(Flow::Working(0.05)),
            }
        }
        let Some(job) = self.job.as_mut() else { return Ok(Flow::Working(0.05)) };
        match modeling_slice(job, fuel.saturating_mul(BATCH)).kernel()? {
            MeshModelingStep::Working(progress) => Ok(Flow::Working(0.05 + 0.9 * ratio(progress))),
            MeshModelingStep::Done(mesh) => Ok(Flow::Done(mesh_output(mesh))),
            MeshModelingStep::Cancelled(_) => Err(cancelled_fault()),
        }
    }

    fn cancel(&mut self) {
        if let Some(job) = self.job.as_mut() {
            job.cancel();
        }
    }
}

/// 🛠️ Starts a widget job that reads a polygon source with the parser `make` builds inside the first step and reconstructs it with the kernel's polygon source job.
pub fn from_source(kind: &Kind, make: impl FnOnce() -> Result<Parse, WidgetFault> + Send + 'static) -> Box<dyn WidgetJob> {
    run(kind, move || Ok(Box::new(FromSource { parse: make()?, job: None }) as Box<dyn Machine>))
}
//#endregion 🔖️Machines
