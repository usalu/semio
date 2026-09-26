//! 🔺️ `change-consequence-class` sparse diff.

use super::ChangeConsequenceClass;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeConsequenceClass, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.consequence_class == mutation.new_consequence_class {
        return MutationOutcome::empty().warn("mutation.no-op", "consequence_class already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        consequence_class: Some(mutation.new_consequence_class),
        ..En1990Diff::default()
    })
}
