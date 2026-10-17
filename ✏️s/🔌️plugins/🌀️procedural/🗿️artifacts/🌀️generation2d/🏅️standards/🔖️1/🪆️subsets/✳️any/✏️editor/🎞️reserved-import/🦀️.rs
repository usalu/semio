//! 🎞️ Reserved import job of the generation2d editor: the media import job with its `InteractiveJob` implementation. See [`super`].

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
    pub(super) port: Option<String>,
    media_json: Option<String>,
    media_intrinsic: Option<semio_framework_value::DslValue>,
    media_position: usize,
    pub(super) snapshot: Option<std::sync::Arc<Generation2dSnapshot>>,
    pub(super) mutations: Option<Vec<Generation2dMutation>>,
    decoded: bool,
    completed: bool,
    pub(super) closing: bool,
    pub(super) completion: Option<ArtifactToolCompletion<EditorApp<Generation2dPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Generation2dPlayApp>>>,
    pub(super) publication: semio_framework_job_publication::RetainedJobPublication,
    publishing: Option<semio_framework_job_publication::JobPublicationKind>,
    pub(super) source: Vec<u8>,
    delivered: bool,
    active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
}

impl Generation2dImportJob {
    pub(super) fn new(request: ArtifactReservedToolJobRequest<EditorApp<Generation2dPlayApp>>, port: String, media: semio_framework_plugin::Media) -> Self {
        let (media_json, media_intrinsic) = match media.payload {
            semio_framework_plugin::MediaPayload::Structured { json, .. } => (Some(json), None),
            semio_framework_plugin::MediaPayload::Binary { .. } => (None, None),
            semio_framework_plugin::MediaPayload::Intrinsic { value, .. } => (None, Some(value)),
        };
        Self {
            port: Some(port),
            media_json,
            media_intrinsic,
            media_position: 0,
            snapshot: Some(request.snapshot),
            mutations: Some(Vec::new()),
            decoded: false,
            completed: false,
            closing: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
            publication: semio_framework_job_publication::RetainedJobPublication::new(),
            publishing: None,
            source: Vec::new(),
            delivered: false,
            active: None,
        }
    }

    fn numeric_row(snapshot: &Generation2dSnapshot, key: &str, number: f64) -> Option<Generation2dMutation> {
        let semio_framework_artifact_flow_flow::Widget::InputSlider { id, label, min, max, step, .. } = snapshot.host_snapshot.widgets.iter().find(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget) == key)? else { return None };
        Some(semio_s_artifact_procedural_2d_mut_widgets::replace_widget(semio_framework_artifact_flow_flow::Widget::InputSlider { id: id.clone(), label: label.clone(), value: number, min: *min, max: *max, step: *step }))
    }

    fn stage_fault(&mut self, detail: &str) {
        let bytes = detail.as_bytes();
        self.source = bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)].to_vec();
        self.publishing = Some(semio_framework_job_publication::JobPublicationKind::Fault);
    }

    fn stage_checkpoint(&mut self, applied_progress: u64) {
        self.source = vec![1];
        self.publishing = Some(semio_framework_job_publication::JobPublicationKind::Checkpoint { applied_progress });
    }

    fn decode(&mut self) -> Option<&'static str> {
        if self.port.as_deref() != Some(GENERATION2D_IMPORT_PORT) {
            return Some("generation2d import only implements params:in");
        }
        let Some(snapshot) = self.snapshot.as_ref() else { return Some("generation2d import lost its snapshot authority") };
        if let Some(source) = self.media_intrinsic.as_ref() {
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
        let Some(media_json) = self.media_json.as_ref() else {
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
        if self.media_intrinsic.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.media_intrinsic);
        }
        if self.media_json.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.media_json);
        }
        if self.port.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.port);
        }
        if self.snapshot.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.snapshot);
        }
        if self.completion.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion);
        }
        Ok(Default::default())
    }

    fn close_turn(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneProgress, semio_framework_value::ValueError> {
        use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneProgress}, ValueError, ValueRefusalKind};
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
            return self.publication.close_step(child).map(|step| step.progress());
        }
        if self.delivered {
            self.delivered = false;
            return Ok(RetainedCloneProgress { copied_items: 1, ..Default::default() });
        }
        if self.active.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| step.progress());
        }
        if self.pending_completion_rejection.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_completion_rejection, &mut self.active, grant).map(|step| step.progress());
        }
        if self.mutations.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.mutations, &mut self.active, grant).map(|step| step.progress());
        }
        if self.media_intrinsic.is_some() {
            self.media_position = 0;
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.media_intrinsic, &mut self.active, grant).map(|step| step.progress());
        }
        if self.media_json.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.media_json, &mut self.active, grant).map(|step| step.progress());
        }
        if self.port.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.port, &mut self.active, grant).map(|step| step.progress());
        }
        if self.snapshot.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.snapshot, &mut self.active, grant).map(|step| step.progress());
        }
        store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.completion, &mut self.active, grant).map(|step| step.progress())
    }
}

impl InteractiveJob for Generation2dImportJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        if self.delivered {
            let step = self.publication.close_step(cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
                self.delivered = false;
            }
            return Ok(None);
        }
        if self.closing || cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx);
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
            return JobOutcomeBorrow::admit_complete(cx, None, None);
        }
        if !self.decoded {
            cx.set_stage("generation2d-import-decode");
            cx.consume_fuel(1);
            if let Some(detail) = self.decode() {
                self.stage_fault(detail);
                return Ok(None);
            }
            self.stage_checkpoint(self.media_position.max(1) as u64);
            return Ok(None);
        }
        cx.set_stage("generation2d-import-publish");
        let Some(completion) = self.completion.as_ref() else {
            self.stage_fault("generation2d import lost its completion authority");
            return Ok(None);
        };
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
        JobOutcomeBorrow::admit_complete(cx, None, None)
    }

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

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(body)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.depth)
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        self.closing = true;
        match self.close_turn(grant) {
            Ok(progress) if self.terminal_is_empty() => InteractiveJobCloseStep::Complete { progress },
            Ok(progress) => InteractiveJobCloseStep::Pending { progress },
            Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.port.is_none()
            && self.media_json.is_none()
            && self.media_intrinsic.is_none()
            && self.media_position == 0
            && self.snapshot.is_none()
            && self.mutations.is_none()
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
            && self.publication.terminal_is_empty()
            && !self.delivered
            && self.active.is_none()
    }
}

impl ArtifactReservedJob for Generation2dImportJob {}
//#endregion 🎞️ReservedImport
