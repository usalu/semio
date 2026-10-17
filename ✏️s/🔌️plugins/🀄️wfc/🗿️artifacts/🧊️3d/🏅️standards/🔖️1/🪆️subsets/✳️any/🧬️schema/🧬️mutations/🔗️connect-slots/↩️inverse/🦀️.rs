//! ↩️ Inverse for `ConnectSlots` — the `disconnect-slots` of the id it created.

use crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::disconnect_slots;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::ConnectSlots, _base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::DisconnectSlots>,
{
    Ok((|| {
    vec![disconnect_slots(payload.edge.id.clone())]

    })())
}
