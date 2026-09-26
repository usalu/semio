use crate::artifact_schema::mutations::change_beam_action_q_area_pa::ChangeBeamActionQAreaPa;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_beam_action_q_area_pa() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeBeamActionQAreaPa(ChangeBeamActionQAreaPa { index: 0, action_index: 0, new_q_area_pa: 3.0e3 });
    // find an imposed action index
    let ai = base.beams[0].actions.iter().position(|a| a.kind == "imposed").unwrap_or(0);
    let op = En1994Mutation::ChangeBeamActionQAreaPa(ChangeBeamActionQAreaPa { index: 0, action_index: ai, new_q_area_pa: 3.0e3 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    assert!((next.beams[0].actions[ai].q_area_pa - 3.0e3).abs() < 1.0);
    let _ = op;
}
