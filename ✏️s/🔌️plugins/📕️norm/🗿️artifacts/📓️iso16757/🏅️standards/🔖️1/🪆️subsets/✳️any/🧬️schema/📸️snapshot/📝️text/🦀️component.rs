//! 📜️ ISO 16757 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::iso16757::Iso16757Snapshot;

/// 📄️ The `default` example document, handcrafted in the `.iso16757` DSL — a demo HVAC catalogue
/// worked example (control valve product group/class/series/product/variant, ISO 16757-4 dictionary
/// subject/property/controlled list, a box-primitive geometry with an inlet port, a selection
/// request, and a scripted part-number rule), mirroring `Document::reference_fixture()`.
pub const ISO16757_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.iso16757` DSL text into a `Document`.
pub async fn parse_dsl(text: &str) -> Result<Iso16757Snapshot, store::TextError> {
    <Iso16757Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.iso16757` DSL text.
pub async fn print_dsl(document: &Iso16757Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::iso16757::CatalogueValue;

    #[test]
    async fn document_dsl_round_trips_the_reference_fixture() {
        store::os_store::test_support::assert_dsl_round_trip(&Iso16757Snapshot::reference_fixture());
    }

    #[test]
    async fn default_example_dsl_round_trips() {
        let document = parse_dsl(ISO16757_DEFAULT_EXAMPLE_TEXT).expect("parse default .iso16757 example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    async fn catalogue_value_integer_variant_round_trips_through_the_dsl_field_bridge() {
        // ⚡️ Regression: `CatalogueValue`'s `Shape::Value` bridge goes through `dsl::DslValue::Number`
        // (f64-only, no int/float distinction), which used to turn `Integer { value: 50 }` into a
        // JSON float `50.0` that `serde_json::from_value` then rejected for the `i64` field. Not
        // exercised by the reference fixture (it only uses `Decimal`), so covered directly here.
        let value = CatalogueValue::Integer { value: 50 };
        let printed = <CatalogueValue as dsl::DslField>::to_value(&value);
        let parsed = <CatalogueValue as dsl::DslField>::from_value(&printed).expect("integer variant must round trip");
        assert_eq!(parsed, value);
    }
}
