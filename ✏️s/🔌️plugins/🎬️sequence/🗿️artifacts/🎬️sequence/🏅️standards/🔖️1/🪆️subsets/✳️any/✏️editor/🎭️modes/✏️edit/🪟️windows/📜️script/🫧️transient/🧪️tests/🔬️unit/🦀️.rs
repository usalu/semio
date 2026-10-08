use super::*;
use protocol::os_spr::protocol_laws::{assert_mutation_inverse_sum_law};
use protocol::{DiffAlgebra, Mutation, OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn ran() -> SequenceScriptWindowTransient {
    SequenceScriptWindowTransient { last_run_json: "{\"steps\":3}".into() }
}

#[semio_framework_async_macros::async_test]
async fn the_diff_names_only_the_slot_that_differs_and_an_unchanged_root_is_an_empty_diff() {
    let outcome = SequenceScriptWindowTransientMutation::Snapshot { transient: ran() }.diff(&SequenceScriptWindowTransient::default());
    assert_eq!(outcome.diff(), &SequenceScriptWindowTransientDiff { last_run_json: Some("{\"steps\":3}".into()) });
    assert!(SequenceScriptWindowTransientMutation::Snapshot { transient: ran() }.diff(&ran()).diff().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_root_and_its_mutation_round_trip_through_text_pack_and_the_op_codecs() {
    assert_eq!(SequenceScriptWindowTransient::parse_dsl(&ran().print_dsl()).expect("text"), ran());
    assert_eq!(SequenceScriptWindowTransient::decode_pack(&ran().encode_pack()).expect("pack"), ran());
    let mutation = SequenceScriptWindowTransientMutation::Snapshot { transient: ran() };
    assert_eq!(SequenceScriptWindowTransientMutation::parse_op(&mutation.print_op()).expect("op text"), mutation);
    assert_eq!(SequenceScriptWindowTransientMutation::decode_op(&mutation.encode_op().expect("encode")).expect("op binary"), mutation);
}
