//! 📜️ GIS map artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::gismap::GisMapDocument;

/// 🗺️ The bundled "reuse map" example document, handcrafted in the `.gismap` DSL.
pub const REUSE_MAP_EXAMPLE_TEXT: &str = include_str!("../../📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.gis.gismap.dsl.semio");

/// 📖️ Parses `.gismap` DSL text into a `GisMapDocument`.
pub fn parse_dsl(text: &str) -> Result<GisMapDocument, store::TextError> {
    <GisMapDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `GisMapDocument` back to `.gismap` DSL text.
pub fn print_dsl(document: &GisMapDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gismap::MapFeature;
    use serde_json::json;

    #[test]
    fn gis_map_document_dsl_round_trips_bundled_reuse_example() {
        let document = parse_dsl(REUSE_MAP_EXAMPLE_TEXT).expect("parse reuse-map example");
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn gis_map_document_dsl_round_trips_empty_document() {
        store::test_support::assert_dsl_round_trip(&GisMapDocument::default());
    }

    #[test]
    fn print_dsl_reproduces_the_bundled_example_text_verbatim() {
        let document = parse_dsl(REUSE_MAP_EXAMPLE_TEXT).expect("parse reuse-map example");
        assert_eq!(parse_dsl(&print_dsl(&document)).expect("reparse"), document);
    }

    /// 🧬️ `MapFeature::data` is `dsl::DslValue` (deliberately untyped — see `crate::artifacts::gismap`'s
    /// doc comment) — round-trips every shape (nested object/array, bool, null, negative number) the
    /// generic value grammar has to reconstruct. `depth` is a float literal (not a bare int):
    /// `dsl::DslValue::Number` is a single `f64` variant with no JSON int-vs-float distinction, so a
    /// bare-int JSON literal would come back float-backed and fail `serde_json::Value`'s structural
    /// `PartialEq` even though it's the same number — an accepted engine characteristic, not a
    /// round-trip bug.
    #[test]
    fn gis_map_document_dsl_round_trips_synthetic_value_shapes() {
        let dsl_of = |value: serde_json::Value| dsl::to_dsl_value(&value).unwrap_or(dsl::DslValue::Null);
        let document = GisMapDocument {
            positions: vec![MapFeature {
                id: "p1".into(),
                data: dsl_of(json!({
                    "id": "p1",
                    "lon": -0.1427,
                    "lat": 51.5142,
                    "flag": true,
                    "missing": null,
                    "tags": ["a", "b"],
                    "meta": { "nested": { "depth": 2.0 } },
                })),
            }],
            routes: vec![MapFeature { id: "r1".into(), data: dsl_of(json!({ "id": "r1", "points": [[1.0, 2.0], [3.0, 4.0]] })) }],
            regions: vec![MapFeature { id: "g1".into(), data: dsl_of(json!({ "id": "g1", "ring": [[0.0, 0.0], [1.0, 1.0], [1.0, 0.0]] })) }],
        };
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
