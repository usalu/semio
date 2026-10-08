//! 🔺️ Diff constructor for `DeleteStorey`: the storey and everything on it (walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces and the openings of its walls) leave in one sparse diff together with everything that depends on them
//! (see the shared cascade), including the properties and classifications of every removed element. A storey that a
//! surviving element's top constraint still points at cannot cascade and is refused as `mutation.target-referenced`.

use super::super::cascade;
use super::DeleteStorey;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteStorey, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.storeys.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Storey", Some(&payload.id))
}
