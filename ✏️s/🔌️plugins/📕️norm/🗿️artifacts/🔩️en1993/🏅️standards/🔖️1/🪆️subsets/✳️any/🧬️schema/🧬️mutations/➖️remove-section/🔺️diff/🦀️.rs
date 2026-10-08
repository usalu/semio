//! ➖️ `remove-section` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveSection;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993SectionEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveSection, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.sections.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("section index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { sections: vec![En1993SectionEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
