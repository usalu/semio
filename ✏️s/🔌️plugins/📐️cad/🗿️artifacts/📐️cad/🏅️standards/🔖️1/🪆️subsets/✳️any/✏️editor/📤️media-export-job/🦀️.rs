//! 📤️ Media job of the CAD play app: the interactive export and import job and its command work. See [`super`].

use super::*;

//#region 🔖️PlayApp
/// 🎞️ A media delivery uses the retained command's exact preparation and publication lifecycle.
pub(super) struct CadMediaJob(pub(super) ArtifactRetainedCommandJob<EditorApp<CadPlayApp>>);

impl semio_framework_job::InteractiveJob for CadMediaJob {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> { semio_framework_job::InteractiveJob::step(&mut self.0, cx) }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> { semio_framework_job::InteractiveJob::borrow_outcome(&self.0, descriptor) }
    fn begin_close(&mut self) { semio_framework_job::InteractiveJob::begin_close(&mut self.0); }
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep { semio_framework_job::InteractiveJob::close_step(&mut self.0, grant) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { semio_framework_job::InteractiveJob::next_close_copy_byte_demand(&self.0) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { semio_framework_job::InteractiveJob::next_close_capacity_byte_demand(&self.0, maximum_copy_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { semio_framework_job::InteractiveJob::next_close_release_byte_demand(&self.0) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { semio_framework_job::InteractiveJob::next_close_depth_demand(&self.0) }
    fn terminal_is_empty(&self) -> bool { semio_framework_job::InteractiveJob::terminal_is_empty(&self.0) }
}

impl semio_framework_plugin::ArtifactReservedJob for CadMediaJob {}

pub(super) struct CadMediaWork {
    pub(super) port: String,
    pub(super) media: Option<Media>,
    pub(super) children: Option<ChildContentView>,
    pending: Option<(String, String, semio_framework_value::DslValue)>,
    pub(super) retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    priced_bytes: usize,
    price_checksum: u64,
    pricing_complete: bool,
    pub(super) consumed: bool,
    closing: bool,
}

impl CadMediaWork {
    pub(super) fn new(port: String, media: Media, children: Option<ChildContentView>) -> Self {
        Self { port, media: Some(media), children, pending: None, retirement: None, priced_bytes: 0, price_checksum: 0, pricing_complete: false, consumed: false, closing: false }
    }
    fn borrowed_source(&self) -> Result<&[u8], Fault> {
        match &self.media.as_ref().ok_or_else(|| Fault::from("Cad media input owner is absent"))?.payload {
            MediaPayload::Structured { json, .. } => Ok(json.as_bytes()),
            MediaPayload::Intrinsic { value, .. } => value.as_bytes().or_else(|| value.as_str().map(str::as_bytes)).ok_or_else(|| Fault::new(semio_framework_diagnostic::FaultOrigin::App, semio_framework_diagnostic::FaultCode::new("cad.media-input-shape"), "Cad file media requires borrowed intrinsic text or bytes")),
            MediaPayload::Binary { .. } => Err(Fault::new(semio_framework_diagnostic::FaultOrigin::App, semio_framework_diagnostic::FaultCode::new("cad.media-input-shape"), "Cad file media requires a retained source rather than a binary address")),
        }
    }
    fn price_source(&mut self, maximum_bytes: usize) -> Result<usize, Fault> {
        if maximum_bytes == 0 || self.pricing_complete { return Ok(0); }
        let source = self.borrowed_source()?;
        if source.len() > semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES {
            return Err(Fault::new(semio_framework_diagnostic::FaultOrigin::App, semio_framework_diagnostic::FaultCode::new("cad.media-input-capacity"), "Cad media source exceeds its declared retained raw-byte authority"));
        }
        let end = self.priced_bytes.saturating_add(maximum_bytes.min(64)).min(source.len());
        let mut checksum = self.price_checksum;
        for byte in &source[self.priced_bytes..end] { checksum = checksum.wrapping_mul(31).wrapping_add(u64::from(*byte)); }
        let complete = end == source.len();
        let charged = end - self.priced_bytes;
        self.price_checksum = checksum;
        self.priced_bytes = end;
        self.pricing_complete = complete;
        Ok(charged)
    }
    fn price_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, Fault> {
        if cx.is_cancelled() { return Err(Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("interactive-job.cancelled"), "Cad media pricing was cancelled before conversion")); }
        if cx.should_yield() { return Ok(false); }
        if !self.pricing_complete {
            self.price_source(64)?;
            cx.consume_fuel(1);
            return Ok(false);
        }
        Ok(true)
    }
}

impl ArtifactCommandWork<EditorApp<CadPlayApp>> for CadMediaWork {
    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, EditorApp<CadPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
    fn tool_id(&self) -> &'static str { "importCadFile" }
    fn workspace_identity(&self) -> u64 { self.borrowed_source().map_or(0, |source| (source.as_ptr() as usize as u64).wrapping_add(source.len() as u64)) }
    fn extent(&self, _: &CadCommand, _: &CadSnapshot, _: &protocol::InteractionState, _: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<CadPlayApp>>>) -> Option<usize> { Some(1) }
    fn checkpoint_byte(&self,index:usize)->Option<u8>{match index{0..=7=>Some(((self.priced_bytes as u64)>>(index*8))as u8),8..=15=>Some((self.price_checksum>>((index-8)*8))as u8),16=>Some(u8::from(self.pricing_complete)),17..=23=>Some(0),_=>None}}
    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 24 || checkpoint[16] > 1 || checkpoint[17..].iter().any(|byte| *byte != 0) { return Err(Fault::from("Cad media pricing checkpoint shape")); }
        let offset = usize::try_from(u64::from_le_bytes(checkpoint[..8].try_into().map_err(|_| Fault::from("Cad media pricing offset"))?)).map_err(|_| Fault::from("Cad media pricing offset authority"))?;
        let source = self.borrowed_source()?;
        if offset > source.len() || (checkpoint[16] == 1 && offset != source.len()) { return Err(Fault::from("Cad media pricing checkpoint source extent")); }
        self.priced_bytes = offset;
        self.price_checksum = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("Cad media pricing checksum"))?);
        self.pricing_complete = checkpoint[16] == 1;
        Ok(())
    }
    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<CadPlayApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<CadPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed { return Err(Fault::from("Cad media operation was already consumed")); }
        let before = (self.priced_bytes, self.pricing_complete);
        if !self.price_step(cx)? {
            let preview = br#"{"en":"Pricing media","de":"Medienaufwand bestimmen"}"#;
            return Ok(if before == (self.priced_bytes, self.pricing_complete) { ArtifactCommandWorkStep::Replay { stage: "cad-media-pricing", preview } } else { ArtifactCommandWorkStep::Progress { stage: "cad-media-pricing", preview } });
        }
        self.consumed = true;
        cx.set_stage("cad-media-decode");
        let mut operation = input.operation.clone();
        operation.operation_id = cx.operation().0;
        operation.generation = cx.generation().0;
        let doc = ArtifactView::with_children(input.snapshot, input.history, self.children.as_ref().ok_or_else(|| Fault::from("Cad media child read owner is absent"))?.clone()).bound_to_operation(operation);
        let media = self.media.as_ref().ok_or_else(|| Fault::from("Cad media input owner is absent"))?;
        let emit = CadPlayApp::import_media(&self.port, media, &doc).map_err(|error| Fault::from(error.to_string()))?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
    fn begin_close(&mut self) { self.closing = true; }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demand(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demand(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demand(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demand(0)?.depth) }
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_value::retained_clone::RetainedCloneStep;
        if !self.closing || grant.maximum_items == 0 { return InteractiveJobCloseStep::Pending { progress: Default::default() }; }
        let child = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        let mapped = |step: Result<RetainedCloneStep, semio_framework_value::ValueError>| match step {
            Ok(RetainedCloneStep::Progress(progress)) | Ok(RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Pending { progress },
            Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
        };
        if self.retirement.is_some() { return mapped(store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, child)); }
        if self.pending.is_some() { return mapped(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending, &mut self.retirement, child)); }
        if self.media.is_some() {
            let payload = self.media.take().expect("admitted media owner").payload;
            let (schema, value) = match payload {
                MediaPayload::Structured { schema, json } => (schema, semio_framework_value::DslValue::String(json)),
                MediaPayload::Intrinsic { schema, value } => (schema, value),
                MediaPayload::Binary { format_kind, blob_hash } => (format_kind, semio_framework_value::DslValue::String(blob_hash)),
            };
            self.pending = Some((std::mem::take(&mut self.port), schema, value));
            return InteractiveJobCloseStep::Pending { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() } };
        }
        if self.children.take().is_some() {
            return InteractiveJobCloseStep::Pending { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() } };
        }
        InteractiveJobCloseStep::Complete { progress: Default::default() }
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.media.is_none() && self.port.is_empty() && self.children.is_none() && self.pending.is_none() && self.retirement.is_none() }
}

impl CadMediaWork {
    fn close_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "Cad media close depth overflow"))?;
            Ok(demand)
        };
        if let Some(retirement) = self.retirement.as_ref() {
            return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(retirement, body)?);
        }
        if self.pending.is_some() {
            return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending)?);
        }
        Ok(if self.media.is_some() || self.children.is_some() { semio_framework_value::RetirementDemand { depth: 1, ..Default::default() } } else { Default::default() })
    }
}
//#endregion 🔖️PlayApp
