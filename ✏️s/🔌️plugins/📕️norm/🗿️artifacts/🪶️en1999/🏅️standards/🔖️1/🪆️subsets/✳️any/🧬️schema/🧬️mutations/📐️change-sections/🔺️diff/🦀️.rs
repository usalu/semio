//! 🔺️ `change-sections` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_sections::ChangeSections;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeSections, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.sections == &payload.sections {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "List unchanged.");
    }
    protocol::MutationOutcome::new(En1999Diff { sections: Some(payload.sections.clone()), ..Default::default() })
}
