use super::InsertEffect; use crate::En1990Mutation; use crate::En1990Snapshot;
pub fn inverse(_payload: &InsertEffect, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    vec![En1990Mutation::ChangeEffects(crate::standards::v1::subsets::any::schema::mutations::change_effects::ChangeEffects { new_effects: base.effects.clone() })]
}
