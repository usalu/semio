//! ⚡️ Procedural3d artifact — OpText/OpBinary codecs + grammar for `Procedural3dMutation`.
//!
//! Wire codecs live in `📡️spr` (DSL mirror); this facet keeps grammar + re-exports.

pub use crate::artifacts::procedural3d::schema::mutations::{apply_procedural3d_mutation, generation_mutation_to_procedural3d, inverse_procedural3d_mutation, procedural3d_fixture_operations, Procedural3dMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
