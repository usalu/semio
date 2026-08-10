//! 🔧 layout artifact — OpText/OpBinary bridge for `LayoutMutation`.

pub use crate::artifacts::layout::mutations::{apply_frame_patch, apply_layout_mutation, inverse_layout_mutation, LayoutMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

