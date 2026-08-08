use crate::artifacts::flow::FlowFixture;
use crate::artifacts::flow::mutations::FlowMutation;

pub fn apply(projection: &mut FlowFixture, mutation: &FlowMutation) {
    crate::artifacts::flow::mutations::apply_flow_mutation(projection, mutation);
}
