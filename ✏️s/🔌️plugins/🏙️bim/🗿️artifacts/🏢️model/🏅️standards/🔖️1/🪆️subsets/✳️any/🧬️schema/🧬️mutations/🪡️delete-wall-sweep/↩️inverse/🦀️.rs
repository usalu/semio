//! ↩️ Inverse of `DeleteWallSweep`: one concrete create per removed record and one setter per removed property or classification, in storage order (dependants first, the target last).

use super::super::cascade;
use super::DeleteWallSweep;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteWallSweep, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
