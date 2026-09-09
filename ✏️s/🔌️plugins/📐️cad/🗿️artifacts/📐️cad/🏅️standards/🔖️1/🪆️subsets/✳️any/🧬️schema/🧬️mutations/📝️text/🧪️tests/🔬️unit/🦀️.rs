use crate::mutations::tests::every_mutation;

/// ⚖️ The pinned pre-migration byte fixture retired with the generic `Patch*`/`Set*` variants
/// it exercised (SEMANTIC-MUTATIONS-OVERHAUL, 26/08/12): the wire format legitimately changed —
/// greenfield, no backward compat — so round-tripping every current variant (below) is the law
/// that matters now, not byte-for-byte parity with a vocabulary that no longer exists.
#[semio_framework_async_macros::async_test]
async fn cad_mutation_print_op_round_trips_every_variant_as_one_line() {
    for op in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&op);
        store::os_store::test_support::assert_op_text_binary_equivalence(&op);
    }
}
