use crate::artifacts::forms::FormSpec;
use crate::artifacts::forms::mutations::FormMutation;
use protocol::Mutation;

pub fn inverse(base: &FormSpec, mutation: &FormMutation) -> Vec<FormMutation> {
    <FormMutation as Mutation<FormSpec>>::inverse(mutation, base)
}
