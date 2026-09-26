use super::*;

#[semio_framework_async_macros::async_test]
async fn every_variant_op_binary_round_trips() {
    for mutation in crate::artifact_schema::mutations::text::demo_mutation_cases() {
        let bytes = <Din16798Mutation as protocol::OpBinary>::encode_op(&mutation).expect("encode");
        let back = <Din16798Mutation as protocol::OpBinary>::decode_op(&bytes).expect("decode");
        assert_eq!(mutation, back);
    }
}
