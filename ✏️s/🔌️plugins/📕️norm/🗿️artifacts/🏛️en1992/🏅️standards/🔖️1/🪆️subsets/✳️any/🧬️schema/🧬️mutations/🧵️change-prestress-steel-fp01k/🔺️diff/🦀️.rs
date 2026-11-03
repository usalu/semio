//! 🧵 `change-prestress-steel-fp01k` diff: patches `f_p01k` of the addressed prestressing steel row.

use crate::change_prestress_steel_f_p01k::ChangePrestressSteelFP01k;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992PrestressSteelsPatch, En1992PrestressSteelsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangePrestressSteelFP01k, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_f_p01k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.prestress_steels.iter().find(|row| row.id == payload.grade_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Prestressing steel {} does not exist.", payload.grade_id), [payload.grade_id.clone()]);
    };
    if r0.f_p0_1k == payload.new_f_p01k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The prestressing steel already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { prestress_steels: Some(En1992PrestressSteelsRows::modification(&payload.grade_id, En1992PrestressSteelsPatch { f_p0_1k: Some(payload.new_f_p01k), ..Default::default() })), ..Default::default() })
}
