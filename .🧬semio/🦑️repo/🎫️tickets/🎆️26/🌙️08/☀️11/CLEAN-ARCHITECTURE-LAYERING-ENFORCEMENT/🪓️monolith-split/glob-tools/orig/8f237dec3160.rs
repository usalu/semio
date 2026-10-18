//! 🏗️ Job admission and close of the layout export job: the job record, its admission from the reserved request, the bounded close of its buffers, the close and publication steps and the publication helpers. See [`super`].

use super::*;

//#region 🧩️Job
pub struct LayoutExportJob {
    pub(super) operation: Operation,
    pub(super) source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
    pub(super) request: Option<LayoutExportRequest>,
    pub(super) stage: ExportStage,
    pub(super) validation_group: u8,
    pub(super) validation_index: usize,
    pub(super) validation_total_frames: usize,
    pub(super) validation_total_items: usize,
    pub(super) json_validation: Option<JsonValidationCursor>,
    pub(super) typed_validation: Option<TypedJsonCursor>,
    pub(super) target_page: Option<usize>,
    pub(super) plan_cursor: usize,
    pub(super) parent_frame_count: usize,
    pub(super) rects: Vec<ExportRect>,
    pub(super) encode_cursor: usize,
    pub(super) output: ChunkRope,
    pub(super) encoded: ChunkRope,
    pub(super) base64_cursor: usize,
    pub(super) base64_tail: Vec<u8>,
    pub(super) png_adler_s1: u32,
    pub(super) png_adler_s2: u32,
    pub(super) png_idat_crc: u32,
    pub(super) png_row: Vec<u8>,
    pub(super) png_row_section: PngRowSection,
    pub(super) png_row_byte_cursor: usize,
    pub(super) png_rect_cursor: usize,
    pub(super) png_fill_x: u32,
    pub(super) pdf_offsets: Vec<u32>,
    pub(super) pdf_section: PdfSection,
    pub(super) pdf_cursor: usize,
    pub(super) pdf_xref_offset: usize,
    pub(super) pdf_items: Vec<PdfItem>,
    pub(super) pdf_pages: Vec<PdfPagePlan>,
    pub(super) pdf_page_cursor: usize,
    pub(super) pdf_item_cursor: usize,
    pub(super) pdf_font_cursor: usize,
    pub(super) pdf_content_start: usize,
    pub(super) pdf_engine: Option<semio_s_artifact_layout_layout_engine::scene::LayoutEngine>,
    pub(super) package_section: PackageSection,
    pub(super) package_index: usize,
    pub(super) package_byte_cursor: usize,
    pub(super) package_json: Option<TypedJsonCursor>,
    pub(super) zip_central_offset: u32,
    pub(super) zip: ZipState,
    pub media_output_credit: Option<ArtifactMediaExportCredit>,
    pub output_chunks: Option<ArtifactOutputChunks>,
    pub(super) output_retirement:Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(super) commit_cursor: usize,
    pub(super) completed_units: u64,
    pub(super) restore_target: Option<LayoutExportCheckpoint>,
    pub(super) snapshot_close: Option<ArtifactSnapshotCloseLease<LayoutSnapshot>>,
    pub(super) slot: LayoutPublicationSlot,
    pub(super) closing: bool,
    pub(super) close_stage: LayoutExportCloseStage,
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
            validation_total_frames: 0,
            validation_total_items: 0,
            json_validation: None,
            typed_validation: None,
            target_page: None,
            plan_cursor: 0,
            parent_frame_count: 0,
            rects: Vec::new(),
            encode_cursor: 0,
            output: ChunkRope::new(),
            encoded: ChunkRope::new(),
            base64_cursor: 0,
            base64_tail: Vec::new(),
            png_adler_s1: 1,
            png_adler_s2: 0,
            png_idat_crc: 0,
            png_row: Vec::new(),
            png_row_section: PngRowSection::Initialize,
            png_row_byte_cursor: 0,
            png_rect_cursor: 0,
            png_fill_x: 0,
            pdf_offsets: Vec::new(),
            pdf_section: PdfSection::Header,
            pdf_cursor: 0,
            pdf_xref_offset: 0,
            pdf_items: Vec::new(),
            pdf_pages: Vec::new(),
            pdf_page_cursor: 0,
            pdf_item_cursor: 0,
            pdf_font_cursor: 0,
            pdf_content_start: 0,
            pdf_engine: None,
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
            completed_units: 0,
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

    fn close_string(value: &mut String, maximum_bytes: usize) -> Option<usize> {
        if value.is_empty() {
            return None;
        }
        let mut start = value.len().saturating_sub(maximum_bytes.min(value.len()));
        while start < value.len() && !value.is_char_boundary(start) {
            start += 1;
        }
        if start == value.len() {
            return Some(0);
        }
        let released = value.len() - start;
        value.truncate(start);
        Some(released)
    }

    fn close_byte_buffer(value: &mut Vec<u8>, maximum_bytes: usize) -> Option<usize> {
        if value.is_empty() {
            return None;
        }
        let released = maximum_bytes.min(value.len());
        value.truncate(value.len() - released);
        Some(released)
    }

    fn close_usize_buffer(value: &mut Vec<usize>, maximum_bytes: usize) -> usize {
        let released_items = (maximum_bytes / size_of::<usize>()).min(value.len());
        value.truncate(value.len() - released_items);
        released_items.saturating_mul(size_of::<usize>())
    }

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

    fn close_json_cursor(cursor: &mut Option<JsonValidationCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage) -> LayoutCloseTurn {
        if let Some(cursor) = cursor.as_mut() {
            if cursor.stack.pop().is_some() {
                return LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 };
            }
            debug_assert!(cursor.stack.is_empty());
        }
        drop(cursor.take());
        *stage = next;
        LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 }
    }

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
        drop(cursor.take());
        *stage = next;
        LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 }
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
        if maximum_items == 0 {
            return Ok(LayoutCloseTurn::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.close_stage {
            LayoutExportCloseStage::Publication => Err(Fault::from("layout-export-publication-close-not-dispatched")),
            LayoutExportCloseStage::JsonValidation => Ok(Self::close_json_cursor(&mut self.json_validation, LayoutExportCloseStage::TypedValidation, &mut self.close_stage)),
            LayoutExportCloseStage::TypedValidation => Ok(Self::close_typed_cursor(&mut self.typed_validation, LayoutExportCloseStage::PackageJson, &mut self.close_stage, maximum_bytes)),
            LayoutExportCloseStage::PackageJson => Ok(Self::close_typed_cursor(&mut self.package_json, LayoutExportCloseStage::Rects, &mut self.close_stage, maximum_bytes)),
            LayoutExportCloseStage::Rects => {
                if self.rects.pop().is_some() {
                    return Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 });
                }
                debug_assert!(self.rects.is_empty());
                drop(std::mem::take(&mut self.rects));
                self.close_stage = LayoutExportCloseStage::Output;
                Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 })
            }
            LayoutExportCloseStage::Output => match self.output.close_take_chunk(maximum_bytes) {
                Some(Ok(bytes)) => Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: bytes }),
                Some(Err(())) => Ok(LayoutCloseTurn::Pending { released_items: 0, released_bytes: 0 }),
                None => {
                    debug_assert!(self.output.chunks.is_empty());
                    drop(std::mem::take(&mut self.output.chunks));
                    self.close_stage = LayoutExportCloseStage::Encoded;
                    Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 })
                }
            },
            LayoutExportCloseStage::Encoded => match self.encoded.close_take_chunk(maximum_bytes) {
                Some(Ok(bytes)) => Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: bytes }),
                Some(Err(())) => Ok(LayoutCloseTurn::Pending { released_items: 0, released_bytes: 0 }),
                None => {
                    debug_assert!(self.encoded.chunks.is_empty());
                    drop(std::mem::take(&mut self.encoded.chunks));
                    self.close_stage = LayoutExportCloseStage::Base64Tail;
                    Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 })
                }
            },
            LayoutExportCloseStage::Base64Tail => {
                if let Some(released) = Self::close_byte_buffer(&mut self.base64_tail, maximum_bytes) {
                    return Ok(LayoutCloseTurn::Pending { released_items: usize::from(released != 0), released_bytes: released });
                }
                debug_assert!(self.base64_tail.is_empty());
                drop(std::mem::take(&mut self.base64_tail));
                self.close_stage = LayoutExportCloseStage::PngRow;
                Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 })
            }
            LayoutExportCloseStage::PngRow => {
                if let Some(released) = Self::close_byte_buffer(&mut self.png_row, maximum_bytes) {
                    return Ok(LayoutCloseTurn::Pending { released_items: usize::from(released != 0), released_bytes: released });
                }
                debug_assert!(self.png_row.is_empty());
                drop(std::mem::take(&mut self.png_row));
                self.close_stage = LayoutExportCloseStage::PdfOffsets;
                Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 })
            }
            LayoutExportCloseStage::PdfOffsets => {
                if self.pdf_offsets.pop().is_some() {
                    return Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 });
                }
                debug_assert!(self.pdf_offsets.is_empty());
                drop(std::mem::take(&mut self.pdf_offsets));
                self.close_stage = LayoutExportCloseStage::PdfItems;
                Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 })
            }
            LayoutExportCloseStage::PdfItems => {
                if self.pdf_items.pop().is_some() {
                    return Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 });
                }
                debug_assert!(self.pdf_items.is_empty());
                drop(std::mem::take(&mut self.pdf_items));
                self.close_stage = LayoutExportCloseStage::PdfPages;
                Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 })
            }
            LayoutExportCloseStage::PdfPages => {
                if self.pdf_pages.pop().is_some() {
                    return Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 });
                }
                debug_assert!(self.pdf_pages.is_empty());
                drop(std::mem::take(&mut self.pdf_pages));
                self.close_stage = LayoutExportCloseStage::PdfEngine;
                Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 })
            }
            LayoutExportCloseStage::PdfEngine => {
                let released = usize::from(self.pdf_engine.take().is_some());
                self.close_stage = LayoutExportCloseStage::ZipEntries;
                Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: released * size_of::<semio_s_artifact_layout_layout_engine::scene::LayoutEngine>() })
            }
            LayoutExportCloseStage::ZipEntries => {
                let bytes = self.zip.entries.last().map_or(0, |entry| entry.name.len());
                if bytes > maximum_bytes {
                    let released = Self::close_string(&mut self.zip.entries.last_mut().expect("zip close entry remains owned").name, maximum_bytes).unwrap_or(0);
                    return Ok(LayoutCloseTurn::Pending { released_items: usize::from(released != 0), released_bytes: released });
                }
                if self.zip.entries.pop().is_some() {
                    return Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: bytes });
                }
                debug_assert!(self.zip.entries.is_empty());
                drop(std::mem::take(&mut self.zip.entries));
                self.close_stage = LayoutExportCloseStage::ZipCurrentName;
                Ok(LayoutCloseTurn::Pending { released_items: 1, released_bytes: 0 })
            }
            LayoutExportCloseStage::ZipCurrentName => Ok(Self::close_optional_string(&mut self.zip.current_name, LayoutExportCloseStage::PageAuthority, &mut self.close_stage, maximum_bytes)),
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
            && self.rects.is_empty()
            && self.output.chunks.is_empty()
            && self.encoded.chunks.is_empty()
            && self.base64_tail.is_empty()
            && self.png_row.is_empty()
            && self.pdf_offsets.is_empty()
            && self.pdf_items.is_empty()
            && self.pdf_pages.is_empty()
            && self.pdf_engine.is_none()
            && self.zip.entries.is_empty()
            && self.zip.current_name.is_none()
            && self.request.is_none()
            && self.output_chunks.is_none() && self.output_retirement.is_none()
            && self.media_output_credit.is_none()
            && self.snapshot_close.is_none()

    }

    #[cfg(test)]
    pub fn restore(operation: Operation, request: LayoutExportRequest, state: &[u8]) -> Result<Self, String> {
        let checkpoint = decode_checkpoint(&operation, &request, state)?;
        let mut job = Self::from_admitted(operation,(operation.operation,operation.generation),request,ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES));
        job.restore_target = Some(checkpoint);
        Ok(job)
    }

    pub(super) fn active_output(&self) -> &ChunkRope {
        if self.request.as_ref().unwrap().kind.binary() && matches!(self.stage, ExportStage::Base64 | ExportStage::PackageCommit | ExportStage::Complete) {
            &self.encoded
        } else {
            &self.output
        }
    }

    pub(super) fn checkpoint_publication(&self) -> Result<LayoutExportPublication, String> {
        let checkpoint = LayoutExportCheckpoint { completed_units: self.completed_units, output_bytes: self.active_output().len as u64, output_digest: self.active_output().digest };
        let state = encode_checkpoint(&self.operation, self.request.as_ref().unwrap(), &checkpoint)?;
        LayoutExportPublication::new(LayoutExportPublicationKind::Checkpoint { applied_progress: self.completed_units }, &state)
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
