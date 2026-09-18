//! ↩️ Inverse for `ChangeSeed` — a scalar field always exists, so this is never a no-op.

use crate::mutations::{change_seed, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(_payload: &super::ChangeSeed, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
    vec![change_seed(base.seed)]
}
