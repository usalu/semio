//! 🎞️ Reserved import job of the layout play app: the import tool id and port, the import job with its interactive job and reserved job implementations and the export tool job builder. See [`super`].

use super::*;

//#region 🎞️ReservedImport
/// 🎞️ The framework registers the reserved `import-media` factory for every app but never a concrete
/// job, so `VcsArtifactApp::import_media` fails closed with `interactive-job.missing-reserved-builder`
/// until the app hands one back from [`ArtifactEditor::build_reserved_tool_job`]. Layout's one inbound
/// port is `fields:in`.
pub(super) const LAYOUT_IMPORT_TOOL_ID: &str = "import-media";

const LAYOUT_IMPORT_PORT: &str = "fields:in";

/// 🎞️ Layout's ONE concrete resumable importer. Two bounded steps: decode the `fields:in`
/// `form.dictionary` through [`LayoutPlayApp::import_media`] — the single decoding authority, shared
/// with every non-interactive caller — then publish its mutations through the completion authority.
pub(super) struct LayoutImportJob {
    pub(super) port: Option<String>,
    pub(super) media: Option<Media>,
    pub(super) snapshot: Option<std::sync::Arc<LayoutSnapshot>>,
    pub(super) history: Option<std::sync::Arc<semio_framework_plugin::HistoryView>>,
    pub(super) mutations: Option<Vec<LayoutMutation>>,
    pub(super) active: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    checkpoint_publication: semio_framework_job_publication::RetainedJobPublication,
    checkpoint_delivered: bool,
    pub(super) fault_publication: semio_framework_job_publication::RetainedJobPublication,
    fault_detail: Option<Vec<u8>>,
    pub(super) decoded: bool,
    pub(super) completed: bool,
    closing: bool,
    pub(super) completion: Option<ArtifactToolCompletion<EditorApp<LayoutPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<LayoutPlayApp>>>,
}

type LayoutJobTurn<'a> = Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError>;

impl LayoutImportJob {
    pub(super) fn new(request: ArtifactReservedToolJobRequest<EditorApp<LayoutPlayApp>>, port: String, media: Media) -> Self {
        Self {
            port: Some(port),
            media: Some(media),
            snapshot: Some(request.snapshot),
            history: Some(request.history),
            mutations: None,
            active: None,
            checkpoint_publication: semio_framework_job_publication::RetainedJobPublication::new(),
            checkpoint_delivered: false,
            fault_publication: semio_framework_job_publication::RetainedJobPublication::new(),
            fault_detail: None,
            decoded: false,
            completed: false,
            closing: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
        }
    }

    pub(super) fn fault<'a>(&'a mut self, cx: &mut StepContext<'_>, detail: &str) -> LayoutJobTurn<'a> {
        if self.fault_detail.is_none() {
            let bytes = detail.as_bytes();
            self.fault_detail = Some(bytes[..bytes.len().min(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES)].to_vec());
        }
        let bytes = self.fault_detail.as_deref().unwrap_or_default();
        self.fault_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, bytes, cx)
    }

    fn decode(&mut self) -> Result<(), &'static str> {
        if self.port.as_deref() != Some(LAYOUT_IMPORT_PORT) {
            return Err("layout import only implements fields:in");
        }
        let decoded = {
            let (Some(media), Some(snapshot), Some(history)) = (self.media.as_ref(), self.snapshot.as_ref(), self.history.as_ref()) else {
                return Err("layout import lost its media, snapshot or history authority");
            };
            let doc = ArtifactView::new(snapshot.as_ref(), history.as_ref());
            LayoutPlayApp::import_media(LAYOUT_IMPORT_PORT, media, &doc)
        };
        match decoded {
            Ok(emit) => {
                self.mutations = Some(emit.artifact_mutations);
                self.decoded = true;
                Ok(())
            }
            Err(_) => Err("layout import media decode refused"),
        }
    }

    fn nested(mut demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "layout import close depth overflow"))?;
        Ok(demand)
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(active) = self.active.as_ref() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?);
        }
        if let Some(rejected) = self.pending_completion_rejection.as_ref() {
            if let Ok(emit) = rejected.emit.as_ref() {
                if let Some(demand) = emit.child_close_demands(body)? {
                    return Ok(demand);
                }
            }
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if self.mutations.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.mutations)?);
        }
        if self.media.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.media)?);
        }
        if self.port.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.port)?);
        }
        if self.history.is_some() || self.completion.is_some() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if self.snapshot.is_some() {
            return Ok(semio_framework_value::RetirementDemand { capacity_bytes: semio_framework_value::shared_retirement_birth_bytes::<LayoutSnapshot>(), depth: 2, ..Default::default() });
        }
        if !self.fault_publication.terminal_is_empty() {
            return Self::nested(self.fault_publication.retirement_demands()?);
        }
        if !self.checkpoint_publication.terminal_is_empty() {
            return Self::nested(self.checkpoint_publication.retirement_demands()?);
        }
        Ok(Default::default())
    }

    fn close_original(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let unit = || Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let child = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(grant).map_err(|fault| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, fault.message))? {
                    return Ok(RetainedCloneStep::Progress(step.progress().unwrap_or_default()));
                }
            }
            self.pending_completion_rejection = None;
            return unit();
        }
        if self.mutations.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.mutations, &mut self.active, child);
        }
        if self.media.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.media, &mut self.active, child);
        }
        if self.port.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.port, &mut self.active, child);
        }
        if self.history.take().is_some() || self.completion.take().is_some() {
            return unit();
        }
        if let Some(snapshot) = self.snapshot.take() {
            return match semio_framework_value::admit_shared_retirement(snapshot, child, true) {
                Ok((owner, progress)) => {
                    self.active = Some(owner);
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, snapshot)) => {
                    self.snapshot = Some(snapshot);
                    Err(error)
                }
            };
        }
        if !self.fault_publication.terminal_is_empty() {
            return self.fault_publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if !self.checkpoint_publication.terminal_is_empty() {
            return self.checkpoint_publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

impl LayoutImportJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> LayoutJobTurn<'a> {
        if cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx);
        }
        if self.fault_detail.is_some() {
            return self.fault(cx, "");
        }
        if self.pending_completion_rejection.is_some() {
            return self.fault(cx, "layout import completion remains rejected");
        }
        if !self.decoded {
            cx.set_stage("layout-import-decode");
            if let Err(detail) = self.decode() {
                return self.fault(cx, detail);
            }
            cx.consume_fuel(1);
        }
        if !self.checkpoint_delivered {
            let outcome = self.checkpoint_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Checkpoint { applied_progress: 1 }, &[1], cx)?;
            if outcome.is_some() {
                self.checkpoint_delivered = true;
            }
            return Ok(outcome);
        }
        cx.set_stage("layout-import-publish");
        if !self.completed {
            let mutations = self.mutations.take().unwrap_or_default();
            let Some(completion) = self.completion.as_ref() else {
                return self.fault(cx, "layout import lost its completion authority");
            };
            if !completion.has_mounted_consumer() {
                return self.fault(cx, "layout import completion consumer is absent");
            }
            if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }), semio_framework_plugin::EphemeralEmit::default()) {
                let message = rejected.fault.message.clone();
                self.pending_completion_rejection = Some(rejected);
                return self.fault(cx, &message);
            }
            self.completed = true;
        }
        semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None)
    }
}

impl InteractiveJob for LayoutImportJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::CheckpointReady { .. } => self.checkpoint_publication.borrow_outcome(descriptor),
            semio_framework_job::JobOutcomeKind::Fault => self.fault_publication.borrow_outcome(descriptor),
            semio_framework_job::JobOutcomeKind::PreviewReady => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "layout import publishes no preview")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            self.closing = true;
            Ok(match self.close_original(grant) {
                Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)) => semio_framework_job::InteractiveJobCloseStep::Pending { progress },
                Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)) => semio_framework_job::InteractiveJobCloseStep::Complete { progress },
                Err(error) => return Err(error),
            })
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.port.is_none()
            && self.media.is_none()
            && self.snapshot.is_none()
            && self.history.is_none()
            && self.mutations.is_none()
            && self.active.is_none()
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
            && self.fault_publication.terminal_is_empty()
            && self.checkpoint_publication.terminal_is_empty()
    }
}

impl ArtifactReservedJob for LayoutImportJob {}
//#endregion 🎞️ReservedImport

pub(super) fn layout_build_export_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<LayoutPlayApp>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
    use semio_framework_plugin::__value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
    use semio_s_artifact_layout_layout_engine_export::{LayoutExportKind, LayoutExportRequest};
    let original = request.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original layout export request is absent"))?;
    let (kind, page_id) = match &*original.command {
        LayoutCommand::ExportPng(payload) => (LayoutExportKind::Png, payload.page_id.clone().or_else(|| Some(blueprint::config::from_snapshot(original.window_config.as_ref()).active_page_id))),
        LayoutCommand::ExportSvg(payload) => (LayoutExportKind::Svg, payload.page_id.clone().or_else(|| Some(blueprint::config::from_snapshot(original.window_config.as_ref()).active_page_id))),
        LayoutCommand::ExportPdf(payload) => (LayoutExportKind::Pdf, payload.page_id.clone()),
        LayoutCommand::ExportPackage(_) => (LayoutExportKind::Package, None),
        _ => return Ok(false),
    };
    if original.tool_id != kind.tool_id() {
        return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "layout export command does not match its exact registered tool").into());
    }
    if original.checkpoint.is_some() || original.input.as_ref().is_some_and(|input| input.declared_bytes() > MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES) {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "layout export retained ingress rejects a checkpoint or an oversized command").into());
    }
    let demand = semio_framework::action_bus::ToolJobAdmissionDemand { items: 1, copy_bytes: 0, capacity_bytes: std::mem::size_of::<LayoutExportToolJob>() + std::mem::size_of::<LayoutExportToolPayload>(), release_bytes: 0, depth: 1 };
    semio_framework::action_bus::admit_original_tool_construction(cx, output, demand, |_| {
        let mut taken = request.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original layout export request is absent"))?;
        let canonical_base_revision_hex = taken.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect::<Vec<_>>().join("");
        drop(taken.output_chunks);
        let payload = LayoutExportToolPayload {
            request: LayoutExportRequest { kind, page_id, snapshot: taken.snapshot, preflight_json: None, parent_document_id: taken.parent_document_id, canonical_base_revision_hex },
            output_chunks: semio_framework_plugin::app::ArtifactOutputChunks::new(semio_s_artifact_layout_layout_engine_export::MAX_LAYOUT_EXPORT_OUTPUT_BYTES),
            completion: Some(taken.completion),
        };
        let job = match taken.input.take() {
            Some(input) => LayoutExportToolJob::from_wire_payload(taken.operation, payload, input).map_err(|error| ValueError::new(ValueRefusalKind::AllocationFailed, error.detail))?,
            None => LayoutExportToolJob::from_payload(taken.operation, payload),
        };
        Ok((semio_framework_plugin::ErasedToolJob::from_owned(Box::new(job)), RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: std::mem::size_of::<LayoutExportToolJob>(), ..Default::default() }))
    })
}
