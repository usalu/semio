//! ↩️ Inverse of `SetRoofFootprint`: an absolute `SetRoofFootprint` back to the base footprint, none when the roof is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_roof_footprint::SetRoofFootprint;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetRoofFootprint, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_roof_footprint::SetRoofFootprint>,
{
    match base.roofs.get(&payload.id) {
        Some(roof) => vec![Op::from(SetRoofFootprint { id: payload.id.clone(), footprint: roof.footprint.clone() })],
        None => Vec::new(),
    }
}
