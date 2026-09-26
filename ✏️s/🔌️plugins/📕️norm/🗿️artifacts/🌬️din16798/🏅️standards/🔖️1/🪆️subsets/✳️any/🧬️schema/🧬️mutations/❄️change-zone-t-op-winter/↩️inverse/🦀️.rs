//! ↩️ `change-zone-t-op-winter` inverse.
use super::ChangeZoneTOpWinter;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneTOpWinter, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneTOpWinter(ChangeZoneTOpWinter { zone_id: payload.zone_id.clone(), new_t_op_winter_c: z.t_op_winter_c })]
}
