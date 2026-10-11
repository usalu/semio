//! ✒️ Writer semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf.

use crate::{WriterDiff, WriterSnapshot};
use serde::{Deserialize, Serialize};

pub use super::change_language::{change_language, ChangeLanguage};
pub use super::change_uri::{change_uri, ChangeUri};
pub use super::edit_text::{edit_text, EditText};
pub use super::splice_text::{splice_text, SpliceText};
pub use super::rename_writer::{rename_writer, RenameWriter};
pub use crate::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Semantic Writer document mutation vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = WriterSnapshot, diff = WriterDiff, schema = "writer.writer")]
pub enum WriterMutation {
    RenameWriter(RenameWriter),
    ChangeUri(ChangeUri),
    ChangeLanguage(ChangeLanguage),
    EditText(EditText),
    SpliceText(SpliceText),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
