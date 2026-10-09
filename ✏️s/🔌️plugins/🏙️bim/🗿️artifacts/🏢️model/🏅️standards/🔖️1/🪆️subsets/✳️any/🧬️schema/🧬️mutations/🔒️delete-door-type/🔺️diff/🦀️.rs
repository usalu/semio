//! 🔺️ Diff constructor for `DeleteDoorType`: one deleted door type entry together with the properties and classifications keyed by the type; refused while openings or curtain panels still use it.

use super::super::cascade;
use super::DeleteDoorType;
use crate::{CurtainPanel, Entry, KeyedDelta, ModelDiff, ModelSnapshot, OpeningKind};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteDoorType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.door_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Door type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.openings.values().any(|row| matches!(&row.kind, OpeningKind::Door { door_type } if *door_type == payload.id)) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Door type \"{}\" is still used by openings.", payload.id), [payload.id.clone()]);
    }
    let panel = |panel: &CurtainPanel| matches!(panel, CurtainPanel::Door { door_type } if *door_type == payload.id);
    if base.curtain_wall_types.values().any(|row| panel(&row.panel)) || base.curtain_panel_overrides.values().any(|row| panel(&row.panel)) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Door type \"{}\" is still used by curtain panels.", payload.id), [payload.id.clone()]);
    }
    let mut removal = cascade::data_diff(base, &payload.id);
    removal.door_types = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    MutationOutcome::new(removal)
}
