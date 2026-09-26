//! ↩️ `change-zone-turbulence` inverse.
use super::ChangeZoneTurbulence;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneTurbulence, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneTurbulence(ChangeZoneTurbulence { zone_id: payload.zone_id.clone(), new_turbulence_intensity_percent: z.turbulence_intensity_percent })]
}
