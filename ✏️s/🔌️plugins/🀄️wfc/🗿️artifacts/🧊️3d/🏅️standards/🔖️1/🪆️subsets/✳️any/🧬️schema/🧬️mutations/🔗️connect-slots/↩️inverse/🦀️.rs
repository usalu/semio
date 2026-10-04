//! ↩️ Inverse for `ConnectSlots` — the `disconnect-slots` of the id it created.

use crate::mutations::{disconnect_slots, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::ConnectSlots, _base: &Wfc3dSnapshot) -> Result<Vec<Wfc3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![disconnect_slots(payload.edge.id.clone())]

    })())
}
