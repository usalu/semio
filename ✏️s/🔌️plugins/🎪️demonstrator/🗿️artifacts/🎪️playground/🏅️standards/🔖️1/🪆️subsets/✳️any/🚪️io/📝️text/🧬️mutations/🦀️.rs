//! 📝️ Playground mutation text framing and direct-owner registry.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this mutation facet.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::mutations::PlaygroundMutation;

/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[("ChangeSchema", crate::standards::v1::subsets::any::schema::mutations::change_schema::TEXT_OPCODE)];

#[path = "✒️change-schema/🦀️.rs"]
pub mod change_schema;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
