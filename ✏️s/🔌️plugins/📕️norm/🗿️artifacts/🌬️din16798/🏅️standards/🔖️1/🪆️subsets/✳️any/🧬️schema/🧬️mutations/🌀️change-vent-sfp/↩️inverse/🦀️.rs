//! ↩️ `change-vent-sfp` inverse.
use super::ChangeVentSfp;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeVentSfp, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeVentSfp(ChangeVentSfp { vent_id: payload.vent_id.clone(), new_sfp_w_m3_s: v.sfp_w_m3_s })]
}
