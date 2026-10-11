//! 🪞️ `curtain-layout`: the resolved vertical extent, axis length, cell grid and panels of every curtain wall. Like `wall-layout` it is a
//! node of the model graph over storeys: a curtain wall depends on its own storey and on the storey its top is constrained to, so a storey
//! height edit re-infers exactly the curtain walls resolved by it. A curtain wall has no layers and no joins; its grid is cut from the axis
//! length and height by the grid rule of each direction (the rule of the wall when it has one of its own, else the rule of its type): equal
//! cells of about a spacing, or the explicit interior grid lines. The panel of every cell is the default panel of the type unless an override
//! addresses the cell (`curtain_panel_overrides`, keyed by wall, `u` and `v`); overrides outside the grid and grid lines outside the extent are reported, never dropped silently.

use super::super::storey_levels::{vertical_of, StoreyLevel};
use crate::standards::v1::subsets::any::schema::authored::plan::axis_length;
use crate::{CurtainGrid, CurtainPanel, CurtainWall, CurtainWallType, ModelSnapshot};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

//#region 🔖️Values
/// 🎯️ The panel of one in-grid cell that its override sets, with the id of the override.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct CellPanel {
    pub u: u32,
    pub v: u32,
    pub panel: CurtainPanel,
    pub id: String,
}

/// 🪞️ Resolved layout of one curtain wall, in metres and square metres: `area` is `length * height` before mullions and openings. The cells lie between the edges `u_edges`
/// (metres from the start of the axis, first `0`, last `length`) and `v_edges` (metres above the base, first `0`, last `height`), so there are `u_panels * v_panels` of them.
/// `panel` is the default panel of the type (none while the type is missing), `overrides` the in-grid overrides ordered by row, column and id, `stray` the ids of the overrides
/// outside the grid, `repeated` the ids of overrides that address a cell already addressed by an earlier id, `ignored_u` and `ignored_v` the grid lines outside the extent or repeated.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct CurtainLayout {
    pub base_z: f64,
    pub top_z: f64,
    pub height: f64,
    pub length: f64,
    pub area: f64,
    pub u_panels: u32,
    pub v_panels: u32,
    pub u_edges: Vec<f64>,
    pub v_edges: Vec<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub panel: Option<CurtainPanel>,
    pub overrides: Vec<CellPanel>,
    pub stray: Vec<String>,
    pub repeated: Vec<String>,
    pub ignored_u: Vec<f64>,
    pub ignored_v: Vec<f64>,
}

impl CurtainLayout {
    /// 🪟️ The panel that fills the cell `(u, v)`: its override, else the default panel of the type.
    pub fn panel_of(&self, u: u32, v: u32) -> Option<&CurtainPanel> {
        self.overrides.iter().find(|cell| cell.u == u && cell.v == v).map(|cell| &cell.panel).or(self.panel.as_ref())
    }

    /// 📏️ The width and height of the cell `(u, v)` in metres, none outside the grid.
    pub fn cell(&self, u: u32, v: u32) -> Option<(f64, f64)> {
        let (width, height) = (self.u_edges.get(u as usize + 1)? - self.u_edges.get(u as usize)?, self.v_edges.get(v as usize + 1)? - self.v_edges.get(v as usize)?);
        Some((width, height))
    }
}
//#endregion 🔖️Values

//#region 🔖️Geometry
/// 🪟️ The depth in metres of the mullions of a curtain wall while its type is missing: the thickness its openings and its body fall back to.
pub const DEFAULT_MULLION_DEPTH: f64 = 0.1;

const EPS: f64 = 1e-9;

fn panels(extent: f64, spacing: f64) -> u32 {
    if spacing > 0.0 && extent > 0.0 {
        ((extent / spacing) - 1e-9).ceil().max(1.0) as u32
    } else {
        1
    }
}

/// 🕸️ The cell edges `0 ..= extent` of one direction under a grid rule and the lines of the rule that were left out (outside the open extent, or repeated). No rule gives one cell.
pub fn edges_of(extent: f64, rule: Option<&CurtainGrid>) -> (Vec<f64>, Vec<f64>) {
    match rule {
        Some(CurtainGrid::Spacing { spacing }) => {
            let count = panels(extent, *spacing);
            ((0..=count).map(|k| if k == count { extent } else { extent * f64::from(k) / f64::from(count) }).collect(), Vec::new())
        }
        Some(CurtainGrid::Lines { positions }) => {
            let mut sorted: Vec<f64> = positions.iter().copied().filter(|position| position.is_finite()).collect();
            sorted.sort_by(f64::total_cmp);
            let (mut kept, mut ignored): (Vec<f64>, Vec<f64>) = (Vec::new(), positions.iter().copied().filter(|position| !position.is_finite()).collect());
            for position in sorted {
                if position > EPS && position < extent - EPS && kept.last().is_none_or(|last| position - last > EPS) {
                    kept.push(position);
                } else {
                    ignored.push(position);
                }
            }
            (std::iter::once(0.0).chain(kept).chain(std::iter::once(extent)).collect(), ignored)
        }
        None => (vec![0.0, extent], Vec::new()),
    }
}

/// 🧮️ The layout of a curtain wall `id` from the levels of the storeys it is resolved by, its type and the overrides that address it.
pub fn curtain_layout_of(snapshot: &ModelSnapshot, id: &str, curtain: &CurtainWall, own: &StoreyLevel, target: Option<&StoreyLevel>) -> CurtainLayout {
    let kind = snapshot.curtain_wall_types.get(&curtain.curtain_wall_type);
    let (base_z, top_z) = vertical_of(curtain.base_offset, &curtain.top, own, target);
    let (height, length) = (top_z - base_z, axis_length(&curtain.axis));
    let ((u_edges, ignored_u), (v_edges, ignored_v)) = (edges_of(length, curtain.u_grid.as_ref().or(kind.map(|kind| &kind.u_grid))), edges_of(height, curtain.v_grid.as_ref().or(kind.map(|kind| &kind.v_grid))));
    let (u_panels, v_panels) = (u_edges.len() as u32 - 1, v_edges.len() as u32 - 1);
    let (mut overrides, mut stray, mut repeated): (Vec<CellPanel>, Vec<String>, Vec<String>) = (Vec::new(), Vec::new(), Vec::new());
    let mut held: BTreeMap<(u32, u32), &str> = BTreeMap::new();
    for (override_id, row) in snapshot.curtain_panel_overrides.iter().filter(|(_, row)| row.curtain == id) {
        if row.u >= u_panels || row.v >= v_panels {
            stray.push(override_id.clone());
        } else if held.insert((row.u, row.v), override_id).is_some() {
            repeated.push(override_id.clone());
        } else {
            overrides.push(CellPanel { u: row.u, v: row.v, panel: row.panel.clone(), id: override_id.clone() });
        }
    }
    overrides.sort_by(|a, b| (a.v, a.u, &a.id).cmp(&(b.v, b.u, &b.id)));
    CurtainLayout { base_z, top_z, height, length, area: length * height, u_panels, v_panels, u_edges, v_edges, panel: kind.map(|kind| kind.panel.clone()), overrides, stray, repeated, ignored_u, ignored_v }
}

/// 🔑️ What `curtain_layout_of` reads of a curtain wall besides the levels: the wall, its type and the overrides that address it.
pub fn dependency(snapshot: &ModelSnapshot, id: &str, curtain: &CurtainWall) -> DslValue {
    use semio_framework_value::ToValue;
    let kind: Option<&CurtainWallType> = snapshot.curtain_wall_types.get(&curtain.curtain_wall_type);
    DslValue::object([
        ("axis".to_string(), curtain.axis.to_value()),
        ("base_offset".to_string(), curtain.base_offset.to_value()),
        ("top".to_string(), curtain.top.to_value()),
        ("u_grid".to_string(), curtain.u_grid.to_value()),
        ("v_grid".to_string(), curtain.v_grid.to_value()),
        ("type_u_grid".to_string(), kind.map(|kind| kind.u_grid.clone()).to_value()),
        ("type_v_grid".to_string(), kind.map(|kind| kind.v_grid.clone()).to_value()),
        ("type_panel".to_string(), kind.map(|kind| kind.panel.clone()).to_value()),
        ("overrides".to_string(), DslValue::object(snapshot.curtain_panel_overrides.iter().filter(|(_, row)| row.curtain == id).map(|(override_id, row)| (override_id.clone(), row.to_value())))),
    ])
}
//#endregion 🔖️Geometry

//#region 🔖️Projection
/// 🪞️ The layout of every curtain wall (the `CurtainLayout` nodes of the model graph).
#[cfg(test)]
pub fn compute_curtain_layout(snapshot: &ModelSnapshot) -> BTreeMap<String, CurtainLayout> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::CURTAINS }>(snapshot).curtain_layout)
}
//#endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
