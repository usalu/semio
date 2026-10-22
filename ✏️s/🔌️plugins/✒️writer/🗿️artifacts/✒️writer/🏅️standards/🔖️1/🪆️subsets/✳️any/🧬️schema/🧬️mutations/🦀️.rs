//! ✒️ Writer semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf.

use crate::{WriterDiff, WriterSnapshot};
use serde::{Deserialize, Serialize};

pub use crate::standards::v1::subsets::any::schema::mutations::change_language::ChangeLanguage;
pub use crate::standards::v1::subsets::any::schema::mutations::change_uri::ChangeUri;
pub use crate::standards::v1::subsets::any::schema::mutations::edit_text::EditText;
pub use crate::standards::v1::subsets::any::schema::mutations::splice_text::{splice_text, SpliceText};
pub use crate::standards::v1::subsets::any::schema::mutations::rename_writer::RenameWriter;
pub use crate::schema::operations::*;

#[path = "../../../../../../🌉️apply/🦀️.rs"]
pub mod central_apply;

//#region 🔖️Aggregate
/// 🧮️ Semantic Writer document mutation vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = WriterSnapshot, diff = WriterDiff, schema = "writer.writer", reference = WriterSnapshot, reference_diff = WriterDiff)]
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

//#region 🔖️ConcreteBuilders
/// 🏗️ Builder of the `change_language` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_language(new_language_id: String) -> WriterMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_language::change_language::<WriterMutation>(new_language_id)
}

/// 🏗️ Builder of the `change_uri` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_uri(new_uri: String) -> WriterMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_uri::change_uri::<WriterMutation>(new_uri)
}

/// 🏗️ Builder of the `edit_text` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn edit_text(text: String) -> WriterMutation {
    crate::standards::v1::subsets::any::schema::mutations::edit_text::edit_text::<WriterMutation>(text)
}

/// 🏗️ Builder of the `rename_writer` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn rename_writer(new_id: String) -> WriterMutation {
    crate::standards::v1::subsets::any::schema::mutations::rename_writer::rename_writer::<WriterMutation>(new_id)
}
//#endregion 🔖️ConcreteBuilders
