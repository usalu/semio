//! ↩️ Inverse of `CreateCurtainPanelOverride`: the concrete `DeleteCurtainPanelOverride` of the id it created, none when the id was already taken.

use super::super::delete_curtain_panel_override::DeleteCurtainPanelOverride;
use super::CreateCurtainPanelOverride;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateCurtainPanelOverride, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.curtain_panel_overrides.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteCurtainPanelOverride(DeleteCurtainPanelOverride { id: payload.id.clone() })]
}
