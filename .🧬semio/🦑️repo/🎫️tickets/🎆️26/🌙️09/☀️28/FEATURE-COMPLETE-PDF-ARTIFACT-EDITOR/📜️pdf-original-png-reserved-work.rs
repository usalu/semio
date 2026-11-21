//! 🖼️ Runs the registered picker PNG format under its retained document and input lease.

use super::{PdfMutation, PdfSnapshot, PngFormatWork};
use semio_plugin::{
    app::{ArtifactMutationOutcome, ArtifactReservedToolInput, ArtifactReservedToolJobRequest, ArtifactReservedWork},
    ArtifactApp, Emit, EphemeralEmit, NoConfigMutation, NoDraftMutation, original_media::OriginalMediaLoan,
    __job::{
        InteractiveJobCloseStep, InteractiveJobStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView,
        OriginalJobReturnClaim, OriginalTurnFailure, StepContext, close_original_effect_step,
    },
    __value::{OriginalRetirementFailure, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll,
        RetirementDemand, ValueError, retained_clone::RetainedCloneClose},
};
type Emission<A> = (ArtifactMutationOutcome<PdfMutation>, EphemeralEmit<A>);

/// 🧰 Keeps decoded PNG work and its one paged edit under the reserved job's Original owner.
pub struct PickerPngWork<A: ArtifactApp> {
    format: Option<Box<PngFormatWork>>,
    emit: Option<Emission<A>>,
    retirement: RetainedCloneClose,
    published: bool,
}

impl<A: ArtifactApp> Default for PickerPngWork<A> {
    fn default() -> Self {
        Self { format: None, emit: None, retirement: RetainedCloneClose::default(), published: false }
    }
}

impl<A> PickerPngWork<A>
where
    A: ArtifactApp<Snapshot = PdfSnapshot, Mutation = PdfMutation, ConfigMutation = NoConfigMutation, DraftMutation = NoDraftMutation>,
{
    fn advance(&mut self, request: &mut ArtifactReservedToolJobRequest<A>, cx: &mut StepContext<'_>)
        -> Result<Option<JobOutcomeBorrow<'_>>, OriginalTurnFailure>
    {
        if self.published { return Ok(JobOutcomeBorrow::admit_complete(cx, None, None)?); }
        let Some(ArtifactReservedToolInput::OriginalPicker { input }) = request.input.as_mut() else {
            return Err(ValueError::invariant("PNG picker job lost its Original input").into());
        };
        let identity = input.loan();
        let live = OriginalMediaLoan {
            operation_id: identity.operation_id, generation: identity.generation, instance_id: request.app_instance_id,
            document_id: &request.parent_document_id, base_revision: &request.canonical_base_revision,
        };
        if identity != live {
            return Err(ValueError::invariant("PNG picker job changed its document identity").into());
        }
        if self.emit.is_none() {
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Option<Emission<A>>>(), ..Default::default() };
            if cx.should_yield() || cx.is_cancelled() || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
                return Ok(None);
            }
            cx.consume_retained(progress)?;
            cx.consume_fuel(1);
            self.emit = Some((Ok(Emit::default()), EphemeralEmit::default()));
            return Ok(None);
        }
        let emit = self.emit.as_mut().unwrap().0.as_mut().expect("picker emission starts as a successful edit");
        if emit.artifact_mutation_pages.is_none() {
            if !PngFormatWork::admit(&mut self.format, cx)? { return Ok(None); }
            self.format.as_mut().unwrap().advance(input, live, request.snapshot_read.get(), &mut emit.artifact_mutation_pages, cx)?;
            return Ok(None);
        }
        if self.format.is_some() {
            cx.with_original_retirement(|cx| PngFormatWork::close(&mut self.format, cx))?;
            return Ok(None);
        }
        if request.completion.complete_emit_original(&mut self.emit, &mut self.published, cx)? {
            return Ok(JobOutcomeBorrow::admit_complete(cx, None, None)?);
        }
        Ok(None)
    }
}

impl<A> ArtifactReservedWork<A> for PickerPngWork<A>
where
    A: ArtifactApp<Snapshot = PdfSnapshot, Mutation = PdfMutation, ConfigMutation = NoConfigMutation, DraftMutation = NoDraftMutation>,
{
    fn step<'a>(&'a mut self, request: &'a mut ArtifactReservedToolJobRequest<A>, cx: &mut StepContext<'_>,
        _: &mut OriginalJobReturnClaim<'_>) -> InteractiveJobStep<'a>
    {
        InteractiveJobStep::from_callback(self.advance(request, cx))
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        descriptor.complete(None, None)
    }

    fn begin_close(&mut self) {}

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.format.is_some() { return PngFormatWork::close_demand(&self.format, body); }
        self.retirement.next_owner_demands::<Emission<A>>(self.emit.is_some(), &None, body)
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.format.is_some() {
            let before = cx.retained_progress();
            let _ = PngFormatWork::close(&mut self.format, cx)?;
            return Ok(InteractiveJobCloseStep::Pending { progress: semio_plugin::app::original_lifecycle_delta(before, cx.retained_progress())? });
        }
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = self.close_demands(body)?;
        close_original_effect_step(cx, demand, |effect| {
            if self.retirement.is_empty() {
                let turn = self.retirement.begin_granted(&mut self.emit, effect);
                RetainedWorkPoll::Complete(RetainedWorkOutcome {
                    work: turn.work,
                    result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Complete(Default::default())))
                        .map_err(OriginalRetirementFailure::from),
                })
            } else {
                self.retirement.step_granted(body, effect)
            }
        })
    }

    fn terminal_is_empty(&self) -> bool {
        self.format.is_none() && self.emit.is_none() && self.retirement.is_empty()
    }
}
