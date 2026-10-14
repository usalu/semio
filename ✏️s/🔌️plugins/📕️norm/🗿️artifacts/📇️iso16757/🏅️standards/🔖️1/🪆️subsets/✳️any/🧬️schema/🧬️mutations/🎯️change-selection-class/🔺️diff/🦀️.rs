//! 🔺️ `change-selection-class` — sparse diff construction.

use crate::change_selection_class::mutation::ChangeSelectionClass;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_artifact_norm_iso16757_diff::Iso16757Diff;

//#region 🔖️Diff

pub fn diff(payload: &ChangeSelectionClass, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.selection.class_id == payload.new_class_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Selection class is already \"{}\".", payload.new_class_id));
    }
    protocol::MutationOutcome::new(Iso16757Diff { selection_class_id: Some(payload.new_class_id.clone()), ..Default::default() })
}
