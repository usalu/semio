//! ↩️ Inverse of `MoveOpening`: an absolute `MoveOpening` back to the base offset, and back to the base host when the move named one,
//! none when the opening is absent.

use crate::standards::v1::subsets::any::schema::mutations::move_opening::MoveOpening;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &MoveOpening, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::move_opening::MoveOpening>,
{
    match base.openings.get(&payload.id) {
        Some(opening) => vec![Op::from(MoveOpening { id: payload.id.clone(), offset: opening.offset, host: payload.host.as_ref().map(|_| opening.host.clone()) })],
        None => Vec::new(),
    }
}
