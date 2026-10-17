//! ↩ Inverse constructor for `ReorderShots` — reconstructed from BASE state.

use super::ReorderShots;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &ReorderShots, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::reorder_shots::ReorderShots>,
{
    Ok((|| {
    match base.shots.iter().position(|shot| shot.id == payload.id) {
        Some(original_index) => vec![Op::from(ReorderShots { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }

    })())
}
