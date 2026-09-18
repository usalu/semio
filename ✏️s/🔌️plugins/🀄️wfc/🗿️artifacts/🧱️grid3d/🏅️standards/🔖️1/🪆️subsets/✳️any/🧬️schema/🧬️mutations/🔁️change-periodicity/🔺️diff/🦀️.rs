//! 🔺️ Sparse diff builder for `ChangePeriodicity` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;

pub fn diff(payload: &super::ChangePeriodicity, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    if (base.periodic_x, base.periodic_y, base.periodic_z) == (payload.periodic_x, payload.periodic_y, payload.periodic_z) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The grid already carries this periodicity.".to_string());
    }
    protocol::MutationOutcome::new(Grid3dDiff { periodic_x: Some(payload.periodic_x), periodic_y: Some(payload.periodic_y), periodic_z: Some(payload.periodic_z), ..Default::default() })
}
