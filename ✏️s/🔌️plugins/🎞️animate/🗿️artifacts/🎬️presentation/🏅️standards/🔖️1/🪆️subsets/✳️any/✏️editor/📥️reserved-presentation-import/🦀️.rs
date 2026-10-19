//! 📥️ Reserved presentation import: the import tool id and port, the import job with its actions, its interactive job implementation and its reserved job implementation. See [`super`].

use super::*;

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
    pub(super) port: Option<String>,
    pub(super) media: Option<Media>,
    pub(super) snapshot: Option<std::sync::Arc<PresentationSnapshot>>,
    pub(super) history: Option<std::sync::Arc<semio_framework_plugin::HistoryView>>,
    pub(super) mutations: Option<Vec<PresentationMutation>>,
    pub(super) decoded: bool,
    completed: bool,
    pub(super) closing: bool,
    pub(super) completion: Option<ArtifactToolCompletion<EditorApp<AnimatePresentationPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<AnimatePresentationPlayApp>>>,
    publication: RetainedJobPublication,
    publishing: Option<JobPublicationKind>,
    pub(super) source: Vec<u8>,
    delivered: bool,
    active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
}

impl PresentationImportJob {
    pub(super) fn new(request: ArtifactReservedToolJobRequest<EditorApp<AnimatePresentationPlayApp>>, port: String, media: Media) -> Self {
        Self {
            port: Some(port),
            media: Some(media),
            snapshot: Some(request.snapshot),
            history: Some(request.history),
            mutations: Some(Vec::new()),
            decoded: false,
            completed: false,
            closing: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
            publication: RetainedJobPublication::new(),
            publishing: None,
            source: Vec::new(),
            delivered: false,
            active: None,
        }
    }

    /// 🧯️ Stages one bounded fault description for the next publication turns.
    fn stage_fault(&mut self, detail: &str) {
        let bytes = detail.as_bytes();
        self.source = bytes[..bytes.len().min(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES)].to_vec();
        self.publishing = Some(JobPublicationKind::Fault);
    }

    pub(super) fn decode(&mut self) -> bool {
        if self.port.as_deref() != Some(PRESENTATION_IMPORT_PORT) {
            self.stage_fault("presentation import only implements frames:in");
            return false;
        }
        let decoded = {
            let (Some(media), Some(snapshot), Some(history)) = (self.media.as_ref(), self.snapshot.as_ref(), self.history.as_ref()) else {
                self.stage_fault("presentation import lost its media, snapshot or history authority");
                return false;
            };
            let doc = ArtifactView::new(snapshot.as_ref(), history.as_ref());
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
    pub(super) fn advance(&mut self) -> PresentationImportAction {
        if self.pending_completion_rejection.is_some() {
            self.stage_fault("presentation import completion remains rejected");
            return PresentationImportAction::Publish;
        }
        if !self.decoded {
            if !self.decode() {
                return PresentationImportAction::Publish;
            }
            self.source = vec![1];
            self.publishing = Some(JobPublicationKind::Checkpoint { applied_progress: 1 });
            return PresentationImportAction::Publish;
        }
        if !self.completed {
            let mutations = self.mutations.take().unwrap_or_default();
            let Some(completion) = self.completion.as_ref() else {
                self.stage_fault("presentation import lost its completion authority");
                return PresentationImportAction::Publish;
            };
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

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presentation import close depth overflow"))?; Ok(demand) };
        if !self.publication.terminal_is_empty() { return self.publication.retirement_demands(); }
        if let Some(active) = self.active.as_ref() { return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?); }
        if self.pending_completion_rejection.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_completion_rejection)?); }
        if self.mutations.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.mutations)?); }
        if self.media.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.media)?); }
        if self.port.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.port)?); }
        if self.history.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.history)?); }
        if self.snapshot.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.snapshot)?); }
        if self.completion.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion)?); }
        Ok(semio_framework_value::RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }
}

enum PresentationImportAction {
    Publish,
    Complete,
}

impl PresentationImportJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        if self.delivered {
            let step = self.publication.close_step(cx.retained_grant())?;
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
            if matches!(self.advance(), PresentationImportAction::Complete) {
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

impl InteractiveJob for PresentationImportJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

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

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            let demand = match self.close_demands(grant.maximum_copy_bytes) { Ok(demand) => demand, Err(error) => return Err(error) };
            if self.terminal_is_empty() { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }); }
            if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(InteractiveJobCloseStep::Pending { progress: Default::default() }); }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
            macro_rules! owned {
                ($slot:expr) => {
                    if $slot.is_some() { return Ok(match store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut $slot, &mut self.active, child) { Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() }, Err(error) => return Err(error) }); }
                };
            }
            if !self.publication.terminal_is_empty() { return Ok(match self.publication.close_step(grant) { Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() }, Err(error) => return Err(error) }); }
            if self.delivered { self.delivered = false; return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }); }
            if self.active.is_some() { return Ok(match store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child) { Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() }, Err(error) => return Err(error) }); }
            owned!(self.pending_completion_rejection);
            owned!(self.mutations);
            owned!(self.media);
            owned!(self.port);
            owned!(self.history);
            owned!(self.snapshot);
            owned!(self.completion);
            Ok(InteractiveJobCloseStep::Complete { progress: Default::default() })
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && !self.delivered
            && self.publication.terminal_is_empty()
            && self.active.is_none()
            && self.port.is_none()
            && self.media.is_none()
            && self.snapshot.is_none()
            && self.history.is_none()
            && self.mutations.is_none()
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
    }
}

impl ArtifactReservedJob for PresentationImportJob {}
//#endregion 🎞️ReservedImport
