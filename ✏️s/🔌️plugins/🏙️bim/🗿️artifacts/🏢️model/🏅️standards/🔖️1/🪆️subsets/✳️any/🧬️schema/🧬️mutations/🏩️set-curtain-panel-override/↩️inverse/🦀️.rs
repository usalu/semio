//! ↩️ Inverse of `SetCurtainPanelOverride`: an absolute `SetCurtainPanelOverride` back to the base panel, none when the override is absent.

use super::SetCurtainPanelOverride;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetCurtainPanelOverride, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.curtain_panel_overrides.get(&payload.id) {
        Some(record) => vec![ModelMutation::SetCurtainPanelOverride(SetCurtainPanelOverride { id: payload.id.clone(), panel: record.panel.clone() })],
        None => Vec::new(),
    }
}
