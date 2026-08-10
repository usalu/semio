//! 📜️ Lowpoly artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::lowpoly::LowpolySnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📜️ The reuse example, handcrafted against `COMPONENT_GRAMMAR_SEMIO` — structured half-edge mesh
/// productions (no `mesh-json`). Derive-based `parse_dsl` does not yet consume this shape; the
/// recognizer / handcrafted codec will.
pub const LOWPOLY_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.lowpoly` DSL text into a `LowpolySnapshot`.
pub fn parse_dsl(text: &str) -> Result<LowpolySnapshot, store::TextError> {
    <LowpolySnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `LowpolySnapshot` back to `.lowpoly` DSL text.
pub fn print_dsl(document: &LowpolySnapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsl_round_trips_the_default_snapshot() {
        let projection = crate::artifacts::lowpoly::engine::default_snapshot();
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
    }

    #[test]
    fn dsl_round_trips_a_projection_with_a_painted_layer() {
        let mut projection = crate::artifacts::lowpoly::engine::default_snapshot();
        projection.objects[0].paint_layers[0].pixels[0] = 7;
        projection.objects[0].paint_layers[0].pixels[1] = 9;
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
    }

    #[test]
    fn handcrafted_example_text_has_no_mesh_json_smuggle() {
        assert!(!LOWPOLY_EXAMPLE_TEXT.contains("mesh-json"));
        assert!(LOWPOLY_EXAMPLE_TEXT.contains("mesh {") || LOWPOLY_EXAMPLE_TEXT.contains("mesh{"));
        assert!(COMPONENT_GRAMMAR_SEMIO.contains("halfedge"));
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
        let text = "schema=\"lowpoly.document\" objects=[ id=\"o\" name=\"O\" transform { position=@0,0,0 rotation=0,0,0 scale=1,1,1 } smooth-shading=notabool mesh-json=\"{}\" paint-layers=[] ]";
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
        let text = "schema=\"lowpoly.document\" objects=[ id=\"o\" name=\"O\" transform { position=@notanumber,0,0 rotation=0,0,0 scale=1,1,1 } smooth-shading=false mesh-json=\"{}\" paint-layers=[] ]";
        let result = parse_dsl(text);
        assert!(result.is_err());
    }

    #[test]
    fn dsl_parse_skips_comment_lines() {
        let text = "# a leading comment\nschema=\"lowpoly.document\" objects=[] # trailing comment\n";
        let projection = parse_dsl(text).expect("comments are not significant");
        assert_eq!(projection.schema, crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA);
        assert!(projection.objects.is_empty());
    }

    #[test]
    fn dsl_parse_handles_escaped_characters_in_quoted_strings() {
        let text = "schema=\"lowpoly.document\" objects=[ id=\"o1\" name=\"Quote \\\" and \\\\ and newline\\ndone\" transform { position=@0,0,0 rotation=0,0,0 scale=1,1,1 } smooth-shading=false mesh-json=\"{}\" paint-layers=[] ]";
        let projection = parse_dsl(text).expect("escapes must decode");
        assert_eq!(projection.objects[0].name, "Quote \" and \\ and newline\ndone");
    }
}
//#endregion 🧪️Tests
