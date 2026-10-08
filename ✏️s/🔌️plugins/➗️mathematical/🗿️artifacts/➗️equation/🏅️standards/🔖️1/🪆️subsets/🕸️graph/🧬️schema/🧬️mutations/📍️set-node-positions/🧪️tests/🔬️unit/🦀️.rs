use super::{EquationNodePosition, SetNodePositions};
use crate::{EquationMutation, EquationSnapshot};

/// ⚖️ Placing a middle subset of nodes inverts to their base positions whose diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = EquationSnapshot::default();
    let positions = vec![EquationNodePosition { id: "c".into(), x: 1.0, y: 2.0 }, EquationNodePosition { id: "b".into(), x: 3.0, y: 4.0 }];
    let mutation = EquationMutation::SetNodePositions(SetNodePositions { positions });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}
