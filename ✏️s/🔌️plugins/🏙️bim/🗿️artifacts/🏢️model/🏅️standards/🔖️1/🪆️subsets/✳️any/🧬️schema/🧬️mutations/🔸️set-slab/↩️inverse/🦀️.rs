//! ↩️ Inverse of `SetSlab`: an absolute `SetSlab` carrying the base values of exactly the provided fields, none when the slab is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_slab::SetSlab;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Assigned;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetSlab, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_slab::SetSlab>,
{
    match base.slabs.get(&payload.id) {
        Some(slab) => vec![Op::from(SetSlab {
            id: payload.id.clone(),
            slab_type: payload.slab_type.as_ref().map(|_| slab.slab_type.clone()),
            offset: payload.offset.map(|_| slab.offset),
            slope: payload.slope.as_ref().map(|_| Assigned::new(slab.slope)),
            name: payload.name.as_ref().map(|_| slab.name.clone()),
        })],
        None => Vec::new(),
    }
}
