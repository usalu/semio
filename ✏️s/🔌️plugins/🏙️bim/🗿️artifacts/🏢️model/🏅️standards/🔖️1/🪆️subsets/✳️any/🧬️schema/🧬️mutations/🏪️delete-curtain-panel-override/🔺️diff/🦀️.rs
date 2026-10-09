//! 🔺️ Diff constructor for `DeleteCurtainPanelOverride`: the override leaves in one sparse diff (see the shared cascade); the cell is filled by the default panel of the type again.

use super::super::cascade;
use super::DeleteCurtainPanelOverride;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteCurtainPanelOverride, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.curtain_panel_overrides.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Panel override \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Panel override", Some(&payload.id))
}
