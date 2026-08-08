use crate::artifacts::flow::FlowFixture;
use crate::artifacts::flow::mutations::FlowMutation;
use protocol::Mutation;

pub fn inverse(base: &FlowFixture, mutation: &FlowMutation) -> Vec<FlowMutation> {
    <FlowMutation as Mutation<FlowFixture>>::inverse(mutation, base)
}
