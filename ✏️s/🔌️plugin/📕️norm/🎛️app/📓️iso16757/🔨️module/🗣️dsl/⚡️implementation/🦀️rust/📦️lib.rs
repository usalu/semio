//! 📜️ ISO 16757 app — textual document grammar surface + laws (constitutional: dsl).

use iso16757::Document;

/// 📄️ The `default` example document, handcrafted in the `.iso16757` DSL — a demo HVAC catalogue
/// worked example (control valve product group/class/series/product/variant, ISO 16757-4 dictionary
/// subject/property/controlled list, a box-primitive geometry with an inlet port, a selection
/// request, and a scripted part-number rule), mirroring `Document::reference_fixture()`.
pub const ISO16757_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/📕️norm/📚️example/📓️iso16757/📕️default.iso16757");

/// 📖️ Parses `.iso16757` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.iso16757` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iso16757::CatalogueValue;

    #[test]
    fn document_dsl_round_trips_the_reference_fixture() {
        store::test_support::assert_dsl_round_trip(&Document::reference_fixture());
    }

    #[test]
    fn default_example_dsl_round_trips() {
        let document = parse_dsl(ISO16757_DEFAULT_EXAMPLE_TEXT).expect("parse default .iso16757 example");
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn catalogue_value_integer_variant_round_trips_through_the_dsl_field_bridge() {
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
