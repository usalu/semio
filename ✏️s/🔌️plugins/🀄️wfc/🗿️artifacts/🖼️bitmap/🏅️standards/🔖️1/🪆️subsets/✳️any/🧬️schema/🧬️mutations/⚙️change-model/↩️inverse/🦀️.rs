//! ↩️ Inverse for `ChangeModel` — the prior parameter record; the model always exists, so this is
//! only empty when the forward mutation was itself a no-op.

use crate::mutations::{change_model, BitmapMutation};
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse(_payload: &super::ChangeModel, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
    vec![change_model(base.model.pattern_size, base.model.symmetry, base.model.periodic_input, base.model.ground)]
}
