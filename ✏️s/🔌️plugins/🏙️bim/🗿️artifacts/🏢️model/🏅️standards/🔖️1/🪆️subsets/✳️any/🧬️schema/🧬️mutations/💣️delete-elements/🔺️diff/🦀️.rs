//! 🔺️ Diff constructor for `DeleteElements`: every listed element leaves in one sparse diff together with everything that depends on it
//! (buildings of a site, storeys and grid lines of a building, the contents of a storey, openings of a wall or curtain wall) and with the
//! property and classification entries of every removed element. A storey that a surviving element's top constraint still points at,
//! and an unknown id, are refused.

use crate::standards::v1::subsets::any::schema::mutations::cascade;
use crate::standards::v1::subsets::any::schema::mutations::delete_elements::DeleteElements;
use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteElements, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if payload.ids.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "No element is selected.", ["ids"]);
    }
    cascade::outcome(base, &payload.ids, "Element", None)
}
