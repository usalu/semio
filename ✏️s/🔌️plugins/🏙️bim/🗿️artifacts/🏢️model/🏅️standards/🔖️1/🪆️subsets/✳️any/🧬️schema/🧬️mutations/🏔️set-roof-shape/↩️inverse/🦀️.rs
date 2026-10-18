//! ↩️ Inverse of `SetRoofShape`: an absolute `SetRoofShape` carrying the base values of exactly the provided fields, none when the roof
//! is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_roof_shape::SetRoofShape;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetRoofShape, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_roof_shape::SetRoofShape>,
{
    match base.roofs.get(&payload.id) {
        Some(roof) => vec![Op::from(SetRoofShape {
            id: payload.id.clone(),
            shape: payload.shape.as_ref().map(|_| roof.shape.clone()),
            overhang: payload.overhang.map(|_| roof.overhang),
            base_offset: payload.base_offset.map(|_| roof.base_offset),
        })],
        None => Vec::new(),
    }
}
