//! 🔺️ `change-sections` diff.

use crate::mutations::change_sections::ChangeSections;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999SectionsRows};

pub fn diff(payload: &ChangeSections, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.sections == &payload.sections {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "List unchanged.");
    }
    if let Some((_, row)) = payload.sections.iter().enumerate().find(|(at, row)| payload.sections[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Section id {} appears twice.", row.id), [row.id.clone()]);
    }
    protocol::MutationOutcome::new(En1999Diff { sections: Some(En1999SectionsRows::setting(&base.sections, &payload.sections)), ..Default::default() })
}
