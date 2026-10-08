use super::*;
use protocol::os_spr::protocol_laws::{assert_mutation_inverse_sum_law};
use protocol::{Mutation, OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn busy() -> BimWindowTransient {
    BimWindowTransient { engagement_input: "Living".into(), pointer_generation: 4, preview: "{\"marks\":[]}".into() }
}

#[semio_framework_async_macros::async_test]
async fn the_transient_defaults_to_nothing_typed() {
    assert_eq!(BimWindowTransient::default(), BimWindowTransient { engagement_input: String::new(), pointer_generation: 0, preview: String::new() });
}

#[semio_framework_async_macros::async_test]
async fn the_transient_round_trips_through_text_and_pack() {
    let transient = busy();
    assert_eq!(BimWindowTransient::parse_dsl(&transient.print_dsl()).expect("text"), transient);
    assert_eq!(BimWindowTransient::decode_pack(&transient.encode_pack()).expect("pack"), transient);
}

#[semio_framework_async_macros::async_test]
async fn the_mutation_round_trips_through_the_op_codecs() {
    let mutation = BimWindowTransientMutation::Snapshot { transient: busy() };
    assert_eq!(BimWindowTransientMutation::parse_op(&mutation.print_op()).expect("op text"), mutation);
    assert_eq!(BimWindowTransientMutation::decode_op(&mutation.encode_op().expect("encode")).expect("op binary"), mutation);
}

#[semio_framework_async_macros::async_test]
async fn the_diff_names_only_the_fields_that_differ() {
    let base = BimWindowTransient { engagement_input: "Living".into(), ..BimWindowTransient::default() };
    let outcome = BimWindowTransientMutation::Snapshot { transient: BimWindowTransient { pointer_generation: 1, ..base.clone() } }.diff(&base);
    assert_eq!(outcome.diff(), &BimWindowTransientDiff { pointer_generation: Some(1), ..BimWindowTransientDiff::default() });
}

