//! 📍️ `placeElements`: puts the given elements, or the selected ones, where the author typed: the reference point of the first element (the first point of its placement) lands on the
//! typed `x, y` and every other element moves by the same vector, so the group keeps its shape. It writes one `place-elements` mutation, whose absolute placements are the exact
//! inverse of any later move, in one history row. Openings are not placed: they follow their host.

use crate::editor::bim::gestures::typed::point_of;
use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::mutations::elements::{placement, Placement};
use crate::{ModelMutation, ModelSnapshot, Point2};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use std::collections::BTreeMap;
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "place-at")]
pub struct PlaceAt {
    pub ids: Vec<String>,
    pub at: String,
}

/// 📍️ The reference point of a placement: its first point.
pub fn reference_of(placement: &Placement) -> Option<Point2> {
    match placement.numbers().as_slice() {
        [x, y, ..] => Some(Point2 { x: *x, y: *y }),
        _ => None,
    }
}

pub fn handle(payload: &PlaceAt, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let wanted = if payload.ids.is_empty() { &ctx.selected } else { &payload.ids };
    if wanted.is_empty() {
        return Err(fault("bim.place.target-missing", "no element to place"));
    }
    let target = point_of(&payload.at).ok_or_else(|| fault("bim.place.point-invalid", format!("'{}' is not a position 'x, y'", payload.at)))?;
    let placements: Vec<(&String, Placement)> = wanted.iter().filter_map(|id| placement(snapshot, id).map(|placed| (id, placed))).collect();
    let Some(reference) = placements.first().and_then(|(_, placed)| reference_of(placed)) else { return Err(fault("bim.place.unsupported", "none of the targets has a placement to move")) };
    let vector = Point2 { x: target[0] - reference.x, y: target[1] - reference.y };
    if vector.x.hypot(vector.y) < 1e-9 {
        return Ok(Emit::default());
    }
    let moved: BTreeMap<String, Placement> = placements.into_iter().map(|(id, placed)| (id.clone(), placed.translated(vector))).collect();
    Ok(Emit::mutations(vec![ModelMutation::PlaceElements(crate::mutations::place_elements::PlaceElements { placements: moved })]))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
