//! ⚡️ DIN 4108 app — operation type + laws (constitutional: op).
//!
//! 📌️ DIN 4108 has no bespoke operation enum: every session mutation is a whole-document replace,
//! so `Mutation` is a re-export of `norm_core`'s generic `SetDocumentMutation<Din4108Snapshot>`, which
//! already carries its own `Mutation`/`OpText`/`OpBinary` impls — nothing to implement here.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::din4108::Din4108Snapshot;

pub use crate::artifacts::din4108::mutations::Din4108Mutation;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_snapshot_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&Din4108Mutation::SetSnapshot { snapshot: Din4108Snapshot::default() });
    }
}
//#endregion 🧪️Tests
