//! ✏️ `change-foundation-liq-unit-wt-nm3` diff: patches `liq_unit_weight_n_m3` of the addressed foundation row.

use crate::change_foundation_liq_unit_wt_n_m3::ChangeFoundationLiqUnitWtNM3;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998FoundationDelta, En1998FoundationPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeFoundationLiqUnitWtNM3, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_liq_unit_weight_n_m3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.foundations.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("foundation {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.liq_unit_weight_n_m3 == payload.new_liq_unit_weight_n_m3 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The foundation already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { foundations: En1998FoundationDelta::modification(&payload.id, En1998FoundationPatch { liq_unit_weight_n_m3: Some(payload.new_liq_unit_weight_n_m3), ..Default::default() }), ..Default::default() })
}
