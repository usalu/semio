//! 📖️ Inference text grammar: inference values are computed from a snapshot and never authored as DSL text, so this facet declares
//! the grammar only.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak.
pub type ModelInferenceText = String;
