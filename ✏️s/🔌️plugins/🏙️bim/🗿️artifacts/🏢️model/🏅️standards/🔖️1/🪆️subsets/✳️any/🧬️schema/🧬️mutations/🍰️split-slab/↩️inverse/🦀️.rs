//! ↩️ Inverse of `SplitSlab` in storage order, replayed reversed by the store: the original outline and holes are restored first as one
//! `SetSlabBoundary` of the exact base loops, then the created slab, which holds no data of its own by then, is deleted. Empty when the
//! split is refused.

use crate::standards::v1::subsets::any::schema::mutations::delete_slab::DeleteSlab;
use crate::standards::v1::subsets::any::schema::mutations::set_slab_boundary::SetSlabBoundary;
use crate::standards::v1::subsets::any::schema::mutations::split_slab::diff::pieces;
use crate::standards::v1::subsets::any::schema::mutations::split_slab::SplitSlab;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SplitSlab, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_slab::DeleteSlab> + From<crate::standards::v1::subsets::any::schema::mutations::set_slab_boundary::SetSlabBoundary>,
{
    if pieces(payload, base).is_err() {
        return Vec::new();
    }
    let Some(slab) = base.slabs.get(&payload.id) else {
        return Vec::new();
    };
    vec![
        Op::from(DeleteSlab { id: payload.new_id.clone() }),
        Op::from(SetSlabBoundary { id: payload.id.clone(), boundary: slab.boundary.clone(), holes: slab.holes.clone() }),
    ]
}
