use super::*;
use crate::document::AnnexChoice;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1990Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_agrees_with_print_parse_wrappers() {
    let document = En1990Snapshot::default();
    let printed = print_dsl(&document);
    assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
}

#[semio_framework_async_macros::async_test]
async fn high_consequence_office_example_fixture_parses_and_round_trips() {
    // 🌱️ `q_k` is a composed `s.stdio.semio.table` child slot (ticket
    // 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2) — the printed DSL text only encodes
    // the content-addressed HANDLE, never the entries themselves (that's the whole point of
    // composition). Seeding the working-scene cache via the canonical fixture builder first
    // means the handle `parse_dsl` decodes from the file matches an already-cached entry list
    // with the SAME content hash, so `en1990_qk` resolves real entries here — same documented
    // bridge pattern every other wave-4 composed-child exemplar uses (see
    // `🗿️artifacts/⚖️en1990/🦀️.rs`'s `🔖️WorkingScene` doc comment for the staleness
    // gap this depends on).
    let _ = crate::standards::v1::subsets::any::examples::high_consequence_office::reference_snapshot();
    let document = parse_dsl(EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT).expect("parse high consequence office example");
    assert_eq!(document.consequence_class, 3);
    assert_eq!(document.annex, AnnexChoice::En);
    assert_eq!(document.seismic_a_ed_kn, 0.0);
    assert_eq!(crate::en1990_qk(&document).len(), 3);
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
