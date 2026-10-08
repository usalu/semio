//! ⏱️ Phased kernel jobs: the stepped widget job of every compute that imports shape values into a fresh kernel session, runs kernel steps against it and exports the result.
//!
//! A [`Pipeline`] is an ordered list of steps over one [`Work`] (the session, the imported inputs and the groups of result handles). A step that is a lane J operation job is advanced with the remaining fuel of the grant, every other step is one unit, so a widget job yields between import, kernel call and export. A panic inside the kernel becomes a `generation3d.geometry.kernel` fault instead of tearing the engine down.

use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::brep::engine::{BrepOperation, BrepOperationAdmission, BrepOperationJob, BrepOperationStep, GeometryHandle, GeometryKind, ImportedShape, MeshTransfer, ShapeTessellationJob, ShapeValue};
use semio_framework_3d::brep::queries::tessellation::TessellationStep;
use semio_framework_3d::brep::representation::topology::history::PersistentLabel;
use std::collections::{BTreeSet, VecDeque};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

//#region 🔖️Faults
pub const CANCELLED: &str = "generation3d.geometry.cancelled";
pub const SELECTION_STALE: &str = "generation3d.geometry.selection-stale";
pub const SELECTION_COMPONENT: &str = "generation3d.geometry.selection-component";
pub const PIPELINE: &str = "generation3d.geometry.pipeline";
pub const KERNEL: &str = "generation3d.geometry.kernel";

/// 🛑️ The fault of a job that was cancelled before it finished.
pub fn cancelled_fault() -> WidgetFault {
    WidgetFault::new(CANCELLED, "The computation was cancelled.", "Die Berechnung wurde abgebrochen.")
}

fn pipeline_fault() -> WidgetFault {
    WidgetFault::new(PIPELINE, "The computation was set up without the value it needs.", "Die Berechnung wurde ohne den benötigten Wert aufgesetzt.")
}

fn panic_fault() -> WidgetFault {
    WidgetFault::new(KERNEL, "The geometry kernel stopped unexpectedly while computing this widget.", "Der Geometriekern wurde beim Berechnen dieses Widgets unerwartet beendet.")
}

fn stale_fault(port: &str, component: SelectionKind) -> WidgetFault {
    let (en, de) = match component {
        SelectionKind::Face => ("faces", "Flächen"),
        SelectionKind::Edge => ("edges", "Kanten"),
        SelectionKind::Vertex => ("vertices", "Ecken"),
    };
    WidgetFault::new(SELECTION_STALE, format!("The selected {en} no longer exist on the input shape; select them again."), format!("Die gewählten {de} existieren an der Eingabeform nicht mehr; wähle sie erneut.")).at(port)
}

fn component_fault(port: &str, expected: SelectionKind, actual: SelectionKind) -> WidgetFault {
    let name = |kind: SelectionKind| match kind {
        SelectionKind::Face => ("faces", "Flächen"),
        SelectionKind::Edge => ("edges", "Kanten"),
        SelectionKind::Vertex => ("vertices", "Ecken"),
    };
    let (expected, actual) = (name(expected), name(actual));
    WidgetFault::new(SELECTION_COMPONENT, format!("This input takes {} but the selection holds {}.", expected.0, actual.0), format!("Dieser Eingang erwartet {}, die Auswahl enthält aber {}.", expected.1, actual.1)).at(port)
}
//#endregion 🔖️Faults

//#region 🔖️Work
/// 🧰️ Everything one running pipeline owns: its fresh kernel session, the imported inputs and the handle and value groups the steps hand to each other.
pub struct Work {
    pub session: KernelSession,
    sources: Vec<Arc<ShapeValue>>,
    imported: Vec<ImportedShape>,
    pub groups: Vec<Vec<GeometryHandle>>,
    pub values: Vec<Vec<GeometryValue>>,
    pub meshes: Vec<MeshTransfer>,
}

impl Work {
    fn new(sources: Vec<Arc<ShapeValue>>) -> Self {
        Self { session: KernelSession::new(), sources, imported: Vec::new(), groups: Vec::new(), values: Vec::new(), meshes: Vec::new() }
    }

    fn import(&mut self, index: usize) -> Result<(), WidgetFault> {
        let source = self.sources.get(index).cloned().ok_or_else(pipeline_fault)?;
        let imported = self.session.import(&source)?;
        self.imported.push(imported);
        Ok(())
    }

    /// 🧱️ The input shape value at `index`, imported or not.
    pub fn source(&self, index: usize) -> Result<Arc<ShapeValue>, WidgetFault> {
        self.sources.get(index).cloned().ok_or_else(pipeline_fault)
    }

    /// 📥️ The session handle of the imported input at `index`.
    pub fn handle(&self, index: usize) -> Result<GeometryHandle, WidgetFault> {
        self.imported.get(index).map(|imported| imported.handle.clone()).ok_or_else(pipeline_fault)
    }

    /// 📥️ The session handles of every imported input, in import order.
    pub fn handles(&self) -> Vec<GeometryHandle> {
        self.imported.iter().map(|imported| imported.handle.clone()).collect()
    }

    /// 🎯️ The first handle of result group `index`.
    pub fn result(&self, index: usize) -> Result<GeometryHandle, WidgetFault> {
        self.groups.get(index).and_then(|group| group.first()).cloned().ok_or_else(pipeline_fault)
    }

    /// 📤️ A session handle as a shape value.
    pub fn export(&self, handle: &GeometryHandle) -> Result<GeometryValue, WidgetFault> {
        self.session.export(handle).map(GeometryValue::shape)
    }

    /// 🧲️ The session handles of the sub-elements a selection names on the imported input `index`, resolved through the labels of the value the selection was taken from. A label the value no longer carries is `selection-stale`.
    pub fn pick(&mut self, index: usize, selection: &SelectionValue, expected: SelectionKind, port: &str) -> Result<Vec<GeometryHandle>, WidgetFault> {
        if selection.component != expected {
            return Err(component_fault(port, expected, selection.component));
        }
        let kind = match expected {
            SelectionKind::Face => GeometryKind::Face,
            SelectionKind::Edge => GeometryKind::Edge,
            SelectionKind::Vertex => GeometryKind::Vertex,
        };
        let source = self.sources.get(index).cloned().ok_or_else(pipeline_fault)?;
        let imported = self.imported.get(index).cloned().ok_or_else(pipeline_fault)?;
        let known: BTreeSet<u64> = source.components(kind).iter().map(|component| component.label.0).collect();
        let wanted: BTreeSet<u64> = selection.ids.iter().copied().collect();
        let mut handles = Vec::with_capacity(wanted.len());
        for id in wanted {
            if !known.contains(&id) {
                return Err(stale_fault(port, expected));
            }
            let handle = self.session.brep().handle_for_label(imported.session_label(PersistentLabel(id)));
            handles.push(handle.ok_or_else(|| stale_fault(port, expected))?);
        }
        Ok(handles)
    }
}
//#endregion 🔖️Work

//#region 🔖️Pipeline
/// 🪜️ What one step of a pipeline reports: finished, needs more calls with a 0..=1 fraction of its own, or the outputs of the whole widget.
pub enum Flow {
    Next,
    Working(f32),
    Finish(Outputs),
}

type Step = Box<dyn FnMut(&mut Work, usize) -> Result<Flow, WidgetFault> + Send>;

/// 🧵️ The builder of a phased widget job.
pub struct Pipeline {
    quality: Quality,
    sources: Vec<Arc<ShapeValue>>,
    steps: Vec<Step>,
}

/// 🚀️ Builds the pipeline of a compute, or answers with the fault that stopped it from being built.
pub fn launch(kind: &Kind, build: impl FnOnce() -> Result<Pipeline, WidgetFault>) -> Box<dyn WidgetJob> {
    match build() {
        Ok(pipeline) => pipeline.start(),
        Err(fault) => failed(fault, kind.quality),
    }
}

fn fraction(done: usize, total: usize) -> f32 {
    if total == 0 {
        0.0
    } else {
        (done as f32 / total as f32).clamp(0.0, 1.0)
    }
}

impl Pipeline {
    pub fn new(kind: &Kind) -> Self {
        Self { quality: kind.quality, sources: Vec::new(), steps: Vec::new() }
    }

    /// 📥️ Imports a shape value into the session; its handle is `Work::handle(n)` for the n-th import.
    pub fn import(mut self, shape: &Arc<ShapeValue>) -> Self {
        let index = self.sources.len();
        self.sources.push(shape.clone());
        self.step(move |work, _| work.import(index).map(|()| Flow::Next))
    }

    /// 🧱️ Adds an input shape value that stays out of the session, for steps that read the value itself.
    pub fn input(mut self, shape: &Arc<ShapeValue>) -> Self {
        self.sources.push(shape.clone());
        self
    }

    /// 🔺️ Tessellates the input at `index` into `Work::meshes`, a slice of units per call.
    pub fn tessellate(self, index: usize, deflection: f64) -> Self {
        let mut running: Option<ShapeTessellationJob> = None;
        self.step(move |work, fuel| {
            if running.is_none() {
                running = Some(work.source(index)?.tessellate_job(deflection).map_err(|error| kernel_fault(&error))?);
            }
            let job = running.as_mut().ok_or_else(pipeline_fault)?;
            match job.step(fuel).map_err(|error| kernel_fault(&error))? {
                TessellationStep::Working(progress) => Ok(Flow::Working(fraction(progress.units_done, progress.units_total))),
                TessellationStep::Done(_) => {
                    let (mesh, _) = running.take().and_then(ShapeTessellationJob::into_mesh).ok_or_else(pipeline_fault)?;
                    work.meshes.push(mesh);
                    Ok(Flow::Next)
                }
                TessellationStep::Cancelled(_) => Err(cancelled_fault()),
            }
        })
    }

    /// 🪜️ A step that may take several calls.
    pub fn step(mut self, run: impl FnMut(&mut Work, usize) -> Result<Flow, WidgetFault> + Send + 'static) -> Self {
        self.steps.push(Box::new(run));
        self
    }

    /// 🪜️ A step that is one unit of work.
    pub fn once(self, run: impl FnOnce(&mut Work) -> Result<(), WidgetFault> + Send + 'static) -> Self {
        let mut run = Some(run);
        self.step(move |work, _| match run.take() {
            Some(run) => run(work).map(|()| Flow::Next),
            None => Err(pipeline_fault()),
        })
    }

    /// ⏱️ A lane J operation job: planned from the work on its first call, advanced with the remaining fuel, its result handles become group 0.
    pub fn operation(self, plan: impl FnOnce(&mut Work) -> Result<BrepOperation, WidgetFault> + Send + 'static) -> Self {
        self.operations(move |work| plan(work).map(|operation| vec![operation]))
    }

    /// ⏱️ Lane J operation jobs run one after the other, each advanced with the remaining fuel; the result handles of all of them, in order, become group 0.
    pub fn operations(self, plan: impl FnOnce(&mut Work) -> Result<Vec<BrepOperation>, WidgetFault> + Send + 'static) -> Self {
        let mut plan = Some(plan);
        let mut queue: VecDeque<BrepOperation> = VecDeque::new();
        let mut total = 0usize;
        let mut running: Option<BrepOperationJob> = None;
        let mut collected: Vec<GeometryHandle> = Vec::new();
        self.step(move |work, fuel| {
            if let Some(plan) = plan.take() {
                queue = plan(work)?.into();
                total = queue.len();
            }
            loop {
                if running.is_none() {
                    let Some(operation) = queue.pop_front() else {
                        work.groups = vec![std::mem::take(&mut collected)];
                        return Ok(Flow::Next);
                    };
                    match work.session.brep().operation_job_sync(operation).map_err(|error| kernel_fault(&error))? {
                        BrepOperationAdmission::Answered(handles) => {
                            collected.extend(handles);
                            continue;
                        }
                        BrepOperationAdmission::Job(job) => running = Some(job),
                    }
                }
                let job = running.as_mut().ok_or_else(pipeline_fault)?;
                let finished = total.saturating_sub(queue.len() + 1);
                match work.session.brep().step_operation_job_sync(job, fuel).map_err(|error| kernel_fault(&error))? {
                    BrepOperationStep::Working(progress) => return Ok(Flow::Working((finished as f32 + fraction(progress.done, progress.total)) / total.max(1) as f32)),
                    BrepOperationStep::Ready(handles) => {
                        collected.extend(handles);
                        running = None;
                        if !queue.is_empty() {
                            return Ok(Flow::Working((finished + 1) as f32 / total.max(1) as f32));
                        }
                    }
                    BrepOperationStep::Cancelled(_) => return Err(cancelled_fault()),
                }
            }
        })
    }

    /// 📤️ Exports every handle of every group to `Work::values`, as many per call as the fuel allows.
    pub fn export_groups(self) -> Self {
        let mut position = (0usize, 0usize);
        self.step(move |work, fuel| {
            if work.values.len() != work.groups.len() {
                work.values = vec![Vec::new(); work.groups.len()];
            }
            let total: usize = work.groups.iter().map(Vec::len).sum();
            let mut done: usize = work.groups.iter().take(position.0).map(Vec::len).sum::<usize>() + position.1;
            let mut budget = fuel.max(1);
            while position.0 < work.groups.len() && budget > 0 {
                let Some(handle) = work.groups[position.0].get(position.1).cloned() else {
                    position = (position.0 + 1, 0);
                    continue;
                };
                let value = work.export(&handle)?;
                work.values[position.0].push(value);
                position.1 += 1;
                done += 1;
                budget -= 1;
            }
            while position.0 < work.groups.len() && position.1 >= work.groups[position.0].len() {
                position = (position.0 + 1, 0);
            }
            if position.0 >= work.groups.len() {
                Ok(Flow::Next)
            } else {
                Ok(Flow::Working(fraction(done, total)))
            }
        })
    }

    /// 🏁️ Ends the pipeline with the outputs `build` makes from the work.
    pub fn finish(self, build: impl FnOnce(&mut Work) -> Result<Outputs, WidgetFault> + Send + 'static) -> Box<dyn WidgetJob> {
        let mut build = Some(build);
        self.step(move |work, _| match build.take() {
            Some(build) => build(work).map(Flow::Finish),
            None => Err(pipeline_fault()),
        })
        .start()
    }

    /// 🏁️ Ends the pipeline with the first result handle exported as the shape output `port`.
    pub fn exported(self, port: &'static str) -> Box<dyn WidgetJob> {
        self.finish(move |work| {
            let handle = work.result(0)?;
            Ok(outputs([(port, work.export(&handle)?)]))
        })
    }

    /// 🚀️ Starts the job.
    pub fn start(self) -> Box<dyn WidgetJob> {
        let work = Work::new(self.sources);
        Box::new(PhasedJob { quality: self.quality, work: Some(work), steps: self.steps, cursor: 0, inner: 0.0, finished: None })
    }
}
//#endregion 🔖️Pipeline

//#region 🔖️Job
struct PhasedJob {
    quality: Quality,
    work: Option<Work>,
    steps: Vec<Step>,
    cursor: usize,
    inner: f32,
    finished: Option<WidgetEvaluation>,
}

impl PhasedJob {
    fn progress(&self) -> f32 {
        fraction(self.cursor, self.steps.len()) + self.inner / self.steps.len().max(1) as f32
    }

    fn end(&mut self, result: Result<Outputs, WidgetFault>) -> WidgetStep {
        self.work = None;
        self.steps.clear();
        let evaluation = match result {
            Ok(outputs) => WidgetEvaluation::ok(outputs, self.quality),
            Err(fault) => WidgetEvaluation::faulted(fault, self.quality),
        };
        self.finished = Some(evaluation.clone());
        WidgetStep::Done(evaluation)
    }
}

impl WidgetJob for PhasedJob {
    fn step(&mut self, fuel: usize) -> WidgetStep {
        if let Some(evaluation) = &self.finished {
            return WidgetStep::Done(evaluation.clone());
        }
        let mut left = fuel.max(1);
        loop {
            let (Some(work), Some(step)) = (self.work.as_mut(), self.steps.get_mut(self.cursor)) else {
                return self.end(Err(pipeline_fault()));
            };
            let outcome = catch_unwind(AssertUnwindSafe(|| step(work, left))).unwrap_or_else(|_| Err(panic_fault()));
            match outcome {
                Err(fault) => return self.end(Err(fault)),
                Ok(Flow::Finish(outputs)) => return self.end(Ok(outputs)),
                Ok(Flow::Working(inner)) => {
                    self.inner = inner.clamp(0.0, 1.0);
                    return WidgetStep::Working { progress: self.progress().min(0.999) };
                }
                Ok(Flow::Next) => {
                    self.cursor += 1;
                    self.inner = 0.0;
                    left = left.saturating_sub(1);
                    if left == 0 {
                        return WidgetStep::Working { progress: self.progress().min(0.999) };
                    }
                }
            }
        }
    }

    fn cancel(&mut self) {
        if self.finished.is_none() {
            self.end(Err(cancelled_fault()));
        }
    }
}
//#endregion 🔖️Job

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
