//! ↩️ Inverse for `ChangeSeed` — restores the PRIOR seed from a real BASE lookup (the seed field
//! always exists, so this is never a no-op).

use crate::artifacts::assembly::mutations::{change_seed, AssemblyMutation};
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub async fn inverse(_payload: &super::mutation::ChangeSeed, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    vec![change_seed(base.seed)]
}
