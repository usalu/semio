use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::FlowSnapshot;

pub fn apply(snapshot: &mut FlowSnapshot, mutation: &FlowMutation) {
    crate::artifacts::flow::schema::mutations::apply_flow_mutation(snapshot, mutation);
}
