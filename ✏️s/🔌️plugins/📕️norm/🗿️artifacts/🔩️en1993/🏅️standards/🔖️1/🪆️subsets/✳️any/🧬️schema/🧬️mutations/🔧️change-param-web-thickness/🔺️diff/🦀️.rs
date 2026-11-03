//! 🔺️ `change-param-web-thickness` diff: patches `web_thickness` of the addressed parametric section row.

use crate::change_param_web_thickness::ChangeParamWebThickness;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993ParametricSectionDelta, En1993ParametricSectionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeParamWebThickness, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_web_thickness.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.parametric_sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "param-section", Vec::<String>::new());
    };
    if row.web_thickness == payload.new_web_thickness {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The parametric section already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { parametric_sections: En1993ParametricSectionDelta::modification(payload.id.clone(), En1993ParametricSectionPatch { web_thickness: Some(payload.new_web_thickness), ..Default::default() }), ..Default::default() })
}
