//! ↩️ Inverse for `ChangeSeed` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::mutations::{change_seed, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse(_payload: &super::ChangeSeed, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
    vec![change_seed(base.seed)]
}
