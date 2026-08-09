//! ⚡️ EN 1995 artifact — the operation type + its laws.
//!
//! 🧩️ EN family artifacts carry no bespoke operation enum: the sole mutation is a whole-document
//! replace, already generically implemented as `crate::document::En1995Mutation<D>` (its
//! `OpText`/`OpBinary` impls are blanket ones bounded on `D: En1995SnapshotDsl`/`En1995SnapshotPack`, satisfied
//! for free by this artifact's `#[derive(dsl::DslRecord)]`). The `NormFamily` binding that ties
//! `En1995Snapshot` to `evaluate` lives in `⚙️engine`, next to the compute it names.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1995::En1995Snapshot;

pub use crate::artifacts::en1995::mutations::En1995Mutation;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_snapshot_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::SetSnapshot { snapshot: En1995Snapshot::default() });
    }
}
//#endregion 🧪️Tests
