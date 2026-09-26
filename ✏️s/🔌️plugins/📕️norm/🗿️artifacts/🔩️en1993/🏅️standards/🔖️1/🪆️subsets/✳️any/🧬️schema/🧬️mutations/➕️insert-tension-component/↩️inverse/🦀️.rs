use super::InsertTensionComponent;
use crate::mutations::{remove_tension_component, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertTensionComponent, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.tension_components.len());
    vec![En1993Mutation::RemoveTensionComponent(remove_tension_component::RemoveTensionComponent { index: at })]
}
