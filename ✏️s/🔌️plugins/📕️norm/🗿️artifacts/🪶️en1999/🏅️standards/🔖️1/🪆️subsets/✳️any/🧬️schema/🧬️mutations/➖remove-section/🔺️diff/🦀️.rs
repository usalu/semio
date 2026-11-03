//! 🔺️ `remove-section` diff.

use crate::remove_section::RemoveSection;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SectionsRows};

pub fn diff(payload: &RemoveSection, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.sections.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown section id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { sections: Some(En1999SectionsRows::removal(&base.sections, index)), ..Default::default() })
}
