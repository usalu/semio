//! 🔺️ `change-plated-plastic-moment` diff: patches `plastic_moment` of the addressed plated panel row.

use crate::change_plated_plastic_moment::ChangePlatedPlasticMoment;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PlatedPanelDelta, En1993PlatedPanelPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangePlatedPlasticMoment, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_plastic_moment.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.plated_panels.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "plated", Vec::<String>::new());
    };
    if row.plastic_moment == payload.new_plastic_moment {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The plated panel already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { plated_panels: En1993PlatedPanelDelta::modification(payload.id.clone(), En1993PlatedPanelPatch { plastic_moment: Some(payload.new_plastic_moment), ..Default::default() }), ..Default::default() })
}
