//! 📝️ Playground mutation text framing and direct-owner registry.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this mutation facet.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::PlaygroundMutation;

/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[("ChangeSchema", super::change_schema::text::TEXT_OPCODE)];
