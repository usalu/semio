//! 📜 Reasoning wires app — textual document grammar surface + laws (constitutional: dsl).
//!
//! The `.wires` textual DSL and op-text grammar are declared, not hand-rolled — see the
//! `#[derive(dsl::DslDocument)]` on `MindmapWiresDocument` (in `reasoning_wires`) and
//! `#[derive(dsl::DslOps)]` on `MindmapWiresOperation` (in `reasoning_wires_op`). Both
//! `wires_fixture`/`board_fixture` (and the `Value`/`serde_json::Map<String, Value>` operation
//! payload fields) bind directly through `dsl`'s built-in `Shape::Value` escape hatch for
//! opaque/freeform JSON — no local mirror type or hand-rolled tokenizer needed.
//!
//! 🕸️ The unified `a:Kind@port->b@port` wire syntax (`dsl::Wire`/`Shape::Wire`) does NOT apply here:
//! edges live inside the opaque `board_fixture`/`wires_fixture` `Value` trees (plain JSON objects with
//! `source`/`target` string fields), not as typed Rust fields a `#[dsl(...)]` attribute could target —
//! that's the whole point of keeping this crate free of board-engine schema types. Introducing a
//! wire-literal encoding for those JSON edges would mean hand-rolling a bespoke sub-printer for one
//! field shape inside the generic `Shape::Value` escape hatch, and the same `source`/`target` JSON
//! shape is shared by every other generic board/map fixture in the repo (`reasoning.mindmap.fixture`,
//! tiled-map, puzzle boards, ...) — a structural, cross-crate schema change out of scope here.

use reasoning_wires::MindmapWiresDocument;

/// 📄 The `metabolism` example, handcrafted in the `.wires` DSL — source of truth for every
/// "metabolism" example call site (`setActiveExample`, `.example` manifest registration, tests).
pub const REASONING_WIRES_EXAMPLE_METABOLISM_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/💡reasoning/🎛️app/🔌wires/⚡️implementation/🦀rust/📚example/🔌metabolism.wires");

/// 📖 Parses `.wires` DSL text into a `MindmapWiresDocument`.
pub fn parse_dsl(text: &str) -> Result<MindmapWiresDocument, store::TextError> {
    <MindmapWiresDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `MindmapWiresDocument` back to `.wires` DSL text.
pub fn print_dsl(document: &MindmapWiresDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsl_round_trip_empty_document() {
        let document = MindmapWiresDocument {
            wires_fixture: serde_json::json!({
                "schema": reasoning_wires::MINDMAP_WIRES_SCHEMA,
                "identities": [],
                "relationships": [],
                "board": {
                    "schema": reasoning_wires::MINDMAP_BOARD_SCHEMA,
                    "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
                    "nodes": [],
                    "edges": [],
                    "wires": []
                }
            }),
            board_fixture: serde_json::json!({
                "schema": reasoning_wires::MINDMAP_BOARD_SCHEMA,
                "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
                "nodes": [],
                "edges": [],
                "wires": []
            }),
        };
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn dsl_round_trip_metabolism_fixture() {
        let document = parse_dsl(REASONING_WIRES_EXAMPLE_METABOLISM_TEXT).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(document.wires_fixture["identities"].as_array().unwrap().len(), 7);
        assert_eq!(document.wires_fixture["relationships"].as_array().unwrap().len(), 9);
        assert_eq!(document.board_fixture["nodes"].as_array().unwrap().len(), 7);
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪Tests
