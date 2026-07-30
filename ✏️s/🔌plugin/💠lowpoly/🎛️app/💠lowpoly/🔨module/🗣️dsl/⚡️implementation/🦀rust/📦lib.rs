//! 📜 Lowpoly app — textual document grammar surface + laws (constitutional: dsl).

use lowpoly::LowpolyProjection;

/// 📜 The `Concrete Forest Left` example, handcrafted in the `.lowpoly` DSL (produced by
/// `#[derive(dsl::DslDocument)]` on `LowpolyProjection`) instead of a raw mesh-only JSON fixture — every
/// object, its full half-edge geometry and its paint layers are real textual DSL, not a JSON-shaped
/// placeholder.
pub const LOWPOLY_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/💠lowpoly/📚example/💠concrete-forest-left.lowpoly");

/// 📖 Parses `.lowpoly` DSL text into a `LowpolyProjection`.
pub fn parse_dsl(text: &str) -> Result<LowpolyProjection, store::TextError> {
    <LowpolyProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `LowpolyProjection` back to `.lowpoly` DSL text.
pub fn print_dsl(document: &LowpolyProjection) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsl_round_trips_the_default_concrete_forest_projection() {
        let projection = parse_dsl(LOWPOLY_EXAMPLE_TEXT).expect("default projection DSL parses");
        store::test_support::assert_dsl_round_trip(&projection);
    }

    #[test]
    fn dsl_round_trips_a_projection_with_a_painted_layer() {
        let mut projection = parse_dsl(LOWPOLY_EXAMPLE_TEXT).expect("default projection DSL parses");
        projection.objects[0].paint_layers[0].pixels[0] = 7;
        projection.objects[0].paint_layers[0].pixels[1] = 9;
        store::test_support::assert_dsl_round_trip(&projection);
    }

    #[test]
    fn dsl_parse_rejects_text_missing_required_schema_field() {
        let result = parse_dsl("objects=[]");
        assert!(result.is_err());
    }

    #[test]
    fn dsl_parse_rejects_unterminated_string_literal() {
        let result = parse_dsl("schema=\"unterminated");
        assert!(result.is_err());
    }

    #[test]
    fn dsl_parse_rejects_invalid_bool_value() {
        let text = "schema=\"lowpoly.document\" objects=[ id=\"o\" name=\"O\" transform { position=0,0,0 rotation=0,0,0 scale=1,1,1 } smooth-shading=notabool mesh-json=\"{}\" paint-layers=[] ]";
        let result = parse_dsl(text);
        assert!(result.is_err());
    }

    #[test]
    fn dsl_parse_rejects_object_missing_required_field() {
        let text = "schema=\"lowpoly.document\" objects=[ id=\"o\" ]";
        let result = parse_dsl(text);
        assert!(result.is_err());
    }

    #[test]
    fn dsl_parse_rejects_malformed_value_inside_a_nested_block() {
        let text = "schema=\"lowpoly.document\" objects=[ id=\"o\" name=\"O\" transform { position=notanumber,0,0 rotation=0,0,0 scale=1,1,1 } smooth-shading=false mesh-json=\"{}\" paint-layers=[] ]";
        let result = parse_dsl(text);
        assert!(result.is_err());
    }

    #[test]
    fn dsl_parse_skips_comment_lines() {
        let text = "# a leading comment\nschema=\"lowpoly.document\" objects=[] # trailing comment\n";
        let projection = parse_dsl(text).expect("comments are not significant");
        assert_eq!(projection.schema, lowpoly::LOWPOLY_DOCUMENT_SCHEMA);
        assert!(projection.objects.is_empty());
    }

    #[test]
    fn dsl_parse_handles_escaped_characters_in_quoted_strings() {
        let text = "schema=\"lowpoly.document\" objects=[ id=\"o1\" name=\"Quote \\\" and \\\\ and newline\\ndone\" transform { position=0,0,0 rotation=0,0,0 scale=1,1,1 } smooth-shading=false mesh-json=\"{}\" paint-layers=[] ]";
        let projection = parse_dsl(text).expect("escapes must decode");
        assert_eq!(projection.objects[0].name, "Quote \" and \\ and newline\ndone");
    }
}
//#endregion 🧪Tests
