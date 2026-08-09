use crate::artifacts::forms::FormsSnapshot;
use crate::artifacts::forms::mutations::FormMutation;
use protocol::Mutation;

pub fn inverse(base: &FormsSnapshot, mutation: &FormMutation) -> Vec<FormMutation> {
    <FormMutation as Mutation<FormsSnapshot>>::inverse(mutation, base)
}
