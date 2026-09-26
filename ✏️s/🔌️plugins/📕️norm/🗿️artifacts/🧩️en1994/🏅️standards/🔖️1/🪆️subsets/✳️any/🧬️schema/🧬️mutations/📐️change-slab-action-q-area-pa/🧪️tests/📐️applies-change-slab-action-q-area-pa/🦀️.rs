use crate::artifact_schema::mutations::change_slab_action_q_area_pa::ChangeSlabActionQAreaPa;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_slab_action_q_area_pa() {
    let base = En1994Snapshot::default();
    let ai = base.slabs[0].actions.iter().position(|a| a.kind == "imposed").unwrap_or(0);
    let op = En1994Mutation::ChangeSlabActionQAreaPa(ChangeSlabActionQAreaPa { index: 0, action_index: ai, new_q_area_pa: 4.0e3 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    assert!((next.slabs[0].actions[ai].q_area_pa - 4.0e3).abs() < 1.0);
}
