//! ⚡️ Generation3d artifact — OpText/OpBinary codecs + grammar for `Generation3dMutation`.
//!
//! Wire codecs live in `📡️spr` (DSL mirror); this facet keeps grammar + re-exports.

pub use crate::artifacts::generation3d::schema::mutations::{apply_generation3d_mutation, generation_mutation_to_generation3d, inverse_generation3d_mutation, generation3d_fixture_operations, Generation3dMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar
