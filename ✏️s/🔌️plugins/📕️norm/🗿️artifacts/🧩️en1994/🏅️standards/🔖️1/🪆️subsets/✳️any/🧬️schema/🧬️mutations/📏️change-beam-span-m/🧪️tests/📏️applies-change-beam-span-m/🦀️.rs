use crate::artifact_schema::mutations::change_beam_span_m::ChangeBeamSpanM;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_beam_span_m() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeBeamSpanM(ChangeBeamSpanM { index: 0, new_span_m: 10.0 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
