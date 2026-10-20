//! 🔣️ Layout export JSON: the bounded JSON validation and typed JSON writing cursors of the layout export job.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use semio_s_artifact_layout_layout_model::{Frame, GridSettings, LayoutBounds, Page, PageOverride};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_action_bus::RetainedToolWireInput;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::{Generation, InteractiveStage, RevisionId};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, Operation, StepContext};
use semio_framework_plugin::app::{ArtifactDownloadOutput, ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactReservedToolJob, ArtifactSnapshotCloseLease, ArtifactToolCompletion};
use semio_framework_plugin::{ArtifactReservedJob, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, EphemeralEmit, Fault, MediaClass, MediaForm, MediaType};
use semio_framework_io_ports::{VectorNode, VectorPoint2, VectorPoint3, VectorQuaternion, VectorRgba, VectorScene, VectorSegment, VectorTransform};
use std::sync::Arc;
use crate::dictionary::DictionaryJsonCursor;

#[path = "🧾️limits-and-string-sources/🦀️.rs"]
mod limits_and_string_sources;
pub use limits_and_string_sources::{MAX_LAYOUT_EXPORT_DECODED_ITEMS, MAX_LAYOUT_EXPORT_JSON_NODES, MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES, MAX_LAYOUT_EXPORT_STRING_BYTES, StringSource};
use limits_and_string_sources::{JSON_OUTPUT_BYTES_PER_UNIT, JsonStringWriteCursor, RecordOwner, StringArraySource, TopArrayKind};

#[path = "🌳️typed-json-nodes-and-scalars/🦀️.rs"]
mod typed_json_nodes_and_scalars;
pub use typed_json_nodes_and_scalars::{JsonScalar, TypedJsonCursor, TypedJsonNode, finite_json_number, typed_json_node_owned_bytes};

#[path = "🏗️cursor-construction-and-owner-pages/🦀️.rs"]
mod cursor_construction_and_owner_pages;
pub use cursor_construction_and_owner_pages::frame_bounds;

#[path = "📖️expand-document/🦀️.rs"]
mod expand_document;

macro_rules! integer_json_scalar {
    ($($scalar:ty),* $(,)?) => {
        $(impl JsonScalar for $scalar {
            fn json_bytes(&self) -> Result<Vec<u8>, String> {
                Ok(self.to_string().into_bytes())
            }
        })*
    };
}

integer_json_scalar!(u8, u32, u64, usize);

//#region 🪢️TaxonomyMounts
#[path = "../🧾️dictionary/🦀️.rs"]
pub mod dictionary;
#[path = "✅️json-validation/🦀️.rs"]
mod json_validation;
pub use json_validation::JsonValidationCursor;
//#endregion 🪢️TaxonomyMounts

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
