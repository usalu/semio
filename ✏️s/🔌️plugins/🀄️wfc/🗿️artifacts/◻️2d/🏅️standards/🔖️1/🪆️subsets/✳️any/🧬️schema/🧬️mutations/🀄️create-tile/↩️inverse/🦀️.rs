//! ↩️ Inverse for `CreateTile` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::mutations::{delete_tile, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse(payload: &super::CreateTile, _base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    vec![delete_tile(payload.tile.id.clone())]
}
