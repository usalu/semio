//! ⚡️ EN 1999 artifact — the operation type + its laws.
//!
//! 🧩️ EN family artifacts carry no bespoke operation enum: the sole mutation is a whole-document
//! replace, already generically implemented as `crate::document::En1999Mutation<D>` (its
//! `OpText`/`OpBinary` impls are blanket ones bounded on `D: En1999SnapshotDsl`/`En1999SnapshotPack`, satisfied
//! for free by this artifact's `#[derive(dsl::DslRecord)]`). The `NormFamily` binding that ties
//! `En1999Snapshot` to `evaluate` lives in `⚙️engine`, next to the compute it names.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1999::En1999Snapshot;

pub use crate::artifacts::en1999::mutations::En1999Mutation;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_snapshot_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&En1999Mutation::SetSnapshot { snapshot: En1999Snapshot::default() });
    }
}
//#endregion 🧪️Tests
