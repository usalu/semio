//! 📤️ Export tool job of the layout play app: the export tool payload, the export tool job with its step, close and interactive job implementations and the export job factory. See [`super`].

use super::*;

pub struct LayoutExportToolPayload {
    pub request: LayoutExportRequest,
    pub output_chunks: ArtifactOutputChunks,
    pub completion: Option<ArtifactToolCompletion<EditorApp<LayoutPlayApp>>>,
}

pub struct LayoutExportToolJob {
    pub(super) inner: Option<LayoutExportJob>,
    pub(super) pending_operation: Option<Operation>,
    pub(super) pending_request: Option<LayoutExportRequest>,
    pub(super) pending_output_chunks: Option<ArtifactOutputChunks>,
    pub(super) completion: Option<ArtifactToolCompletion<EditorApp<LayoutPlayApp>>>,
    pub(super) kind: LayoutExportKind,
    pub(super) name: String,
    pub(super) raw_input: Option<RetainedToolWireInput>,
    pub(super) raw_bytes: Vec<u8>,
    pub(super) raw_page_cursor: usize,
    pub(super) raw_validated: bool,
    pub(super) completed: bool,
    pub(super) slot: LayoutPublicationSlot,
    pub(super) pending_download: Option<ArtifactDownloadOutput>,
    pub(super) rejected_download: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
}

impl LayoutExportToolJob {
    fn materialize_decoded_job(&mut self,source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation)) -> bool {
        if self.inner.is_some() {
            return true;
        }
        let (Some(operation), Some(request), Some(output_chunks)) = (self.pending_operation.take(), self.pending_request.take(), self.pending_output_chunks.take()) else {
            return false;
        };
        self.name = output_name(&request);
        self.inner = Some(LayoutExportJob::from_admitted(operation,source_identity,request,output_chunks));
        self.inner.is_some()
    }

    /// 🔐️ The retained wire is the command's `OpBinary` encoding (ticket
    /// 26/09/18/LAYOUT-PDF-EXPORT-END-TO-END) — the JSON-text state machine that used to sit here
    /// faulted at the first byte of every real export, which the shell only showed as an empty
    /// `typed-operation failed:`. Verb and page target must match the reducer-built request.
    fn decoded_wire_command_matches(&self) -> bool {
        let Ok(command) = <crate::LayoutCommand as protocol::OpBinary>::decode_op(&self.raw_bytes) else { return false };
        if command.command_id() != self.kind.tool_id() {
            return false;
        }
        let page_id = match &command {
            crate::LayoutCommand::ExportPng(payload) => payload.page_id.clone(),
            crate::LayoutCommand::ExportSvg(payload) => payload.page_id.clone(),
            crate::LayoutCommand::ExportPdf(payload) => payload.page_id.clone(),
            _ => None,
        };
        match self.kind {
            LayoutExportKind::Package => page_id.is_none(),
            _ => page_id.as_ref().is_none_or(|page_id| self.pending_request.as_ref().and_then(|request| request.page_id.as_ref()) == Some(page_id)),
        }
    }
}

impl LayoutExportToolJob {
    pub(super) fn fault<'a>(&'a mut self, context: &mut StepContext<'_>, detail: &str) -> LayoutExportTurn<'a> {
        self.slot.publication = Some(LayoutExportJob::fault_publication(detail));
        self.slot.advance(context)
    }

    pub(super) fn checkpoint<'a>(&'a mut self, context: &mut StepContext<'_>, applied_progress: u64) -> LayoutExportTurn<'a> {
        let cursor = (self.raw_page_cursor as u64).to_le_bytes();
        self.slot.publication = LayoutExportPublication::new(LayoutExportPublicationKind::Checkpoint { applied_progress }, &cursor).ok();
        self.slot.advance(context)
    }

    pub(super) fn close_demand(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        let one=semio_framework_value::RetirementDemand{depth:1,..Default::default()};
        if let Some(owner)=self.rejected_download.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,0)}
        if self.pending_download.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_download)}
        if let Some(inner)=self.inner.as_ref(){return if inner.terminal_is_empty(){Ok(one)}else{inner.close_demand()}}
        if self.pending_request.is_some(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<LayoutExportJob>(),..one})}
        if self.raw_bytes.capacity()!=0{return Ok(semio_framework_value::RetirementDemand{release_bytes:self.raw_bytes.capacity(),..one})}
        if let Some(input)=self.raw_input.as_ref(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:input.next_close_copy_byte_demand()?,capacity_bytes:input.next_close_capacity_byte_demand(0)?,release_bytes:input.next_close_release_byte_demand()?,depth:input.next_close_depth_demand()?.max(1)})}
        if self.name.capacity()!=0{return Ok(semio_framework_value::RetirementDemand{release_bytes:self.name.capacity(),..one})}
        if !self.slot.terminal_is_empty(){return self.slot.retirement_demands()}
        if self.completion.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion)}
        Ok(Default::default())
    }

}

impl InteractiveJob for LayoutExportToolJob {
    fn step<'a>(&'a mut self, context: &mut StepContext<'_>) -> LayoutExportTurn<'a> {
        if !self.slot.settle(context)? {
            return Ok(None);
        }
        if self.slot.publication.is_some() {
            return self.slot.advance(context);
        }
        if !self.raw_validated {
            if context.is_cancelled() {
                return JobOutcomeBorrow::admit_cancelled(context).map_err(Into::into);
            }
            if context.should_yield() || context.fuel_remaining() == 0 {
                return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);
            }
            context.set_stage("layout-export-retained-wire-decode");
            let Some(input) = self.raw_input.as_ref() else {
                return self.fault(context, "layout-export-wire-input-missing");
            };
            if let Some(page) = input.page(self.raw_page_cursor) {
                self.raw_bytes.extend_from_slice(page);
                self.raw_page_cursor = self.raw_page_cursor.saturating_add(1);
                context.consume_fuel(1);
                let progress = self.raw_bytes.len() as u64;
                return self.checkpoint(context, progress);
            }
            if !self.decoded_wire_command_matches() {
                return self.fault(context, "layout-export-wire-command-mismatch");
            }
            if !self.materialize_decoded_job((context.operation(),context.generation())) {
                return self.fault(context, "layout-export-wire-job-refused");
            }
            self.raw_validated = true;
            context.consume_fuel(1);
            let progress = self.raw_bytes.len().saturating_add(1) as u64;
            return self.checkpoint(context, progress);
        }
        let Some(inner) = self.inner.as_mut() else {
            self.slot.publication = Some(LayoutExportJob::fault_publication("layout-export-inner-missing"));
            return self.slot.advance(context);
        };
        if inner.commit_ready() && !self.completed && self.completion.is_some() {
            // 📤️ The download OWNS the sealed chunk queue from here: `ArtifactOutputChunks` clones
            // share one queue, and the job's own close (`LayoutExportCloseStage::OutputChunks`) pops it —
            // which drained every export to a 0-byte `Demo.pdf` before the host could take a single
            // chunk (ticket 26/09/18/LAYOUT-PDF-EXPORT-END-TO-END). The job keeps an empty stand-in.
            let chunks = inner.output_chunks.take().expect("layout publication owns its admitted output");
            let download = ArtifactDownloadOutput::new(format!("{}.{}", sanitize_filename(&self.name), self.kind.extension()), self.kind.mime_type(), self.kind.binary().then(|| "base64".into()), chunks);
            if let Some(completion) = &self.completion {
                if let Err(rejected) = completion.complete_download(download, EphemeralEmit::<EditorApp<LayoutPlayApp>>::default()) {
                    if let Ok(download) = rejected.download {
                        self.pending_download = Some(download);
                    }
                    self.slot.publication = Some(LayoutExportJob::fault_publication("layout-export-download-rejected"));
                    return self.slot.advance(context);
                }
            }
            self.completed = true;
        }
        inner.step(context)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            _ if self.slot.delivered => self.slot.borrow_outcome(descriptor),
            _ => self.inner.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "layout export outcome lost its job"))?.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
        if let Some(inner) = self.inner.as_mut() {
            inner.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};self.begin_close();
        if let Some(inner)=self.inner.as_mut(){if !inner.terminal_is_empty(){return inner.close_step(cx).map(|step|InteractiveJobCloseStep::Pending{progress:step.progress()})}}
        if self.inner.is_none()&&self.pending_request.is_none(){if let Some(input)=self.raw_input.as_mut(){let step=input.close_step(cx)?;if input.terminal_is_empty(){self.raw_input.take();}return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}}
        let source_identity=(cx.operation(),cx.generation());let demand=self.close_demand()?;let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
            let item=RetainedCloneProgress{copied_items:1,..Default::default()};
            if self.rejected_download.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.rejected_download,grant)}
            if self.pending_download.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_download,&mut self.rejected_download,grant)}
            if self.inner.is_some(){self.inner.take();return Ok(RetainedCloneStep::Progress(item))}
            if self.pending_request.is_some(){if self.pending_operation.is_none()||self.pending_output_chunks.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout pending producer lost one of its paired original inputs"))}let operation=self.pending_operation.take().unwrap();let request=self.pending_request.take().unwrap();let chunks=self.pending_output_chunks.take().unwrap();let mut inner=LayoutExportJob::from_admitted(operation,source_identity,request,chunks);inner.begin_close();self.inner=Some(inner);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:size_of::<LayoutExportJob>(),..item}))}
            if self.raw_bytes.capacity()!=0{let bytes=std::mem::take(&mut self.raw_bytes).capacity();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes:bytes,..item}))}
            if self.name.capacity()!=0{let bytes=std::mem::take(&mut self.name).capacity();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes:bytes,..item}))}
            if !self.slot.terminal_is_empty(){return self.slot.close_step(grant)}
            if self.completion.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.completion,&mut self.rejected_download,grant)}
            Ok(RetainedCloneStep::Complete(Default::default()))
        })?;Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,_body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.depth)}

    fn terminal_is_empty(&self) -> bool {
        self.rejected_download.is_none() && self.pending_download.is_none() && self.slot.terminal_is_empty() && self.name.capacity()==0 && self.completion.is_none() && self.raw_input.is_none() && self.raw_bytes.capacity()==0 && self.inner.is_none() && self.pending_operation.is_none() && self.pending_request.is_none() && self.pending_output_chunks.is_none()
    }
}

pub struct LayoutExportJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}
