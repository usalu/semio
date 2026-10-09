//! 🔺️ Diff constructor for `ArrayElements`: the copies of the pattern as one set of created records, copy `k` being the image of the
//! sources under the k-th translation (linear) or the k-th turn about the centre (radial), computed from the sources so that no rounding
//! accumulates. The records of copy `k` are minted as `prefix-k-position`, openings of copied walls ride along, and a copied space or
//! grid line takes the next free number or label. Refused: a pattern with no copies, more than 1024, a spacing or step without effect,
//! a blank prefix, a minted id that is taken and any refusal of the copy itself.

use super::super::modify::{self, Map};
use super::ArrayElements;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

/// 🗺️ The map of every copy of the pattern, in copy order; none when the pattern is refused.
pub fn maps_of(payload: &ArrayElements) -> Option<Vec<Map>> {
    payload.pattern.flaw().is_none().then(|| (1..=payload.pattern.count()).map(|copy| payload.pattern.map_of(copy)).collect())
}

pub fn diff(payload: &ArrayElements, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some((path, message)) = payload.pattern.flaw() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, [path]);
    }
    let Some(maps) = maps_of(payload) else {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "The pattern makes no copies.", ["pattern"]);
    };
    match modify::duplicate(base, &payload.ids, &payload.prefix, &maps) {
        Err(refusal) => refusal.outcome(),
        Ok(built) => {
            let carried = built.carried;
            let outcome = MutationOutcome::new(built.diff);
            if carried == 0 {
                outcome
            } else {
                outcome.info(OutcomeCode::Cascade, format!("{carried} opening(s) were copied with their hosts."))
            }
        }
    }
}
