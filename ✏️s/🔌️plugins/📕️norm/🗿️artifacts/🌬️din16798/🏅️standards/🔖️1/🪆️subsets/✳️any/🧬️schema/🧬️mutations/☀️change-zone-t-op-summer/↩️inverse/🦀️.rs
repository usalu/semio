//! ↩️ `change-zone-t-op-summer` inverse.
use super::ChangeZoneTOpSummer;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneTOpSummer, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneTOpSummer(ChangeZoneTOpSummer { zone_id: payload.zone_id.clone(), new_t_op_summer_c: z.t_op_summer_c })]
}
