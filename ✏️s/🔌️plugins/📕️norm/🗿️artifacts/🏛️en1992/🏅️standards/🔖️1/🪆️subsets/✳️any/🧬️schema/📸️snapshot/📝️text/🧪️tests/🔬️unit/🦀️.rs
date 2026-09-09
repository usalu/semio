use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1992Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_agrees_with_print_parse_wrappers() {
    let document = En1992Snapshot::default();
    let printed = print_dsl(&document);
    assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
}

#[semio_framework_async_macros::async_test]
async fn document_dsl_parse_error_reports_the_real_line_of_the_bad_field() {
    // The engine's per-token spans are a concrete improvement over the old `dsl_kv` printer,
    // whose errors always reported `TextSpan::at(1, 1)` regardless of which line actually
    // failed. `fire-rating` (kebab-cased from `fire_rating`) is the 16th `key value` line in
    // `print_dsl`'s fixed field order.
    let printed = print_dsl(&En1992Snapshot::default());
    let bad = printed.replacen("fire-rating=r60", "fire-rating=not-a-rating", 1);
    assert_ne!(bad, printed, "fire_rating's printed line must match the literal replaced above");
    // Spans are relative to the document body after preamble strip.
    let body = bad.split_once('\n').map(|(_, rest)| rest).unwrap_or(bad.as_str());
    let bad_line = body.lines().position(|l| l.contains("not-a-rating")).expect("bad line present") as u32 + 1;
    let error = parse_dsl(&bad).expect_err("an unknown fire_rating tag must fail to parse");
    assert_eq!(error.span.line, bad_line, "error span must point at the actual malformed line, not (1, 1)");
}

#[semio_framework_async_macros::async_test]
async fn liquid_retaining_fem_anchor_example_fixture_parses_and_round_trips() {
    use crate::document::AnnexChoice;
    use crate::part_1_2::FireRating;
    use crate::part_3::TightnessClass;
    let document = parse_dsl(EN1992_LIQUID_RETAINING_FEM_ANCHOR_EXAMPLE_TEXT).expect("parse liquid retaining fem anchor example");
    assert_eq!(document.annex, AnnexChoice::En);
    assert_eq!(document.fire_rating, FireRating::R90);
    assert_eq!(document.tightness_class, TightnessClass::Tc2);
    assert!(document.use_fem);
    assert!(document.anchor_cracked);
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
