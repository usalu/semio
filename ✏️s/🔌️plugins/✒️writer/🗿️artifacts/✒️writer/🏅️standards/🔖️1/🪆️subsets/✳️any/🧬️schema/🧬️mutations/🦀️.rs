//! ✒️ Writer semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf.

use crate::{WriterDiff, WriterSnapshot};
use serde::{Deserialize, Serialize};

pub use super::change_language::{change_language, ChangeLanguage};
pub use super::change_uri::{change_uri, ChangeUri};
pub use super::edit_text::{edit_text, EditText};
pub use super::rename_writer::{rename_writer, RenameWriter};
pub use crate::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Semantic Writer document mutation vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations, dsl::ToValue, dsl::FromValue)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = WriterSnapshot, diff = WriterDiff, schema = "writer.writer")]
pub enum WriterMutation {
    RenameWriter(RenameWriter),
    ChangeUri(ChangeUri),
    ChangeLanguage(ChangeLanguage),
    EditText(EditText),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
