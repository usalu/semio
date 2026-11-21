//! 🔗️ Captures two typed mesh inputs and runs the selected retained polygon operation.

use super::{modeling_slice, mesh_output, ratio, run, Flow, KernelResult, Machine};
use semio_framework_3d::mesh::{HalfedgeMesh, MeshAuthoredDifference, MeshAuthoredIntersection, MeshAuthoredUnion, MeshKernelError, MeshModelingJob, MeshModelingOps, MeshModelingStep, MeshPolygonSourceCapture, MeshPolygonSourceStep, PolygonCombine};
use semio_framework_mesh_engine::PolygonMeshSource;
use semio_framework_value::{RetainedAllocationEffect, RetirementDemand, ValueError};
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, StepContext, WidgetJob};
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_s_artifact_procedural_3d_geometry_value::WidgetFault;
use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use std::sync::Arc;

type StartPair = fn(PolygonMeshSource, PolygonMeshSource) -> Result<Box<dyn PairProducer>, ValueError>;

trait PairProducer: Send {
    fn demand(&self, copy: usize) -> Result<RetirementDemand, ValueError>;
    fn step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<PolygonMeshSource>, ValueError>;
    fn take_progress(&mut self) -> RetainedCloneProgress;
    fn progress(&self) -> f32;
    fn cancel(&mut self) {}
}

impl PairProducer for PolygonCombine {
    fn demand(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        PolygonCombine::demand(self, copy)
    }

    fn step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<PolygonMeshSource>, ValueError> {
        PolygonCombine::step(self, effect)
    }

    fn take_progress(&mut self) -> RetainedCloneProgress {
        PolygonCombine::take_progress(self)
    }

    fn progress(&self) -> f32 {
        PolygonCombine::progress(self)
    }
}

macro_rules! authored_pair {
    ($producer:ty) => {
        impl PairProducer for $producer {
            fn demand(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
                <$producer>::demand(self, copy)
            }

            fn step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<PolygonMeshSource>, ValueError> {
                <$producer>::step(self, effect)
            }

            fn take_progress(&mut self) -> RetainedCloneProgress {
                <$producer>::take_progress(self)
            }

            fn progress(&self) -> f32 {
                <$producer>::progress(self)
            }

            fn cancel(&mut self) {
                <$producer>::cancel(self);
            }
        }
    };
}

authored_pair!(MeshAuthoredIntersection);
authored_pair!(MeshAuthoredDifference);
authored_pair!(MeshAuthoredUnion);

enum MeshPair {
    Capture {
        inputs: [Arc<HalfedgeMesh>; 2],
        capture: Box<MeshPolygonSourceCapture>,
        first: Option<PolygonMeshSource>,
        start: StartPair,
    },
    Produce(Box<dyn PairProducer>),
    Import(MeshModelingJob),
    Done,
}

fn pair_job(kind: &Kind, inputs: WidgetInputs, start: StartPair) -> Box<dyn WidgetJob> {
    run(kind, move || Ok(Box::new(MeshPair::Capture {
        inputs: [inputs.mesh("first")?.clone(), inputs.mesh("second")?.clone()],
        capture: Box::new(MeshPolygonSourceCapture::new()),
        first: None,
        start,
    })))
}

/// 🧲️ Starts a cancellable join of two disjoint polygon sources.
pub fn combine(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    pair_job(kind, inputs, |first, second| {
        let producer = PolygonCombine::new(first, second).map_err(|(error, _, _)| error)?;
        Ok(Box::new(producer))
    })
}

/// 🔀️ Starts a cancellable solid intersection of two convex polygon sources.
pub fn intersect(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    pair_job(kind, inputs, |first, second| Ok(Box::new(MeshAuthoredIntersection::new(first, second))))
}

/// ⚖️ Starts a cancellable solid difference for supported cuts, slabs, tunnels, pockets and cavities.
pub fn difference(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    pair_job(kind, inputs, |first, second| Ok(Box::new(MeshAuthoredDifference::new(first, second))))
}

/// 🤝️ Starts a cancellable union of convex polygon sources.
pub fn union(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    pair_job(kind, inputs, |first, second| Ok(Box::new(MeshAuthoredUnion::new(first, second))))
}

/// 🎟️ Advances a mesh-source capture with the caller's remaining work and allocation credit.
pub fn polygon_capture_slice(capture: &mut MeshPolygonSourceCapture, mesh: &HalfedgeMesh, cx: &mut StepContext<'_>) -> Result<MeshPolygonSourceStep, MeshKernelError> {
    for event in 0..fuel_budget(cx).min(super::BATCH) {
        if cx.should_yield() { break; }
        let grant = cx.retained_grant();
        let demand = capture.demand(mesh, grant.maximum_copy_bytes).map_err(MeshKernelError::Retained)?;
        let admitted = if event == 0 { cx.admit_work_step(demand).map_err(MeshKernelError::Retained)? } else { cx.retained_work_can_enter(demand) };
        if !admitted { break; }
        let Some((result, work)) = cx.with_original_effect(grant, |_, effect| capture.step(mesh, effect)).map_err(MeshKernelError::Retained)? else {
            break;
        };
        cx.consume_work(work);
        let receipt = result.as_ref().map_or_else(|error| error.retained_progress(), |(_, receipt)| *receipt);
        if receipt != Default::default() {
            cx.consume_retained(receipt).map_err(MeshKernelError::Retained)?;
        }
        let (step, _) = result.map_err(MeshKernelError::Retained)?;
        if !matches!(step, MeshPolygonSourceStep::Working) || work == 0 {
            return Ok(step);
        }
    }
    Ok(MeshPolygonSourceStep::Working)
}

impl Machine for MeshPair {
    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<Flow, WidgetFault> {
        let progress = match self {
            Self::Capture { inputs, capture, first, start } => {
                let index = usize::from(first.is_some());
                match polygon_capture_slice(capture, &inputs[index], cx).kernel()? {
                    MeshPolygonSourceStep::Working => {
                        let (done, total) = capture.progress(&inputs[index]);
                        0.1 * (index as f32 + 0.99 * done as f32 / total.max(1) as f32)
                    }
                    MeshPolygonSourceStep::Done(source) => match first.take() {
                        Some(first) => {
                            let producer = start(first, source).map_err(MeshKernelError::Retained).kernel()?;
                            *self = Self::Produce(producer);
                            0.2
                        }
                        None => {
                            *first = Some(source);
                            **capture = MeshPolygonSourceCapture::new();
                            0.1
                        }
                    },
                    MeshPolygonSourceStep::Cancelled => return Err(super::cancelled_fault()),
                }
            }
            Self::Produce(producer) => {
                for event in 0..fuel_budget(cx).min(super::BATCH) {
                    if cx.should_yield() { break; }
                    let grant = cx.retained_grant();
                    let demand = producer.demand(grant.maximum_copy_bytes).map_err(MeshKernelError::Retained).kernel()?;
                    let admitted = if event == 0 { cx.admit_work_step(demand).map_err(MeshKernelError::Retained).kernel()? } else { cx.retained_work_can_enter(demand) };
                    if !admitted { break; }
                    let Some((source, work)) = cx.with_original_effect(grant, |_, effect| producer.step(effect)).map_err(MeshKernelError::Retained).kernel()? else {
                        break;
                    };
                    cx.consume_work(work);
                    let progress = producer.take_progress();
                    if progress != Default::default() {
                        cx.consume_retained(progress).map_err(MeshKernelError::Retained).kernel()?;
                    }
                    if let Some(source) = source.map_err(MeshKernelError::Retained).kernel()? {
                        *self = Self::Import(HalfedgeMesh::polygon_source_job(source).kernel()?);
                        return Ok(Flow::Working(0.6));
                    }
                    if work == 0 { break; }
                }
                0.2 + 0.4 * producer.progress()
            }
            Self::Import(import) => match modeling_slice(import, fuel_budget(cx).saturating_mul(super::BATCH), cx).kernel()? {
                MeshModelingStep::Working(progress) => 0.6 + 0.4 * ratio(progress),
                MeshModelingStep::Done(mesh) => return Ok(Flow::Done(mesh_output(mesh))),
                MeshModelingStep::Cancelled(_) => return Err(super::cancelled_fault()),
            },
            Self::Done => return Err(super::cancelled_fault()),
        };
        Ok(Flow::Working(progress))
    }

    fn cancel(&mut self) {
        match self {
            Self::Capture { capture, .. } => capture.cancel(),
            Self::Produce(producer) => producer.cancel(),
            Self::Import(import) => import.cancel(),
            Self::Done => {},
        }
        *self = Self::Done;
    }
}
