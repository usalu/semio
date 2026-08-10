//! ⚡️ EN 1997 artifact — the operation type + its laws.
//!
//! 🧩️ EN family artifacts carry no bespoke operation enum: the sole mutation is a whole-document
//! replace, already generically implemented as `crate::document::En1997Mutation<D>` (its
//! `OpText`/`OpBinary` impls are blanket ones bounded on `D: En1997SnapshotDsl`/`En1997SnapshotPack`, satisfied
//! for free by this artifact's `#[derive(dsl::DslRecord)]`). The `NormFamily` binding that ties
//! `En1997Snapshot` to `evaluate` lives in `⚙️engine`, next to the compute it names.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1997::En1997Snapshot;

pub use crate::artifacts::en1997::schema::mutations::En1997Mutation;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_snapshot_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&En1997Mutation::SetSnapshot { snapshot: En1997Snapshot::default() });
    }
}
//#endregion 🧪️Tests
