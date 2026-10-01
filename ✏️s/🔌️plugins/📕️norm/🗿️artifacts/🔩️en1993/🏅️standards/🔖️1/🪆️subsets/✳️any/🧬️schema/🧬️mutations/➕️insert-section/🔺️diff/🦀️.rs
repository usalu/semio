use super::InsertSection;
use crate::diff::En1993SectionList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertSection, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.sections.iter().any(|existing| existing.id == payload.section.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Section id {} already exists.", payload.section.id), [payload.section.id.clone()]);
    }
    let mut values = base.sections.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.section.clone());
    protocol::MutationOutcome::new(En1993Diff { sections: Some(En1993SectionList { values }), ..Default::default() })
}
