//! 🔺️ Diff constructor for `SetColumnTilt`: a one-field column patch that assigns the tilt (absent in the payload clears it, the column is plumb again). A tilt leans by a
//! positive angle of at most 60 degrees towards a finite direction. The tilted solid, its volume and the joins follow by inference; the position stays the base point.

use super::super::wall_geometry::tilt_flaw;
use super::SetColumnTilt;
use crate::{Assigned, ColumnPatch, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetColumnTilt, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(column) = base.columns.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Column \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = payload.tilt.as_ref().and_then(tilt_flaw) {
        return flaw.under(&["tilt"]).refuse();
    }
    if column.tilt == payload.tilt {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Column \"{}\" already has this tilt.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::columns(payload.id.clone(), Entry::Patched(ColumnPatch { tilt: Some(Assigned::new(payload.tilt)), ..Default::default() })))
}
