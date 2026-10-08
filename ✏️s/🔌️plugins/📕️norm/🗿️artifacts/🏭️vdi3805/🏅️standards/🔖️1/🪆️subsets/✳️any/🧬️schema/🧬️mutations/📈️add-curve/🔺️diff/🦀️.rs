//! 🔺️ `add-curve` — sparse diff construction.

use super::AddCurve;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805CurvesRows, Vdi3805CurvesEntry};

//#region 🔖️Diff
/// 🔺️ A duplicate id is `mutation.duplicate-id` — an id-keyed entity that already exists cannot be
/// "created" again.

pub fn diff(payload: &AddCurve, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.curves.contains_key(&payload.curve.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A curve with id \"{}\" already exists.", payload.curve.id), [payload.curve.id.clone()]);
    }
    protocol::MutationOutcome::new(Vdi3805Diff { curves: Some(Vdi3805CurvesRows { added: vec![Vdi3805CurvesEntry { key: payload.curve.id.clone(), value: payload.curve.clone() }], ..Default::default() }), ..Default::default() })
}
