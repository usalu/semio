use crate::artifact_schema::mutations::change_column_action_force_n::ChangeColumnActionForceN;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_column_action_force_n() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeColumnActionForceN(ChangeColumnActionForceN { index: 0, action_index: 0, new_n_k_n: 1800e3 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    assert!((next.columns[0].actions[0].n_k_n - 1800e3).abs() < 1.0);
}
