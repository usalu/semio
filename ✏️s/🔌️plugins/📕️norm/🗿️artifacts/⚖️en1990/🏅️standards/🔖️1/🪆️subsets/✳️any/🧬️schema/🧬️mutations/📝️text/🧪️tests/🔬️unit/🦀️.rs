use super::*;
use protocol::OpBinary;

#[semio_framework_async_macros::async_test]
async fn every_demo_mutation_roundtrips_text_and_binary() {
    for mutation in demo_mutation_cases() {
        let text = protocol::OpText::print_op(&mutation);
        let decoded = <En1990Mutation as protocol::OpText>::parse_op(&text).expect("decode text");
        assert_eq!(decoded, mutation);
        let bin = mutation.encode_op().expect("encode bin");
        let decoded_bin = En1990Mutation::decode_op(&bin).expect("decode bin");
        assert_eq!(decoded_bin, mutation);
    }
}
