//! 🔺️ `change-param-label` diff: patches `label` of the addressed parametric section row.

use crate::change_param_label::ChangeParamLabel;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993ParametricSectionDelta, En1993ParametricSectionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeParamLabel, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.parametric_sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "param-section", Vec::<String>::new());
    };
    if row.label == payload.new_label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The parametric section already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { parametric_sections: En1993ParametricSectionDelta::modification(payload.id.clone(), En1993ParametricSectionPatch { label: Some(payload.new_label.clone()), ..Default::default() }), ..Default::default() })
}
