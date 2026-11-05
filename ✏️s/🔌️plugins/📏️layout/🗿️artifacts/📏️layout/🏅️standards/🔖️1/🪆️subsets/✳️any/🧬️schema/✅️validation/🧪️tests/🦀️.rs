//! 🧪️ Retained Layout validation and original-reference laws through public model owners.
#![cfg(test)]
use semio_s_artifact_layout_layout_model::{snapshot,validation,Frame,LayoutRect,TextStyleRun,DrawingSourceAddress,LayoutDrawingMember};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_s_artifact_layout_layout_model::validation::{graph::*,retained::LayoutValidationCursor};
use semio_s_artifact_layout_layout_model::drawing_member::source::reference::LAYOUT_DRAWING_REFERENCE_LENDER;
use semio_plugin::app::child_read::ChildReadReferenceLenderV1;
use semio_framework_value::{RetirementDemand,retained_clone::RetainedCloneGrant};
#[path="../🧳️retained/🧪️tests.rs"]
mod retained;
#[path="../👁️graph/🧪️tests.rs"]
mod graph;
#[path="../../🪆️drawing/📖️source/🧷️reference/🧪️tests.rs"]
mod reference;
use semio_s_artifact_layout_layout_model::validation::{validate_document,graph::bindings::FinalGraphSourceBindings};
use semio_framework_value::retained_clone::{RetainedCloneProgress,retained_clone_grant_admits};
#[path="../👁️graph/🔗️bindings/🧪️tests.rs"]
mod bindings;
#[path="."]
mod reference_scope {
 use super::*;
 #[path="../🧳️retained/🪆️references/🧪️tests/🦀️.rs"]
 mod cases;
}
#[path="../🧳️retained/📦️owned/🧪️tests/🦀️.rs"]
mod owned_graph;
