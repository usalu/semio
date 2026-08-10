//! ⚡️ DIN V 18599 app — operation type + laws (constitutional: op).
//!
//! 📌️ DIN V 18599 has no bespoke operation enum: every session mutation is a whole-document
//! replace, so `Mutation` is a re-export of `norm_core`'s generic `SetArtifactMutation<Din18599Snapshot>`,
//! which already carries its own `Mutation`/`OpText`/`OpBinary` impls — nothing to implement here.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::din18599::Din18599Snapshot;

pub use crate::artifacts::din18599::schema::mutations::Din18599Mutation;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_snapshot_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&Din18599Mutation::SetSnapshot { snapshot: Din18599Snapshot::default() });
    }
}
//#endregion 🧪️Tests
