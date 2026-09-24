use super::*;

/// 🕸️ The live half-edge mesh JSON is not a field of `LowpolyObject` at all (round 2 of this
/// ticket's round-trip law fix — see that struct's own doc comment and
/// `📸️snapshot/🦀️.rs`'s module doc comment), so `default_snapshot()` alone is already an
/// honest round-trip fixture: nothing needs clearing before `assert_dsl_round_trip` compares full
/// struct equality, unlike the pre-fix version of these tests.
#[semio_framework_async_macros::async_test]
async fn dsl_round_trips_the_default_snapshot() {
    let projection = crate::schema::default_snapshot();
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trips_a_projection_with_a_painted_layer() {
    let mut projection = crate::schema::default_snapshot();
    projection.objects[0].paint_layers[0].pixels = crate::empty_paint_pixels();
    projection.objects[0].paint_layers[0].pixels[0] = 7;
    projection.objects[0].paint_layers[0].pixels[1] = 9;
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
}

#[semio_framework_async_macros::async_test]
async fn handcrafted_example_text_is_concrete_forest_left_with_mesh_content() {
    let parsed = parse_dsl(LOWPOLY_EXAMPLE_TEXT).expect("handcrafted example should parse");
    assert_eq!(parsed.objects.len(), 1);
    assert_eq!(parsed.objects[0].id, "obj-1");
    assert_eq!(parsed.objects[0].name, crate::schema::LOWPOLY_DEFAULT_EXAMPLE_LABEL);
    assert!(!parsed.objects[0].mesh_content.is_empty());
    assert!(parsed.objects[0].mesh.is_some());
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

/// 🧬️ Printed text of the default projection, the base every derived-grammar failure case edits.
fn default_text() -> String {
    print_dsl(&crate::schema::default_snapshot())
}

#[semio_framework_async_macros::async_test]
async fn dsl_parse_rejects_invalid_bool_value() {
    let text = default_text().replacen("smooth-shading=false", "smooth-shading=notabool", 1);
    assert!(parse_dsl(&text).is_err());
}

#[semio_framework_async_macros::async_test]
async fn dsl_parse_rejects_object_missing_required_field() {
    let text = default_text().replacen(" id=obj-1", "", 1);
    assert!(parse_dsl(&text).is_err());
}

#[semio_framework_async_macros::async_test]
async fn dsl_parse_rejects_malformed_value_inside_a_nested_block() {
    let text = default_text().replacen("scale=1,1,1", "scale=notanumber,1,1", 1);
    assert!(parse_dsl(&text).is_err());
}

#[semio_framework_async_macros::async_test]
async fn dsl_parse_rejects_unrecognized_fields() {
    let text = default_text().replacen("smooth-shading=false", "smooth-shading=false unknown-field=1", 1);
    assert!(parse_dsl(&text).is_err());
}

#[semio_framework_async_macros::async_test]
async fn dsl_parse_round_trips_quotes_backslashes_and_newlines() {
    let tricky_name = "Quote \" and \\ and newline\ndone";
    let mut projection = crate::schema::default_snapshot();
    projection.objects[0].name = tricky_name.into();
    assert_eq!(parse_dsl(&print_dsl(&projection)).expect("escaped strings round-trip").objects[0].name, tricky_name);
}
