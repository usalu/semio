//! 🔺️ `change-element-label-en` sparse diff.

use crate::change_element_label_en::ChangeElementLabelEn;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ElementsRows, Din18599ElementsPatch};

pub fn diff(payload: &ChangeElementLabelEn, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    let Some(row) = base.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.element_id), [payload.element_id.clone()]);
    };
    if row.label_en == payload.new_label_en {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Element label en already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        elements: Some(Din18599ElementsRows::modification(&payload.element_id, Din18599ElementsPatch { label_en: Some(payload.new_label_en.clone()), ..Default::default() })),
        ..Default::default()
    })
}
