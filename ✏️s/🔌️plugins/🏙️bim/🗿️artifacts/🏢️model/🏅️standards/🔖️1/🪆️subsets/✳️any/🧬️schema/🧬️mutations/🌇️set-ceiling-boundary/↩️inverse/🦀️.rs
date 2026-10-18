//! ↩️ Inverse of `SetCeilingBoundary`: an absolute `SetCeilingBoundary` back to the base boundary and holes, none when the ceiling is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_ceiling_boundary::SetCeilingBoundary;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetCeilingBoundary, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_ceiling_boundary::SetCeilingBoundary>,
{
    match base.ceilings.get(&payload.id) {
        Some(ceiling) => vec![Op::from(SetCeilingBoundary { id: payload.id.clone(), boundary: ceiling.boundary.clone(), holes: ceiling.holes.clone() })],
        None => Vec::new(),
    }
}
