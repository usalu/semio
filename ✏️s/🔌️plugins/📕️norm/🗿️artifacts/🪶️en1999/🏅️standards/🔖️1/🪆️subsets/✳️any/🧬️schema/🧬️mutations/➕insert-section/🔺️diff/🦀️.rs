//! 🔺️ `insert-section` diff.

use crate::insert_section::InsertSection;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SectionsRows};

pub fn diff(payload: &InsertSection, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.sections.iter().any(|existing| existing.id == payload.section.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Section id {} already exists.", payload.section.id), [payload.section.id.clone()]);
    }
    if (payload.index as usize) > base.sections.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.sections.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { sections: Some(En1999SectionsRows::insertion(payload.index as usize, payload.section.clone())), ..Default::default() })
}
