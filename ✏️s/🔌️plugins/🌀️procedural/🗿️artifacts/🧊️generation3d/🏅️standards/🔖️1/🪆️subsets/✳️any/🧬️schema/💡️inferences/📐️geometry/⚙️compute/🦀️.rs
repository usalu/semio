//! ⚙️ The compute contract: a widget compute starts a job; the job is stepped with fuel and may be cancelled.

use super::inputs::WidgetInputs;
use super::value::{kernel_fault, Outputs, WidgetEvaluation, WidgetFault};
use crate::standards::v1::subsets::any::schema::catalogue::{Kind, Quality};
use semio_framework_3d::brep::engine::{Brep, GeometryHandle, ImportedShape, ShapeValue};

//#region 🔖️Job
/// 🪜️ One slice of a widget job: it needs more calls (`Working`, with a 0..=1 progress) or it is finished.
#[derive(Clone, Debug, PartialEq)]
pub enum WidgetStep {
    Working { progress: f32 },
    Done(WidgetEvaluation),
}

/// 🧵️ A started widget compute. `step(fuel)` does at most `fuel` units of work; a `Working` slice has consumed the whole grant, a `Done` slice one unit.
/// `cancel` is idempotent and releases whatever the job holds; the job is dropped afterwards.
pub trait WidgetJob: Send {
    fn step(&mut self, fuel: usize) -> WidgetStep;
    fn cancel(&mut self);
}

/// 🏁️ A job that is already finished: every step answers with its evaluation.
pub struct ReadyJob {
    evaluation: WidgetEvaluation,
}

impl ReadyJob {
    pub fn new(evaluation: WidgetEvaluation) -> Self {
        Self { evaluation }
    }
}

impl WidgetJob for ReadyJob {
    fn step(&mut self, _fuel: usize) -> WidgetStep {
        WidgetStep::Done(self.evaluation.clone())
    }

    fn cancel(&mut self) {}
}

/// 🏁️ An already-finished job from a compute result, at the catalogue quality of `kind`.
pub fn finish(kind: &Kind, result: Result<Outputs, WidgetFault>) -> Box<dyn WidgetJob> {
    finish_with_quality(kind, result.map(|outputs| (outputs, kind.quality)))
}

/// 🏁️ An already-finished job whose quality the compute decided itself.
pub fn finish_with_quality(kind: &Kind, result: Result<(Outputs, Quality), WidgetFault>) -> Box<dyn WidgetJob> {
    Box::new(ReadyJob::new(match result {
        Ok((outputs, quality)) => WidgetEvaluation::ok(outputs, quality),
        Err(fault) => WidgetEvaluation::faulted(fault, kind.quality),
    }))
}

/// 🚫️ An already-finished job that failed.
pub fn failed(fault: WidgetFault, quality: Quality) -> Box<dyn WidgetJob> {
    Box::new(ReadyJob::new(WidgetEvaluation::faulted(fault, quality)))
}

/// 🚀️ Starts the job of one widget from its kind and resolved inputs.
pub type StartFn = fn(&Kind, WidgetInputs) -> Box<dyn WidgetJob>;

/// 🗃️ One registration: a catalogue kind id and the compute that starts its job.
#[derive(Clone, Copy)]
pub struct ComputeEntry {
    pub id: &'static str,
    pub start: StartFn,
}
//#endregion 🔖️Job

//#region 🔖️Kernel
/// 🧊️ A fresh kernel session of one widget compute: inputs are imported into it, the result is exported out of it.
pub struct KernelSession {
    brep: Brep,
}

impl Default for KernelSession {
    fn default() -> Self {
        Self::new()
    }
}

impl KernelSession {
    pub fn new() -> Self {
        Self { brep: Brep::new() }
    }

    /// 🧊️ The session's kernel.
    pub fn brep(&mut self) -> &mut Brep {
        &mut self.brep
    }

    /// 📥️ Imports a shape value; labels held against the value translate through the result's `session_label`.
    pub fn import(&mut self, shape: &ShapeValue) -> Result<ImportedShape, WidgetFault> {
        self.brep.import_shape_mapped(shape).map_err(|error| kernel_fault(&error))
    }

    /// 📤️ Exports the shape a handle names as a session-free value.
    pub fn export(&self, handle: &GeometryHandle) -> Result<ShapeValue, WidgetFault> {
        self.brep.export_shape(handle).map_err(|error| kernel_fault(&error))
    }
}
//#endregion 🔖️Kernel
