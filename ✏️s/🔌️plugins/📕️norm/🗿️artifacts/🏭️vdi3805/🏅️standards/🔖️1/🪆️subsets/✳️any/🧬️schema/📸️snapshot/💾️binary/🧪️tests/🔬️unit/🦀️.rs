use super::*;

#[test]
// 🪲️ Blocked on the confirmed upstream `pack` crate bug root-caused by the `draw` wave-2 family
// (`.🧬semio/🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/wave2-draw.txt` §4):
// `pack/value/rs/lib.rs`'s `decode_table_soa` fallback branch drops the column's `Shape` (passes
// `None` where `encode_table`'s matching branch passes `Some(&field.shape)`), so a `#[dsl(table)]`
fn document_dsl_pack_equivalence_the_reference_fixture() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&crate::reference_fixture());
}

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_the_reference_fixture() {
    let document = crate::reference_fixture();
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}
