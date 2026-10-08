use super::MoveNodes;
use crate::{EquationMutation, EquationSnapshot};

/// ⚖️ Moving a middle subset of nodes inverts to absolute positions whose diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::MoveNodes(MoveNodes { ids: vec!["b".into(), "c".into()], dx: 5.0, dy: -3.0 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}
