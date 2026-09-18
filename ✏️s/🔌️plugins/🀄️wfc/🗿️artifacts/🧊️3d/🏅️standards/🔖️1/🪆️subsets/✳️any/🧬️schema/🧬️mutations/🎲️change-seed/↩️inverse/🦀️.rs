//! ↩️ Inverse for `ChangeSeed` — restores the PRIOR seed from a real BASE lookup (the seed field
//! always exists, so this is never a no-op).

use crate::mutations::{change_seed, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(_payload: &super::ChangeSeed, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    vec![change_seed(base.seed)]
}
