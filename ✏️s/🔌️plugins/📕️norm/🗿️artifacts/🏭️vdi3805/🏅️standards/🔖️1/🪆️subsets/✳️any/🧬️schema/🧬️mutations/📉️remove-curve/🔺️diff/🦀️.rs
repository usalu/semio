//! 🔺️ `remove-curve` — sparse diff construction.

use super::RemoveCurve;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805CurvesRows};

//#region 🔖️Diff

pub fn diff(payload: &RemoveCurve, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if !base.curves.contains_key(&payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Curve \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Vdi3805Diff { curves: Some(Vdi3805CurvesRows { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
