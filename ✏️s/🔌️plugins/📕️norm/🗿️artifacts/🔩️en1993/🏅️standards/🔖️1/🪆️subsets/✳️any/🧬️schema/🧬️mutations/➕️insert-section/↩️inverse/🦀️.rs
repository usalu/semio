use super::InsertSection;
use crate::mutations::{remove_section, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertSection, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.sections.len());
    vec![En1993Mutation::RemoveSection(remove_section::RemoveSection { index: at })]
}
