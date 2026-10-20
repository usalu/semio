//! 📜️ Export contract and state of the layout export jobs: the tool ids, payload schemas and size caps, the export kinds and requests, the chunk rope and zip state, the export stages and close ladder, the publication and its slot. See [`super`].

use super::*;

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
pub const OUTPUT_CHUNK_BYTES: usize = ArtifactOutputChunks::CHUNK_BYTES;

pub const MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS: usize = MAX_LAYOUT_EXPORT_OUTPUT_BYTES.div_ceil(OUTPUT_CHUNK_BYTES);

pub(super) const BASE64_INPUT_BYTES_PER_UNIT: usize = 3_072;

pub(super) const PNG_PIXELS_PER_UNIT: u32 = 256;

pub(super) const MAX_LAYOUT_EXPORT_AUTHORITY_BYTES: usize = 256;

pub(super) const LAYOUT_EXPORT_CHECKPOINT_MAGIC: &[u8; 4] = b"LXC2";

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

    pub fn binary(self) -> bool {
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

#[cfg(any(test, feature = "testkit"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutExportCommit {
    pub filename: String,
    pub mime_type: String,
    pub encoding: Option<String>,
    pub data: String,
}

#[cfg(any(test, feature = "testkit"))]
impl LayoutExportCommit {
    pub fn from_output(kind: LayoutExportKind, name: &str, bytes: Vec<u8>) -> Self {
        // SAFETY: every final route emits only ASCII: SVG's owned numeric vocabulary or the owned
        // base64 encoder. The invariant is enforced before bytes enter the final commit cursor.
        let data = unsafe { String::from_utf8_unchecked(bytes) };
        Self { filename: format!("{}.{}", sanitize_filename(name), kind.extension()), mime_type: kind.mime_type().into(), encoding: kind.binary().then(|| "base64".into()), data }
    }

    pub fn from_chunks(kind: LayoutExportKind, name: &str, chunks: &ArtifactOutputChunks) -> Result<Self, String> {
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
pub struct ChunkRope {
    pub chunks: std::collections::VecDeque<Vec<u8>>,
    pub len: usize,
    pub digest: u64,
    pub front_byte_cursor: usize,
}

impl ChunkRope {
    pub fn close_take_chunk(&mut self, maximum_bytes: usize) -> Option<Result<usize, ()>> {
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
    pub fn new() -> Self {
        Self { chunks: std::collections::VecDeque::with_capacity(MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS), len: 0, digest: 0xcbf29ce484222325, front_byte_cursor: 0 }
    }

    pub fn append(&mut self, mut bytes: &[u8]) -> Result<(), String> {
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

    pub fn take_chunk(&mut self) -> Option<Vec<u8>> {
        if self.front_byte_cursor != 0 {
            return None;
        }
        self.chunks.pop_front()
    }

    pub fn take_prefix(&mut self, maximum: usize) -> Vec<u8> {
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
pub struct ExportRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub rgba: [u8; 4],
}

#[derive(Clone, Debug)]
pub struct ZipEntry {
    pub name: String,
    pub crc: u32,
    pub size: u32,
    pub offset: u32,
}

#[derive(Clone, Debug, Default)]
pub struct ZipState {
    pub entries: Vec<ZipEntry>,
    pub current_name: Option<String>,
    pub current_crc: u32,
    pub current_size: u32,
    pub current_offset: u32,
}

//#endregion 🧱️Storage

//#region 🧩️Job
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ExportStage {
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
pub(super) enum PdfSection {
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
pub(super) struct PdfRaster {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) rgb: Vec<u8>,
}

/// 📕️ One painted element of a page's content stream, in page (top-left, y-down) coordinates.
#[derive(Clone, Debug)]
pub(super) enum PdfItem {
    Stroke { points: Vec<(f32, f32)>, closed: bool, color: [f32; 4], width: f32, fill: Option<[f32; 4]> },
    Rect { x: f32, y: f32, width: f32, height: f32, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]> },
    Glyph { x: f32, y: f32, size: f32, glyph_id: u16, color: [f32; 4], italic: bool },
    Image { x: f32, y: f32, width: f32, height: f32, rotation: f32, placeholder: bool, mark: String, raster: Option<PdfRaster> },
}

/// 📕️ One exported page: its media box and how many `pdf_items` it owns (items are contiguous in
/// page order, so the page's slice starts where the previous page's ended).
#[derive(Clone, Copy, Debug)]
pub(super) struct PdfPagePlan {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) items: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PngRowSection {
    Initialize,
    Fill,
    Header,
    Data,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PackageSection {
    Begin,
    Scalars,
    Preflight,
    Manifest,
    CentralDirectory,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutExportCloseStage {
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
pub(super) enum LayoutCloseTurn {
    Pending { released_items: usize, released_bytes: usize },
    Granted(semio_framework_value::retained_clone::RetainedCloneProgress),
    AwaitingInput { reason: &'static str },
    Blocked { reason: &'static str },
    Complete,
}

/// 🔔️ The close budget one staged turn is quoted for; stages release at most one output chunk per turn.
pub(super) const LAYOUT_EXPORT_CLOSE_TURN_BYTES: usize = ArtifactOutputChunks::CHUNK_BYTES;

pub(super) fn layout_close_step(turn:Result<LayoutCloseTurn,Fault>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
    Ok(match turn?{
        LayoutCloseTurn::Pending{released_items,released_bytes}=>InteractiveJobCloseStep::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()}},
        LayoutCloseTurn::Granted(progress)=>InteractiveJobCloseStep::Pending{progress},
        LayoutCloseTurn::Complete=>InteractiveJobCloseStep::Complete{progress:Default::default()},
        LayoutCloseTurn::AwaitingInput{..}|LayoutCloseTurn::Blocked{..}=>InteractiveJobCloseStep::Blocked,
    })
}

pub type LayoutExportTurn<'a> = Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure>;

pub struct LayoutExportPublication {
    pub(super) kind: LayoutExportPublicationKind,
    pub(super) bytes: [u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES],
    pub(super) length: usize,
    pub(super) publication: RetainedJobPublication,
}

impl LayoutExportPublication {
    pub fn new(kind: LayoutExportPublicationKind, bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES {
            return Err("layout-export-publication-limit".into());
        }
        let mut storage = [0; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES];
        storage[..bytes.len()].copy_from_slice(bytes);
        Ok(Self { kind, bytes: storage, length: bytes.len(), publication: RetainedJobPublication::new() })
    }

    pub fn advance<'a>(&'a mut self, context: &mut StepContext<'_>) -> LayoutExportTurn<'a> {
        let kind = match self.kind {
            LayoutExportPublicationKind::Preview => JobPublicationKind::Preview,
            LayoutExportPublicationKind::Checkpoint { applied_progress } => JobPublicationKind::Checkpoint { applied_progress },
            LayoutExportPublicationKind::Fault => JobPublicationKind::Fault,
            LayoutExportPublicationKind::Commit => return JobOutcomeBorrow::admit_complete(context, None, None).map_err(Into::into),
        };
        self.publication.advance_from_source(kind, &self.bytes[..self.length], context).map_err(Into::into)
    }

    pub fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.publication.borrow_outcome(descriptor)
    }

    pub fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.publication.close_step(grant)
    }

    pub fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.publication.retirement_demands()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.publication.terminal_is_empty()
    }
}

/// 📮️ Holds the one scheduled publication of a job and closes a delivered one before the job resumes.
#[derive(Default)]
pub struct LayoutPublicationSlot {
    pub publication: Option<LayoutExportPublication>,
    pub delivered: bool,
}

impl LayoutPublicationSlot {
    /// ♻️ Closes a delivered publication under the step's grant; `true` once the slot is free for the next one.
    pub fn settle(&mut self, context: &mut StepContext<'_>) -> Result<bool, semio_framework_value::ValueError> {
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

    pub fn advance<'a>(&'a mut self, context: &mut StepContext<'_>) -> LayoutExportTurn<'a> {
        let Some(publication) = self.publication.as_mut() else {
            return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);
        };
        let outcome = publication.advance(context)?;
        if outcome.is_some() {
            self.delivered = true;
        }
        Ok(outcome)
    }

    pub fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Complete => descriptor.complete(None, None),
            _ => self.publication.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "layout export outcome lost its publication"))?.borrow_outcome(descriptor),
        }
    }

    pub fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
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

    pub fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.publication.as_ref().map_or(Ok(Default::default()), LayoutExportPublication::retirement_demands)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.publication.as_ref().is_none_or(LayoutExportPublication::terminal_is_empty)
    }
}
//#endregion 🧩️Job
