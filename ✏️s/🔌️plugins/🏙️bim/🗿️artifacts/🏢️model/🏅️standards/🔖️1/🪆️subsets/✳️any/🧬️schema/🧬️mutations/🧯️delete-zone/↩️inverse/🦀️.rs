//! ↩️ Inverse of `DeleteZone`: one concrete row per removed record and removed property or classification (the shared cascade, the zone created
//! first when the store replays the vector reversed) and, in front of them so they replay after the zone exists, one `SetSpace` per former member
//! that gives the space its zone back.

use super::super::cascade;
use super::super::set_space::SetSpace;
use super::super::elements;
use super::DeleteZone;
use crate::{Assigned, ModelMutation, ModelSnapshot, SpacePatch};

pub fn inverse(payload: &DeleteZone, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let removal = match cascade::closure(base, std::slice::from_ref(&payload.id)) {
        Ok(removal) if base.zones.contains_key(&payload.id) && !base.area_schemes.values().any(|row| row.zones.contains(&payload.id)) => removal,
        _ => return Vec::new(),
    };
    let belonging = elements::zone_members(base, &payload.id).into_iter().map(|space| ModelMutation::SetSpace(SetSpace::from_patch(space, SpacePatch { zone: Some(Assigned::new(Some(payload.id.clone()))), ..Default::default() })));
    belonging.chain(removal.inverse(base)).collect()
}
