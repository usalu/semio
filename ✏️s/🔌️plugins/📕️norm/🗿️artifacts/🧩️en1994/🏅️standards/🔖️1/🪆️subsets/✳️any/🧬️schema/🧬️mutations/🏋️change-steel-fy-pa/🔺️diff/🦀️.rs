//! 🔺️ Diff for `change-steel-fy-pa`.
use crate::change_steel_fy_pa::ChangeSteelFYPa;
use semio_s_artifact_norm_en1994_diff::En1994Diff;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn diff(payload: &ChangeSteelFYPa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_steel_f_y_pa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "invalid value", Vec::<String>::new());
    }
    if base.steel_f_y_pa == payload.new_steel_f_y_pa {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { steel_f_y_pa: Some(payload.new_steel_f_y_pa), ..Default::default() })
}
