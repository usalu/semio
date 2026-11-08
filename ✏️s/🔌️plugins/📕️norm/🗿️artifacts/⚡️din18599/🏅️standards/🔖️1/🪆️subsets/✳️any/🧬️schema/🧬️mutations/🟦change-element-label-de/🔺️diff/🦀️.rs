//! 🔺️ `change-element-label-de` sparse diff.

use crate::change_element_label_de::ChangeElementLabelDe;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ElementsRows, Din18599ElementsPatch};

pub fn diff(payload: &ChangeElementLabelDe, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    let Some(row) = base.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.element_id), [payload.element_id.clone()]);
    };
    if row.label_de == payload.new_label_de {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Element label de already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        elements: Some(Din18599ElementsRows::modification(&payload.element_id, Din18599ElementsPatch { label_de: Some(payload.new_label_de.clone()), ..Default::default() })),
        ..Default::default()
    })
}
