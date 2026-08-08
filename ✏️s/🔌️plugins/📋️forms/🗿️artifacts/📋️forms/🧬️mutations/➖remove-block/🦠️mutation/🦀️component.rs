use crate::artifacts::forms::FormSpec;
use crate::artifacts::forms::mutations::FormMutation;

pub fn apply(projection: &mut FormSpec, mutation: &FormMutation) {
    crate::artifacts::forms::mutations::apply_form_edit_mutation(projection, mutation);
}
