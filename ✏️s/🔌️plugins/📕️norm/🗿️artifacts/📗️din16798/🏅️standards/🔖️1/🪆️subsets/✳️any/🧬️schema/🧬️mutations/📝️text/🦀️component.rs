//! ⚡️ DIN EN 16798 app — operation type + laws (constitutional: op).
//!
//! 📌️ DIN EN 16798 has no bespoke operation enum: every session mutation is a whole-document
//! replace, so `Mutation` is a re-export of `norm_core`'s generic `SetArtifactMutation<Din16798Snapshot>`,
//! which already carries its own `Mutation`/`OpText`/`OpBinary` impls — nothing to implement here.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::din16798::Din16798Snapshot;

pub use crate::artifacts::din16798::schema::mutations::Din16798Mutation;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_snapshot_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&Din16798Mutation::SetSnapshot { snapshot: Din16798Snapshot::default() });
    }
}
//#endregion 🧪️Tests
