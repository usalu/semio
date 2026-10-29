//! 🏗️ Job admission and close of the layout export job: the job record, its admission from the reserved request, the bounded close of its buffers, the close and publication steps and the publication helpers. See [`super`].

use super::*;

//#region 🧩️Job
pub struct LayoutExportJob {
    pub(super) operation: Operation,
    pub(super) source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
    pub request: Option<LayoutExportRequest>,
    pub(super) stage: ExportStage,
    pub(super) validation_group: u8,
    pub(super) validation_index: usize,
    pub(super) validation_nested: NestedValidationCursor,
    pub(super) validation_total_items: usize,
    pub json_validation: Option<JsonValidationCursor>,
    pub typed_validation: Option<TypedJsonCursor>,
    pub(super) target_page: Option<usize>,
    pub(super) pdf_background_cursor: u8,
    pub(super) encode_cursor: usize,
    pub output: ChunkRope,
    pub encoded: ChunkRope,
    pub(super) base64_cursor: usize,
    pub(super) media_projection_phase:u8,
    pub(super) png_adler_s1: u32,
    pub(super) png_adler_s2: u32,
    pub(super) png_idat_crc: u32,
    pub png_row: Vec<u8>,
    pub(super) png_row_section: PngRowSection,
    pub(super) png_row_byte_cursor: usize,
    pub(super) png_shape_cursor: usize,
    pub(super) png_fill_x: u32,
    pub pdf_offsets: PlanOffsets,
    pub(super) pdf_section: PdfSection,
    pub(super) pdf_cursor: usize,
    pub(super) pdf_xref_offset: usize,
    pub(super) pdf_items: PlanItems,
    pub(super) pdf_pages: PlanPages,
    pub(super) pdf_page_cursor: usize,
    pub(super) pdf_page_started: bool,
    pub(super) pdf_plan_items: usize,
    pub(super) pdf_item_cursor: usize,
    pub(super) pdf_font_cursor: usize,
    pub(super) pdf_content_start: usize,
    pub(super) interchange_session: Option<InterchangeExportSession>,
    pub(super) fidelity: Option<FidelityState>,
    pub(super) fidelity_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(super) drawing: Option<DrawingCompositionState>,
    pub(super) drawing_source:Option<DrawingSourceAddress>,
    pub(super) drawing_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(super) frame_plan: Option<super::frame_planning::FramePlanState>,
    pub(super) frame_plan_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(super) text: Option<ExportTextState>,
    pub(super) plan_work_demand:semio_framework_value::RetirementDemand,
    pub(super) text_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(super) pdf_primitive_cursor: usize,
    pub(super) pdf_text_phase: u8,
    pub(super) pdf_text_byte: usize,
    pub(super) pdf_text_count: usize,
    pub(super) pdf_text_marked: bool,
    pub(super) pdf_path_current: [f64; 2],
    pub(super) pdf_path_first: [f64; 2],
    pub(super) package_section: PackageSection,
    pub(super) package_index: usize,
    pub(super) package_byte_cursor: usize,
    pub package_json: Option<TypedJsonCursor>,
    pub(super) zip_central_offset: u32,
    pub zip: ZipState,
    pub media_output_credit: Option<ArtifactMediaExportCredit>,
    pub output_chunks: Option<ArtifactOutputChunks>,
    pub(super) output_retirement:Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(super) commit_cursor: usize,
    pub(super) work_units: u64,
    pub(super) committed_units: u64,
    pub(super) restore_target: Option<LayoutExportCheckpoint>,
    pub(super) snapshot_close: Option<ArtifactSnapshotCloseLease<LayoutSnapshot>>,
    pub(super) slot: LayoutPublicationSlot,
    pub(super) closing: bool,
    pub close_stage: LayoutExportCloseStage,
}

impl LayoutExportJob {
    pub fn from_admitted(operation:Operation,source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),request:LayoutExportRequest,output_chunks:ArtifactOutputChunks)->Self {
        Self {
            operation,
            source_identity,
            request:Some(request),
            stage: ExportStage::Validate,
            validation_group: 0,
            validation_index: 0,
            validation_nested: NestedValidationCursor::default(),
            validation_total_items: 0,
            json_validation: None,
            typed_validation: None,
            target_page: None,
            pdf_background_cursor: 0,
            encode_cursor: 0,
            output: ChunkRope::new(),
            encoded: ChunkRope::new(),
            base64_cursor: 0,
            media_projection_phase:0,
            png_adler_s1: 1,
            png_adler_s2: 0,
            png_idat_crc: 0,
            png_row: Vec::new(),
            png_row_section: PngRowSection::Initialize,
            png_row_byte_cursor: 0,
            png_shape_cursor: 0,
            png_fill_x: 0,
            pdf_offsets: PlanOffsets::new(),
            pdf_section: PdfSection::Header,
            pdf_cursor: 0,
            pdf_xref_offset: 0,
            pdf_items: PlanItems::new(),
            pdf_pages: PlanPages::new(),
            pdf_page_cursor: 0,
            pdf_page_started: false,
            pdf_plan_items: 0,
            pdf_item_cursor: 0,
            pdf_font_cursor: 0,
            pdf_content_start: 0,
            interchange_session: None,
            fidelity: None,
            fidelity_retirement: None,
            drawing: None,
            drawing_source:None,
            drawing_retirement: None,
            frame_plan: None,
            frame_plan_retirement: None,
            text: None,
            plan_work_demand:Default::default(),
            text_retirement: None,
            pdf_primitive_cursor: 0,
            pdf_text_phase: 0,
            pdf_text_byte: 0,
            pdf_text_count: 0,
            pdf_text_marked: false,
            pdf_path_current: [0.0; 2],
            pdf_path_first: [0.0; 2],
            package_section: PackageSection::Begin,
            package_index: 0,
            package_byte_cursor: 0,
            package_json: None,
            zip_central_offset: 0,
            zip: ZipState::default(),
            media_output_credit: None,
            output_chunks:Some(output_chunks),
            output_retirement:None,
            commit_cursor: 0,
            work_units: 0,
            committed_units: 0,
            restore_target: None,
            snapshot_close: None,
            slot: LayoutPublicationSlot::default(),
            closing: false,
            close_stage: LayoutExportCloseStage::JsonValidation,
        }
    }

    pub fn with_snapshot_close_lease(mut self, snapshot_close: ArtifactSnapshotCloseLease<LayoutSnapshot>) -> Self {
        self.snapshot_close = Some(snapshot_close);
        self
    }

    pub fn with_media_output_credit(mut self, credit: ArtifactMediaExportCredit) -> Self {
        self.media_output_credit = Some(credit);
        self
    }

    
    pub(super) fn next_manual_close_release_bytes(&self)->usize{match self.close_stage{
        LayoutExportCloseStage::JsonValidation=>self.json_validation.as_ref().map_or(0,JsonValidationCursor::retirement_bytes),
        LayoutExportCloseStage::TypedValidation=>self.typed_validation.as_ref().map_or(0,|cursor|cursor.stack.last().map_or_else(||cursor.retirement_stack_bytes(),typed_json_node_owned_bytes)),
        LayoutExportCloseStage::PackageJson=>self.package_json.as_ref().map_or(0,|cursor|cursor.stack.last().map_or_else(||cursor.retirement_stack_bytes(),typed_json_node_owned_bytes)),
        LayoutExportCloseStage::PngRow=>self.png_row.capacity(),
        LayoutExportCloseStage::PdfOffsets=>0,
        LayoutExportCloseStage::PdfItems=>0,
        LayoutExportCloseStage::PdfPages=>0,
        LayoutExportCloseStage::ZipEntries=>0,
        LayoutExportCloseStage::ZipCurrentMember=>0,
        LayoutExportCloseStage::PageAuthority=>self.request.as_ref().and_then(|request|request.page_id.as_ref()).map_or(0,String::capacity),
        LayoutExportCloseStage::Preflight=>self.request.as_ref().and_then(|request|request.preflight_json.as_ref()).map_or(0,String::capacity),
        LayoutExportCloseStage::ParentAuthority=>self.request.as_ref().map_or(0,|request|request.parent_document_id.capacity()),
        LayoutExportCloseStage::RevisionAuthority=>self.request.as_ref().map_or(0,|request|request.canonical_base_revision_hex.capacity()),_=>0}}

    fn close_string(value:&mut String,maximum_bytes:usize)->Option<usize>{let bytes=value.capacity();if bytes==0{return None;}if bytes>maximum_bytes{return Some(0);}drop(std::mem::take(value));Some(bytes)}
    fn close_copy_buffer<T:Copy>(value:&mut Vec<T>,maximum_bytes:usize)->Option<usize>{let bytes=value.capacity().saturating_mul(size_of::<T>());if bytes==0{return None;}if bytes>maximum_bytes{return Some(0);}drop(std::mem::take(value));Some(bytes)}
    fn close_byte_buffer(value:&mut Vec<u8>,maximum_bytes:usize)->Option<usize>{Self::close_copy_buffer(value,maximum_bytes)}
    fn close_usize_buffer(value:&mut Vec<usize>,maximum_bytes:usize)->usize{Self::close_copy_buffer(value,maximum_bytes).unwrap_or(0)}
    fn close_string_source(source: &mut StringSource, maximum_bytes: usize) -> usize {
        match source {
            StringSource::DrawNodeValue { path, .. } | StringSource::DrawNodeStyle { path, .. } | StringSource::DrawNodeMime { path, .. } => Self::close_usize_buffer(path, maximum_bytes),
            _ => 0,
        }
    }

    fn close_typed_node_payload(node: &mut TypedJsonNode, maximum_bytes: usize) -> usize {
        match node {
            TypedJsonNode::Scalar { bytes, .. } => Self::close_byte_buffer(bytes, maximum_bytes).unwrap_or(0),
            TypedJsonNode::String { source, .. } => Self::close_string_source(source, maximum_bytes),
            TypedJsonNode::OwnedString { value, .. } => Self::close_string(value, maximum_bytes).unwrap_or(0),
            TypedJsonNode::DrawNode { path, .. } | TypedJsonNode::DrawChildren { path, .. } | TypedJsonNode::DrawSegments { path, .. } | TypedJsonNode::DrawSegment { path, .. } | TypedJsonNode::DrawBytes { path, .. } => {
                Self::close_usize_buffer(path, maximum_bytes)
            }
            _ => 0,
        }
    }

    fn close_json_cursor(cursor:&mut Option<JsonValidationCursor>,next:LayoutExportCloseStage,stage:&mut LayoutExportCloseStage,maximum_bytes:usize)->LayoutCloseTurn{if let Some(cursor)=cursor.as_mut(){if let Some(released)=cursor.close_one(maximum_bytes){return LayoutCloseTurn::Pending{released_items:usize::from(released!=0),released_bytes:released};}}cursor.take();*stage=next;LayoutCloseTurn::Pending{released_items:1,released_bytes:0}}
    fn close_typed_cursor(cursor: &mut Option<TypedJsonCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> LayoutCloseTurn {
        if let Some(cursor) = cursor.as_mut() {
            if let Some(node) = cursor.stack.last() {
                let bytes = typed_json_node_owned_bytes(node);
                if bytes > maximum_bytes {
                    let released = Self::close_typed_node_payload(cursor.stack.last_mut().expect("typed close node remains owned"), maximum_bytes);
                    return LayoutCloseTurn::Pending { released_items: usize::from(released != 0), released_bytes: released };
                }
                drop(cursor.stack.pop());
                return LayoutCloseTurn::Pending { released_items: 1, released_bytes: bytes };
            }
            debug_assert!(cursor.stack.is_empty());
        }
        let bytes=cursor.as_ref().map_or(0,|cursor|cursor.retirement_stack_bytes());if bytes>maximum_bytes{return LayoutCloseTurn::Pending{released_items:0,released_bytes:0};}
        drop(cursor.take());
        *stage = next;
        LayoutCloseTurn::Pending { released_items: 1, released_bytes: bytes }
    }

    fn close_optional_string(value: &mut Option<String>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> LayoutCloseTurn {
        if let Some(value) = value.as_mut() {
            if let Some(released) = Self::close_string(value, maximum_bytes) {
                return LayoutCloseTurn::Pending { released_items: usize::from(released != 0), released_bytes: released };
            }
            debug_assert!(value.is_empty());
        }
        drop(value.take());
        *stage = next;
        LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 }
    }

    fn close_required_string(value: &mut String, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> LayoutCloseTurn {
        if let Some(released) = Self::close_string(value, maximum_bytes) {
            return LayoutCloseTurn::Pending { released_items: usize::from(released != 0), released_bytes: released };
        }
        debug_assert!(value.is_empty());
        drop(std::mem::take(value));
        *stage = next;
        LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 }
    }

    /// 📮️ The ONE publication close slice BOTH close ladders run first. `begin_close` parks the job on
    /// `LayoutExportCloseStage::Publication`, which `close_export_step` refuses outright — so whichever
    /// ladder runs has to drain the retained publication payload and advance the stage itself. Only the
    /// `InteractiveJob` ladder used to do that; `ArtifactReservedJob::close_step` (the reserved media
    /// export route, `LayoutMediaExportJobFactory`) went straight to `close_export_step` and got the
    /// refusal on EVERY call, which the caller sees as `Blocked` with nothing released. A close loop that
    /// keeps stepping until `terminal_is_empty()` therefore spun forever, minting one `Fault` per turn —
    /// three of this crate's export laws hung exactly there until the 30-minute test-binary watchdog
    /// SIGKILLed the whole binary and took every other layout result with it.
    pub(super) fn close_publication_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> LayoutCloseTurn {
        if !self.slot.terminal_is_empty() {
            return match self.slot.close_step(grant) {
                Ok(step) => LayoutCloseTurn::Granted(step.progress()),
                Err(_) => LayoutCloseTurn::Blocked { reason: "layout export publication awaits its terminal-empty witness" },
            };
        }
        self.slot.publication = None;
        self.slot.delivered = false;
        self.close_stage = LayoutExportCloseStage::JsonValidation;
        LayoutCloseTurn::Pending { released_items: usize::from(grant.maximum_items > 0), released_bytes: 0 }
    }

    pub(super) fn close_export_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<LayoutCloseTurn, Fault> {
        if maximum_items == 0 || maximum_bytes<self.next_manual_close_release_bytes() {
            return Ok(LayoutCloseTurn::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.close_stage {
            LayoutExportCloseStage::Publication => Err(Fault::from("layout-export-publication-close-not-dispatched")),
            LayoutExportCloseStage::JsonValidation => Ok(Self::close_json_cursor(&mut self.json_validation, LayoutExportCloseStage::TypedValidation, &mut self.close_stage, maximum_bytes)),
            LayoutExportCloseStage::TypedValidation => Ok(Self::close_typed_cursor(&mut self.typed_validation, LayoutExportCloseStage::PackageJson, &mut self.close_stage, maximum_bytes)),
            LayoutExportCloseStage::PackageJson => Ok(Self::close_typed_cursor(&mut self.package_json, LayoutExportCloseStage::Output, &mut self.close_stage, maximum_bytes)),
            LayoutExportCloseStage::Output|LayoutExportCloseStage::Encoded=>Err(Fault::from("layout-output-original-close-not-dispatched")),
            LayoutExportCloseStage::PngRow => {
                if let Some(released) = Self::close_byte_buffer(&mut self.png_row, maximum_bytes) {
                    return Ok(LayoutCloseTurn::Pending { released_items: usize::from(released != 0), released_bytes: released });
                }
                debug_assert!(self.png_row.is_empty());
                drop(std::mem::take(&mut self.png_row));
                self.close_stage = LayoutExportCloseStage::PdfOffsets;
                Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 })
            }
            LayoutExportCloseStage::PdfOffsets=>Err(Fault::from("layout plan requires original paired close")),
            LayoutExportCloseStage::PdfItems=>Err(Fault::from("layout plan requires original paired close")),
            LayoutExportCloseStage::PdfPages=>Err(Fault::from("layout plan requires original paired close")),
            LayoutExportCloseStage::Interchange => {
                if let Some(session) = self.interchange_session.as_mut() {
                    if !session.terminal_is_empty() {
                        let bytes = session.next_release_bytes();
                        let released = session.close_one(maximum_bytes);
                        return Ok(LayoutCloseTurn::Pending { released_items: usize::from(released), released_bytes: if released { bytes } else { 0 } });
                    }
                }
                self.interchange_session = None;
                self.close_stage = LayoutExportCloseStage::Fidelity;
                Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 })
            }
            LayoutExportCloseStage::FramePlanning => Err(Fault::from("layout frame plan requires its original quoted close")),
            LayoutExportCloseStage::Drawing => Err(Fault::from("layout drawing requires its original quoted close")),
            LayoutExportCloseStage::Fidelity => Err(Fault::from("layout fidelity requires its original quoted close")),
            LayoutExportCloseStage::Paragraphs => Err(Fault::from("layout paragraphs require their original quoted close")),
            LayoutExportCloseStage::ZipEntries=>{if self.zip.pop().is_none(){self.close_stage=LayoutExportCloseStage::ZipCurrentMember}Ok(LayoutCloseTurn::Pending{released_items:1,released_bytes:0})},
            LayoutExportCloseStage::ZipCurrentMember=>{self.zip.current=None;self.close_stage=LayoutExportCloseStage::PageAuthority;Ok(LayoutCloseTurn::Pending{released_items:1,released_bytes:0})},
            LayoutExportCloseStage::PageAuthority => Ok(Self::close_optional_string(&mut self.request.as_mut().unwrap().page_id, LayoutExportCloseStage::Preflight, &mut self.close_stage, maximum_bytes)),
            LayoutExportCloseStage::Preflight => Ok(Self::close_optional_string(&mut self.request.as_mut().unwrap().preflight_json, LayoutExportCloseStage::ParentAuthority, &mut self.close_stage, maximum_bytes)),
            LayoutExportCloseStage::ParentAuthority => Ok(Self::close_required_string(&mut self.request.as_mut().unwrap().parent_document_id, LayoutExportCloseStage::RevisionAuthority, &mut self.close_stage, maximum_bytes)),
            LayoutExportCloseStage::RevisionAuthority => Ok(Self::close_required_string(&mut self.request.as_mut().unwrap().canonical_base_revision_hex, LayoutExportCloseStage::OutputChunks, &mut self.close_stage, maximum_bytes)),
            LayoutExportCloseStage::OutputChunks|LayoutExportCloseStage::MediaCredit|LayoutExportCloseStage::Snapshot|LayoutExportCloseStage::SnapshotOwner=>Err(Fault::from("layout paired roots require their original quoted close")),
            LayoutExportCloseStage::Complete => Ok(LayoutCloseTurn::Complete),
        }
    }

    pub(super) fn close_terminal_is_empty(&self) -> bool {
        self.close_stage == LayoutExportCloseStage::Complete
            && self.json_validation.is_none()
            && self.typed_validation.is_none()
            && self.package_json.is_none()
            && self.output.terminal_is_empty()
            && self.encoded.terminal_is_empty()
            && self.png_row.capacity()==0
            && self.pdf_offsets.terminal_is_empty()
            && self.pdf_items.terminal_is_empty()
            && self.pdf_pages.terminal_is_empty()
            && self.interchange_session.is_none()
            && self.text.is_none()
            && self.frame_plan.is_none() && self.frame_plan_retirement.is_none()
            && self.drawing.is_none() && self.drawing_retirement.is_none() && self.drawing_source.is_none()
            && self.text_retirement.is_none()
            && self.fidelity.is_none()
            && self.fidelity_retirement.is_none()
            && self.zip.terminal_is_empty()
            && self.request.is_none()
            && self.output_chunks.is_none() && self.output_retirement.is_none()
            && self.media_output_credit.is_none()
            && self.snapshot_close.is_none()

    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn restore(operation: Operation, request: LayoutExportRequest, state: &[u8]) -> Result<Self, String> {
        let checkpoint = decode_checkpoint(&operation, &request, state)?;
        let mut job = Self::from_admitted(operation,(operation.operation,operation.generation),request,ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES));
        job.restore_target = Some(checkpoint);
        Ok(job)
    }

    pub(super) fn projected_media(&self)->bool{self.media_output_credit.is_some()&&self.request.as_ref().is_some_and(|request|request.kind==LayoutExportKind::Svg)}

    pub(super) fn active_output(&self) -> &ChunkRope {
        if (self.request.as_ref().unwrap().kind.binary()||self.projected_media()) && matches!(self.stage, ExportStage::Base64 | ExportStage::PackageCommit | ExportStage::Complete) {
            &self.encoded
        } else {
            &self.output
        }
    }

    pub(super) fn checkpoint_publication(&self) -> Result<LayoutExportPublication, String> {
        let checkpoint = LayoutExportCheckpoint { work_units: self.work_units, committed_units:self.committed_units, output_bytes: self.active_output().len as u64, output_digest: self.active_output().digest };
        let state = encode_checkpoint(&self.operation, self.request.as_ref().unwrap(), &checkpoint)?;
        LayoutExportPublication::new(LayoutExportPublicationKind::Checkpoint { applied_progress: self.work_units }, &state)
    }

    pub(super) fn preview_publication(&self) -> Result<LayoutExportPublication, String> {
        LayoutExportPublication::new(LayoutExportPublicationKind::Preview, self.stage_name().as_bytes())
    }

    pub fn fault_publication(error: &str) -> LayoutExportPublication {
        let bytes = &error.as_bytes()[..error.len().min(MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES)];
        LayoutExportPublication::new(LayoutExportPublicationKind::Fault, bytes).expect("bounded layout fault publication")
    }

    pub(super) fn drive_publication<'a>(&'a mut self, context: &mut StepContext<'_>) -> LayoutExportTurn<'a> {
        self.slot.advance(context)
    }

    pub(super) fn page(&self) -> Result<&Page, String> {
        self.target_page.and_then(|index| self.request.as_ref().unwrap().snapshot.pages.get(index)).ok_or_else(|| "layout-export-page-missing".into())
    }

}
//#endregion 🧩️Job
