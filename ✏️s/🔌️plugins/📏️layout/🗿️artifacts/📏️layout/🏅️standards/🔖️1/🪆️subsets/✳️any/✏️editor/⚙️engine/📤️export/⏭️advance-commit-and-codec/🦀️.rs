//! ⏭️ Advance, commit and codec of the layout export job: the base64 stage, the advance, the package commit, the close turn helpers, the interactive job step, the checkpoint codec, the file name helpers, the base64 decoder and the headless batch oracles. See [`super`].

use super::*;

//#region 🧩️Job
impl LayoutExportJob {
    fn base64_one(&mut self,work:&mut OutputWork)->Result<(),String>{
        const TABLE:&[u8;64]=b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let remaining=self.output.len.saturating_sub(self.base64_cursor);if remaining==0{self.stage=ExportStage::PackageCommit;return Ok(());}
        let take=remaining.min(BASE64_INPUT_BYTES_PER_UNIT);let mut input=[0u8;BASE64_INPUT_BYTES_PER_UNIT];self.output.copy_range(self.base64_cursor,&mut input[..take],work)?;
        let mut encoded=[0u8;BASE64_INPUT_BYTES_PER_UNIT.div_ceil(3)*4];let length=take.div_ceil(3)*4;work.copy(length,1)?;
        for(at,triple)in input[..take].chunks(3).enumerate(){let a=triple[0];let b=triple.get(1).copied().unwrap_or(0);let c=triple.get(2).copied().unwrap_or(0);let out=&mut encoded[at*4..at*4+4];out[0]=TABLE[(a>>2)as usize];out[1]=TABLE[((a&3)<<4|b>>4)as usize];out[2]=if triple.len()>1{TABLE[((b&15)<<2|c>>6)as usize]}else{b'='};out[3]=if triple.len()>2{TABLE[(c&63)as usize]}else{b'='};}
        self.encoded.append(&encoded[..length],work)?;self.base64_cursor+=take;Ok(())
    }

    pub(super) fn stage_name(&self) -> &'static str {
        match self.stage {
            ExportStage::Validate => "layout.export.validate",
            ExportStage::Plan => "layout.export.plan",
            ExportStage::BeginEncode=>"layout.export.begin-encode",
            ExportStage::Encode => "layout.export.encode",
            ExportStage::Base64 => "layout.export.base64",
            ExportStage::PackageCommit => "layout.export.package-commit",
            ExportStage::Complete => "layout.export.complete",
        }
    }

    pub(super) fn advance_one(&mut self,context:&mut StepContext<'_>,drawing_read:Option<&semio_framework_io_ports::ChildAdmissionRead<'_>>)->Result<(),semio_framework_job::OriginalTurnFailure>{match self.stage{ExportStage::Plan=>self.plan_one_in(context,drawing_read),ExportStage::BeginEncode=>self.begin_encode_one(context).map(|_|()),ExportStage::Encode=>self.encode_one(context).map(|_|()),ExportStage::Base64=>self.emit_output_one(context,true,|job,work|job.base64_one(work)).map(|_|()),ExportStage::Validate=>self.validate_one().map_err(|error|Fault::from(error).into()),ExportStage::PackageCommit=>self.package_commit_one(context).map(|_|()),ExportStage::Complete=>Ok(())}}

    pub(super) fn milestone(&self)->checkpoint::ExportMilestone{checkpoint::ExportMilestone{stage:self.stage as u8,raw_bytes:self.output.len,encoded_bytes:self.encoded.len,published_chunks:self.commit_cursor}}
    pub(super) fn commit_work_turn(&mut self,committed:bool,context:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let mut work_units=self.work_units.checked_add(1).ok_or_else(||Fault::from("layout-export-work-overflow"))?;let committed_units=self.committed_units.checked_add(u64::from(committed)).ok_or_else(||Fault::from("layout-export-work-overflow"))?;let matched=if let Some(target)=self.restore_target.as_ref(){checkpoint::replay_position(committed_units,target.committed_units,self.active_output().len as u64==target.output_bytes&&self.active_output().digest==target.output_digest,self.stage==ExportStage::Complete).map_err(Fault::from)?}else{false};if matched{work_units=work_units.max(self.restore_target.as_ref().unwrap().work_units);self.restore_target=None;}self.work_units=work_units;if committed{self.committed_units=committed_units;}context.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1+usize::from(committed)+usize::from(matched),copied_bytes:size_of::<u64>()*(1+usize::from(committed))+if matched{size_of::<Option<LayoutExportCheckpoint>>()}else{0},..Default::default()})?;Ok(matched)}

    fn package_commit_one(&mut self,context:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let output=if self.request.as_ref().unwrap().kind.binary(){&mut self.encoded}else{&mut self.output};
        let queue=self.output_chunks.as_ref().unwrap();
        if output.publishing.is_some(){
            let demand=ArtifactOutputChunks::original_push_demands();let grant=context.retained_grant();let own=std::mem::size_of::<usize>();if grant.maximum_items<2||grant.maximum_copy_bytes<demand.copy_bytes.saturating_add(own)||grant.maximum_depth<demand.depth{return Ok(false);}
            let accepted=context.with_retained_loan(semio_framework_value::RetainedCloneGrant{maximum_items:grant.maximum_items-1,maximum_copy_bytes:grant.maximum_copy_bytes-own,..grant},|same|queue.push_original(&mut output.publishing,self.media_output_credit.as_ref(),same))?;
            if accepted{self.commit_cursor+=1;context.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:own,..Default::default()})?;}return Ok(accepted);
        }
        let result=output.transfer_one(context.retained_grant());let progress=match &result{Ok((_,_,progress))=>*progress,Err(error)=>error.retained_progress()};context.consume_retained(progress)?;let(chunk,done,_)=result?;*output.publishing=chunk;
        if done{let demand=ArtifactOutputChunks::original_seal_demands();let grant=context.retained_grant();let own=size_of::<ExportStage>();if grant.maximum_items<2||grant.maximum_copy_bytes<demand.copy_bytes.saturating_add(own)||grant.maximum_depth<demand.depth{return Ok(false);}let accepted=context.with_retained_loan(semio_framework_value::RetainedCloneGrant{maximum_items:grant.maximum_items-1,maximum_copy_bytes:grant.maximum_copy_bytes-own,..grant},|same|queue.seal_original(same))?;if !accepted{return Ok(false);}self.stage=ExportStage::Complete;context.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:own,..Default::default()})?;return Ok(true);}Ok(false)
    }

    fn commit_publication(&mut self) -> Result<LayoutExportPublication, String> {
        let checkpoint = LayoutExportCheckpoint { work_units: self.work_units, committed_units:self.committed_units, output_bytes: self.active_output().len as u64, output_digest: self.active_output().digest };
        let state = encode_checkpoint(&self.operation, self.request.as_ref().unwrap(), &checkpoint)?;
        LayoutExportPublication::new(LayoutExportPublicationKind::Commit, &state)
    }
}

impl LayoutExportJob {
    /// 🔁️ Whether the sealed export is ready for its owner to take custody of the output queue before completion is lent.
    pub fn commit_ready(&self) -> bool {
        matches!(self.stage, ExportStage::Complete) && !self.slot.delivered
    }

    fn close_turn(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<LayoutCloseTurn, Fault> {
        if self.close_stage == LayoutExportCloseStage::Publication {
            return Ok(self.close_publication_step(grant));
        }
        self.close_export_step(grant.maximum_items, grant.maximum_release_bytes)
    }

    pub fn close_demand(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        let one=semio_framework_value::RetirementDemand{depth:1,..Default::default()};
        if self.close_stage==LayoutExportCloseStage::Publication&&!self.slot.terminal_is_empty(){return self.slot.retirement_demands()}
        if self.close_terminal_is_empty(){return Ok(Default::default())}
        if self.close_stage==LayoutExportCloseStage::Output||self.close_stage==LayoutExportCloseStage::Encoded{let output=if self.close_stage==LayoutExportCloseStage::Output{&self.output}else{&self.encoded};return if output.terminal_is_empty(){Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<LayoutExportCloseStage>(),..one})}else{output.close_demand()};}
        if self.close_stage==LayoutExportCloseStage::Interchange{return Ok(semio_framework_value::RetirementDemand{release_bytes:self.interchange_session.as_ref().map_or(0,InterchangeExportSession::next_release_bytes),..one})}
        if self.close_stage==LayoutExportCloseStage::FramePlanning { if let Some(owner)=self.frame_plan_retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,0)} if self.frame_plan.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.frame_plan)} return Ok(one) }
        if self.close_stage==LayoutExportCloseStage::Drawing { if let Some(owner)=self.drawing_retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,0)} if self.drawing.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.drawing)} return Ok(one) }
        if self.close_stage==LayoutExportCloseStage::Paragraphs { if let Some(owner)=self.text_retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,0)} if self.text.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.text)} return Ok(one) }
        if self.close_stage==LayoutExportCloseStage::PdfOffsets{return if self.pdf_offsets.terminal_is_empty(){Ok(one)}else{self.pdf_offsets.next_close_demand()};}
        if self.close_stage==LayoutExportCloseStage::PdfItems{return if self.pdf_items.terminal_is_empty(){Ok(one)}else{self.pdf_items.next_close_demand()};}
        if self.close_stage==LayoutExportCloseStage::PdfPages{return if self.pdf_pages.terminal_is_empty(){Ok(one)}else{self.pdf_pages.next_close_demand()};}
        if self.close_stage==LayoutExportCloseStage::Fidelity { if let Some(owner)=self.fidelity_retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,0)} if self.fidelity.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.fidelity)} return Ok(one) }
        if matches!(self.close_stage,LayoutExportCloseStage::OutputChunks|LayoutExportCloseStage::MediaCredit){
            if let Some(owner)=self.output_retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,0)}
            if self.close_stage==LayoutExportCloseStage::OutputChunks&&self.output_chunks.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.output_chunks)}
            if self.close_stage==LayoutExportCloseStage::MediaCredit&&self.media_output_credit.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.media_output_credit)}return Ok(one)
        }
        if self.close_stage==LayoutExportCloseStage::Snapshot{return Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<LayoutExportRequest>(),..one})}
        if self.close_stage==LayoutExportCloseStage::SnapshotOwner{return Ok(self.snapshot_close.as_ref().map_or(one,|lease|if lease.terminal_is_empty(){one}else{lease.retirement_demands()}))}
        Ok(semio_framework_value::RetirementDemand{release_bytes:self.next_manual_close_release_bytes(),..one})
    }
    fn close_paired_turn(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};let item=RetainedCloneProgress{copied_items:1,..Default::default()};
        if self.output_retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.output_retirement,grant)}
        match self.close_stage{
            LayoutExportCloseStage::PdfOffsets=>{if !self.pdf_offsets.terminal_is_empty(){return self.pdf_offsets.close_one(grant);}self.close_stage=LayoutExportCloseStage::PdfItems;}
            LayoutExportCloseStage::PdfItems=>{if !self.pdf_items.terminal_is_empty(){return self.pdf_items.close_one(grant);}self.close_stage=LayoutExportCloseStage::PdfPages;}
            LayoutExportCloseStage::PdfPages=>{if !self.pdf_pages.terminal_is_empty(){return self.pdf_pages.close_one(grant);}self.close_stage=LayoutExportCloseStage::Interchange;}

            LayoutExportCloseStage::Output|LayoutExportCloseStage::Encoded=>{let output=if self.close_stage==LayoutExportCloseStage::Output{&mut self.output}else{&mut self.encoded};if !output.terminal_is_empty(){return output.close_one(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}self.close_stage=if self.close_stage==LayoutExportCloseStage::Output{LayoutExportCloseStage::Encoded}else{LayoutExportCloseStage::PngRow};return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:size_of::<LayoutExportCloseStage>(),..item}));}
            LayoutExportCloseStage::FramePlanning=>{if self.frame_plan_retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.frame_plan_retirement,grant)}if self.frame_plan.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.frame_plan,&mut self.frame_plan_retirement,grant)}self.close_stage=LayoutExportCloseStage::ZipEntries;}
            LayoutExportCloseStage::Drawing=>{if self.drawing_retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.drawing_retirement,grant)}if self.drawing.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.drawing,&mut self.drawing_retirement,grant)}self.drawing_source=None;self.close_stage=LayoutExportCloseStage::Paragraphs;}
            LayoutExportCloseStage::Paragraphs=>{if self.text_retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.text_retirement,grant)}if self.text.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.text,&mut self.text_retirement,grant)}self.close_stage=LayoutExportCloseStage::FramePlanning;}
            LayoutExportCloseStage::Fidelity=>{if self.fidelity_retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.fidelity_retirement,grant)}if self.fidelity.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.fidelity,&mut self.fidelity_retirement,grant)}self.close_stage=LayoutExportCloseStage::Drawing;}
            LayoutExportCloseStage::OutputChunks=>{if self.output_chunks.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.output_chunks,&mut self.output_retirement,grant)}self.close_stage=LayoutExportCloseStage::MediaCredit;}
            LayoutExportCloseStage::MediaCredit=>{if self.media_output_credit.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.media_output_credit,&mut self.output_retirement,grant)}self.close_stage=LayoutExportCloseStage::Snapshot;}
            LayoutExportCloseStage::Snapshot=>{if let Some(request)=self.request.as_ref(){if !self.snapshot_close.as_ref().map_or_else(||Arc::strong_count(&request.snapshot)>1,|lease|lease.can_release(&request.snapshot)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout snapshot lost its paired original supplier"))}if request.page_id.is_some()||request.preflight_json.is_some()||!request.parent_document_id.is_empty()||!request.canonical_base_revision_hex.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout request metadata remains live before snapshot return"))}self.request.take();self.close_stage=LayoutExportCloseStage::SnapshotOwner;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:size_of::<LayoutExportRequest>(),..item}))}self.close_stage=LayoutExportCloseStage::SnapshotOwner;}
            LayoutExportCloseStage::SnapshotOwner=>{if let Some(lease)=self.snapshot_close.as_mut(){if lease.terminal_is_empty(){self.snapshot_close.take();}else{return lease.close_step(grant)}}else{self.close_stage=LayoutExportCloseStage::Complete;}}
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout paired roots were closed outside their authored stage")),
        }Ok(RetainedCloneStep::Progress(item))
    }

}

impl LayoutExportJob{
    pub fn step_with_drawing_read<'a>(&'a mut self, context: &mut StepContext<'_>, drawing_read:Option<&semio_framework_io_ports::ChildAdmissionRead<'_>>) -> LayoutExportTurn<'a> {
        if !self.slot.settle(context)? {
            return Ok(None);
        }
        if self.slot.publication.is_some() {
            return self.drive_publication(context);
        }
        if context.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(context).map_err(Into::into);
        }
        if (context.operation(),context.generation())!=self.source_identity {
            self.slot.publication = Some(Self::fault_publication("layout-export-stale-operation"));
            return self.drive_publication(context);
        }
        context.set_stage(self.stage_name());
        loop {
            if context.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(context).map_err(Into::into);}if context.should_yield(){return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);}
            let grant=context.retained_grant();let restoring=self.restore_target.is_some();let own_items=2+usize::from(restoring);let own_copy=size_of::<u64>()*2+if restoring{size_of::<Option<LayoutExportCheckpoint>>()}else{0};if grant.maximum_items<own_items||grant.maximum_copy_bytes<own_copy||grant.maximum_depth==0{return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);}
            let required=self.required_drawing_source();let before=self.milestone();context.with_retained_loan(semio_framework_value::RetainedCloneGrant{maximum_items:grant.maximum_items-own_items,maximum_copy_bytes:grant.maximum_copy_bytes-own_copy,..grant},|same|self.advance_one(same,drawing_read))?;
            let committed=checkpoint::committed_milestone(before,self.milestone());let matched=self.commit_work_turn(committed,context)?;context.consume_fuel(1);if self.required_drawing_source()!=required{return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);}
            if context.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(context).map_err(Into::into);}if matched{self.slot.publication=self.preview_publication().ok();return self.drive_publication(context);}if self.restore_target.is_some(){if context.should_yield(){return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);}continue;}
            if matches!(self.stage, ExportStage::Complete) {
                self.slot.publication = Some(match self.commit_publication() {
                    Ok(publication) => publication,
                    Err(error) => Self::fault_publication(&error),
                });
                return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);
            }
            if self.work_units.is_multiple_of(64) {
                self.slot.publication = Some(match self.checkpoint_publication() {
                    Ok(publication) => publication,
                    Err(error) => Self::fault_publication(&error),
                });
                return self.drive_publication(context);
            }
            if self.work_units.is_multiple_of(16) {
                self.slot.publication = self.preview_publication().ok();
                return self.drive_publication(context);
            }
            if context.should_yield() {
                return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);
            }
        }
    }

}

impl InteractiveJob for LayoutExportJob {
    fn step<'a>(&'a mut self,context:&mut StepContext<'_>)->LayoutExportTurn<'a>{self.step_with_drawing_read(context,None)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            _ => self.slot.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        self.close_stage = LayoutExportCloseStage::Publication;
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        self.begin_close();let demand=self.close_demand()?;
        if matches!(self.close_stage,LayoutExportCloseStage::Output|LayoutExportCloseStage::Encoded|LayoutExportCloseStage::PdfOffsets|LayoutExportCloseStage::PdfItems|LayoutExportCloseStage::PdfPages|LayoutExportCloseStage::Fidelity|LayoutExportCloseStage::Drawing|LayoutExportCloseStage::Paragraphs|LayoutExportCloseStage::FramePlanning|LayoutExportCloseStage::OutputChunks|LayoutExportCloseStage::MediaCredit|LayoutExportCloseStage::Snapshot|LayoutExportCloseStage::SnapshotOwner){return semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_paired_turn(grant))}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let step=layout_close_step(self.close_turn(grant))?;cx.consume_retained(step.progress())?;if step.progress().copied_items>0{cx.consume_fuel(1)}Ok(step)
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,_body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.depth)}

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.slot.terminal_is_empty() && self.close_terminal_is_empty()
    }
}

impl ArtifactReservedJob for LayoutExportJob {}
//#endregion 🧩️Job

//#region 🔧️CodecPrimitives
#[cfg(any(test, feature = "testkit"))]
struct CheckpointReader<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) cursor: usize,
}

#[cfg(any(test, feature = "testkit"))]
impl<'a> CheckpointReader<'a> {
    pub(super) fn take(&mut self, count: usize) -> Result<&'a [u8], String> {
        let end = self.cursor.checked_add(count).ok_or("layout-export-checkpoint-decode")?;
        let value = self.bytes.get(self.cursor..end).ok_or("layout-export-checkpoint-decode")?;
        self.cursor = end;
        Ok(value)
    }

    pub(super) fn byte(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    fn fixed_authority(&mut self) -> Result<&'a [u8], String> {
        let length = u16::from_le_bytes(self.take(2)?.try_into().map_err(|_| "layout-export-checkpoint-decode")?) as usize;
        if length > MAX_LAYOUT_EXPORT_AUTHORITY_BYTES {
            return Err("layout-export-checkpoint-decode".into());
        }
        let storage = self.take(MAX_LAYOUT_EXPORT_AUTHORITY_BYTES)?;
        if storage[length..].iter().any(|byte| *byte != 0) {
            return Err("layout-export-checkpoint-decode".into());
        }
        Ok(&storage[..length])
    }

    pub(super) fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().map_err(|_| "layout-export-checkpoint-decode")?))
    }
}

fn checkpoint_kind(kind: LayoutExportKind) -> u8 {
    match kind {
        LayoutExportKind::Png => 0,
        LayoutExportKind::Svg => 1,
        LayoutExportKind::Pdf => 2,
        LayoutExportKind::Package => 3,
        LayoutExportKind::Step => 4,
        LayoutExportKind::Idml => 5,
    }
}

struct CheckpointWriter {
    pub(super) bytes: [u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES],
    pub(super) cursor: usize,
}

impl CheckpointWriter {
    pub(super) fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
        let end = self.cursor.checked_add(bytes.len()).ok_or("layout-export-checkpoint-encode")?;
        self.bytes.get_mut(self.cursor..end).ok_or("layout-export-checkpoint-encode")?.copy_from_slice(bytes);
        self.cursor = end;
        Ok(())
    }

    fn authority(&mut self, value: &str) -> Result<(), String> {
        if value.len() > MAX_LAYOUT_EXPORT_AUTHORITY_BYTES {
            return Err("layout-export-authority-invalid".into());
        }
        self.write(&(value.len() as u16).to_le_bytes())?;
        self.write(value.as_bytes())?;
        self.write(&[0; MAX_LAYOUT_EXPORT_AUTHORITY_BYTES][..MAX_LAYOUT_EXPORT_AUTHORITY_BYTES - value.len()])
    }
}

pub(super) fn encode_checkpoint(operation: &Operation, request: &LayoutExportRequest, checkpoint: &LayoutExportCheckpoint) -> Result<[u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], String> {
    if checkpoint.committed_units>checkpoint.work_units||checkpoint.output_bytes>MAX_LAYOUT_EXPORT_OUTPUT_BYTES as u64{return Err("layout-export-checkpoint-progress".into());}
    let mut state = CheckpointWriter { bytes: [0; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], cursor: 0 };
    state.write(LAYOUT_EXPORT_CHECKPOINT_MAGIC)?;
    state.write(&[checkpoint_kind(request.kind), u8::from(request.page_id.is_some())])?;
    state.authority(request.page_id.as_deref().unwrap_or_default())?;
    state.authority(&request.parent_document_id)?;
    state.write(request.canonical_base_revision_hex.as_bytes())?;
    for value in [operation.operation.0, operation.base_revision.0, operation.generation.0, checkpoint.work_units, checkpoint.committed_units, checkpoint.output_bytes, checkpoint.output_digest] {
        state.write(&value.to_le_bytes())?;
    }
    if state.cursor != MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES {
        return Err("layout-export-checkpoint-encode".into());
    }
    Ok(state.bytes)
}

#[cfg(any(test, feature = "testkit"))]
pub(super) fn decode_checkpoint(operation: &Operation, request: &LayoutExportRequest, state: &[u8]) -> Result<LayoutExportCheckpoint, String> {
    if state.len() != MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES {
        return Err("layout-export-checkpoint-limit".into());
    }
    let mut reader = CheckpointReader { bytes: state, cursor: 0 };
    if reader.take(LAYOUT_EXPORT_CHECKPOINT_MAGIC.len())? != LAYOUT_EXPORT_CHECKPOINT_MAGIC {
        return Err("layout-export-checkpoint-decode".into());
    }
    let kind = reader.byte()?;
    let page_present = reader.byte()?;
    let page_id = reader.fixed_authority()?;
    let parent_document_id = reader.fixed_authority()?;
    let canonical_base_revision_hex = reader.take(64)?;
    let operation_id = reader.u64()?;
    let base_revision = reader.u64()?;
    let generation = reader.u64()?;
    let checkpoint = LayoutExportCheckpoint { work_units: reader.u64()?, committed_units:reader.u64()?, output_bytes: reader.u64()?, output_digest: reader.u64()? };
    if checkpoint.committed_units>checkpoint.work_units||checkpoint.output_bytes>MAX_LAYOUT_EXPORT_OUTPUT_BYTES as u64{return Err("layout-export-checkpoint-progress".into());}
    let expected_page = request.page_id.as_deref().map(str::as_bytes);
    let decoded_page = match page_present {
        0 if page_id.is_empty() => None,
        1 => Some(page_id),
        _ => return Err("layout-export-checkpoint-decode".into()),
    };
    if reader.cursor != state.len()
        || kind != checkpoint_kind(request.kind)
        || decoded_page != expected_page
        || parent_document_id != request.parent_document_id.as_bytes()
        || canonical_base_revision_hex != request.canonical_base_revision_hex.as_bytes()
        || operation_id != operation.operation.0
        || base_revision != operation.base_revision.0
        || generation != operation.generation.0
    {
        return Err("layout-export-checkpoint-authority-mismatch".into());
    }
    Ok(checkpoint)
}

pub fn crc32_update(mut crc: u32, bytes: &[u8]) -> u32 {
    crc = !crc;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb88320 & 0u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}

pub(super) fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

pub(super) fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

pub fn sanitize_filename(value: &str) -> String {
    let value: String = value.chars().take(128).map(|character| if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') { character } else { '-' }).collect();
    if value.is_empty() {
        "layout".into()
    } else {
        value
    }
}

pub fn output_name(request: &LayoutExportRequest) -> String {
    if matches!(request.kind, LayoutExportKind::Package) {
        sanitize_filename(&request.snapshot.name)
    } else {
        request.page_id.as_deref().map_or_else(|| sanitize_filename(&request.snapshot.name), sanitize_filename)
    }
}

pub fn decode_base64(value: &str) -> Result<Vec<u8>, String> {
    let mut result = Vec::with_capacity(value.len() / 4 * 3);
    for quartet in value.as_bytes().chunks_exact(4) {
        let a = base64_value(quartet[0])?;
        let b = base64_value(quartet[1])?;
        let c = base64_value(quartet[2])?;
        let d = base64_value(quartet[3])?;
        result.push(a << 2 | b >> 4);
        if c != 64 {
            result.push(b << 4 | c >> 2);
        }
        if d != 64 {
            result.push(c << 6 | d);
        }
    }
    if value.len() % 4 != 0 {
        return Err("layout-export-invalid-base64".into());
    }
    Ok(result)
}

fn base64_value(byte: u8) -> Result<u8, String> {
    match byte {
        b'A'..=b'Z' => Ok(byte - b'A'),
        b'a'..=b'z' => Ok(byte - b'a' + 26),
        b'0'..=b'9' => Ok(byte - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        b'=' => Ok(64),
        _ => Err("layout-export-invalid-base64".into()),
    }
}
//#endregion 🔧️CodecPrimitives
