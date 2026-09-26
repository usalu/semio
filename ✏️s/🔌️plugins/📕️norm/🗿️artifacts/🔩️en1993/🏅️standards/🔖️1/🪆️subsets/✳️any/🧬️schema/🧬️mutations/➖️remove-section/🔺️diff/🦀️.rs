use super::RemoveSection;
use crate::diff::En1993SectionList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveSection, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.sections.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("section index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.sections.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { sections: Some(En1993SectionList { values }), ..Default::default() })
}
