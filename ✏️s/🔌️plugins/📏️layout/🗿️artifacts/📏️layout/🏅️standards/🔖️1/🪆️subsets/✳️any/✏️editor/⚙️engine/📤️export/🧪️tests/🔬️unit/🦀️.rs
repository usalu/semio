//! 🧪️ Companion white-box laws of `semio-s-artifact-layout-layout-engine-export`: the export jobs driven on an original caller, the factory dispatch of the layout editor, the caps, the close ladder and the pdf laws.

#![cfg(test)]
#![allow(unused_imports, unused_qualifications)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_framework_io_ports::testkit::{bind, scene_content};
use semio_framework_job::{Generation, InteractiveJob, InteractiveJobCloseStep, Operation, OriginalJobFixtureCaller, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RevisionId};
use semio_framework_plugin::app::{ArtifactOutputChunks, ArtifactReservedToolJob};
use semio_s_artifact_layout_layout_editor::{LayoutExportJobFactory, LayoutExportToolPayload, LayoutMediaExportJobFactory};
use semio_s_artifact_layout_layout_engine_export::testkit::*;
use semio_s_artifact_layout_layout_engine_export::*;
use semio_s_artifact_layout_layout_engine_json::{JsonValidationCursor, MAX_LAYOUT_EXPORT_JSON_NODES, MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES, MAX_LAYOUT_EXPORT_STRING_BYTES, TypedJsonCursor, TypedJsonNode};
use semio_s_artifact_layout_layout_model::PageOverride;
use std::sync::Arc;

#[global_allocator]
static LAYOUT_EXPORT_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

include!("📕️pdf-structure/🦀️.rs");
include!("🎟️harness-pdf-and-dispatch-laws/🦀️.rs");
include!("🚧️caps-json-checkpoint-pdf-style/🦀️.rs");
include!("✒️pdf-drawing-laws/🦀️.rs");
