//! ↩ Inverse constructor for `DeleteShot` — reconstructed from BASE state.

use super::DeleteShot;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &DeleteShot, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_shot::CreateShot>,
{
    Ok(match base.shots.iter().position(|entry| entry.id == payload.id) {
        Some(index) => vec![Op::from(crate::mutations::create_shot::CreateShot { shot: base.shots[index].clone(), index: Some(index) })],
        None => Vec::new(),
    })
}
