//! ↩️ Inverse of `DeleteCurtainPanelOverride`: the concrete create of the removed override, in storage order (see the shared cascade).

use super::super::cascade;
use super::DeleteCurtainPanelOverride;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &DeleteCurtainPanelOverride, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
