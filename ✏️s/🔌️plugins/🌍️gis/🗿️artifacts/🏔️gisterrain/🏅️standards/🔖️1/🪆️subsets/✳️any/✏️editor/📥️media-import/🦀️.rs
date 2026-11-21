//! 📥️ Retained typed media input for the registered GIS Terrain editor.

use super::{EditorApp, Gis3dPlayApp, GisTerrainMutation};
use semio_framework_job::{
    InteractiveJobCloseStep, InteractiveJobStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind,
    JobOutcomeView, OriginalJobReturnClaim, OriginalTurnFailure, StepContext,
};
use semio_framework_value::{
    RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll,
    RetirementDemand, ValueError, retained_clone::RetainedCloneClose,
};
use semio_plugin::app::EphemeralEmit;
use semio_plugin::plugin_app_close_prelude::ArtifactReservedWork;
use semio_plugin::{
    ArtifactReservedToolInput, ArtifactReservedToolJobRequest, Emit, Fault, NoConfigMutation,
};

type App = EditorApp<Gis3dPlayApp>;
type Publication = (Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault>, EphemeralEmit<App>);

#[derive(Default)]
pub(super) struct MediaImport {
    publication: Option<Publication>,
    retirement: RetainedCloneClose,
    published: bool,
}

impl MediaImport {
    fn advance(&mut self, request: &mut ArtifactReservedToolJobRequest<App>, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'_>>, OriginalTurnFailure> {
        if cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        }
        if self.published {
            return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into);
        }
        if self.publication.is_none() {
            let demand = RetirementDemand { copy_bytes: size_of::<Publication>(), depth: 1, ..Default::default() };
            if cx.should_yield() || !cx.retained_work_can_enter(demand) {
                return Ok(None);
            }
            let result = match request.input.as_ref() {
                Some(ArtifactReservedToolInput::Media { port, media }) => super::media_import_emit(port, media).map_err(|error| Fault::from(error.to_string())),
                _ => Err(Fault::from("GIS terrain import requires media input")),
            };
            self.publication = Some((result, EphemeralEmit::default()));
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(None);
        }
        request.completion.complete_emit_original(&mut self.publication, &mut self.published, cx)?;
        Ok(None)
    }
}

impl ArtifactReservedWork<App> for MediaImport {
    fn step<'a>(&'a mut self, request: &'a mut ArtifactReservedToolJobRequest<App>, cx: &mut StepContext<'_>, _: &mut OriginalJobReturnClaim<'_>) -> InteractiveJobStep<'a> {
        InteractiveJobStep::from_callback(self.advance(request, cx))
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Yield => descriptor.yielded(),
            _ => descriptor.complete(None, None),
        }
    }

    fn begin_close(&mut self) {}

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.retirement.next_owner_demands::<Publication>(self.publication.is_some(), &None, body)
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = self.close_demands(body)?;
        let step = semio_framework_job::close_original_effect_step(cx, demand, |effect| {
            if self.retirement.is_empty() {
                let turn = self.retirement.begin_granted(&mut self.publication, effect);
                RetainedWorkPoll::Complete(RetainedWorkOutcome {
                    work: turn.work,
                    result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Complete(Default::default()))).map_err(Into::into),
                })
            } else {
                self.retirement.step_granted(body, effect)
            }
        })?;
        Ok(if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress: step.progress() } } else { step })
    }

    fn terminal_is_empty(&self) -> bool {
        self.publication.is_none() && self.retirement.is_empty()
    }
}
