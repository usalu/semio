//! 🎞️ Generation2d import work borrows the same reserved request until its funded terminal close.

use super::*;

//#region 🎞️ReservedImport
pub(super) const GENERATION2D_IMPORT_TOOL_ID: &str = "import-media";

const GENERATION2D_IMPORT_PORT: &str = "params:in";

/// 🎞️ The ONE concrete resumable importer this app owns. The framework registers the reserved
/// `import-media` factory for every app (`register_framework_reserved_factories`) but never a
/// concrete job, so `VcsArtifactApp::import_media` fails closed with
/// `interactive-job.missing-reserved-builder` until the app hands one back here. Two bounded steps:
/// decode the `params:in` object into `replace-widget` rows, then publish them through the
/// completion authority (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) struct Generation2dImportJob {
    media_position: usize,
    pub(super) mutations: Option<Vec<Generation2dMutation>>,
    decoded: bool,
    completed: bool,
    pub(super) closing: bool,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Generation2dPlayApp>>>,
    pub(super) publication: semio_framework_job_publication::RetainedJobPublication,
    publishing: Option<semio_framework_job_publication::JobPublicationKind>,
    pub(super) source: Vec<u8>,
    delivered: bool,
    active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    last_close_work: u64,
}

impl Default for Generation2dImportJob {
    fn default() -> Self {
        Self {
            media_position: 0,
            mutations: Some(Vec::new()),
            decoded: false,
            completed: false,
            closing: false,
            pending_completion_rejection: None,
            publication: semio_framework_job_publication::RetainedJobPublication::new(),
            publishing: None,
            source: Vec::new(),
            delivered: false,
            active: None,
            last_close_work: 0,
        }
    }
}

impl Generation2dImportJob {
    fn numeric_row(snapshot: &Generation2dSnapshot, key: &str, number: f64) -> Option<Generation2dMutation> {
        let semio_framework_artifact_flow_flow::Widget::InputSlider { id, label, min, max, step, .. } = snapshot.host_snapshot.widgets.iter().find(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget) == key)? else { return None };
        Some(semio_s_artifact_procedural_2d_mut_widgets::replace_widget(semio_framework_artifact_flow_flow::Widget::InputSlider { id: id.clone(), label: label.clone(), value: number, min: *min, max: *max, step: *step }))
    }

    fn stage_fault(&mut self, detail: &str) {
        let bytes = detail.as_bytes();
        self.source = bytes[..bytes.len().min(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES)].to_vec();
        self.publishing = Some(semio_framework_job_publication::JobPublicationKind::Fault);
    }

    fn stage_checkpoint(&mut self, applied_progress: u64) {
        self.source = vec![1];
        self.publishing = Some(semio_framework_job_publication::JobPublicationKind::Checkpoint { applied_progress });
    }

    fn decode(&mut self, request: &ArtifactReservedToolJobRequest<EditorApp<Generation2dPlayApp>>) -> Option<&'static str> {
        let Some(ArtifactReservedToolInput::Media { port, media }) = request.input.as_ref() else { return Some("generation2d import lost its original media owner") };
        if port != GENERATION2D_IMPORT_PORT {
            return Some("generation2d import only implements params:in");
        }
        let snapshot = &request.snapshot;
        if let semio_framework_plugin::MediaPayload::Intrinsic { value: source, .. } = &media.payload {
            let Some(object) = source.as_object() else { return Some("generation2d params:in intrinsic payload must be an object") };
            if let Some((key, value)) = object.get(self.media_position) {
                if let Some(mutation) = value.as_f64().and_then(|number| Self::numeric_row(snapshot, key, number)) {
                    self.mutations.get_or_insert_with(Vec::new).push(mutation);
                }
                self.media_position += 1;
            }
            if self.media_position >= object.len() {
                self.decoded = true;
            }
            return None;
        }
        let semio_framework_plugin::MediaPayload::Structured { json: media_json, .. } = &media.payload else {
            return Some("generation2d params:in requires a structured payload");
        };
        let Ok(parsed) = semio_framework_pack_json::parse(media_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            return Some("generation2d params:in payload is not valid json");
        };
        let Some(object) = parsed.as_object() else {
            return Some("generation2d params:in payload must be a JSON object");
        };
        let mut rows = Vec::new();
        for (widget_id_key, value) in object.iter() {
            let Some(number) = value.as_f64() else { continue };
            if let Some(mutation) = Self::numeric_row(snapshot, widget_id_key, number) {
                rows.push(mutation);
            }
        }
        self.mutations = Some(rows);
        self.decoded = true;
        None
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if self.delivered {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(owner) = self.active.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body);
        }
        if self.pending_completion_rejection.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_completion_rejection);
        }
        if self.mutations.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.mutations);
        }
        if self.source.capacity() != 0 {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<Vec<u8>>(), release_bytes: self.source.capacity(), depth: 1, ..Default::default() });
        }
        if self.media_position != 0 {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<usize>(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    fn close_turn(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneProgress, semio_framework_value::ValueError> {
        use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneProgress}, ValueError, ValueRefusalKind};
        self.last_close_work = 0;
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if demand == Default::default() {
            return Ok(Default::default());
        }
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "generation2d import close exceeds its admitted depth"));
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(Default::default());
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if !self.publication.terminal_is_empty() {
            let turn = self.publication.close_step(child);
            self.last_close_work = turn.work;
            return turn.result.map(|step| step.progress());
        }
        if self.delivered {
            self.delivered = false;
            self.last_close_work = 1;
            return Ok(RetainedCloneProgress { copied_items: 1, ..Default::default() });
        }
        if self.active.is_some() {
            let turn = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant);
            self.last_close_work = turn.work;
            return turn.result.map(|step| step.progress());
        }
        if self.pending_completion_rejection.is_some() {
            let turn = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_completion_rejection, &mut self.active, grant);
            self.last_close_work = turn.work;
            return turn.result.map(|step| step.progress());
        }
        if self.mutations.is_some() {
            let turn = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.mutations, &mut self.active, grant);
            self.last_close_work = turn.work;
            return turn.result.map(|step| step.progress());
        }
        self.last_close_work = 1;
        if self.source.capacity() != 0 {
            drop(std::mem::take(&mut self.source));
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() });
        }
        self.media_position = 0;
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }
}

impl Generation2dImportJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, request: &ArtifactReservedToolJobRequest<EditorApp<Generation2dPlayApp>>, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if self.delivered {
            let turn = self.publication.close_step(cx.retained_grant());
            cx.consume_fuel(turn.work);
            let step = turn.result?;
            cx.consume_retained(step.progress())?;
            if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
                self.delivered = false;
            }
            return Ok(None);
        }
        if self.closing || cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        }
        if cx.should_yield() {
            return Ok(None);
        }
        if let Some(kind) = self.publishing {
            let result = self.publication.advance_from_source(kind, &self.source, cx)?;
            if result.is_some() {
                self.delivered = true;
                if matches!(kind, semio_framework_job_publication::JobPublicationKind::Checkpoint { .. }) {
                    self.publishing = None;
                }
            }
            return Ok(result);
        }
        if self.pending_completion_rejection.is_some() {
            self.stage_fault("generation2d import completion remains rejected");
            return Ok(None);
        }
        if self.completed {
            return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into);
        }
        if !self.decoded {
            cx.set_stage("generation2d-import-decode");
            cx.consume_fuel(1);
            if let Some(detail) = self.decode(request) {
                self.stage_fault(detail);
                return Ok(None);
            }
            self.stage_checkpoint(self.media_position.max(1) as u64);
            return Ok(None);
        }
        cx.set_stage("generation2d-import-publish");
        let completion = &request.completion;
        if !completion.has_mounted_consumer() {
            self.stage_fault("generation2d import completion consumer is absent");
            return Ok(None);
        }
        let mutations = self.mutations.take().unwrap_or_default();
        if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }), EphemeralEmit::default()) {
            let message = rejected.fault.message.clone();
            self.pending_completion_rejection = Some(rejected);
            self.stage_fault(&message);
            return Ok(None);
        }
        self.completed = true;
        JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into)
    }
}

impl semio_framework_plugin::tool_contract::ArtifactReservedWork<EditorApp<Generation2dPlayApp>> for Generation2dImportJob {
    fn step<'a>(&'a mut self,request:&'a mut ArtifactReservedToolJobRequest<EditorApp<Generation2dPlayApp>>,cx:&mut semio_framework_job::StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(self.step_value(request,cx))}

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

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Generation2dImportJob::close_demands(self,body)
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            use semio_framework_job::InteractiveJobCloseStep;
            self.closing = true;
            let result = self.close_turn(grant).map(|progress| if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress } } else { InteractiveJobCloseStep::Pending { progress } });
            semio_framework_value::RetainedWorkOutcome { result, work: self.last_close_work }
        })
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.media_position == 0
            && self.mutations.is_none()
            && self.pending_completion_rejection.is_none()
            && self.publication.terminal_is_empty()
            && !self.delivered
            && self.active.is_none()
            && self.source.capacity() == 0
    }
}

//#endregion 🎞️ReservedImport
