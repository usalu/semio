//! 🏛️ `placeGridColumns`: puts a column at every crossing of the grid lines of a storey's building (or of the given or selected grid lines only), where the storey has no column yet. The editor
//! authors nothing new here: every column is an ordinary `create-column` in one history row, with the selected column type of the library, else the first one. A model without a crossing,
//! without a storey or without a column type is refused with its own code instead of doing nothing silently.

use crate::editor::bim::entities::{id_taken, ordered_storeys};
use crate::editor::bim::kit::{fault, IdMint};
use crate::editor::bim::modes::edit::windows::plan;
use crate::editor::bim::BimDispatchCtx;
use crate::{Column, GridLine, ModelMutation, ModelSnapshot, Phase, Point2, TopConstraint};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// 📏️ How close an existing column may stand to a crossing before the crossing counts as taken, in metres.
const TAKEN_REACH: f64 = 0.05;
const EPS: f64 = 1e-9;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "place-grid-columns")]
pub struct PlaceGridColumns {
    pub storey: String,
    pub ids: Vec<String>,
}

/// 🔀️ Where the two grid lines cross, none when they are parallel or do not reach each other.
pub fn crossing(a: &GridLine, b: &GridLine) -> Option<Point2> {
    let (p, r) = (a.start, Point2 { x: a.end.x - a.start.x, y: a.end.y - a.start.y });
    let (q, s) = (b.start, Point2 { x: b.end.x - b.start.x, y: b.end.y - b.start.y });
    let denominator = r.x * s.y - r.y * s.x;
    if denominator.abs() < EPS {
        return None;
    }
    let (t, u) = (((q.x - p.x) * s.y - (q.y - p.y) * s.x) / denominator, ((q.x - p.x) * r.y - (q.y - p.y) * r.x) / denominator);
    let within = |value: f64| (-EPS..=1.0 + EPS).contains(&value);
    (within(t) && within(u)).then(|| Point2 { x: p.x + t * r.x, y: p.y + t * r.y })
}

/// 🔀️ Every crossing of the grid lines `lines` with their two labels, ordered by the lines and not repeated where three lines meet.
pub fn crossings(lines: &[&GridLine]) -> Vec<(Point2, String)> {
    let mut found: Vec<(Point2, String)> = Vec::new();
    for (index, a) in lines.iter().enumerate() {
        for b in &lines[index + 1..] {
            let Some(at) = crossing(a, b) else { continue };
            if !found.iter().any(|(other, _)| (other.x - at.x).hypot(other.y - at.y) < TAKEN_REACH) {
                found.push((at, format!("{}/{}", a.label, b.label)));
            }
        }
    }
    found
}

fn storey_of(snapshot: &ModelSnapshot, payload: &PlaceGridColumns, ctx: &BimDispatchCtx) -> Option<String> {
    let holds = |id: &String| snapshot.storeys.contains_key(id);
    Some(payload.storey.clone()).filter(holds).or_else(|| plan::active_storey(snapshot, &ctx.plan)).filter(holds).or_else(|| snapshot.buildings.keys().find_map(|building| ordered_storeys(snapshot, building).into_iter().next()))
}

pub fn handle(payload: &PlaceGridColumns, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let storey = storey_of(snapshot, payload, ctx).ok_or_else(|| fault("bim.grid-columns.storey-missing", "the model has no storey to place columns on"))?;
    let building = snapshot.storeys.get(&storey).map(|row| row.building.clone()).unwrap_or_default();
    let column_type = ctx.library_selected.iter().find(|id| snapshot.column_types.contains_key(*id)).or_else(|| snapshot.column_types.keys().next()).cloned().ok_or_else(|| fault("bim.grid-columns.type-missing", "the library holds no column type"))?;
    let wanted = if payload.ids.is_empty() { &ctx.selected } else { &payload.ids };
    let chosen: Vec<&GridLine> = snapshot.grids.iter().filter(|(id, grid)| grid.building == building && (wanted.iter().all(|id| !snapshot.grids.contains_key(id)) || wanted.contains(id))).map(|(_, grid)| grid).collect();
    let free: Vec<(Point2, String)> = crossings(&chosen).into_iter().filter(|(at, _)| !snapshot.columns.values().any(|column| column.storey == storey && (column.position.x - at.x).hypot(column.position.y - at.y) < TAKEN_REACH)).collect();
    if free.is_empty() {
        return Err(fault("bim.grid-columns.crossing-missing", "no free crossing of grid lines to place a column on"));
    }
    let kind = ctx.labels().map_or("Column", |labels| labels.kind_column.as_str()).to_string();
    let mut mint = IdMint::new(doc.operation_optional());
    Ok(Emit::mutations(
        free.into_iter()
            .map(|(position, label)| {
                let id = mint.mint("column", |id| id_taken(snapshot, id));
                let column = Column { storey: storey.clone(), column_type: column_type.clone(), position, rotation: 0.0, tilt: None, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, phase: Phase::New, name: format!("{kind} {label}") };
                ModelMutation::CreateColumn(crate::mutations::create_column::CreateColumn { id, column })
            })
            .collect(),
    ))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
