//! 🔺️ Diff constructor for `MirrorElements`. In place, one sparse patch per element holds exactly the fields the reflection changes: the
//! axis of a wall (run the other way, its end join preferences traded), the axis of a curtain wall, the position and rotation of a
//! column, the points of a beam, grid line or railing, the loops of a slab, ceiling, roof, ramp or space, the fall of a slab and the ridge
//! of a roof, the hand and direction of a stair; the openings of a mirrored wall get their offset from the other end and the other hand,
//! those of a curtain wall both flips. With a prefix the mirror images are created as copies instead. Refused: a mirror line without
//! length, an empty selection, unknown ids, elements without a placement and a stair with a fixed hand; a selection that is already
//! symmetric is `mutation.no-op`.

use super::super::modify::{self, Map};
use super::MirrorElements;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

/// 🪞️ The reflection a payload names.
pub fn map_of(payload: &MirrorElements) -> Map {
    Map::Mirror { start: payload.line_start, end: payload.line_end }
}

pub fn diff(payload: &MirrorElements, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let map = map_of(payload);
    if !map.finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A mirror line needs finite points.", ["line_start"]);
    }
    if map.degenerate() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A mirror line needs two different points.", ["line_end"]);
    }
    let carried_note = |outcome: MutationOutcome<ModelDiff>, carried: usize| if carried == 0 { outcome } else { outcome.info(OutcomeCode::Cascade, format!("{carried} opening(s) were mirrored with their hosts.")) };
    match &payload.prefix {
        Some(prefix) => match modify::duplicate(base, &payload.ids, prefix, &[map]) {
            Err(refusal) => refusal.outcome(),
            Ok(built) => carried_note(MutationOutcome::new(built.diff), built.carried),
        },
        None => match modify::in_place(base, &payload.ids, &map) {
            Err(refusal) => refusal.outcome(),
            Ok(placed) if placed.is_empty() => MutationOutcome::refuse(OutcomeCode::NoOp, "Every element already is symmetric about this line.", ["ids"]),
            Ok(placed) => carried_note(MutationOutcome::new(placed.diff), placed.carried),
        },
    }
}
