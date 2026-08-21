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
pub async fn parse_dsl(text: &str) -> Result<LowpolySnapshot, store::TextError> {
    <LowpolySnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `LowpolySnapshot` back to `.lowpoly` DSL text.
pub async fn print_dsl(document: &LowpolySnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🕸️ The live half-edge mesh JSON is not a field of `LowpolyObject` at all (round 2 of this
    /// ticket's round-trip law fix — see that struct's own doc comment and
    /// `📸️snapshot/🦀️component.rs`'s module doc comment), so `default_snapshot()` alone is already an
    /// honest round-trip fixture: nothing needs clearing before `assert_dsl_round_trip` compares full
    /// struct equality, unlike the pre-fix version of these tests.
    #[semio_framework_async_macros::async_test]
    async fn debug_dump_fixture_bytes() {
        let mesh_workspace = crate::artifacts::lowpoly::schema::default_mesh_workspace();
        let mesh_json = mesh_workspace.get("obj-1").expect("default workspace entry");
        let mesh = crate::artifacts::lowpoly::mesh_child_handle("obj-1", mesh_json);
        let object = crate::artifacts::lowpoly::LowpolyObject { id: "obj-1".into(), name: "Unit Box".into(), transform: Default::default(), smooth_shading: false, mesh: Some(mesh), paint_layers: Vec::new() };
        let projection = LowpolySnapshot { schema: crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA.into(), objects: vec![object] };
        let text = print_dsl(&projection);
        eprintln!("[DEBUG] FIXTURE_TEXT_START");
        eprintln!("{text}");
        eprintln!("[DEBUG] FIXTURE_TEXT_END");
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_round_trips_the_default_snapshot() {
        let projection = crate::artifacts::lowpoly::schema::default_snapshot();
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_round_trips_a_projection_with_a_painted_layer() {
        let mut projection = crate::artifacts::lowpoly::schema::default_snapshot();
        projection.objects[0].paint_layers[0].pixels[0] = 7;
        projection.objects[0].paint_layers[0].pixels[1] = 9;
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
    }

    #[semio_framework_async_macros::async_test]
    async fn handcrafted_example_text_uses_structural_object_codec() {
        assert!(!LOWPOLY_EXAMPLE_TEXT.contains("mesh-json"));
        let parsed = parse_dsl(LOWPOLY_EXAMPLE_TEXT).expect("handcrafted example should parse");
        assert_eq!(parsed.objects.len(), 1);
        assert_eq!(parsed.objects[0].id, "obj-1");
        assert!(parsed.objects[0].mesh.is_none());
        assert!(COMPONENT_GRAMMAR_SEMIO.contains("halfedge"));
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_parse_rejects_text_missing_required_schema_field() {
        let result = parse_dsl("objects=[]");
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_parse_rejects_unterminated_string_literal() {
        let result = parse_dsl("schema=\"unterminated");
        assert!(result.is_err());
    }

    // ⚠️ The four tests below were rewritten for the hand-rolled hex/bracket codec (this ticket,
    // `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — see `📸️snapshot/🦀️component.rs`'s module doc
    // comment for why `dsl::DslRecord`'s derive-based quoted-string grammar had to go). The old
    // literal test strings used the RETIRED derive grammar (`schema="…" objects=[ id="…" … ]`,
    // backslash-escaped quotes, `#`-comments) and no longer exercise this parser at all.

    #[semio_framework_async_macros::async_test]
    async fn dsl_parse_rejects_invalid_bool_value() {
        use crate::artifacts::lowpoly::schema::snapshot::enc_str;
        let text = format!("schema={}\nobjects=[[{},{},[0,0,0,0,0,0,1,1,1],notabool,[],[]]]", enc_str("lowpoly.document"), enc_str("o"), enc_str("O"),);
        let result = parse_dsl(&text);
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_parse_rejects_object_missing_required_field() {
        use crate::artifacts::lowpoly::schema::snapshot::enc_str;
        let text = format!("schema={}\nobjects=[[{}]]", enc_str("lowpoly.document"), enc_str("o"));
        let result = parse_dsl(&text);
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_parse_rejects_malformed_value_inside_a_nested_block() {
        use crate::artifacts::lowpoly::schema::snapshot::enc_str;
        let text = format!("schema={}\nobjects=[[{},{},[notanumber,0,0,0,0,0,1,1,1],false,[],[]]]", enc_str("lowpoly.document"), enc_str("o"), enc_str("O"),);
        let result = parse_dsl(&text);
        assert!(result.is_err());
    }

    /// 🧬️ The hand-rolled parser does not skip `#` comment lines (matching `✳️object`/`✳️kit`'s own
    /// hand-rolled codecs, which have no comment support either — the old derive-based grammar's
    /// comment handling did not survive the switch). An unrecognized line is a hard parse error.
    #[semio_framework_async_macros::async_test]
    async fn dsl_parse_rejects_unrecognized_lines() {
        use crate::artifacts::lowpoly::schema::snapshot::enc_str;
        let text = format!("# a leading comment\nschema={}\nobjects=[]\n", enc_str("lowpoly.document"));
        let result = parse_dsl(&text);
        assert!(result.is_err(), "comment lines are not a recognized field, unlike the retired derive grammar");
    }

    /// 🧬️ Hex-encoding sidesteps escaping ENTIRELY — a stronger guarantee than the old
    /// backslash-escape grammar: ANY string content (quotes, backslashes, newlines) round-trips
    /// with zero special-casing, because it is never interpreted as DSL syntax in the first place.
    #[semio_framework_async_macros::async_test]
    async fn dsl_parse_handles_arbitrary_characters_via_hex_encoding() {
        use crate::artifacts::lowpoly::schema::snapshot::enc_str;
        let tricky_name = "Quote \" and \\ and newline\ndone";
        let text = format!("schema={}\nobjects=[[{},{},[0,0,0,0,0,0,1,1,1],false,[],[]]]", enc_str("lowpoly.document"), enc_str("o1"), enc_str(tricky_name),);
        let projection = parse_dsl(&text).expect("hex-encoded strings never need escaping");
        assert_eq!(projection.objects[0].name, tricky_name);
    }
}
//#endregion 🧪️Tests
