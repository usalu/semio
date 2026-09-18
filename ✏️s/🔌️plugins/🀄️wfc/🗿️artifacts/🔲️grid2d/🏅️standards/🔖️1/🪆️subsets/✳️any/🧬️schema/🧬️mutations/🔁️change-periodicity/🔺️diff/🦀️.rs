//! 🔺️ Sparse diff builder for `ChangePeriodicity` — two boolean lanes, no collection touched.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::ChangePeriodicity, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    if base.periodic_x == payload.periodic_x && base.periodic_y == payload.periodic_y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Periodicity is already x={} y={}.", payload.periodic_x, payload.periodic_y));
    }
    protocol::MutationOutcome::new(Grid2dDiff { periodic_x: Some(payload.periodic_x), periodic_y: Some(payload.periodic_y), ..Default::default() })
}
