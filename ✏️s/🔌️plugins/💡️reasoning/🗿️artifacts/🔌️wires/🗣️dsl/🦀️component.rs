//! 📜️ Wires artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! The `.wires` textual DSL and op-text grammar are declared, not hand-rolled — see
//! `impl store::DocumentDsl for MindmapWiresDocument` (in `crate::artifacts::wires`, `🔖️Dsl` region) and
//! `#[derive(dsl::DslOps)]` on `MindmapWiresOperation` (in `crate::artifacts::wires::op`).
//! `MindmapWiresDocument` itself keeps `wires_fixture`/`board_fixture` as opaque `dsl::DslValue` (the
//! `op`/`ui`/`engine` code addresses board nodes/edges and wires relationships generically by id for
//! mergeable, granular JSON-patch edits), but the TEXTUAL `.wires` surface is fully typed via the
//! `WiresFixtureDsl`/`BoardFixtureDsl`/... local DSL-mirror twins declared in the artifact component's
//! `🔖️DslMirror` region — converted Value<->typed right at the `parse_dsl`/`print_dsl`/pack boundary,
//! same "local twin" pattern as `procedural_3d`'s `CameraJsonDsl`/`WidgetDsl`/`SynapseSpecDsl`.
//!
//! 🕸️ The unified `a:Kind@port->b@port` wire syntax (`dsl::Wire`/`Shape::Wire`) does NOT apply here:
//! `EdgeDsl::source`/`target` are plain `#[dsl(refs = "node")]` strings against `NodeDsl`'s
//! `#[dsl(defines = "node")]` id — not `dsl::Wire` — because the same bare `source`/`target` string-id
//! shape is shared by every other generic board/map fixture in the repo (`reasoning.mindmap.fixture`,
//! tiled-map, puzzle boards, ...); adopting `dsl::Wire` here alone, ahead of those siblings, would just
//! fork one shared shape into two incompatible encodings for no present benefit — a structural,
//! cross-crate schema change out of scope here.

use crate::artifacts::wires::MindmapWiresDocument;

/// 📄️ The `metabolism` example, handcrafted in the `.wires` DSL — source of truth for every
/// "metabolism" example call site (`setActiveExample`, `.example` manifest registration, tests).
pub const REASONING_WIRES_EXAMPLE_METABOLISM_TEXT: &str = include_str!("../../../📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.reasoning.wires.dsl.semio");

/// 📖️ Parses `.wires` DSL text into a `MindmapWiresDocument`.
pub fn parse_dsl(text: &str) -> Result<MindmapWiresDocument, store::TextError> {
    <MindmapWiresDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `MindmapWiresDocument` back to `.wires` DSL text.
pub fn print_dsl(document: &MindmapWiresDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsl_round_trip_empty_document() {
        let document = crate::artifacts::wires::empty_mindmap_wires_document();
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn dsl_round_trip_metabolism_fixture() {
        let document = parse_dsl(REASONING_WIRES_EXAMPLE_METABOLISM_TEXT).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(document.wires_fixture.get("identities").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
        assert_eq!(document.wires_fixture.get("relationships").and_then(|value| value.as_array()).map(|items| items.len()), Some(9));
        assert_eq!(document.board_fixture.get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
