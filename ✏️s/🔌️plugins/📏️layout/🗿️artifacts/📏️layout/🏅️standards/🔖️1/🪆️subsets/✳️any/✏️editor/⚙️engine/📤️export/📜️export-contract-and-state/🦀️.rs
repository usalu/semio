//! 📜️ Export contract and state of the layout export jobs: the tool ids, payload schemas and size caps, the export kinds and requests, the chunk rope and zip state, the export stages and close ladder, the publication and its slot. See [`super`].

use super::*;

//#region 🔖️Contract
pub const LAYOUT_EXPORT_TOOL_IDS: &[&str] = &["exportPng", "exportSvg", "exportPdf", "exportPackage", "exportStep", "exportIdml"];

pub const LAYOUT_EXPORT_PAYLOAD_SCHEMA: &str = "layout.layout.tool-command.v1";

pub const LAYOUT_MEDIA_EXPORT_PAYLOAD_SCHEMA: &str = "layout.layout.media-export.v1";

pub const LAYOUT_MEDIA_EXPORT_TOOL_ID: &str = "export-media:layout:out";

pub const LAYOUT_MEDIA_EXPORT_SCHEMA: &str = "2d.layout";

pub const MAX_LAYOUT_EXPORT_RAW_BYTES: usize = 2 << 20;

pub const MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES: usize = 4_096;

pub const MAX_LAYOUT_EXPORT_PAGES: usize = 64;

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

pub const MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES: usize = 642;

/// 📤️ Derived, never a literal: the exact per-chunk cap [`ArtifactOutputChunks::push`] enforces. A
/// literal that drifts from it turns a correct producer into a runtime fault on the wire.
pub const OUTPUT_CHUNK_BYTES: usize = ArtifactOutputChunks::CHUNK_BYTES;

pub const MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS: usize = MAX_LAYOUT_EXPORT_OUTPUT_BYTES.div_ceil(OUTPUT_CHUNK_BYTES);

pub(super) const BASE64_INPUT_BYTES_PER_UNIT: usize = 3_072;

pub(super) const MAX_LAYOUT_EXPORT_AUTHORITY_BYTES: usize = 256;

pub(super) const LAYOUT_EXPORT_CHECKPOINT_MAGIC: &[u8; 4] = b"LXC3";

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
    Step,
    Idml,
}

impl LayoutExportKind {
    pub fn tool_id(self) -> &'static str {
        match self {
            Self::Png => "exportPng",
            Self::Svg => "exportSvg",
            Self::Pdf => "exportPdf",
            Self::Package => "exportPackage",
            Self::Step => "exportStep",
            Self::Idml => "exportIdml",
        }
    }

    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Svg => "image/svg+xml",
            Self::Pdf => "application/pdf",
            Self::Package => "application/zip",
            Self::Step => "model/step",
            Self::Idml => "application/vnd.adobe.indesign-idml-package",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Svg => "svg",
            Self::Pdf => "pdf",
            Self::Package => "layout-package.zip",
            Self::Step => "step",
            Self::Idml => "idml",
        }
    }

    pub fn binary(self) -> bool {
        !matches!(self, Self::Svg | Self::Step)
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
    pub work_units: u64,
    pub committed_units: u64,
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
pub use super::output::ChunkRope;

pub use super::pdf_png_and_package_encoding::zip::{ZipEntry,ZipState,PackageMember};

//#endregion 🧱️Storage

//#region 🧩️Job
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(super) enum ExportStage {
    Validate,
    Plan,
    BeginEncode,
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
    PageResources,
    PageGraphicsStates,
    ContentBegin,
    ContentItem,
    ContentEnd,
    ContentLength,
    ImageBegin,
    ImageRgb,
    ImageRgbEnd,
    ImageAlphaBegin,
    ImageAlpha,
    ImageAlphaEnd,
    UnicodeBegin,
    UnicodeRanges,
    UnicodeEnd,
    UnicodeLength,
    EncodingCmap,
    XrefBegin,
    XrefEntries,
    Trailer,
    Complete,
}

#[derive(Clone, Debug,semio_framework_value::RetireOwned)]
pub(super) struct PdfRaster {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) resource: Option<usize>,
    pub(super) rgba: Vec<u8>,
}

/// 📕️ One painted element of a page's content stream, in page (top-left, y-down) coordinates.
#[derive(Clone, Debug,semio_framework_value::RetireOwned)]
pub(super) enum PdfItem {
    DrawingPath { index: usize },
    DrawingOutline { label: usize, glyph: usize, paint: semio_s_artifact_layout_layout_engine::scene::drawing_planning::DrawingPaint, affine: [f64; 6] },
    Outline { source: usize, glyph: usize, color: [f32; 4], italic: bool, rotation: f32, center: [f32; 2] },
    Stroke { points: Vec<(f32, f32)>, closed: bool, color: [f32; 4], width: f32, fill: Option<[f32; 4]> },
    Rect { x: f64, y: f64, width: f64, height: f64, rotation: f64, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]> },
    Image { x: f64, y: f64, width: f64, height: f64, rotation: f64, affine: Option<[f64; 6]>, link: Option<usize>, placeholder: bool, mark: String, proxy: Option<String>, raster: Option<PdfRaster> },
}

/// 📕️ One exported page: its media box and how many `pdf_items` it owns (items are contiguous in
/// page order, so the page's slice starts where the previous page's ended).
#[derive(Clone, Copy, Debug,semio_framework_value::RetireOwned)]
pub(super) struct PdfPagePlan {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) items: usize,
    pub(super) first_item: usize,
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
    Output,
    Encoded,
    PngRow,
    PdfOffsets,
    PdfItems,
    PdfPages,
    Interchange,
    Fidelity,
    Drawing,
    Paragraphs,
    FramePlanning,
    ZipEntries,
    ZipCurrentMember,
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

#[derive(Default)]
pub(super) struct NestedValidationCursor {
    pub(super) started: bool,
    pub(super) group: u8,
    pub(super) index: usize,
    pub(super) layer: usize,
}

pub(super) enum FramePlanTurn { Pending, Frame(super::frame_planning::FramePlanAddress), Complete }
