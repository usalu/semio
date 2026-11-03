//! 🔺️ `change-plated-hogging` diff: patches `hogging` of the addressed plated panel row.

use crate::change_plated_hogging::ChangePlatedHogging;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PlatedPanelDelta, En1993PlatedPanelPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangePlatedHogging, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.plated_panels.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "plated", Vec::<String>::new());
    };
    if row.hogging == payload.new_hogging {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The plated panel already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { plated_panels: En1993PlatedPanelDelta::modification(payload.id.clone(), En1993PlatedPanelPatch { hogging: Some(payload.new_hogging), ..Default::default() }), ..Default::default() })
}
