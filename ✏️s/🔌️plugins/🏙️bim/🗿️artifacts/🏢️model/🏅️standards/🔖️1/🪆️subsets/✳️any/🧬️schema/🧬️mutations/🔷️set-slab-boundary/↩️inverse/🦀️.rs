//! ↩️ Inverse of `SetSlabBoundary`: an absolute `SetSlabBoundary` back to the base boundary and holes, none when the slab is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_slab_boundary::SetSlabBoundary;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetSlabBoundary, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_slab_boundary::SetSlabBoundary>,
{
    match base.slabs.get(&payload.id) {
        Some(slab) => vec![Op::from(SetSlabBoundary { id: payload.id.clone(), boundary: slab.boundary.clone(), holes: slab.holes.clone() })],
        None => Vec::new(),
    }
}
