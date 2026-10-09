//! 🔺️ Diff constructor for `CreateCurtainWallType`: one created curtain wall type entry. Both grid rules are sound (a positive spacing, or explicit lines at
//! positive distances in strictly ascending order), both mullion sections have positive dimensions, the default panel names existing types or materials and both
//! materials exist.

use super::super::elements;
use super::super::wall_geometry::curtain_type_flaw;
use super::CreateCurtainWallType;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateCurtainWallType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(flaw) = curtain_type_flaw(base, &payload.curtain_wall_type) {
        return flaw.under(&["curtain_wall_type"]).refuse();
    }
    MutationOutcome::new(ModelDiff::curtain_wall_types(payload.id.clone(), Entry::Created(payload.curtain_wall_type.clone())))
}
