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
    // 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2) — the printed DSL text encodes the
    // content-addressed HANDLE only, never the entries themselves (that IS composition), and a
    // handle parsed from text carries no local owner until the host materializes its child
    // document. The three variable actions are therefore pinned through the content address: the
    // `qK=` line must decode to the exact handle the canonical builder mints for the three-entry
    // list, which (being a content hash) can name no other entry list.
    let reference = crate::standards::v1::subsets::any::examples::high_consequence_office::reference_snapshot();
    assert_eq!(crate::en1990_qk(&reference).len(), 3, "the canonical builder must materialize exactly the three variable actions");
    let document = parse_dsl(EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT).expect("parse high consequence office example");
    assert_eq!(document.consequence_class, 3);
    assert_eq!(document.annex, AnnexChoice::En);
    assert_eq!(document.seismic_a_ed_kn, 0.0);
    assert_eq!(document.q_k.child_id, reference.q_k.child_id, "the example text must name the content address of the three-entry variable-action child");
    assert_eq!(document.q_k.target, reference.q_k.target, "the example text must target the same composed table child");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
