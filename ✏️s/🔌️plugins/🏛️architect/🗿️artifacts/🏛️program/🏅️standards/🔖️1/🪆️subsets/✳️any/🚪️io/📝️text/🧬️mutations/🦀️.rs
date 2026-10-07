//! ⚡️ Architect program artifact — OpText/OpBinary codecs + grammar for serializing `ProgramMutation`.
//! Mutation apply/inverse live in `🧬️mutations`.

use crate::schema::mutations::ProgramMutation;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// 📝️ Compact JSON-line OpText for `ProgramMutation` (collection wrappers block DslEnum).
impl protocol::OpText for ProgramMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line.trim(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("invalid program mutation: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
}

//#region 🏷️WireTags
/// 🏷️ `ProgramMutation`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.

//#endregion 🏷️WireTags

/// 🌱️ Binary twin of the OpText escape hatch — plain JSON bytes.


//#endregion 🔖️HandcraftedOpCodecs

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 📥️ Decodes the internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) projection the
/// committed `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` vectors carry.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_program_mutation_json(text: &str) -> Result<ProgramMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
