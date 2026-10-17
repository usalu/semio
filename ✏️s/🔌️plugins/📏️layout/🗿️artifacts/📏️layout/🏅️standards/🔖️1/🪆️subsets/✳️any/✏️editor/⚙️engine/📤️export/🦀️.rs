//! 📤️ Deterministic, resumable Layout export jobs.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;


use semio_s_artifact_layout_layout_engine_json::JsonScalar;
use semio_s_artifact_layout_layout_engine_json::JsonValidationCursor;
use semio_s_artifact_layout_layout_engine_json::MAX_LAYOUT_EXPORT_DECODED_ITEMS;
use semio_s_artifact_layout_layout_engine_json::MAX_LAYOUT_EXPORT_STRING_BYTES;
use semio_s_artifact_layout_layout_engine_json::StringSource;
use semio_s_artifact_layout_layout_engine_json::TypedJsonCursor;
use semio_s_artifact_layout_layout_engine_json::TypedJsonNode;
use semio_s_artifact_layout_layout_engine_json::finite_json_number;
use semio_s_artifact_layout_layout_engine_json::frame_bounds;
use semio_s_artifact_layout_layout_engine_json::integer_json_scalar;
use semio_s_artifact_layout_layout_engine_json::typed_json_node_owned_bytes;
use semio_s_artifact_layout_layout_model::{Frame, GridSettings, LayoutBounds, Page, PageOverride};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_action_bus::RetainedToolWireInput;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
#[cfg(test)]
use semio_framework_job::{Generation, InteractiveStage, RevisionId};
#[cfg(test)]
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, Operation, StepContext};
use semio_framework_plugin::app::{ArtifactDownloadOutput, ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactReservedToolJob, ArtifactSnapshotCloseLease, ArtifactToolCompletion};
use semio_framework_plugin::{ArtifactReservedJob, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, EphemeralEmit, Fault, MediaClass, MediaForm, MediaType};
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;
use semio_s_artifact_layout_layout_engine_json::dictionary::DictionaryJsonCursor;

//#region 🔖️Contract
pub const LAYOUT_EXPORT_TOOL_IDS: &[&str] = &["exportPng", "exportSvg", "exportPdf", "exportPackage"];
pub const LAYOUT_EXPORT_PAYLOAD_SCHEMA: &str = "layout.layout.tool-command.v1";
pub const LAYOUT_MEDIA_EXPORT_PAYLOAD_SCHEMA: &str = "layout.layout.media-export.v1";
pub const LAYOUT_MEDIA_EXPORT_TOOL_ID: &str = "export-media:layout:out";
pub const LAYOUT_MEDIA_EXPORT_SCHEMA: &str = "2d.layout";
pub const MAX_LAYOUT_EXPORT_RAW_BYTES: usize = 2 << 20;
pub const MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES: usize = 4_096;
pub const MAX_LAYOUT_EXPORT_PAGES: usize = 64;
pub const MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE: usize = 8;
pub const MAX_LAYOUT_EXPORT_TOTAL_FRAMES: usize = 1_024;
pub const MAX_LAYOUT_EXPORT_STORIES: usize = 512;
pub const MAX_LAYOUT_EXPORT_LINKS: usize = 512;
pub const MAX_LAYOUT_EXPORT_PARENT_PAGES: usize = 64;
pub const MAX_LAYOUT_EXPORT_SPREADS: usize = 64;
pub const MAX_LAYOUT_EXPORT_STYLES: usize = 256;
pub const MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE: usize = 16;
pub const MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE: usize = 64;
pub const MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS: usize = 64;
pub const MAX_LAYOUT_EXPORT_DIMENSION: u32 = 2_048;
pub const MAX_LAYOUT_EXPORT_PIXELS: u64 = 4_194_304;
pub const MAX_LAYOUT_EXPORT_FILES: usize = 3;
/// 📤️ Derived, never a literal: the framework's own end-to-end segmented-download total cap, so this
/// plugin's export budget cannot drift from the bound the shard worker and the host drain enforce
/// (`🎭️actor/📮️shard-client/📤️segmented-download/🧫️fixtures/🔣️.json`'s `contract.maximumTotalBytes`).
pub const MAX_LAYOUT_EXPORT_OUTPUT_BYTES: usize = ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES;
pub const MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES: usize = 634;
/// 📤️ Derived, never a literal: the exact per-chunk cap [`ArtifactOutputChunks::push`] enforces. A
/// literal that drifts from it turns a correct producer into a runtime fault on the wire.
const OUTPUT_CHUNK_BYTES: usize = ArtifactOutputChunks::CHUNK_BYTES;
const MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS: usize = MAX_LAYOUT_EXPORT_OUTPUT_BYTES.div_ceil(OUTPUT_CHUNK_BYTES);
const BASE64_INPUT_BYTES_PER_UNIT: usize = 3_072;
const PNG_PIXELS_PER_UNIT: u32 = 256;
const MAX_LAYOUT_EXPORT_AUTHORITY_BYTES: usize = 256;
const LAYOUT_EXPORT_CHECKPOINT_MAGIC: &[u8; 4] = b"LXC2";

fn empty_close_snapshot() -> LayoutSnapshot {
    LayoutSnapshot {
        schema: String::new(),
        name: String::new(),
        grid: GridSettings { baseline_grid: 0.0, baseline_offset: 0.0, snap_to_baseline: false },
        paragraph_styles: Vec::new(),
        character_styles: Vec::new(),
        stories: Vec::new(),
        links: Vec::new(),
        parent_pages: Vec::new(),
        spreads: Vec::new(),
        pages: Vec::new(),
        print_target: None,
        data_fields: None,
        background_drawing: None,
        referenced_model: None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub enum LayoutExportKind {
    Png,
    Svg,
    Pdf,
    Package,
}

impl LayoutExportKind {
    pub fn tool_id(self) -> &'static str {
        match self {
            Self::Png => "exportPng",
            Self::Svg => "exportSvg",
            Self::Pdf => "exportPdf",
            Self::Package => "exportPackage",
        }
    }

    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Svg => "image/svg+xml",
            Self::Pdf => "application/pdf",
            Self::Package => "application/zip",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Svg => "svg",
            Self::Pdf => "pdf",
            Self::Package => "layout-package.zip",
        }
    }

    fn binary(self) -> bool {
        !matches!(self, Self::Svg)
    }
}

#[derive(Clone, Debug)]
pub struct LayoutExportRequest {
    pub kind: LayoutExportKind,
    pub page_id: Option<String>,
    pub snapshot: Arc<LayoutSnapshot>,
    pub preflight_json: Option<String>,
    pub parent_document_id: String,
    pub canonical_base_revision_hex: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutExportCheckpoint {
    pub completed_units: u64,
    pub output_bytes: u64,
    pub output_digest: u64,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutExportCommit {
    pub filename: String,
    pub mime_type: String,
    pub encoding: Option<String>,
    pub data: String,
}

#[cfg(test)]
impl LayoutExportCommit {
    pub fn from_output(kind: LayoutExportKind, name: &str, bytes: Vec<u8>) -> Self {
        // SAFETY: every final route emits only ASCII: SVG's owned numeric vocabulary or the owned
        // base64 encoder. The invariant is enforced before bytes enter the final commit cursor.
        let data = unsafe { String::from_utf8_unchecked(bytes) };
        Self { filename: format!("{}.{}", sanitize_filename(name), kind.extension()), mime_type: kind.mime_type().into(), encoding: kind.binary().then(|| "base64".into()), data }
    }

    fn from_chunks(kind: LayoutExportKind, name: &str, chunks: &ArtifactOutputChunks) -> Result<Self, String> {
        let mut bytes = Vec::with_capacity(chunks.bytes());
        while let Some(chunk) = chunks.take_chunk().map_err(|error| error.message)? {
            bytes.extend_from_slice(&chunk);
        }
        Ok(Self::from_output(kind, name, bytes))
    }
}
//#endregion 🔖️Contract

//#region 🧱️Storage
#[derive(Debug)]
struct ChunkRope {
    chunks: std::collections::VecDeque<Vec<u8>>,
    len: usize,
    digest: u64,
    front_byte_cursor: usize,
}

impl ChunkRope {
    fn close_take_chunk(&mut self, maximum_bytes: usize) -> Option<Result<usize, ()>> {
        let bytes = self.chunks.front().map_or(0, Vec::len);
        if bytes == 0 {
            return None;
        }
        if bytes > maximum_bytes {
            return Some(Err(()));
        }
        drop(self.chunks.pop_front());
        Some(Ok(bytes))
    }
}

impl ChunkRope {
    fn new() -> Self {
        Self { chunks: std::collections::VecDeque::with_capacity(MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS), len: 0, digest: 0xcbf29ce484222325, front_byte_cursor: 0 }
    }

    fn append(&mut self, mut bytes: &[u8]) -> Result<(), String> {
        if self.len.checked_add(bytes.len()).is_none_or(|len| len > MAX_LAYOUT_EXPORT_OUTPUT_BYTES) {
            return Err("layout-export-output-limit".into());
        }
        while !bytes.is_empty() {
            if self.chunks.back().is_none_or(|chunk| chunk.len() == OUTPUT_CHUNK_BYTES) {
                if self.chunks.len() == MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS {
                    return Err("layout-export-output-limit".into());
                }
                self.chunks.push_back(Vec::with_capacity(OUTPUT_CHUNK_BYTES));
            }
            let chunk = self.chunks.back_mut().expect("chunk exists");
            let take = bytes.len().min(OUTPUT_CHUNK_BYTES - chunk.len());
            for byte in &bytes[..take] {
                self.digest = (self.digest ^ u64::from(*byte)).wrapping_mul(0x100000001b3);
            }
            chunk.extend_from_slice(&bytes[..take]);
            self.len += take;
            bytes = &bytes[take..];
        }
        Ok(())
    }

    fn take_chunk(&mut self) -> Option<Vec<u8>> {
        if self.front_byte_cursor != 0 {
            return None;
        }
        self.chunks.pop_front()
    }

    fn take_prefix(&mut self, maximum: usize) -> Vec<u8> {
        let mut output = Vec::with_capacity(maximum);
        while output.len() < maximum {
            let Some(chunk) = self.chunks.front() else { break };
            let take = (maximum - output.len()).min(chunk.len() - self.front_byte_cursor);
            output.extend_from_slice(&chunk[self.front_byte_cursor..self.front_byte_cursor + take]);
            self.front_byte_cursor += take;
            if self.front_byte_cursor == chunk.len() {
                self.chunks.pop_front();
                self.front_byte_cursor = 0;
            }
        }
        output
    }
}

#[derive(Clone, Debug)]
struct ExportRect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    rgba: [u8; 4],
}

#[derive(Clone, Debug)]
struct ZipEntry {
    name: String,
    crc: u32,
    size: u32,
    offset: u32,
}

#[derive(Clone, Debug, Default)]
struct ZipState {
    entries: Vec<ZipEntry>,
    current_name: Option<String>,
    current_crc: u32,
    current_size: u32,
    current_offset: u32,
}


integer_json_scalar!(u8, u32, u64, usize);








//#endregion 🧱️Storage

//#region 🧩️Job
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExportStage {
    Validate,
    Plan,
    Encode,
    Base64,
    PackageCommit,
    Complete,
}

/// 📕️ The PDF route streams one object per unit in id order: `1` catalog, `2` pages, `3`–`6` the
/// embedded `MapLabelSans` CIDFontType2 (Type0 → CIDFont → descriptor → `FontFile2`), then per page
/// `p` the page object `7+3p`, its content stream `8+3p` (length as the indirect `9+3p`, so no
/// content is ever buffered whole) — see `encode_pdf_one` (ticket 26/09/18/LAYOUT-PDF-EXPORT-END-TO-END).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PdfSection {
    Header,
    Catalog,
    Pages,
    PageKids,
    PagesEnd,
    FontType0,
    FontCid,
    FontDescriptor,
    FontFileBegin,
    FontFileBytes,
    FontFileEnd,
    PageObject,
    ContentBegin,
    ContentItem,
    ContentEnd,
    ContentLength,
    XrefBegin,
    XrefEntries,
    Trailer,
    Complete,
}

#[derive(Clone, Debug)]
struct PdfRaster {
    width: u32,
    height: u32,
    rgb: Vec<u8>,
}

/// 📕️ One painted element of a page's content stream, in page (top-left, y-down) coordinates.
#[derive(Clone, Debug)]
enum PdfItem {
    Stroke { points: Vec<(f32, f32)>, closed: bool, color: [f32; 4], width: f32, fill: Option<[f32; 4]> },
    Rect { x: f32, y: f32, width: f32, height: f32, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]> },
    Glyph { x: f32, y: f32, size: f32, glyph_id: u16, color: [f32; 4], italic: bool },
    Image { x: f32, y: f32, width: f32, height: f32, rotation: f32, placeholder: bool, mark: String, raster: Option<PdfRaster> },
}

/// 📕️ One exported page: its media box and how many `pdf_items` it owns (items are contiguous in
/// page order, so the page's slice starts where the previous page's ended).
#[derive(Clone, Copy, Debug)]
struct PdfPagePlan {
    width: u32,
    height: u32,
    items: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PngRowSection {
    Initialize,
    Fill,
    Header,
    Data,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PackageSection {
    Begin,
    Scalars,
    Preflight,
    Manifest,
    CentralDirectory,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LayoutExportCloseStage {
    Publication,
    JsonValidation,
    TypedValidation,
    PackageJson,
    Rects,
    Output,
    Encoded,
    Base64Tail,
    PngRow,
    PdfOffsets,
    PdfItems,
    PdfPages,
    PdfEngine,
    ZipEntries,
    ZipCurrentName,
    PageAuthority,
    Preflight,
    ParentAuthority,
    RevisionAuthority,
    OutputChunks,
    MediaCredit,
    Snapshot,
    SnapshotOwner,
    Complete,
}

#[derive(Clone, Copy)]
pub enum LayoutExportPublicationKind {
    Preview,
    Checkpoint { applied_progress: u64 },
    Commit,
    Fault,
}

/// 🧯️ One close turn of the staged export ladder; `Pending` keeps the stage's own item/byte accounting and `Granted` carries a framework receipt.
enum LayoutCloseTurn {
    Pending { released_items: usize, released_bytes: usize },
    Granted(semio_framework_value::retained_clone::RetainedCloneProgress),
    AwaitingInput { reason: &'static str },
    Blocked { reason: &'static str },
    Complete,
}

/// 🔔️ The close budget one staged turn is quoted for; stages release at most one output chunk per turn.
const LAYOUT_EXPORT_CLOSE_TURN_BYTES: usize = ArtifactOutputChunks::CHUNK_BYTES;

fn layout_close_step(turn:Result<LayoutCloseTurn,Fault>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
    Ok(match turn?{
        LayoutCloseTurn::Pending{released_items,released_bytes}=>InteractiveJobCloseStep::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()}},
        LayoutCloseTurn::Granted(progress)=>InteractiveJobCloseStep::Pending{progress},
        LayoutCloseTurn::Complete=>InteractiveJobCloseStep::Complete{progress:Default::default()},
        LayoutCloseTurn::AwaitingInput{..}|LayoutCloseTurn::Blocked{..}=>InteractiveJobCloseStep::Blocked,
    })
}

pub type LayoutExportTurn<'a> = Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure>;

pub struct LayoutExportPublication {
    kind: LayoutExportPublicationKind,
    bytes: [u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES],
    length: usize,
    publication: RetainedJobPublication,
}

impl LayoutExportPublication {
    fn new(kind: LayoutExportPublicationKind, bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES {
            return Err("layout-export-publication-limit".into());
        }
        let mut storage = [0; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES];
        storage[..bytes.len()].copy_from_slice(bytes);
        Ok(Self { kind, bytes: storage, length: bytes.len(), publication: RetainedJobPublication::new() })
    }

    fn advance<'a>(&'a mut self, context: &mut StepContext<'_>) -> LayoutExportTurn<'a> {
        let kind = match self.kind {
            LayoutExportPublicationKind::Preview => JobPublicationKind::Preview,
            LayoutExportPublicationKind::Checkpoint { applied_progress } => JobPublicationKind::Checkpoint { applied_progress },
            LayoutExportPublicationKind::Fault => JobPublicationKind::Fault,
            LayoutExportPublicationKind::Commit => return JobOutcomeBorrow::admit_complete(context, None, None).map_err(Into::into),
        };
        self.publication.advance_from_source(kind, &self.bytes[..self.length], context).map_err(Into::into)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.publication.borrow_outcome(descriptor)
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.publication.close_step(grant)
    }

    fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.publication.retirement_demands()
    }

    fn terminal_is_empty(&self) -> bool {
        self.publication.terminal_is_empty()
    }
}

/// 📮️ Holds the one scheduled publication of a job and closes a delivered one before the job resumes.
#[derive(Default)]
pub struct LayoutPublicationSlot {
    publication: Option<LayoutExportPublication>,
    delivered: bool,
}

impl LayoutPublicationSlot {
    /// ♻️ Closes a delivered publication under the step's grant; `true` once the slot is free for the next one.
    fn settle(&mut self, context: &mut StepContext<'_>) -> Result<bool, semio_framework_value::ValueError> {
        if !self.delivered {
            return Ok(true);
        }
        let Some(publication) = self.publication.as_mut() else {
            self.delivered = false;
            return Ok(true);
        };
        if !publication.terminal_is_empty() {
            let step = publication.close_step(context.retained_grant())?;
            context.consume_retained(step.progress())?;
            return Ok(false);
        }
        self.publication = None;
        self.delivered = false;
        Ok(true)
    }

    fn advance<'a>(&'a mut self, context: &mut StepContext<'_>) -> LayoutExportTurn<'a> {
        let Some(publication) = self.publication.as_mut() else {
            return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);
        };
        let outcome = publication.advance(context)?;
        if outcome.is_some() {
            self.delivered = true;
        }
        Ok(outcome)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Complete => descriptor.complete(None, None),
            _ => self.publication.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "layout export outcome lost its publication"))?.borrow_outcome(descriptor),
        }
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        let Some(publication) = self.publication.as_mut() else {
            return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()));
        };
        let step = publication.close_step(grant)?;
        if publication.terminal_is_empty() {
            self.publication = None;
            self.delivered = false;
        }
        Ok(step)
    }

    fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.publication.as_ref().map_or(Ok(Default::default()), LayoutExportPublication::retirement_demands)
    }

    fn terminal_is_empty(&self) -> bool {
        self.publication.as_ref().is_none_or(LayoutExportPublication::terminal_is_empty)
    }
}

pub struct LayoutExportJob {
    operation: Operation,
    source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
    request: Option<LayoutExportRequest>,
    stage: ExportStage,
    validation_group: u8,
    validation_index: usize,
    validation_total_frames: usize,
    validation_total_items: usize,
    json_validation: Option<JsonValidationCursor>,
    typed_validation: Option<TypedJsonCursor>,
    target_page: Option<usize>,
    plan_cursor: usize,
    parent_frame_count: usize,
    rects: Vec<ExportRect>,
    encode_cursor: usize,
    output: ChunkRope,
    encoded: ChunkRope,
    base64_cursor: usize,
    base64_tail: Vec<u8>,
    png_adler_s1: u32,
    png_adler_s2: u32,
    png_idat_crc: u32,
    png_row: Vec<u8>,
    png_row_section: PngRowSection,
    png_row_byte_cursor: usize,
    png_rect_cursor: usize,
    png_fill_x: u32,
    pdf_offsets: Vec<u32>,
    pdf_section: PdfSection,
    pdf_cursor: usize,
    pdf_xref_offset: usize,
    pdf_items: Vec<PdfItem>,
    pdf_pages: Vec<PdfPagePlan>,
    pdf_page_cursor: usize,
    pdf_item_cursor: usize,
    pdf_font_cursor: usize,
    pdf_content_start: usize,
    pdf_engine: Option<semio_s_artifact_layout_layout_engine::scene::LayoutEngine>,
    package_section: PackageSection,
    package_index: usize,
    package_byte_cursor: usize,
    package_json: Option<TypedJsonCursor>,
    zip_central_offset: u32,
    zip: ZipState,
    media_output_credit: Option<ArtifactMediaExportCredit>,
    output_chunks: Option<ArtifactOutputChunks>,
    output_retirement:Option<Box<dyn store::ErasedSnapshotRetirement>>,
    commit_cursor: usize,
    completed_units: u64,
    restore_target: Option<LayoutExportCheckpoint>,
    snapshot_close: Option<ArtifactSnapshotCloseLease<LayoutSnapshot>>,
    slot: LayoutPublicationSlot,
    closing: bool,
    close_stage: LayoutExportCloseStage,
}


impl LayoutExportJob {
    fn from_admitted(operation:Operation,source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),request:LayoutExportRequest,output_chunks:ArtifactOutputChunks)->Self {
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
    fn close_publication_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> LayoutCloseTurn {
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

    fn close_export_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<LayoutCloseTurn, Fault> {
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

    fn close_terminal_is_empty(&self) -> bool {
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

    fn active_output(&self) -> &ChunkRope {
        if self.request.as_ref().unwrap().kind.binary() && matches!(self.stage, ExportStage::Base64 | ExportStage::PackageCommit | ExportStage::Complete) {
            &self.encoded
        } else {
            &self.output
        }
    }

    fn checkpoint_publication(&self) -> Result<LayoutExportPublication, String> {
        let checkpoint = LayoutExportCheckpoint { completed_units: self.completed_units, output_bytes: self.active_output().len as u64, output_digest: self.active_output().digest };
        let state = encode_checkpoint(&self.operation, self.request.as_ref().unwrap(), &checkpoint)?;
        LayoutExportPublication::new(LayoutExportPublicationKind::Checkpoint { applied_progress: self.completed_units }, &state)
    }

    fn preview_publication(&self) -> Result<LayoutExportPublication, String> {
        LayoutExportPublication::new(LayoutExportPublicationKind::Preview, self.stage_name().as_bytes())
    }

    fn fault_publication(error: &str) -> LayoutExportPublication {
        let bytes = &error.as_bytes()[..error.len().min(MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES)];
        LayoutExportPublication::new(LayoutExportPublicationKind::Fault, bytes).expect("bounded layout fault publication")
    }

    fn drive_publication<'a>(&'a mut self, context: &mut StepContext<'_>) -> LayoutExportTurn<'a> {
        self.slot.advance(context)
    }

    fn page(&self) -> Result<&Page, String> {
        self.target_page.and_then(|index| self.request.as_ref().unwrap().snapshot.pages.get(index)).ok_or_else(|| "layout-export-page-missing".into())
    }

    fn validate_one(&mut self) -> Result<(), String> {
        let snapshot = &self.request.as_ref().unwrap().snapshot;
        match self.validation_group {
            0 => {
                if snapshot.pages.len() > MAX_LAYOUT_EXPORT_PAGES
                    || snapshot.stories.len() > MAX_LAYOUT_EXPORT_STORIES
                    || snapshot.links.len() > MAX_LAYOUT_EXPORT_LINKS
                    || snapshot.parent_pages.len() > MAX_LAYOUT_EXPORT_PARENT_PAGES
                    || snapshot.spreads.len() > MAX_LAYOUT_EXPORT_SPREADS
                    || snapshot.paragraph_styles.len() > MAX_LAYOUT_EXPORT_STYLES
                    || snapshot.character_styles.len() > MAX_LAYOUT_EXPORT_STYLES
                    || !bounded_string(&snapshot.schema)
                    || !bounded_string(&snapshot.name)
                {
                    return Err("layout-export-document-envelope".into());
                }
                self.validation_total_items = snapshot.pages.len() + snapshot.stories.len() + snapshot.links.len() + snapshot.parent_pages.len() + snapshot.spreads.len() + snapshot.paragraph_styles.len() + snapshot.character_styles.len();
                self.validation_group = 1;
            }
            1 => match snapshot.pages.get(self.validation_index) {
                Some(page) => {
                    let width = finite_dimension(page.width)?;
                    let height = finite_dimension(page.height)?;
                    if u64::from(width) * u64::from(height) > MAX_LAYOUT_EXPORT_PIXELS
                        || page.frames.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || page.overrides.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || page.guides.len() > MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE
                        || page.layers.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || page.layer_ids.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || !bounded_id(&page.id)
                        || !bounded_string(&page.name)
                        || !bounded_id(&page.spread_id)
                        || page.parent_page_id.as_ref().is_some_and(|id| !bounded_id(id))
                        || page.frames.iter().any(|frame| !valid_frame(frame))
                        || page.overrides.iter().any(|value| !bounded_id(&value.object_id) || value.bounds.as_ref().is_some_and(|bounds| !valid_bounds(bounds)))
                        || page.guides.iter().any(|guide| !valid_rect(guide))
                        || !page.margins.top.is_finite()
                        || !page.margins.right.is_finite()
                        || !page.margins.bottom.is_finite()
                        || !page.margins.left.is_finite()
                        || page.columns.count as usize > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || !page.columns.gutter.is_finite()
                        || page.layers.iter().any(|layer| layer.object_ids.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE || !bounded_id(&layer.id) || !bounded_string(&layer.name) || layer.object_ids.iter().any(|id| !bounded_id(id)))
                        || page.layer_ids.iter().any(|id| !bounded_id(id))
                    {
                        return Err("layout-export-page-envelope".into());
                    }
                    let nested = page.frames.len() + page.overrides.len() + page.guides.len() + page.layers.len() + page.layer_ids.len() + page.layers.iter().map(|layer| layer.object_ids.len()).sum::<usize>();
                    self.validation_total_frames = self.validation_total_frames.checked_add(page.frames.len()).ok_or("layout-export-item-limit")?;
                    add_validated_items(&mut self.validation_total_items, nested)?;
                    if self.request.as_ref().unwrap().page_id.as_deref().is_some_and(|id| id == page.id) || self.request.as_ref().unwrap().page_id.is_none() && self.target_page.is_none() {
                        self.target_page = Some(self.validation_index);
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 2;
                    self.validation_index = 0;
                }
            },
            2 => match snapshot.stories.get(self.validation_index) {
                Some(story) => {
                    if !bounded_id(&story.id)
                        || !bounded_string(&story.content)
                        || story.style_runs.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || story.style_runs.iter().any(|run| run.paragraph_style_id.as_ref().is_some_and(|id| !bounded_id(id)) || run.character_style_id.as_ref().is_some_and(|id| !bounded_id(id)))
                    {
                        return Err("layout-export-story-envelope".into());
                    }
                    add_validated_items(&mut self.validation_total_items, story.style_runs.len())?;
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 3;
                    self.validation_index = 0;
                }
            },
            3 => match snapshot.links.get(self.validation_index) {
                Some(link) => {
                    if !bounded_id(&link.id)
                        || !bounded_string(&link.path)
                        || link.hash.len() > 256
                        || link.width > MAX_LAYOUT_EXPORT_DIMENSION
                        || link.height > MAX_LAYOUT_EXPORT_DIMENSION
                        || u64::from(link.width) * u64::from(link.height) > MAX_LAYOUT_EXPORT_PIXELS
                        || link.dpi > 9_600
                        || link.proxy_data_url.as_ref().is_some_and(|value| !bounded_string(value))
                        || link.color_profile.as_ref().is_some_and(|value| !bounded_string(value))
                        || link.state.as_ref().is_some_and(|value| !bounded_string(value))
                    {
                        return Err("layout-export-link-envelope".into());
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 4;
                    self.validation_index = 0;
                }
            },
            4 => match snapshot.parent_pages.get(self.validation_index) {
                Some(parent) => {
                    finite_dimension(parent.width)?;
                    finite_dimension(parent.height)?;
                    if parent.frames.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || parent.layers.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || parent.layer_ids.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || !bounded_id(&parent.id)
                        || !bounded_string(&parent.name)
                        || parent.frames.iter().any(|frame| !valid_frame(frame))
                        || parent.layers.iter().any(|layer| layer.object_ids.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE || !bounded_id(&layer.id) || !bounded_string(&layer.name) || layer.object_ids.iter().any(|id| !bounded_id(id)))
                        || parent.layer_ids.iter().any(|id| !bounded_id(id))
                    {
                        return Err("layout-export-parent-page-envelope".into());
                    }
                    let nested = parent.frames.len() + parent.layers.len() + parent.layer_ids.len() + parent.layers.iter().map(|layer| layer.object_ids.len()).sum::<usize>();
                    self.validation_total_frames = self.validation_total_frames.checked_add(parent.frames.len()).ok_or("layout-export-item-limit")?;
                    add_validated_items(&mut self.validation_total_items, nested)?;
                    self.validation_index += 1;
                }
                None => {
                    if self.validation_total_frames > MAX_LAYOUT_EXPORT_TOTAL_FRAMES {
                        return Err("layout-export-frame-envelope".into());
                    }
                    self.validation_group = 5;
                    self.validation_index = 0;
                }
            },
            5 => match snapshot.paragraph_styles.get(self.validation_index) {
                Some(style) => {
                    if !bounded_id(&style.id) || !bounded_string(&style.name) || !bounded_string(&style.font_family) || !bounded_string(&style.alignment) || !style.font_size.is_finite() || !style.leading.is_finite() || !style.tracking.is_finite() {
                        return Err("layout-export-paragraph-style-envelope".into());
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 6;
                    self.validation_index = 0;
                }
            },
            6 => match snapshot.character_styles.get(self.validation_index) {
                Some(style) => {
                    if !bounded_id(&style.id)
                        || style.name.as_ref().is_some_and(|value| !bounded_string(value))
                        || style.font_family.as_ref().is_some_and(|value| !bounded_string(value))
                        || style.font_size.is_some_and(|value| !value.is_finite())
                        || style.tracking.is_some_and(|value| !value.is_finite())
                        || style.color.is_some_and(|color| color.iter().any(|channel| !channel.is_finite()))
                    {
                        return Err("layout-export-character-style-envelope".into());
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 7;
                    self.validation_index = 0;
                }
            },
            7 => match snapshot.spreads.get(self.validation_index) {
                Some(spread) => {
                    if spread.page_ids.len() > MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS || !bounded_id(&spread.id) || !bounded_string(&spread.name) || spread.page_ids.iter().any(|id| !bounded_id(id)) {
                        return Err("layout-export-spread-envelope".into());
                    }
                    add_validated_items(&mut self.validation_total_items, spread.page_ids.len())?;
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 8;
                    self.validation_index = 0;
                }
            },
            8 => {
                if snapshot.print_target.as_ref().is_some_and(|value| !bounded_string(value)) {
                    return Err("layout-export-print-target-envelope".into());
                }
                if !snapshot.grid.baseline_grid.is_finite() || !snapshot.grid.baseline_offset.is_finite() {
                    return Err("layout-export-grid-envelope".into());
                }
                self.validation_group = 9;
            }
            9 => {
                if snapshot.data_fields.is_none() {
                    self.validation_group = 10;
                    return Ok(());
                }
                if self.typed_validation.is_none() {
                    self.typed_validation = Some(TypedJsonCursor::validating(TypedJsonNode::Dictionary(DictionaryJsonCursor::new())));
                }
                if self.typed_validation.as_mut().ok_or("layout-export-json-validator")?.advance(snapshot)?.1 {
                    self.typed_validation = None;
                    self.validation_group = 10;
                }
            }
            10 => {
                let Some(value) = &self.request.as_ref().unwrap().preflight_json else {
                    self.validation_group = 11;
                    return Ok(());
                };
                if self.json_validation.is_none() {
                    self.json_validation = Some(JsonValidationCursor::new(value, true)?);
                }
                if self.json_validation.as_mut().ok_or("layout-export-json-validator")?.advance(value)? {
                    self.json_validation = None;
                    self.validation_group = 11;
                }
            }
            11 => {
                if snapshot.background_drawing.is_none() {
                    self.validation_group = 12;
                    return Ok(());
                }
                if self.typed_validation.is_none() {
                    self.typed_validation = Some(TypedJsonCursor::validating(TypedJsonNode::BackgroundDrawing));
                }
                if self.typed_validation.as_mut().ok_or("layout-export-json-validator")?.advance(snapshot)?.1 {
                    self.typed_validation = None;
                    self.validation_group = 12;
                }
            }
            12 => {
                if snapshot.referenced_model.is_none() {
                    self.validation_group = 13;
                    return Ok(());
                }
                if self.typed_validation.is_none() {
                    self.typed_validation = Some(TypedJsonCursor::validating(TypedJsonNode::ReferencedModel));
                }
                if self.typed_validation.as_mut().ok_or("layout-export-json-validator")?.advance(snapshot)?.1 {
                    self.typed_validation = None;
                    self.validation_group = 13;
                }
            }
            13 => {
                if !matches!(self.request.as_ref().unwrap().kind, LayoutExportKind::Package) && self.target_page.is_none() {
                    return Err("layout-export-page-missing".into());
                }
                self.parent_frame_count =
                    self.target_page.and_then(|index| snapshot.pages.get(index)).and_then(|page| page.parent_page_id.as_ref().and_then(|id| snapshot.parent_pages.iter().find(|parent| &parent.id == id))).map_or(0, |parent| parent.frames.len());
                if matches!(self.request.as_ref().unwrap().kind, LayoutExportKind::Package) {
                    self.begin_zip_entry("document.json")?;
                    self.stage = ExportStage::Encode;
                } else {
                    self.stage = ExportStage::Plan;
                }
            }
            _ => unreachable!("closed validation state"),
        }
        Ok(())
    }

    fn plan_one(&mut self) -> Result<(), String> {
        if matches!(self.request.as_ref().unwrap().kind, LayoutExportKind::Pdf) {
            return self.plan_pdf_one();
        }
        let page = self.page()?;
        let parent = page.parent_page_id.as_ref().and_then(|id| self.request.as_ref().unwrap().snapshot.parent_pages.iter().find(|candidate| &candidate.id == id));
        let frame = if self.plan_cursor < self.parent_frame_count { parent.and_then(|value| value.frames.get(self.plan_cursor)) } else { page.frames.get(self.plan_cursor - self.parent_frame_count) };
        let Some(frame) = frame else {
            self.begin_encoder()?;
            self.stage = ExportStage::Encode;
            return Ok(());
        };
        let inherited = self.plan_cursor < self.parent_frame_count;
        let override_value = if inherited { override_for(page, frame) } else { None };
        if let Some(rect) = export_rect(frame, override_value, finite_dimension(page.width)?, finite_dimension(page.height)?) {
            self.rects.push(rect);
        }
        self.plan_cursor += 1;
        Ok(())
    }

    fn begin_encoder(&mut self) -> Result<(), String> {
        match self.request.as_ref().unwrap().kind {
            LayoutExportKind::Svg => {
                let page = self.page()?;
                let width = finite_dimension(page.width)?;
                let height = finite_dimension(page.height)?;
                self.output.append(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\"><rect width=\"100%\" height=\"100%\" fill=\"white\"/>").as_bytes())?;
            }
            LayoutExportKind::Pdf => self.begin_pdf()?,
            LayoutExportKind::Png => self.begin_png()?,
            LayoutExportKind::Package => unreachable!("package begins after validation"),
        }
        Ok(())
    }

    fn encode_one(&mut self) -> Result<(), String> {
        match self.request.as_ref().unwrap().kind {
            LayoutExportKind::Svg => self.encode_svg_one(),
            LayoutExportKind::Pdf => self.encode_pdf_one(),
            LayoutExportKind::Png => self.encode_png_one(),
            LayoutExportKind::Package => self.encode_package_one(),
        }
    }

    fn encode_svg_one(&mut self) -> Result<(), String> {
        if let Some(rect) = self.rects.get(self.encode_cursor) {
            self.output.append(
                format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{:.4}\"/>", rect.x, rect.y, rect.width, rect.height, rect.rgba[0], rect.rgba[1], rect.rgba[2], f32::from(rect.rgba[3]) / 255.0)
                    .as_bytes(),
            )?;
            self.encode_cursor += 1;
        } else {
            self.output.append(b"</svg>")?;
            self.stage = ExportStage::PackageCommit;
        }
        Ok(())
    }

    /// 📕️ Which pages the PDF carries: the addressed page, or every page when none is addressed.
    fn pdf_page_indices(&self) -> Vec<usize> {
        match self.target_page {
            Some(index) if self.request.as_ref().unwrap().page_id.is_some() => vec![index],
            _ => (0..self.request.as_ref().unwrap().snapshot.pages.len()).collect(),
        }
    }

    /// 📕️ One unit per page: the same display list the Preview window paints (parent-page
    /// inheritance, overrides, parley-shaped glyphs) becomes this page's item slice.
    fn plan_pdf_one(&mut self) -> Result<(), String> {
        let indices = self.pdf_page_indices();
        let Some(&page_index) = indices.get(self.pdf_page_cursor) else {
            self.begin_encoder()?;
            self.stage = ExportStage::Encode;
            return Ok(());
        };
        let snapshot = Arc::clone(&self.request.as_ref().unwrap().snapshot);
        let page = snapshot.pages.get(page_index).ok_or("layout-export-page-missing")?;
        let width = finite_dimension(page.width)?;
        let height = finite_dimension(page.height)?;
        let engine = self.pdf_engine.get_or_insert_with(semio_s_artifact_layout_layout_engine::scene::LayoutEngine::new);
        let list = semio_s_artifact_layout_layout_engine::scene::build_display_list_for_page(engine, &snapshot, page, "", &[], None, false);
        let mut items = 0usize;
        let mut push = |item: PdfItem, pdf_items: &mut Vec<PdfItem>| -> Result<(), String> {
            if pdf_items.len() >= MAX_LAYOUT_EXPORT_DECODED_ITEMS {
                return Err("layout-export-item-limit".into());
            }
            pdf_items.push(item);
            items += 1;
            Ok(())
        };
        for piece in semio_s_artifact_layout_layout_engine::scene::stack_pieces(&list) {
            match piece {
                semio_s_artifact_layout_layout_engine::scene::StackPiece::Stroke(stroke) | semio_s_artifact_layout_layout_engine::scene::StackPiece::FrameStroke(stroke) => {
                    push(PdfItem::Stroke { points: stroke.points.clone(), closed: stroke.closed, color: stroke.color, width: stroke.width, fill: stroke.fill }, &mut self.pdf_items)?;
                }
                semio_s_artifact_layout_layout_engine::scene::StackPiece::Rect(rect) => {
                    push(PdfItem::Rect { x: rect.x, y: rect.y, width: rect.width, height: rect.height, fill: rect.fill.as_ref().map(|color| color.0), stroke: rect.stroke.as_ref().map(|color| color.0) }, &mut self.pdf_items)?;
                }
                semio_s_artifact_layout_layout_engine::scene::StackPiece::Image(image) => {
                    push(PdfItem::Image { x: image.x, y: image.y, width: image.width, height: image.height, rotation: image.rotation, placeholder: image.placeholder, mark: image.preview.clone(), raster: image.proxy_data_url.as_deref().and_then(Self::proxy_png_rgb) }, &mut self.pdf_items)?;
                }
                semio_s_artifact_layout_layout_engine::scene::StackPiece::Text(run) => {
                    for glyph in &run.glyphs {
                        let glyph_id = u16::try_from(glyph.glyph_id).map_err(|_| "layout-export-glyph-range")?;
                        push(PdfItem::Glyph { x: glyph.x, y: glyph.y, size: glyph.font_size, glyph_id, color: glyph.color.0, italic: glyph.italic }, &mut self.pdf_items)?;
                    }
                }
            }
        }
        self.pdf_pages.push(PdfPagePlan { width, height, items });
        self.pdf_page_cursor += 1;
        Ok(())
    }

    fn begin_pdf(&mut self) -> Result<(), String> {
        self.output.append(b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n")?;
        self.pdf_section = PdfSection::Catalog;
        self.pdf_cursor = 0;
        self.pdf_item_cursor = 0;
        Ok(())
    }

    fn pdf_object_offset(&mut self, id: usize) -> Result<(), String> {
        if self.pdf_offsets.len() + 1 != id {
            return Err("layout-export-pdf-object-order".into());
        }
        self.pdf_offsets.push(u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?);
        Ok(())
    }

    fn pdf_object(&mut self, id: usize, body: &[u8]) -> Result<(), String> {
        self.pdf_object_offset(id)?;
        self.output.append(format!("{id} 0 obj\n").as_bytes())?;
        self.output.append(body)?;
        self.output.append(b"\nendobj\n")
    }

    fn pdf_page_object_id(page: usize) -> usize {
        7 + 3 * page
    }


fn proxy_png_rgb(data_url: &str) -> Option<PdfRaster> {
    let payload = data_url.strip_prefix("data:image/png;base64,")?;
    let bytes = decode_base64(payload).ok()?;
    let snapshot = semio_framework_pixels::decode_png(&bytes).ok()?;
    let pixels = u64::from(snapshot.width).checked_mul(u64::from(snapshot.height))?;
    if pixels == 0 || pixels > 65_536 {
        return None;
    }
    let expected = (pixels as usize).checked_mul(4)?;
    if snapshot.pixels.len() != expected {
        return None;
    }
    let mut rgb = Vec::with_capacity(expected / 4 * 3);
    for pixel in snapshot.pixels.chunks_exact(4) {
        let alpha = f32::from(pixel[3]) / 255.0;
        for channel in 0..3 {
            rgb.push(((f32::from(pixel[channel]) / 255.0 * alpha + (1.0 - alpha)) * 255.0).round() as u8);
        }
    }
    Some(PdfRaster { width: snapshot.width, height: snapshot.height, rgb })
}

fn pdf_image_matrix(x: f32, y: f32, width: f32, height: f32, rotation: f32, page_height: f32) -> String {
    if rotation.abs() < 1.0e-4 {
        return format!("{width:.3} 0 0 {height:.3} {x:.3} {:.3}", page_height - y - height);
    }
    let cx = x + width * 0.5;
    let cy = y + height * 0.5;
    let (sin, cos) = rotation.sin_cos();
    let map = |px: f32, py: f32| {
        let dx = px - cx;
        let dy = py - cy;
        let sx = cx + dx * cos - dy * sin;
        let sy = cy + dx * sin + dy * cos;
        (sx, page_height - sy)
    };
    let bottom_left = map(x, y + height);
    let bottom_right = map(x + width, y + height);
    let top_left = map(x, y);
    format!(
        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3}",
        bottom_right.0 - bottom_left.0,
        bottom_right.1 - bottom_left.1,
        top_left.0 - bottom_left.0,
        top_left.1 - bottom_left.1,
        bottom_left.0,
        bottom_left.1
    )
}

fn pdf_inline_image(x: f32, y: f32, width: f32, height: f32, rotation: f32, page_height: f32, raster: &PdfRaster) -> String {
    let mut hex = String::with_capacity(raster.rgb.len() * 2);
    for byte in &raster.rgb {
        hex.push_str(&format!("{byte:02X}"));
    }
    let matrix = Self::pdf_image_matrix(x, y, width, height, rotation, page_height);
    format!("q {matrix} cm\nBI\n/W {} /H {} /CS /RGB /BPC 8 /F /AHx\nID\n{hex}>\nEI\nQ\n", raster.width, raster.height)
}

fn pdf_preview_ops(mark: &str, x: f32, y: f32, width: f32, height: f32, page_height: f32) -> String {
    let inset = 4.0_f32.min(width * 0.2).min(height * 0.2);
    if mark.is_empty() || inset < 0.5 {
        return String::new();
    }
    let left = x + inset;
    let right = x + width - inset;
    let top = page_height - (y + inset);
    let bottom = page_height - (y + height - inset);
    let line = |x1: f32, y1: f32, x2: f32, y2: f32| format!("q 0.2000 0.2500 0.3000 RG 1.5 w {x1:.3} {y1:.3} m {x2:.3} {y2:.3} l S Q\n");
    match mark {
        "page" => format!("q 0.2000 0.2500 0.3000 RG 1.5 w {left:.3} {bottom:.3} {:.3} {:.3} re S Q\n", right - left, top - bottom),
        "stroke" => line(left, top, right, bottom),
        "map" => {
            let mid_x = (left + right) * 0.5;
            let mid_y = (top + bottom) * 0.5;
            format!("{}{}", line(mid_x, top, mid_x, bottom), line(left, mid_y, right, mid_y))
        }
        "curve" => {
            let mid_x = (left + right) * 0.5;
            format!("{}{}", line(left, bottom, mid_x, top), line(mid_x, top, right, bottom))
        }
        "grid" => {
            let step_x = (right - left) / 3.0;
            let step_y = (top - bottom) / 3.0;
            let mut ops = String::new();
            for step in 1..3 {
                let y = bottom + step_y * step as f32;
                let x = left + step_x * step as f32;
                ops.push_str(&line(left, y, right, y));
                ops.push_str(&line(x, bottom, x, top));
            }
            ops
        }
        _ => String::new(),
    }
}

    fn pdf_item_ops(item: &PdfItem, page_height: f32) -> String {
        let color = |rgba: [f32; 4]| format!("{:.4} {:.4} {:.4}", rgba[0].clamp(0.0, 1.0), rgba[1].clamp(0.0, 1.0), rgba[2].clamp(0.0, 1.0));
        match item {
            PdfItem::Stroke { points, closed, color: stroke_color, width, fill } => {
                if points.len() < 2 {
                    return String::new();
                }
                let mut path = String::new();
                for (index, (x, y)) in points.iter().enumerate() {
                    let verb = if index == 0 { "m" } else { "l" };
                    path.push_str(&format!("{x:.3} {:.3} {verb} ", page_height - y));
                }
                if *closed {
                    path.push_str("h ");
                }
                let mut ops = String::new();
                if let Some(fill) = fill.filter(|rgba| rgba[3] > 0.001) {
                    ops.push_str(&format!("q {} rg {path}f Q\n", color(fill)));
                }
                ops.push_str(&format!("q {} RG {width:.3} w {path}S Q\n", color(*stroke_color)));
                ops
            }
            PdfItem::Rect { x, y, width, height, fill, stroke } => {
                let mut ops = String::new();
                let path = format!("{:.3} {:.3} {:.3} {:.3} re", x, page_height - y - height, width, height);
                if let Some(fill) = fill.filter(|rgba| rgba[3] > 0.001) {
                    ops.push_str(&format!("q {} rg {path} f Q\n", color(fill)));
                }
                if let Some(stroke) = stroke.filter(|rgba| rgba[3] > 0.001) {
                    ops.push_str(&format!("q {} RG 1 w {path} S Q\n", color(stroke)));
                }
                ops
            }
            PdfItem::Image { x, y, width, height, rotation, placeholder, mark, raster } => {
                if let Some(raster) = raster {
                    return Self::pdf_inline_image(*x, *y, *width, *height, *rotation, page_height, raster);
                }
                let path = format!("{:.3} {:.3} {:.3} {:.3} re", x, page_height - y - height, width, height);
                let body = if *placeholder {
                    format!("q 0.92 0.88 0.84 rg {path} f Q\nq 0.75 0.35 0.2 RG 1 w {path} S Q\n")
                } else {
                    format!("q 0.85 0.85 0.85 rg {path} f Q\n")
                };
                format!("{body}{}", Self::pdf_preview_ops(mark, *x, *y, *width, *height, page_height))
            }
            PdfItem::Glyph { x, y, size, glyph_id, color: fill, italic } => {
                let tinted = fill[0] > 0.001 || fill[1] > 0.001 || fill[2] > 0.001;
                let open = if tinted { format!("q {} rg ", color(*fill)) } else { String::new() };
                let close = if tinted { " Q" } else { "" };
                let tm = if *italic { format!("1 0 0.250 1 {:.3} {:.3}", x, page_height - y) } else { format!("1 0 0 1 {:.3} {:.3}", x, page_height - y) };
                format!("{open}BT /F1 {:.3} Tf {tm} Tm <{:04X}> Tj ET{close}\n", size, glyph_id)
            }
        }
    }

    fn encode_pdf_one(&mut self) -> Result<(), String> {
        const FONT: &[u8] = semio_s_artifact_layout_layout_engine::scene::LAYOUT_SANS;
        let page_count = self.pdf_pages.len();
        match self.pdf_section {
            PdfSection::Header => return Err("layout-export-pdf-header-state".into()),
            PdfSection::Catalog => {
                self.pdf_object(1, b"<< /Type /Catalog /Pages 2 0 R >>")?;
                self.pdf_section = PdfSection::Pages;
            }
            PdfSection::Pages => {
                self.pdf_object_offset(2)?;
                self.output.append(b"2 0 obj\n<< /Type /Pages /Kids [")?;
                self.pdf_cursor = 0;
                self.pdf_section = PdfSection::PageKids;
            }
            PdfSection::PageKids => {
                if self.pdf_cursor < page_count {
                    self.output.append(format!("{} 0 R ", Self::pdf_page_object_id(self.pdf_cursor)).as_bytes())?;
                    self.pdf_cursor += 1;
                } else {
                    self.pdf_section = PdfSection::PagesEnd;
                }
            }
            PdfSection::PagesEnd => {
                self.output.append(format!("] /Count {page_count} >>\nendobj\n").as_bytes())?;
                self.pdf_section = PdfSection::FontType0;
            }
            PdfSection::FontType0 => {
                self.pdf_object(3, b"<< /Type /Font /Subtype /Type0 /BaseFont /LayoutSans /Encoding /Identity-H /DescendantFonts [4 0 R] >>")?;
                self.pdf_section = PdfSection::FontCid;
            }
            PdfSection::FontCid => {
                self.pdf_object(4, b"<< /Type /Font /Subtype /CIDFontType2 /BaseFont /LayoutSans /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> /FontDescriptor 5 0 R /DW 600 /CIDToGIDMap /Identity >>")?;
                self.pdf_section = PdfSection::FontDescriptor;
            }
            PdfSection::FontDescriptor => {
                self.pdf_object(5, b"<< /Type /FontDescriptor /FontName /LayoutSans /Flags 32 /FontBBox [-200 -300 1200 1000] /ItalicAngle 0 /Ascent 900 /Descent -250 /CapHeight 700 /StemV 80 /FontFile2 6 0 R >>")?;
                self.pdf_section = PdfSection::FontFileBegin;
            }
            PdfSection::FontFileBegin => {
                self.pdf_object_offset(6)?;
                self.output.append(format!("6 0 obj\n<< /Length {} /Length1 {} >>\nstream\n", FONT.len(), FONT.len()).as_bytes())?;
                self.pdf_font_cursor = 0;
                self.pdf_section = PdfSection::FontFileBytes;
            }
            PdfSection::FontFileBytes => {
                let end = (self.pdf_font_cursor + OUTPUT_CHUNK_BYTES).min(FONT.len());
                if self.pdf_font_cursor < end {
                    self.output.append(&FONT[self.pdf_font_cursor..end])?;
                    self.pdf_font_cursor = end;
                } else {
                    self.pdf_section = PdfSection::FontFileEnd;
                }
            }
            PdfSection::FontFileEnd => {
                self.output.append(b"\nendstream\nendobj\n")?;
                self.pdf_cursor = 0;
                self.pdf_item_cursor = 0;
                self.pdf_section = if page_count == 0 { PdfSection::XrefBegin } else { PdfSection::PageObject };
            }
            PdfSection::PageObject => {
                let page = self.pdf_pages.get(self.pdf_cursor).copied().ok_or("layout-export-page-missing")?;
                let id = Self::pdf_page_object_id(self.pdf_cursor);
                let body = format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /Font << /F1 3 0 R >> >> /Contents {} 0 R >>", page.width, page.height, id + 1);
                self.pdf_object(id, body.as_bytes())?;
                self.pdf_section = PdfSection::ContentBegin;
            }
            PdfSection::ContentBegin => {
                let id = Self::pdf_page_object_id(self.pdf_cursor) + 1;
                self.pdf_object_offset(id)?;
                self.output.append(format!("{id} 0 obj\n<< /Length {} 0 R >>\nstream\n", id + 1).as_bytes())?;
                self.pdf_content_start = self.output.len;
                self.pdf_section = PdfSection::ContentItem;
            }
            PdfSection::ContentItem => {
                let page = self.pdf_pages.get(self.pdf_cursor).copied().ok_or("layout-export-page-missing")?;
                let first: usize = self.pdf_pages[..self.pdf_cursor].iter().map(|plan| plan.items).sum();
                if self.pdf_item_cursor < page.items {
                    let item = self.pdf_items.get(first + self.pdf_item_cursor).ok_or("layout-export-item-missing")?;
                    let ops = Self::pdf_item_ops(item, page.height as f32);
                    self.output.append(ops.as_bytes())?;
                    self.pdf_item_cursor += 1;
                } else {
                    self.pdf_section = PdfSection::ContentEnd;
                }
            }
            PdfSection::ContentEnd => {
                self.pdf_xref_offset = self.output.len - self.pdf_content_start;
                self.output.append(b"\nendstream\nendobj\n")?;
                self.pdf_section = PdfSection::ContentLength;
            }
            PdfSection::ContentLength => {
                let id = Self::pdf_page_object_id(self.pdf_cursor) + 2;
                let length = self.pdf_xref_offset;
                self.pdf_object(id, format!("{length}").as_bytes())?;
                self.pdf_cursor += 1;
                self.pdf_item_cursor = 0;
                self.pdf_section = if self.pdf_cursor < page_count { PdfSection::PageObject } else { PdfSection::XrefBegin };
            }
            PdfSection::XrefBegin => {
                self.pdf_xref_offset = self.output.len;
                self.output.append(format!("xref\n0 {}\n0000000000 65535 f \n", self.pdf_offsets.len() + 1).as_bytes())?;
                self.pdf_cursor = 0;
                self.pdf_section = PdfSection::XrefEntries;
            }
            PdfSection::XrefEntries => {
                if let Some(offset) = self.pdf_offsets.get(self.pdf_cursor).copied() {
                    self.output.append(format!("{offset:010} 00000 n \n").as_bytes())?;
                    self.pdf_cursor += 1;
                } else {
                    self.pdf_section = PdfSection::Trailer;
                }
            }
            PdfSection::Trailer => {
                self.output.append(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n", self.pdf_offsets.len() + 1, self.pdf_xref_offset).as_bytes())?;
                self.pdf_section = PdfSection::Complete;
            }
            PdfSection::Complete => self.stage = ExportStage::Base64,
        }
        Ok(())
    }

    fn begin_png(&mut self) -> Result<(), String> {
        let page = self.page()?;
        let width = finite_dimension(page.width)?;
        let height = finite_dimension(page.height)?;
        self.output.append(b"\x89PNG\r\n\x1a\n")?;
        let mut ihdr = Vec::with_capacity(13);
        ihdr.extend_from_slice(&width.to_be_bytes());
        ihdr.extend_from_slice(&height.to_be_bytes());
        ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
        self.png_chunk(b"IHDR", &ihdr)?;
        let row_bytes = width as usize * 4 + 1;
        let idat_len = 2usize.checked_add(height as usize * (row_bytes + 5)).and_then(|value| value.checked_add(4)).ok_or("layout-export-output-limit")?;
        self.output.append(&u32::try_from(idat_len).map_err(|_| "layout-export-output-limit")?.to_be_bytes())?;
        self.output.append(b"IDAT")?;
        self.png_idat_crc = crc32_update(0, b"IDAT");
        self.append_idat(&[0x78, 0x01])
    }

    fn png_chunk(&mut self, kind: &[u8; 4], data: &[u8]) -> Result<(), String> {
        self.output.append(&u32::try_from(data.len()).map_err(|_| "layout-export-output-limit")?.to_be_bytes())?;
        self.output.append(kind)?;
        self.output.append(data)?;
        self.output.append(&crc32_update(crc32_update(0, kind), data).to_be_bytes())
    }

    fn append_idat(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.png_idat_crc = crc32_update(self.png_idat_crc, bytes);
        self.output.append(bytes)
    }

    fn encode_png_one(&mut self) -> Result<(), String> {
        let (width, height) = {
            let page = self.page()?;
            (finite_dimension(page.width)?, finite_dimension(page.height)?)
        };
        if self.encode_cursor < height as usize {
            let y = self.encode_cursor as u32;
            match self.png_row_section {
                PngRowSection::Initialize => {
                    let row_len = width as usize * 4 + 1;
                    if self.png_row.is_empty() {
                        self.png_row = Vec::with_capacity(row_len);
                        self.png_row.push(0);
                    }
                    let end = self.png_row.len().saturating_add(OUTPUT_CHUNK_BYTES).min(row_len);
                    self.png_row.resize(end, 255);
                    if end == row_len {
                        self.png_rect_cursor = 0;
                        self.png_fill_x = 0;
                        self.png_row_section = PngRowSection::Fill;
                    }
                    return Ok(());
                }
                PngRowSection::Fill => {
                    if let Some(rect) = self.rects.get(self.png_rect_cursor) {
                        if y < rect.y || y >= rect.y + rect.height {
                            self.png_rect_cursor += 1;
                            self.png_fill_x = 0;
                            return Ok(());
                        }
                        let start = self.png_fill_x.max(rect.x);
                        let rect_end = rect.x + rect.width;
                        let end = start.saturating_add(PNG_PIXELS_PER_UNIT).min(rect_end);
                        for x in start..end {
                            let index = x as usize * 4 + 1;
                            self.png_row[index..index + 4].copy_from_slice(&rect.rgba);
                        }
                        if end == rect_end {
                            self.png_rect_cursor += 1;
                            self.png_fill_x = 0;
                        } else {
                            self.png_fill_x = end;
                        }
                    } else {
                        self.png_row_section = PngRowSection::Header;
                    }
                    return Ok(());
                }
                PngRowSection::Header => {
                    let len = u16::try_from(self.png_row.len()).map_err(|_| "layout-export-png-row-limit")?;
                    let mut header = [0u8; 5];
                    header[0] = u8::from(self.encode_cursor + 1 == height as usize);
                    header[1..3].copy_from_slice(&len.to_le_bytes());
                    header[3..5].copy_from_slice(&(!len).to_le_bytes());
                    self.append_idat(&header)?;
                    self.png_row_byte_cursor = 0;
                    self.png_row_section = PngRowSection::Data;
                    return Ok(());
                }
                PngRowSection::Data => {
                    let end = self.png_row_byte_cursor.saturating_add(OUTPUT_CHUNK_BYTES).min(self.png_row.len());
                    if self.png_row_byte_cursor < end {
                        let bytes = &self.png_row[self.png_row_byte_cursor..end];
                        for byte in bytes {
                            self.png_adler_s1 = (self.png_adler_s1 + u32::from(*byte)) % 65_521;
                            self.png_adler_s2 = (self.png_adler_s2 + self.png_adler_s1) % 65_521;
                        }
                        self.png_idat_crc = crc32_update(self.png_idat_crc, bytes);
                        self.output.append(bytes)?;
                        self.png_row_byte_cursor = end;
                        return Ok(());
                    }
                    self.png_row.clear();
                    self.png_row_section = PngRowSection::Initialize;
                    self.png_row_byte_cursor = 0;
                    self.png_rect_cursor = 0;
                    self.encode_cursor += 1;
                    return Ok(());
                }
            }
        }
        let adler = (self.png_adler_s2 << 16) | self.png_adler_s1;
        self.append_idat(&adler.to_be_bytes())?;
        self.output.append(&self.png_idat_crc.to_be_bytes())?;
        self.png_chunk(b"IEND", &[])?;
        self.stage = ExportStage::Base64;
        Ok(())
    }

    fn begin_zip_entry(&mut self, name: &str) -> Result<(), String> {
        if self.zip.current_name.is_some() || self.zip.entries.len() >= MAX_LAYOUT_EXPORT_FILES {
            return Err("layout-export-package-entry-state".into());
        }
        let mut header = Vec::with_capacity(30 + name.len());
        push_u32(&mut header, 0x04034b50);
        push_u16(&mut header, 20);
        push_u16(&mut header, 0x0008);
        push_u16(&mut header, 0);
        push_u16(&mut header, 0);
        push_u16(&mut header, 0);
        push_u32(&mut header, 0);
        push_u32(&mut header, 0);
        push_u32(&mut header, 0);
        push_u16(&mut header, u16::try_from(name.len()).map_err(|_| "layout-export-package-name-limit")?);
        push_u16(&mut header, 0);
        header.extend_from_slice(name.as_bytes());
        self.zip.current_offset = u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?;
        self.output.append(&header)?;
        self.zip.current_name = Some(name.into());
        self.zip.current_crc = 0;
        self.zip.current_size = 0;
        Ok(())
    }

    fn append_zip_data(&mut self, bytes: &[u8]) -> Result<(), String> {
        if self.zip.current_name.is_none() {
            return Err("layout-export-package-no-entry".into());
        }
        self.zip.current_crc = crc32_update(self.zip.current_crc, bytes);
        self.zip.current_size = self.zip.current_size.checked_add(u32::try_from(bytes.len()).map_err(|_| "layout-export-output-limit")?).ok_or("layout-export-output-limit")?;
        self.output.append(bytes)
    }

    fn end_zip_entry(&mut self) -> Result<(), String> {
        let name = self.zip.current_name.take().ok_or("layout-export-package-no-entry")?;
        let mut descriptor = Vec::with_capacity(16);
        push_u32(&mut descriptor, 0x08074b50);
        push_u32(&mut descriptor, self.zip.current_crc);
        push_u32(&mut descriptor, self.zip.current_size);
        push_u32(&mut descriptor, self.zip.current_size);
        self.output.append(&descriptor)?;
        self.zip.entries.push(ZipEntry { name, crc: self.zip.current_crc, size: self.zip.current_size, offset: self.zip.current_offset });
        Ok(())
    }

    fn encode_package_one(&mut self) -> Result<(), String> {
        match self.package_section {
            PackageSection::Begin => {
                self.package_json = Some(TypedJsonCursor::document());
                self.package_section = PackageSection::Scalars;
            }
            PackageSection::Scalars => {
                let (bytes, done) = self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.as_ref().unwrap().snapshot)?;
                if !bytes.is_empty() {
                    self.append_zip_data(&bytes)?;
                }
                if done {
                    self.package_json = None;
                    self.end_zip_entry()?;
                    self.begin_zip_entry("preflight-report.json")?;
                    self.package_byte_cursor = 0;
                    if self.request.as_ref().unwrap().preflight_json.is_none() {
                        self.package_json = Some(TypedJsonCursor::preflight());
                    }
                    self.package_section = PackageSection::Preflight;
                }
            }
            PackageSection::Preflight => {
                if let Some(preflight) = &self.request.as_ref().unwrap().preflight_json {
                    let end = self.package_byte_cursor.saturating_add(OUTPUT_CHUNK_BYTES).min(preflight.len());
                    if self.package_byte_cursor < end {
                        let bytes = preflight.as_bytes()[self.package_byte_cursor..end].to_vec();
                        self.append_zip_data(&bytes)?;
                        self.package_byte_cursor = end;
                    } else {
                        self.finish_preflight_entry()?;
                    }
                    return Ok(());
                }
                let (bytes, done) = self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.as_ref().unwrap().snapshot)?;
                if !bytes.is_empty() {
                    self.append_zip_data(&bytes)?;
                }
                if done {
                    self.package_json = None;
                    self.finish_preflight_entry()?;
                }
            }
            PackageSection::Manifest => {
                let (bytes, done) = self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.as_ref().unwrap().snapshot)?;
                if !bytes.is_empty() {
                    self.append_zip_data(&bytes)?;
                }
                if done {
                    self.package_json = None;
                    self.end_zip_entry()?;
                    self.package_index = 0;
                    self.zip_central_offset = u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?;
                    self.package_section = PackageSection::CentralDirectory;
                }
            }
            PackageSection::CentralDirectory => {
                if let Some(entry) = self.zip.entries.get(self.package_index).cloned() {
                    let mut central = Vec::with_capacity(46 + entry.name.len());
                    push_u32(&mut central, 0x02014b50);
                    push_u16(&mut central, 20);
                    push_u16(&mut central, 20);
                    push_u16(&mut central, 0x0008);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u32(&mut central, entry.crc);
                    push_u32(&mut central, entry.size);
                    push_u32(&mut central, entry.size);
                    push_u16(&mut central, u16::try_from(entry.name.len()).map_err(|_| "layout-export-package-name-limit")?);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u32(&mut central, 0);
                    push_u32(&mut central, entry.offset);
                    central.extend_from_slice(entry.name.as_bytes());
                    self.output.append(&central)?;
                    self.package_index += 1;
                } else {
                    let end = u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?;
                    let count = u16::try_from(self.zip.entries.len()).map_err(|_| "layout-export-package-entry-limit")?;
                    let mut eocd = Vec::with_capacity(22);
                    push_u32(&mut eocd, 0x06054b50);
                    push_u16(&mut eocd, 0);
                    push_u16(&mut eocd, 0);
                    push_u16(&mut eocd, count);
                    push_u16(&mut eocd, count);
                    push_u32(&mut eocd, end - self.zip_central_offset);
                    push_u32(&mut eocd, self.zip_central_offset);
                    push_u16(&mut eocd, 0);
                    self.output.append(&eocd)?;
                    self.package_section = PackageSection::Complete;
                }
            }
            PackageSection::Complete => self.stage = ExportStage::Base64,
        }
        Ok(())
    }

    fn finish_preflight_entry(&mut self) -> Result<(), String> {
        self.end_zip_entry()?;
        self.begin_zip_entry("package-manifest.json")?;
        self.package_json = Some(TypedJsonCursor::manifest());
        self.package_index = 0;
        self.package_section = PackageSection::Manifest;
        Ok(())
    }

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

    fn stage_name(&self) -> &'static str {
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
    fn commit_ready(&self) -> bool {
        matches!(self.stage, ExportStage::Complete) && !self.slot.delivered
    }

    fn close_turn(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<LayoutCloseTurn, Fault> {
        if self.close_stage == LayoutExportCloseStage::Publication {
            return Ok(self.close_publication_step(grant));
        }
        self.close_export_step(grant.maximum_items, grant.maximum_release_bytes)
    }

    fn close_demand(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
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

//#region 🔧️PlanPrimitives

fn frame_visible(frame: &Frame) -> bool {
    match frame {
        Frame::Rect { visible, .. } | Frame::Text { visible, .. } | Frame::Image { visible, .. } => visible.unwrap_or(true),
    }
}

fn frame_id(frame: &Frame) -> &str {
    match frame {
        Frame::Rect { id, .. } | Frame::Text { id, .. } | Frame::Image { id, .. } => id,
    }
}

fn override_for<'a>(page: &'a Page, frame: &Frame) -> Option<&'a PageOverride> {
    page.overrides.iter().find(|candidate| candidate.object_id == frame_id(frame))
}

fn export_rect(frame: &Frame, override_value: Option<&PageOverride>, page_width: u32, page_height: u32) -> Option<ExportRect> {
    if !override_value.and_then(|value| value.visible).unwrap_or_else(|| frame_visible(frame)) {
        return None;
    }
    let bounds = override_value.and_then(|value| value.bounds.as_ref()).unwrap_or_else(|| frame_bounds(frame));
    let x = bounds.x.max(0.0).min(f64::from(page_width)) as u32;
    let y = bounds.y.max(0.0).min(f64::from(page_height)) as u32;
    let x1 = (bounds.x + bounds.width).max(0.0).min(f64::from(page_width)) as u32;
    let y1 = (bounds.y + bounds.height).max(0.0).min(f64::from(page_height)) as u32;
    if x1 <= x || y1 <= y {
        return None;
    }
    let rgba = match frame {
        Frame::Rect { fill, .. } => fill.map_or([0, 0, 0, 0], |color| color.map(|channel| (channel.clamp(0.0, 1.0) * 255.0).round() as u8)),
        Frame::Text { .. } => [24, 24, 24, 255],
        Frame::Image { .. } => [220, 220, 220, 255],
    };
    Some(ExportRect { x, y, width: x1 - x, height: y1 - y, rgba })
}

fn finite_dimension(value: f64) -> Result<u32, String> {
    if !value.is_finite() || value < 1.0 || value > f64::from(MAX_LAYOUT_EXPORT_DIMENSION) {
        return Err("layout-export-dimension-limit".into());
    }
    Ok(value.ceil() as u32)
}

pub fn bounded_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 256
}

fn bounded_string(value: &str) -> bool {
    value.len() <= MAX_LAYOUT_EXPORT_STRING_BYTES
}

fn valid_frame(frame: &Frame) -> bool {
    match frame {
        Frame::Rect { id, layer_id, bounds, fill, stroke, .. } => bounded_id(id) && bounded_id(layer_id) && valid_bounds(bounds) && fill.is_none_or(valid_color) && stroke.is_none_or(valid_color),
        Frame::Text { id, layer_id, bounds, story_id, thread_next, columns, inset, wrap_mode, .. } => {
            bounded_id(id)
                && bounded_id(layer_id)
                && valid_bounds(bounds)
                && bounded_id(story_id)
                && thread_next.as_ref().is_none_or(|id| bounded_id(id))
                && (*columns as usize) <= MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                && valid_rect(inset)
                && bounded_string(wrap_mode)
        }
        Frame::Image { id, layer_id, bounds, link_id, .. } => bounded_id(id) && bounded_id(layer_id) && valid_bounds(bounds) && bounded_id(link_id),
    }
}

fn valid_bounds(bounds: &LayoutBounds) -> bool {
    bounds.x.is_finite() && bounds.y.is_finite() && bounds.width.is_finite() && bounds.height.is_finite() && bounds.rotation.is_finite()
}

fn valid_rect(rect: &semio_s_artifact_layout_layout_model::LayoutRect) -> bool {
    rect.x.is_finite() && rect.y.is_finite() && rect.width.is_finite() && rect.height.is_finite()
}

fn valid_color(color: [f32; 4]) -> bool {
    color.iter().all(|channel| channel.is_finite())
}

fn add_validated_items(total: &mut usize, count: usize) -> Result<(), String> {
    *total = total.checked_add(count).ok_or("layout-export-item-limit")?;
    if *total > MAX_LAYOUT_EXPORT_DECODED_ITEMS {
        return Err("layout-export-item-limit".into());
    }
    Ok(())
}

//#endregion 🔧️PlanPrimitives

//#region 🔧️CodecPrimitives
#[cfg(test)]
struct CheckpointReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

#[cfg(test)]
impl<'a> CheckpointReader<'a> {
    fn take(&mut self, count: usize) -> Result<&'a [u8], String> {
        let end = self.cursor.checked_add(count).ok_or("layout-export-checkpoint-decode")?;
        let value = self.bytes.get(self.cursor..end).ok_or("layout-export-checkpoint-decode")?;
        self.cursor = end;
        Ok(value)
    }

    fn byte(&mut self) -> Result<u8, String> {
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

    fn u64(&mut self) -> Result<u64, String> {
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
    bytes: [u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES],
    cursor: usize,
}

impl CheckpointWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
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

fn encode_checkpoint(operation: &Operation, request: &LayoutExportRequest, checkpoint: &LayoutExportCheckpoint) -> Result<[u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], String> {
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

#[cfg(test)]
fn decode_checkpoint(operation: &Operation, request: &LayoutExportRequest, state: &[u8]) -> Result<LayoutExportCheckpoint, String> {
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

fn crc32_update(mut crc: u32, bytes: &[u8]) -> u32 {
    crc = !crc;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb88320 & 0u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
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

fn decode_base64(value: &str) -> Result<Vec<u8>, String> {
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

//#region 🧪️TestOracle
#[cfg(test)]
pub fn run_layout_export_headless_batch(operation: Operation, request: LayoutExportRequest) -> Result<LayoutExportCommit, String> {
    let kind = request.kind;
    let name = output_name(&request);
    let snapshot_owner = Arc::clone(&request.snapshot);
    let job = LayoutExportJob::from_admitted(operation,(operation.operation,operation.generation),request,ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES));
    let output_chunks = job.output_chunks.as_ref().unwrap().clone();
    let params = BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: BatchDriveConfig { work_grant: semio_framework_job::retained_work::NO_RETAINED_WORK, site: "layout.export.batch", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = match semio_framework_job_worker::BatchJobSession::try_new(job, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            return Err("layout-export-test-oracle-admission-rejected".into());
        }
    };
    loop {
        session.step().map_err(|error| format!("layout-export-test-oracle-contention:{error:?}"))?;
        let Some(mut outcome) = session.take_outcome() else { continue };
        let terminal = outcome.is_terminal();
        let result = match &outcome {
            StepOutcome::Complete(_) => Some(LayoutExportCommit::from_chunks(kind, &name, &output_chunks)),
            StepOutcome::Cancelled => Some(Err("layout-export-cancelled".into())),
            StepOutcome::Fault(fault) => Some(Err(fault.detail.single_page().and_then(|page| std::str::from_utf8(page).ok()).unwrap_or("layout-export-fault").to_owned())),
            StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => None,
        };
        while !outcome.terminal_is_empty() {
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        if terminal {
            session.begin_close();
            while !session.terminal_is_empty() {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            drop(snapshot_owner);
            return result.expect("terminal layout test oracle result");
        }
        session.resume().map_err(|error| format!("layout-export-test-oracle-resume:{error:?}"))?;
    }
}

#[cfg(test)]
fn headless_batch_export(kind: LayoutExportKind, snapshot: &LayoutSnapshot, page_id: Option<&str>, preflight_json: Option<&str>) -> Result<LayoutExportCommit, String> {
    let operation = Operation::new(semio_framework_job::allocate_operation_id(), RevisionId(0), Generation(0), 0);
    run_layout_export_headless_batch(
        operation,
        LayoutExportRequest {
            kind,
            page_id: page_id.map(str::to_owned),
            snapshot: Arc::new(snapshot.clone()),
            preflight_json: preflight_json.map(str::to_owned),
            parent_document_id: "layout.headless-batch".into(),
            canonical_base_revision_hex: "0000000000000000000000000000000000000000000000000000000000000000".into(),
        },
    )
}

#[cfg(test)]
pub fn export_document_svg_headless_batch(doc: &LayoutSnapshot, page_id: &str) -> Result<String, semio_s_artifact_layout_layout_model::LayoutError> {
    if !doc.pages.iter().any(|page| page.id == page_id) {
        return Err(semio_s_artifact_layout_layout_model::LayoutError::PageNotFound(page_id.into()));
    }
    headless_batch_export(LayoutExportKind::Svg, doc, Some(page_id), None).map(|commit| commit.data).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)
}

#[cfg(test)]
pub fn export_document_pdf_headless_batch(doc: &LayoutSnapshot, page_id: &str) -> Result<Vec<u8>, semio_s_artifact_layout_layout_model::LayoutError> {
    if !doc.pages.iter().any(|page| page.id == page_id) {
        return Err(semio_s_artifact_layout_layout_model::LayoutError::PageNotFound(page_id.into()));
    }
    let commit = headless_batch_export(LayoutExportKind::Pdf, doc, Some(page_id), None).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)?;
    decode_base64(&commit.data).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)
}

#[cfg(test)]
pub fn export_document_png_headless_batch(doc: &LayoutSnapshot, page_id: &str) -> Result<Vec<u8>, semio_s_artifact_layout_layout_model::LayoutError> {
    if !doc.pages.iter().any(|page| page.id == page_id) {
        return Err(semio_s_artifact_layout_layout_model::LayoutError::PageNotFound(page_id.into()));
    }
    let commit = headless_batch_export(LayoutExportKind::Png, doc, Some(page_id), None).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)?;
    decode_base64(&commit.data).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)
}

#[cfg(test)]
pub fn export_package_zip_headless_batch(doc_json: &str, preflight_json: &str) -> Result<Vec<u8>, semio_s_artifact_layout_layout_model::LayoutError> {
    let snapshot: LayoutSnapshot = semio_framework_pack_json::from_json_str(doc_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
    let commit = headless_batch_export(LayoutExportKind::Package, &snapshot, None, Some(preflight_json)).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)?;
    decode_base64(&commit.data).map_err(semio_s_artifact_layout_layout_model::LayoutError::Svg)
}
//#endregion 🧪️TestOracle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
