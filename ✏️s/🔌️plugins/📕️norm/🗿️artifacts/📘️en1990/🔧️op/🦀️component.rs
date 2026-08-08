//! ⚡️ EN 1990 basis of structural design — operation enum + laws (constitutional: op).
//!
//! 🧩️ Every norm family document shares the same one-shot whole-document replacement operation
//! (`crate::document::SetDocumentMutation<D>`, with its `Mutation`/`MutationDiff`/`OpText`/`OpBinary`
//! impls already generic over any `D: DocumentDsl + DocumentPack` — see `norm_core`'s `🔖️OpText`
//! region) so this slot only needs to bind that generic operation to `crate::artifacts::en1990::En1990Snapshot`.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1990::En1990Snapshot;

pub use crate::artifacts::en1990::mutations::En1990Mutation;

