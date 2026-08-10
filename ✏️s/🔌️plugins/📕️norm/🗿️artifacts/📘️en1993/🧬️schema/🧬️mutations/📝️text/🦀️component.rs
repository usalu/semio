//! ⚡️ EN 1993 design of steel structures — operation enum + laws (constitutional: op).
//!
//! 🧩️ Every norm family document shares the same one-shot whole-document replacement operation
//! (`crate::document::SetDocumentMutation<D>`) so this slot only needs to bind that generic operation to
//! `crate::artifacts::en1993::En1993Snapshot`.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1993::En1993Snapshot;

pub use crate::artifacts::en1993::schema::mutations::En1993Mutation;

