//! 🧬️ Flow artifact — kernel `FlowMutation` facet.
pub use flow::FlowMutation;

use crate::artifacts::flow::FlowFixture;
use protocol::Mutation;

pub fn apply_flow_mutation(projection: &mut FlowFixture, mutation: &FlowMutation) {
    *projection = <FlowMutation as Mutation<FlowFixture>>::diff(mutation, projection).apply(projection);
}

pub fn inverse_flow_mutation(projection: &FlowFixture, mutation: &FlowMutation) -> Vec<FlowMutation> {
    <FlowMutation as Mutation<FlowFixture>>::inverse(mutation, projection)
}
