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
pub use export_contract_and_state::*;

#[path = "🏗️job-admission-and-close/🦀️.rs"]
mod job_admission_and_close;
pub use job_admission_and_close::*;

#[path = "✅️validate-plan-and-svg/🦀️.rs"]
mod validate_plan_and_svg;
pub use validate_plan_and_svg::*;

#[path = "📕️pdf-png-and-package-encoding/🦀️.rs"]
mod pdf_png_and_package_encoding;

#[path = "⏭️advance-commit-and-codec/🦀️.rs"]
mod advance_commit_and_codec;
pub use advance_commit_and_codec::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
