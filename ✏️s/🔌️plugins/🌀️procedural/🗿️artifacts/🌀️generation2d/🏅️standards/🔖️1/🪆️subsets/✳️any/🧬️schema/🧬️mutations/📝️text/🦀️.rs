//! ⚡️ Generation2d artifact — OpText/OpBinary codecs + grammar for `Generation2dMutation`.
//!
//! Wire codecs live in `📡️spr` (DSL mirror); this facet keeps grammar + re-exports.

pub use crate::artifacts::generation2d::schema::mutations::{apply_generation2d_mutation, generation_mutation_to_generation2d, inverse_generation2d_mutation, generation2d_fixture_operations, replace_widget, Generation2dMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar
