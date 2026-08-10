//! ⚡️ EN 1991 actions on structures — operation enum + laws (constitutional: op).
//!
//! 🧩️ Every norm family document shares the same one-shot whole-document replacement operation
//! (`crate::document::SetDocumentMutation<D>`) so this slot only needs to bind that generic operation to
//! `crate::artifacts::en1991::En1991Snapshot`.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1991::En1991Snapshot;

pub use crate::artifacts::en1991::schema::mutations::En1991Mutation;

