//! 📥️ Reserved presentation import: the import tool id and port, the import job with its actions, its interactive job implementation and its reserved job implementation. See [`super`].

use super::*;
use semio_framework_plugin::tool_contract::ArtifactReservedWork;

//#region 🎞️ReservedImport
/// 🎞️ The framework registers the reserved `import-media` factory for every app but never a concrete
/// job, so `VcsArtifactApp::import_media` fails closed with `interactive-job.missing-reserved-builder`
/// until the app hands one back from [`ArtifactEditor::build_reserved_tool_job`]. Presentation's one
/// inbound port is `frames:in`. Same shape as `📏️layout`'s `LayoutImportJob`.
pub(super) const PRESENTATION_IMPORT_TOOL_ID: &str = "import-media";

const PRESENTATION_IMPORT_PORT: &str = "frames:in";

/// 🎞️ Presentation's ONE concrete resumable importer. Two bounded steps: decode the `frames:in` frame
/// through [`AnimatePresentationPlayApp::import_media`] — the single decoding authority, shared with
/// every non-interactive caller — then publish its tile mutation through the completion authority.
pub(super) struct PresentationImportJob {
    pub(super) mutations: Option<Vec<PresentationMutation>>,
    pub(super) decoded: bool,
    completed: bool,
    pub(super) closing: bool,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<AnimatePresentationPlayApp>>>,
    publication: RetainedJobPublication,
    publishing: Option<JobPublicationKind>,
    pub(super) source: Vec<u8>,
    delivered: bool,
    active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    last_close_work: u64,
}

impl Default for PresentationImportJob {
    fn default() -> Self {
        Self {
            mutations: Some(Vec::new()),
            decoded: false,
            completed: false,
            closing: false,
            pending_completion_rejection: None,
            publication: RetainedJobPublication::new(),
            publishing: None,
            source: Vec::new(),
            delivered: false,
            active: None,
            last_close_work: 0,
        }
    }

}

impl PresentationImportJob {
    /// 🧯️ Stages one bounded fault description for the next publication turns.
    fn stage_fault(&mut self, detail: &str) {
        let bytes = detail.as_bytes();
        self.source = bytes[..bytes.len().min(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES)].to_vec();
        self.publishing = Some(JobPublicationKind::Fault);
    }

    pub(super) fn decode(&mut self, request:&ArtifactReservedToolJobRequest<EditorApp<AnimatePresentationPlayApp>>) -> bool {
        let Some(ArtifactReservedToolInput::Media { port, media })=request.input.as_ref() else { self.stage_fault("presentation import lost its original media owner"); return false };
        if port != PRESENTATION_IMPORT_PORT {
            self.stage_fault("presentation import only implements frames:in");
            return false;
        }
        let decoded = {
            let doc = ArtifactView::new(request.snapshot.as_ref(), request.history.as_ref());
            AnimatePresentationPlayApp::import_media(PRESENTATION_IMPORT_PORT, media, &doc)
        };
        match decoded {
            Ok(emit) => {
                self.mutations = Some(emit.artifact_mutations);
                self.decoded = true;
                true
            }
            Err(error) => {
                self.stage_fault(&error.to_string());
                false
            }
        }
    }

    /// ⏭️ Runs the next bounded unit of work and reports what the step must publish.
    pub(super) fn advance(&mut self, request:&ArtifactReservedToolJobRequest<EditorApp<AnimatePresentationPlayApp>>) -> PresentationImportAction {
        if self.pending_completion_rejection.is_some() {
            self.stage_fault("presentation import completion remains rejected");
            return PresentationImportAction::Publish;
        }
        if !self.decoded {
            if !self.decode(request) {
                return PresentationImportAction::Publish;
            }
            self.source = vec![1];
            self.publishing = Some(JobPublicationKind::Checkpoint { applied_progress: 1 });
            return PresentationImportAction::Publish;
        }
        if !self.completed {
            let mutations = self.mutations.take().unwrap_or_default();
            let completion = &request.completion;
            if !completion.has_mounted_consumer() {
                self.stage_fault("presentation import completion consumer is absent");
                return PresentationImportAction::Publish;
            }
            if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }), semio_framework_plugin::EphemeralEmit::default()) {
                let message = rejected.fault.message.clone();
                self.pending_completion_rejection = Some(rejected);
                self.stage_fault(&message);
                return PresentationImportAction::Publish;
            }
            self.completed = true;
        }
        PresentationImportAction::Complete
    }

    pub(super) fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if !self.publication.terminal_is_empty(){return self.publication.retirement_demands()}
        if self.delivered{return Ok(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<bool>(),depth:1,..Default::default()})}
        if let Some(active)=self.active.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body)}
        if self.pending_completion_rejection.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_completion_rejection)}
        if self.mutations.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.mutations)}
        if self.source.capacity()!=0{return Ok(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Vec<u8>>(),release_bytes:self.source.capacity(),depth:1,..Default::default()})}
        Ok(Default::default())
    }
    fn close_value(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,semio_framework_value::ValueError>{
        self.last_close_work=0;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        let d=self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes||grant.maximum_depth<d.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let turn=if !self.publication.terminal_is_empty(){self.publication.close_step(grant)}
        else if self.delivered{self.delivered=false;self.last_close_work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
        else if self.active.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active,grant)}
        else if self.pending_completion_rejection.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_completion_rejection,&mut self.active,grant)}
        else if self.mutations.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.mutations,&mut self.active,grant)}
        else if self.source.capacity()!=0{drop(std::mem::take(&mut self.source));self.last_close_work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}
        else{return Ok(RetainedCloneStep::Complete(Default::default()))};
        self.last_close_work=turn.work;turn.result.map(|step|if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
}

enum PresentationImportAction {
    Publish,
    Complete,
}

impl PresentationImportJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, request:&ArtifactReservedToolJobRequest<EditorApp<AnimatePresentationPlayApp>>, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        if self.delivered {
            let turn = self.publication.close_step(cx.retained_grant());
            cx.consume_fuel(turn.work);
            let step = turn.result?;
            cx.consume_retained(step.progress())?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                self.delivered = false;
            }
            return Ok(None);
        }
        if cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx);
        }
        if self.publishing.is_none() {
            cx.set_stage(if self.decoded { "presentation-import-publish" } else { "presentation-import-decode" });
            if matches!(self.advance(request), PresentationImportAction::Complete) {
                return JobOutcomeBorrow::admit_complete(cx, None, None);
            }
            cx.consume_fuel(1);
            return Ok(None);
        }
        let Some(kind) = self.publishing else { return Ok(None) };
        let result = self.publication.advance_from_source(kind, &self.source, cx)?;
        if result.is_some() {
            self.delivered = true;
            if matches!(kind, JobPublicationKind::Checkpoint { .. }) {
                self.publishing = None;
            }
        }
        Ok(result)
    }
}

impl ArtifactReservedWork<EditorApp<AnimatePresentationPlayApp>> for PresentationImportJob {
    fn step<'a>(&'a mut self,request:&'a mut ArtifactReservedToolJobRequest<EditorApp<AnimatePresentationPlayApp>>,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(self.step_value(request,cx).map_err(semio_framework_job::OriginalTurnFailure::from))}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete if self.completed => descriptor.complete(None, None),
            _ => self.publication.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{PresentationImportJob::close_demands(self,body)}
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        self.begin_close();semio_framework_job::close_original_granted_step(cx,|grant|{let result=self.close_value(grant).map(|step|if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{InteractiveJobCloseStep::Pending{progress:step.progress()}});semio_framework_value::RetainedWorkOutcome{result,work:self.last_close_work}})
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && !self.delivered
            && self.publication.terminal_is_empty()
            && self.active.is_none()
            && self.source.capacity() == 0
            && self.mutations.is_none()
            && self.pending_completion_rejection.is_none()
    }
}

//#endregion 🎞️ReservedImport
