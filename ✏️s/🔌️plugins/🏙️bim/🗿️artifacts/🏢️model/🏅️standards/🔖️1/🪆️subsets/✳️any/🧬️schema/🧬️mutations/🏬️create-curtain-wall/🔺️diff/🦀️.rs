//! 🔺️ Diff constructor for `CreateCurtainWall`: one created curtain wall entry. The storey, a constrained top storey of the same
//! building and both materials must exist, the axis must have length, and spacings, offsets and the mullion profile must be sound.
//! No height is stored: it is inferred from the top constraint.

use super::super::elements;
use super::super::wall_geometry::curtain_wall_flaw;
use super::CreateCurtainWall;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateCurtainWall, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(flaw) = curtain_wall_flaw(base, &payload.curtain_wall) {
        return flaw.under(&["curtain_wall"]).refuse();
    }
    MutationOutcome::new(ModelDiff::curtain_walls(payload.id.clone(), Entry::Created(payload.curtain_wall.clone())))
}
