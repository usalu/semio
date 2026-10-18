//! ⏭️ Advance, commit and codec of the layout export job: the base64 stage, the advance, the package commit, the close turn helpers, the interactive job step, the checkpoint codec, the file name helpers, the base64 decoder and the headless batch oracles. See [`super`].

use super::*;

//#region 🧩️Job
impl LayoutExportJob {
    fn base64_one(&mut self) -> Result<(), String> {
        const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let remaining = self.output.len.saturating_sub(self.base64_cursor);
        if remaining == 0 {
            if !self.base64_tail.is_empty() {
                let a = self.base64_tail[0];
                let b = self.base64_tail.get(1).copied().unwrap_or(0);
                self.encoded.append(&[TABLE[(a >> 2) as usize], TABLE[((a & 3) << 4 | b >> 4) as usize], if self.base64_tail.len() > 1 { TABLE[((b & 15) << 2) as usize] } else { b'=' }, b'='])?;
                self.base64_tail.clear();
            }
            self.stage = ExportStage::PackageCommit;
            return Ok(());
        }
        let take = remaining.min(BASE64_INPUT_BYTES_PER_UNIT);
        let mut input = Vec::with_capacity(self.base64_tail.len() + take);
        input.extend_from_slice(&self.base64_tail);
        let source = self.output.take_prefix(take);
        if source.len() != take {
            return Err("layout-export-base64-cursor".into());
        }
        input.extend_from_slice(&source);
        self.base64_cursor += take;
        let complete = input.len() / 3 * 3;
        let mut encoded = Vec::with_capacity(complete / 3 * 4);
        for triple in input[..complete].as_chunks::<3>().0 {
            encoded.extend_from_slice(&[TABLE[(triple[0] >> 2) as usize], TABLE[((triple[0] & 3) << 4 | triple[1] >> 4) as usize], TABLE[((triple[1] & 15) << 2 | triple[2] >> 6) as usize], TABLE[(triple[2] & 63) as usize]]);
        }
        self.encoded.append(&encoded)?;
        self.base64_tail.clear();
        self.base64_tail.extend_from_slice(&input[complete..]);
        Ok(())
    }

    pub(super) fn stage_name(&self) -> &'static str {
        match self.stage {
            ExportStage::Validate => "layout.export.validate",
            ExportStage::Plan => "layout.export.plan",
            ExportStage::Encode => "layout.export.encode",
            ExportStage::Base64 => "layout.export.base64",
            ExportStage::PackageCommit => "layout.export.package-commit",
            ExportStage::Complete => "layout.export.complete",
        }
    }

    fn advance_one(&mut self) -> Result<(), String> {
        match self.stage {
            ExportStage::Validate => self.validate_one(),
            ExportStage::Plan => self.plan_one(),
            ExportStage::Encode => self.encode_one(),
            ExportStage::Base64 => self.base64_one(),
            ExportStage::PackageCommit => self.package_commit_one(),
            ExportStage::Complete => Ok(()),
        }
    }

    fn verify_restore_target(&mut self) -> Result<bool, String> {
        let Some(target) = &self.restore_target else { return Ok(false) };
        if self.completed_units < target.completed_units {
            return Ok(false);
        }
        if self.completed_units != target.completed_units || self.active_output().len as u64 != target.output_bytes || self.active_output().digest != target.output_digest {
            return Err("layout-export-checkpoint-replay-mismatch".into());
        }
        self.restore_target = None;
        Ok(true)
    }

    fn package_commit_one(&mut self) -> Result<(), String> {
        let chunk = if self.request.as_ref().unwrap().kind.binary() { self.encoded.take_chunk() } else { self.output.take_chunk() };
        if let Some(bytes) = chunk {
            if let Some(credit) = &self.media_output_credit {
                credit.credit(bytes.len()).map_err(|error| error.message)?;
            }
            self.output_chunks.as_ref().unwrap().push(bytes).map_err(|error| error.message)?;
            self.commit_cursor += 1;
        } else {
            self.output_chunks.as_ref().unwrap().seal().map_err(|error| error.message)?;
            self.stage = ExportStage::Complete;
        }
        Ok(())
    }

    fn commit_publication(&mut self) -> Result<LayoutExportPublication, String> {
        let checkpoint = LayoutExportCheckpoint { completed_units: self.completed_units, output_bytes: self.active_output().len as u64, output_digest: self.active_output().digest };
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
        if matches!(self.close_stage,LayoutExportCloseStage::OutputChunks|LayoutExportCloseStage::MediaCredit){
            if let Some(owner)=self.output_retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,0)}
            if self.close_stage==LayoutExportCloseStage::OutputChunks&&self.output_chunks.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.output_chunks)}
            if self.close_stage==LayoutExportCloseStage::MediaCredit&&self.media_output_credit.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.media_output_credit)}return Ok(one)
        }
        if self.close_stage==LayoutExportCloseStage::Snapshot{return Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<LayoutExportRequest>(),..one})}
        if self.close_stage==LayoutExportCloseStage::SnapshotOwner{return Ok(self.snapshot_close.as_ref().map_or(one,|lease|if lease.terminal_is_empty(){one}else{lease.retirement_demands()}))}
        Ok(semio_framework_value::RetirementDemand{release_bytes:LAYOUT_EXPORT_CLOSE_TURN_BYTES,..one})
    }
    fn close_paired_turn(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};let item=RetainedCloneProgress{copied_items:1,..Default::default()};
        if self.output_retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.output_retirement,grant)}
        match self.close_stage{
            LayoutExportCloseStage::OutputChunks=>{if self.output_chunks.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.output_chunks,&mut self.output_retirement,grant)}self.close_stage=LayoutExportCloseStage::MediaCredit;}
            LayoutExportCloseStage::MediaCredit=>{if self.media_output_credit.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.media_output_credit,&mut self.output_retirement,grant)}self.close_stage=LayoutExportCloseStage::Snapshot;}
            LayoutExportCloseStage::Snapshot=>{if let Some(request)=self.request.as_ref(){if !self.snapshot_close.as_ref().map_or_else(||Arc::strong_count(&request.snapshot)>1,|lease|lease.can_release(&request.snapshot)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout snapshot lost its paired original supplier"))}if request.page_id.is_some()||request.preflight_json.is_some()||!request.parent_document_id.is_empty()||!request.canonical_base_revision_hex.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout request metadata remains live before snapshot return"))}self.request.take();self.close_stage=LayoutExportCloseStage::SnapshotOwner;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:size_of::<LayoutExportRequest>(),..item}))}self.close_stage=LayoutExportCloseStage::SnapshotOwner;}
            LayoutExportCloseStage::SnapshotOwner=>{if let Some(lease)=self.snapshot_close.as_mut(){if lease.terminal_is_empty(){self.snapshot_close.take();}else{return lease.close_step(grant)}}else{self.close_stage=LayoutExportCloseStage::Complete;}}
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout paired roots were closed outside their authored stage")),
        }Ok(RetainedCloneStep::Progress(item))
    }

}

impl InteractiveJob for LayoutExportJob {
    fn step<'a>(&'a mut self, context: &mut StepContext<'_>) -> LayoutExportTurn<'a> {
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
            if let Err(error) = self.advance_one() {
                self.slot.publication = Some(Self::fault_publication(&error));
                return self.drive_publication(context);
            }
            self.completed_units = self.completed_units.saturating_add(1);
            context.consume_fuel(1);
            if context.is_cancelled() {
                return JobOutcomeBorrow::admit_cancelled(context).map_err(Into::into);
            }
            match self.verify_restore_target() {
                Ok(true) => {
                    self.slot.publication = self.preview_publication().ok();
                    return self.drive_publication(context);
                }
                Ok(false) => {}
                Err(error) => {
                    self.slot.publication = Some(Self::fault_publication(&error));
                    return self.drive_publication(context);
                }
            }
            if matches!(self.stage, ExportStage::Complete) {
                self.slot.publication = Some(match self.commit_publication() {
                    Ok(publication) => publication,
                    Err(error) => Self::fault_publication(&error),
                });
                return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);
            }
            if self.completed_units.is_multiple_of(64) {
                self.slot.publication = Some(match self.checkpoint_publication() {
                    Ok(publication) => publication,
                    Err(error) => Self::fault_publication(&error),
                });
                return self.drive_publication(context);
            }
            if self.completed_units.is_multiple_of(16) {
                self.slot.publication = self.preview_publication().ok();
                return self.drive_publication(context);
            }
            if context.should_yield() {
                return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);
            }
        }
    }

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
        if matches!(self.close_stage,LayoutExportCloseStage::OutputChunks|LayoutExportCloseStage::MediaCredit|LayoutExportCloseStage::Snapshot|LayoutExportCloseStage::SnapshotOwner){return semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_paired_turn(grant))}
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
    let mut state = CheckpointWriter { bytes: [0; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], cursor: 0 };
    state.write(LAYOUT_EXPORT_CHECKPOINT_MAGIC)?;
    state.write(&[checkpoint_kind(request.kind), u8::from(request.page_id.is_some())])?;
    state.authority(request.page_id.as_deref().unwrap_or_default())?;
    state.authority(&request.parent_document_id)?;
    state.write(request.canonical_base_revision_hex.as_bytes())?;
    for value in [operation.operation.0, operation.base_revision.0, operation.generation.0, checkpoint.completed_units, checkpoint.output_bytes, checkpoint.output_digest] {
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
    let checkpoint = LayoutExportCheckpoint { completed_units: reader.u64()?, output_bytes: reader.u64()?, output_digest: reader.u64()? };
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

pub(super) fn crc32_update(mut crc: u32, bytes: &[u8]) -> u32 {
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

pub(super) fn decode_base64(value: &str) -> Result<Vec<u8>, String> {
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
