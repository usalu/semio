use crate::artifacts::forms::FormsSnapshot;
use crate::artifacts::forms::mutations::FormMutation;

pub fn apply(projection: &mut FormsSnapshot, mutation: &FormMutation) {
    crate::artifacts::forms::mutations::apply_form_edit_mutation(projection, mutation);
}
