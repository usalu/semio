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

#[path = "📜️export-contract-and-state/🦀️.rs"]
mod export_contract_and_state;
pub use export_contract_and_state::{LAYOUT_EXPORT_PAYLOAD_SCHEMA, LAYOUT_EXPORT_TOOL_IDS, LAYOUT_MEDIA_EXPORT_PAYLOAD_SCHEMA, LAYOUT_MEDIA_EXPORT_SCHEMA, LAYOUT_MEDIA_EXPORT_TOOL_ID, LayoutExportCheckpoint, LayoutExportKind, LayoutExportPublication, LayoutExportPublicationKind, LayoutExportRequest, LayoutExportTurn, LayoutPublicationSlot, MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES, MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES, MAX_LAYOUT_EXPORT_DIMENSION, MAX_LAYOUT_EXPORT_FILES, MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE, MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE, MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE, MAX_LAYOUT_EXPORT_LINKS, MAX_LAYOUT_EXPORT_OUTPUT_BYTES, MAX_LAYOUT_EXPORT_PAGES, MAX_LAYOUT_EXPORT_PARENT_PAGES, MAX_LAYOUT_EXPORT_PIXELS, MAX_LAYOUT_EXPORT_RAW_BYTES, MAX_LAYOUT_EXPORT_SPREADS, MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS, MAX_LAYOUT_EXPORT_STORIES, MAX_LAYOUT_EXPORT_STYLES, MAX_LAYOUT_EXPORT_TOTAL_FRAMES};
#[cfg(test)]
pub use export_contract_and_state::LayoutExportCommit;
use export_contract_and_state::{BASE64_INPUT_BYTES_PER_UNIT, ChunkRope, ExportRect, ExportStage, LAYOUT_EXPORT_CHECKPOINT_MAGIC, LAYOUT_EXPORT_CLOSE_TURN_BYTES, LayoutCloseTurn, LayoutExportCloseStage, MAX_LAYOUT_EXPORT_AUTHORITY_BYTES, OUTPUT_CHUNK_BYTES, PNG_PIXELS_PER_UNIT, PackageSection, PdfItem, PdfPagePlan, PdfRaster, PdfSection, PngRowSection, ZipEntry, ZipState, layout_close_step};
#[cfg(test)]
use export_contract_and_state::MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS;

#[path = "🏗️job-admission-and-close/🦀️.rs"]
mod job_admission_and_close;
pub use job_admission_and_close::LayoutExportJob;

#[path = "✅️validate-plan-and-svg/🦀️.rs"]
mod validate_plan_and_svg;
pub use validate_plan_and_svg::bounded_id;
use validate_plan_and_svg::finite_dimension;

#[path = "📕️pdf-png-and-package-encoding/🦀️.rs"]
mod pdf_png_and_package_encoding;

#[path = "⏭️advance-commit-and-codec/🦀️.rs"]
mod advance_commit_and_codec;
pub use advance_commit_and_codec::{output_name, sanitize_filename};
#[cfg(test)]
pub use advance_commit_and_codec::{export_document_pdf_headless_batch, export_document_png_headless_batch, export_document_svg_headless_batch, export_package_zip_headless_batch, run_layout_export_headless_batch};
use advance_commit_and_codec::{crc32_update, decode_base64, encode_checkpoint, push_u16, push_u32};
#[cfg(test)]
use advance_commit_and_codec::{decode_checkpoint, headless_batch_export};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
