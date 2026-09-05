//! 🔺️ `update-bolt-inputs` — sparse diff construction.

use super::UpdateBoltInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateBoltInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_bolt_f_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bolt f ed kn must be a finite number, got {}.", payload.new_bolt_f_ed_kn), Vec::<String>::new());
    }
    if !payload.new_bolt_a_s_mm2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bolt a s mm2 must be a finite number, got {}.", payload.new_bolt_a_s_mm2), Vec::<String>::new());
    }
    if !payload.new_bolt_e1_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bolt e1 mm must be a finite number, got {}.", payload.new_bolt_e1_mm), Vec::<String>::new());
    }
    if !payload.new_bolt_e2_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bolt e2 mm must be a finite number, got {}.", payload.new_bolt_e2_mm), Vec::<String>::new());
    }
    if !payload.new_bolt_d0_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bolt d0 mm must be a finite number, got {}.", payload.new_bolt_d0_mm), Vec::<String>::new());
    }
    if !payload.new_bolt_d_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bolt d mm must be a finite number, got {}.", payload.new_bolt_d_mm), Vec::<String>::new());
    }
    if !payload.new_bolt_t_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bolt t mm must be a finite number, got {}.", payload.new_bolt_t_mm), Vec::<String>::new());
    }
    if !payload.new_bolt_f_u_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bolt f u mpa must be a finite number, got {}.", payload.new_bolt_f_u_mpa), Vec::<String>::new());
    }
    if !payload.new_bolt_f_ub_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bolt f ub mpa must be a finite number, got {}.", payload.new_bolt_f_ub_mpa), Vec::<String>::new());
    }
    if base.bolt_f_ed_kn == payload.new_bolt_f_ed_kn
        && base.bolt_n_bolts == payload.new_bolt_n_bolts
        && base.bolt_a_s_mm2 == payload.new_bolt_a_s_mm2
        && base.bolt_e1_mm == payload.new_bolt_e1_mm
        && base.bolt_e2_mm == payload.new_bolt_e2_mm
        && base.bolt_d0_mm == payload.new_bolt_d0_mm
        && base.bolt_d_mm == payload.new_bolt_d_mm
        && base.bolt_t_mm == payload.new_bolt_t_mm
        && base.bolt_f_u_mpa == payload.new_bolt_f_u_mpa
        && base.bolt_f_ub_mpa == payload.new_bolt_f_ub_mpa
    {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff {
        bolt_f_ed_kn: Some(payload.new_bolt_f_ed_kn),
        bolt_n_bolts: Some(payload.new_bolt_n_bolts),
        bolt_a_s_mm2: Some(payload.new_bolt_a_s_mm2),
        bolt_e1_mm: Some(payload.new_bolt_e1_mm),
        bolt_e2_mm: Some(payload.new_bolt_e2_mm),
        bolt_d0_mm: Some(payload.new_bolt_d0_mm),
        bolt_d_mm: Some(payload.new_bolt_d_mm),
        bolt_t_mm: Some(payload.new_bolt_t_mm),
        bolt_f_u_mpa: Some(payload.new_bolt_f_u_mpa),
        bolt_f_ub_mpa: Some(payload.new_bolt_f_ub_mpa),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
