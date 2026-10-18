//! ↩️ Inverse of `SetOpening`: an absolute `SetOpening` restoring the base values of exactly the provided fields, none when the opening
//! is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_opening::SetOpening;
use crate::standards::v1::subsets::any::schema::{diff::Assigned, snapshot::ModelSnapshot};

pub fn inverse<Op>(payload: &SetOpening, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_opening::SetOpening>,
{
    let Some(opening) = base.openings.get(&payload.id) else {
        return Vec::new();
    };
    vec![Op::from(SetOpening {
        id: payload.id.clone(),
        kind: payload.kind.as_ref().map(|_| opening.kind.clone()),
        sill_override: payload.sill_override.as_ref().map(|_| Assigned::new(opening.sill_override)),
        width: payload.width.as_ref().map(|_| Assigned::new(opening.width)),
        height: payload.height.as_ref().map(|_| Assigned::new(opening.height)),
        flip_hand: payload.flip_hand.map(|_| opening.flip_hand),
        flip_facing: payload.flip_facing.map(|_| opening.flip_facing),
        name: payload.name.as_ref().map(|_| opening.name.clone()),
        reveal_depth: payload.reveal_depth.as_ref().map(|_| Assigned::new(opening.reveal_depth)),
        reveal_material: payload.reveal_material.as_ref().map(|_| Assigned::new(opening.reveal_material.clone())),
    })]
}
