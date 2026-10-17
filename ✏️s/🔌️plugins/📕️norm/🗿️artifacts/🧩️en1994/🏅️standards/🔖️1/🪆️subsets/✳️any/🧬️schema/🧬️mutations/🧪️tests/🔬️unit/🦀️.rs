use crate::*;
use protocol::{Mutation, SemanticMutation};

fn round_trip(base: &En1994Snapshot, operation: &En1994Mutation) -> En1994Snapshot {
    let (forward, _messages) = protocol::apply_mutation(base, operation).expect("valid mutation");
    let mut restored = forward.clone();
    for back in operation.inverse(base).expect("valid retained mutation inverse fixture") {
        let (next, _messages) = protocol::apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
    forward
}

#[test]
fn every_kind_round_trips_through_inverse() {
    let base = En1994Snapshot::default();
    for op in demo_mutation_cases() {
        let _ = round_trip(&base, &op);
    }
}

