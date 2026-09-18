//! ↩️ Inverse for `ChangeSeed` — restores the PRIOR seed from a real BASE lookup (the seed field
//! always exists, so this is never a no-op).

use crate::mutations::{change_seed, BitmapMutation};
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse(_payload: &super::ChangeSeed, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
    vec![change_seed(base.seed)]
}
