use crate::artifacts::flow::mutations::FlowMutation;
use crate::artifacts::flow::FlowSnapshot;
use protocol::Mutation;

pub fn inverse(base: &FlowSnapshot, mutation: &FlowMutation) -> Vec<FlowMutation> {
    <FlowMutation as Mutation<FlowSnapshot>>::inverse(mutation, base)
}
