//! ⚙️ The compute contract: a widget compute starts a job; the job is stepped with fuel and may be cancelled.
#![allow(clippy::result_large_err)]

use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_s_artifact_procedural_3d_geometry_value::kernel_fault;
use semio_s_artifact_procedural_3d_geometry_value::Outputs;
use semio_s_artifact_procedural_3d_geometry_value::WidgetEvaluation;
use semio_s_artifact_procedural_3d_geometry_value::WidgetFault;
use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use semio_s_artifact_procedural_3d_model::catalogue::Quality;
use semio_framework_3d_brep_engine::{Brep, GeometryHandle, ImportedShape, ShapeValue};

#[path="🎟️original/🦀️.rs"]
pub mod original;

//#region 🔖️Job
/// 🪜️ One slice of a widget job: it needs more calls (`Working`, with a 0..=1 progress) or it is finished.
#[derive(Clone, Debug, PartialEq)]
pub enum WidgetStep {
    Working { progress: f32 },
    Done(WidgetEvaluation),
}

/// 🧵️ A widget compute retains its original outputs until one handoff or controlled retirement.
/// Cold oracle stepping uses fuel; original stepping consumes the initiating caller directly.
pub trait WidgetJob: Send {
    fn step(&mut self, fuel: usize) -> WidgetStep;
    fn cancel(&mut self);
    fn step_on_original(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<WidgetStep>, semio_framework_job::OriginalTurnFailure> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "widget job has no original funded producer").into())
    }
}

/// 🏁️ Retains the original evaluation until its single output handoff or controlled retirement.
#[derive(semio_framework_value::RetireOwned)]
pub struct ReadyJob {
    evaluation: Option<WidgetEvaluation>,
    original_source: Option<usize>,
    cancelled: bool,
}

impl ReadyJob {
    pub fn new(evaluation: WidgetEvaluation) -> Self {
        Self { evaluation: Some(evaluation), original_source: None, cancelled: false }
    }

    /// 🎟️ Donates an already-produced evaluation header under its actual initiating caller.
    pub(crate) fn admit(original: &mut Option<WidgetEvaluation>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<Self>, semio_framework_job::OriginalTurnFailure> {
        let source = cx.original_source_parent()?.source_identity()?;
        if original.is_none() || !ready_handoff_open(cx) { return Ok(None); }
        cx.consume_retained(semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(Some(Self { evaluation: original.take(), original_source: Some(source), cancelled: false }))
    }
}

fn ready_handoff_open(cx: &semio_framework_job::StepContext<'_>) -> bool {
    let grant = cx.retained_grant();
    !cx.should_yield() && !cx.is_cancelled() && grant.maximum_items > 0 && grant.maximum_depth > 0
}

impl WidgetJob for ReadyJob {
    fn step(&mut self, fuel: usize) -> WidgetStep {
        if fuel == 0 || self.cancelled { return WidgetStep::Working { progress: 1.0 }; }
        self.evaluation.take().map_or(WidgetStep::Working { progress: 1.0 }, WidgetStep::Done)
    }

    fn cancel(&mut self) { self.cancelled = true; }

    fn step_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<WidgetStep>, semio_framework_job::OriginalTurnFailure> {
        let source = cx.original_source_parent()?.source_identity()?;
        if self.original_source != Some(source) {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "ready widget output requires its original initiating source").into());
        }
        if self.cancelled || self.evaluation.is_none() || !ready_handoff_open(cx) { return Ok(None); }
        cx.consume_retained(semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(self.evaluation.take().map(WidgetStep::Done))
    }
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

/// 🧩️ Explicit compute capabilities admitted by the host of an inference run.
pub trait GeometryComputeContext: Send+Sync {
    fn lookup(&self,kind_id:&str)->Option<StartFn>;
    fn start_original(&self,_kind:&Kind,_inputs:&mut Option<WidgetInputs>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<original::OriginalFixedOutputJob>,semio_framework_job::OriginalTurnFailure>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"host has no admitted original geometry producer").into())}
    fn start_original_widget(&self,_widget:&semio_framework_artifact_flow_flow::Widget,_cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<original::OriginalFixedOutputJob>,semio_framework_job::OriginalTurnFailure>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"host has no admitted original source widget producer").into())}
    fn start(&self,kind:&Kind,inputs:WidgetInputs)->Box<dyn WidgetJob> {
        match self.lookup(&kind.id) {Some(start)=>start(kind,inputs),None=>failed(WidgetFault::new("generation3d.geometry.compute-missing",format!("The widget kind {} has no admitted compute.",kind.label.en),format!("Der Widget-Typ {} hat keine zugelassene Berechnung.",kind.label.de)),kind.quality)}
    }
}

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

#[cfg(test)]
#[path = "../💎️value/🧪️tests/🦀️.rs"]
mod original_output_tests;
