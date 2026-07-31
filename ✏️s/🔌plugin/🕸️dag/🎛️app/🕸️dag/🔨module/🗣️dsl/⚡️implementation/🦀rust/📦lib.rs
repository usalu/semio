//! 📜 DAG app — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::DocumentDsl for DagDocument` is implemented directly in the DAG kernel crate
//! (`infinite_board_port_directed_dag`, see `s/plugin/dag/app/rs/lib.rs` for why); this crate only adds
//! the thin app-facing `parse_dsl`/`print_dsl` wrappers plus the canonical example-fixture constant and
//! its round-trip law.

use dag::DagDocument;

/// 📄 The canonical DAG fixture, handcrafted in the `.dag` DSL — the same file the DAG kernel's own
/// tests parse via `include_str!("../../../../../../../../../🧰framework/🛍️product/💻os/🔨module/♾️infinite/🎲board/🔌port/➡️directed/🕸️dag/⚡️implementation/🦀rust/📚example/🕸️demo.dag")`.
pub const DAG_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../🧰framework/🛍️product/💻os/🔨module/♾️infinite/🎲board/🔌port/➡️directed/🕸️dag/⚡️implementation/🦀rust/📚example/🕸️demo.dag");

/// 📖 Parses `.dag` DSL text into a `DagDocument`.
pub fn parse_dsl(text: &str) -> Result<DagDocument, store::TextError> {
    <DagDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `DagDocument` back to `.dag` DSL text.
pub fn print_dsl(document: &DagDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_fixture_dsl_round_trips() {
        let document = parse_dsl(DAG_EXAMPLE_TEXT).expect("parse default fixture");
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪Tests
