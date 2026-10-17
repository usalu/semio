//! ↩️ Inverse for `DisconnectSlots` — recreates the edge at its BASE position (missing id ⇒ empty).

use crate::standards::v1::subsets::any::schema::mutations::connect_slots::connect_slots;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::DisconnectSlots, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_slots::ConnectSlots>,
{
    Ok((|| {
    let Some(index) = base.edges.iter().position(|edge| edge.id == payload.id) else {
        return Vec::new();
    };
    vec![connect_slots(index, base.edges[index].clone())]

    })())
}
