//! ↩️ Inverse for `DisconnectSlots` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::standards::v1::subsets::any::schema::mutations::connect_slots::connect_slots;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(payload: &super::DisconnectSlots, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_slots::ConnectSlots>,
{
    Ok((|| {
    let Some(edge) = base.edges.iter().find(|edge| edge.id == payload.id) else {
        return Vec::new();
    };
    vec![connect_slots(edge.clone())]

    })())
}
