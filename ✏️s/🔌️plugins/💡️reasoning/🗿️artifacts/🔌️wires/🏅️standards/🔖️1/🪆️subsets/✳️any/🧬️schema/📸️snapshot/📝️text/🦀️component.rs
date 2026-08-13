//! 📜️ Wires artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! The `.wires` text/binary codecs are hand-rolled directly on `WiresSnapshot` — see
//! `impl store::ArtifactDsl for WiresSnapshot` (in `📸️snapshot/🦀️component.rs`'s
//! `🔖️HandcraftedArtifactCodecs` region, ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`) — since
//! `content` is now a composed `store::ArtifactChild<SemioGraphSnapshot>`, which has no
//! `dsl::DslRecord` derive support. `WiresMutation`'s own op-text grammar is unaffected
//! (`#[derive(dsl::DslEnum)]`, in `crate::artifacts::wires::op`).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::wires::WiresSnapshot;

/// 📄️ The `metabolism` example, handcrafted in the `.wires` DSL — source of truth for every
/// "metabolism" example call site (`setActiveExample`, `.example` manifest registration, tests).
pub const REASONING_WIRES_EXAMPLE_METABOLISM_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.wires` DSL text into a `WiresSnapshot`.
pub fn parse_dsl(text: &str) -> Result<WiresSnapshot, store::TextError> {
    <WiresSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `WiresSnapshot` back to `.wires` DSL text.
pub fn print_dsl(document: &WiresSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsl_round_trip_empty_document() {
        let document = crate::artifacts::wires::empty_wires_snapshot();
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn dsl_round_trip_metabolism_fixture() {
        let document = crate::artifacts::wires::schema::metabolism_wires_example_snapshot();
        assert_eq!(document.wires_fixture.get("identities").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
        assert_eq!(document.wires_fixture.get("relationships").and_then(|value| value.as_array()).map(|items| items.len()), Some(9));
        assert_eq!(crate::artifacts::wires::wires_working_board(&document).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
        let reparsed = parse_dsl(&print_dsl(&document)).expect("metabolism dsl round trip");
        assert_eq!(crate::artifacts::wires::wires_working_board(&reparsed).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
    }
}
//#endregion 🧪️Tests
