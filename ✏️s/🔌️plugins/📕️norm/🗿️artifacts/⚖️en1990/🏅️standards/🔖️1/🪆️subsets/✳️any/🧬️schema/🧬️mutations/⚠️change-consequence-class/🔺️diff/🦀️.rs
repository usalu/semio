//! 🔺️ `change-consequence-class` sparse diff.

use crate::change_consequence_class::ChangeConsequenceClass;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeConsequenceClass, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.consequence_class == mutation.new_consequence_class {
        return MutationOutcome::empty().warning("mutation.no-op", "consequence_class already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        consequence_class: Some(mutation.new_consequence_class),
        ..En1990Diff::default()
    })
}
