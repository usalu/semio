//! ↩️ Inverse for `ConnectSlots` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::disconnect_slots;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(payload: &super::ConnectSlots, _base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::DisconnectSlots>,
{
    Ok((|| {
    vec![disconnect_slots(payload.edge.id.clone())]

    })())
}
