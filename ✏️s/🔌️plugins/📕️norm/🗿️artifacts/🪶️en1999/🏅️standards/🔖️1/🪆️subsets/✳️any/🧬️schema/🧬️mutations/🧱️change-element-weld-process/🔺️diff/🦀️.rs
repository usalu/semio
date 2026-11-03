//! 🔺️ `change-element-weld-process` diff: patches `weld_process` of the addressed section element row.

use crate::change_element_weld_process::ChangeElementWeldProcess;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SectionsElementsPatch, En1999SectionsElementsRows, En1999SectionsPatch, En1999SectionsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeElementWeldProcess, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.sections.iter().find(|row| row.id == payload.section_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "section", Vec::<String>::new());
    };
    let Some(r1) = r0.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "element", Vec::<String>::new());
    };
    if r1.weld_process == payload.new_weld_process {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The section element already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { sections: Some(En1999SectionsRows::modification(&payload.section_id, En1999SectionsPatch { elements: Some(En1999SectionsElementsRows::modification(&payload.element_id, En1999SectionsElementsPatch { weld_process: Some(payload.new_weld_process), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
