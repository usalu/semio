use super::RemoveSection;
use crate::mutations::{insert_section, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveSection, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if payload.index >= base.sections.len() { return Vec::new(); }
    vec![En1993Mutation::InsertSection(insert_section::InsertSection { index: payload.index, section: base.sections[payload.index].clone() })]
}
