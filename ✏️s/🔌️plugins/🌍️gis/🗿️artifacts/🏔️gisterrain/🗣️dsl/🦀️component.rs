//! 📜️ GIS terrain artifact — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::gisterrain::Gis3dTerrainDocument;

/// 🏔️ The bundled "reuse terrain" example document, handcrafted in the `.gisterrain` DSL.
pub const REUSE_TERRAIN_EXAMPLE_TEXT: &str = include_str!("../📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.gis.gisterrain.dsl.semio");

/// 📖️ Parses `.gisterrain` DSL text into a `Gis3dTerrainDocument`.
pub fn parse_dsl(text: &str) -> Result<Gis3dTerrainDocument, store::TextError> {
    <Gis3dTerrainDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Gis3dTerrainDocument` back to `.gisterrain` DSL text.
pub fn print_dsl(document: &Gis3dTerrainDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gis3d_terrain_document_dsl_round_trips_bundled_reuse_example() {
        let document = parse_dsl(REUSE_TERRAIN_EXAMPLE_TEXT).expect("parse reuse-terrain example");
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn gis3d_terrain_document_dsl_round_trips_arbitrary_exaggeration() {
        store::test_support::assert_dsl_round_trip(&Gis3dTerrainDocument { exaggeration: 2.75, imported_features_json: String::new() });
    }

    #[test]
    fn gis3d_terrain_document_dsl_round_trips_imported_features_json() {
        store::test_support::assert_dsl_round_trip(&Gis3dTerrainDocument { exaggeration: 1.0, imported_features_json: r#"{"positions":[{"id":"p1","lon":1.0,"lat":2.0}],"routes":[],"regions":[]}"#.into() });
    }

    #[test]
    fn print_dsl_reparses_to_the_same_document() {
        let document = parse_dsl(REUSE_TERRAIN_EXAMPLE_TEXT).expect("parse reuse-terrain example");
        assert_eq!(parse_dsl(&print_dsl(&document)).expect("reparse"), document);
    }
}
//#endregion 🧪️Tests
