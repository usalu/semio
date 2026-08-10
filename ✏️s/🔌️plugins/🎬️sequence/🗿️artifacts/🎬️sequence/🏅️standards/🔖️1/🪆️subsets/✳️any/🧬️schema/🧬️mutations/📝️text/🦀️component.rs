//! 🔧 sequence artifact — OpText/OpBinary bridge for `SequenceMutation`.

pub use crate::artifacts::sequence::schema::mutations::{apply_sequence_mutation, inverse_sequence_mutation, sequence_snapshot_mutations, SequenceMutation, SequenceEnvelope, SequenceStore};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

