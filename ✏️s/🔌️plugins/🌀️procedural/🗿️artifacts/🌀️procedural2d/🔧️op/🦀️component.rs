//! ⚡️ Procedural2d artifact — OpText/OpBinary codecs + grammar for `Procedural2dMutation`.
//!
//! Wire codecs live in `📡️spr` (DSL mirror); this facet keeps grammar + re-exports.

pub use crate::artifacts::procedural2d::mutations::{
    apply_procedural2d_mutation, inverse_procedural2d_mutation, procedural2d_fixture_operations, Procedural2dMutation,
};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
