//! ↩️ Inverse of `PlaceElements`: another `PlaceElements` carrying the exact base placement of every element whose placement the call
//! changes, read from the base. Empty when any listed element has no placement of the given kind, any placement is not finite, or no
//! placement changes.

use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::Placement;
use crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use std::collections::BTreeMap;

pub fn inverse<Op>(payload: &PlaceElements, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    let rows: Option<Vec<(String, Placement, bool)>> = payload
        .placements
        .iter()
        .map(|(id, target)| {
            let current = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::state(base, id).filter(|current| current.same_kind(target) && target.numbers().iter().all(|number| number.is_finite()))?;
            let changes = current != target.clone().mounted_like(&current);
            Some((id.clone(), current, changes))
        })
        .collect();
    let before: BTreeMap<String, Placement> = rows.into_iter().flatten().filter(|(_, _, changes)| *changes).map(|(id, current, _)| (id, current)).collect();
    if before.is_empty() {
        Vec::new()
    } else {
        vec![Op::from(PlaceElements { placements: before })]
    }
}
