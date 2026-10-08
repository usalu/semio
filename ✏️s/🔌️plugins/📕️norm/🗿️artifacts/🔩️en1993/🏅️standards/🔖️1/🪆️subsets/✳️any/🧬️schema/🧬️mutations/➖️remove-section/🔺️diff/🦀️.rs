//! ➖️ `remove-section` diff — removes the row at the index.

use super::RemoveSection;
use crate::diff::{En1993Diff, En1993SectionDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveSection, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.sections.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("section index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { sections: En1993SectionDelta::removal(&base.sections, payload.index), ..Default::default() })
}
