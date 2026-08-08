//! 🧬️ Forms artifact — kernel `PlaybookMutation` as `FormMutation`.
pub use flow::playbook::PlaybookMutation as FormMutation;

use crate::artifacts::forms::FormSpec;
use protocol::Mutation;

pub fn apply_form_edit_mutation(spec: &FormSpec, mutation: &FormMutation) -> FormSpec {
    flow::playbook::apply_playbook_edit_mutation(spec, mutation)
}

pub fn inverse_form_mutation(spec: &FormSpec, mutation: &FormMutation) -> Vec<FormMutation> {
    <FormMutation as Mutation<FormSpec>>::inverse(mutation, spec)
}
