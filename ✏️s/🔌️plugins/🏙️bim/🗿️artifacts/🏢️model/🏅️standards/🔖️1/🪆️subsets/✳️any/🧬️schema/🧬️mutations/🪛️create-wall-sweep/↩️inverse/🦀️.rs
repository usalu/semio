//! ↩️ Inverse of `CreateWallSweep`: the concrete `DeleteWallSweep` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_wall_sweep::DeleteWallSweep;
use crate::standards::v1::subsets::any::schema::mutations::create_wall_sweep::CreateWallSweep;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateWallSweep, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_wall_sweep::DeleteWallSweep>,
{
    if base.wall_sweeps.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteWallSweep { id: payload.id.clone() })]
}
