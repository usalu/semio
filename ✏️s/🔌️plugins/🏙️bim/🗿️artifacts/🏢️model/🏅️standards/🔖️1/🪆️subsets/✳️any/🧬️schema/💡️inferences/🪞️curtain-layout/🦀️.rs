//! 🪞️ `curtain-layout`: the resolved vertical extent, axis length and panel grid of every curtain wall. Like `wall-layout` it is a
//! node of the model graph over storeys: a curtain wall depends on its own storey and on the storey its top is constrained to, so a storey
//! height edit re-infers exactly the curtain walls resolved by it. A curtain wall has no layers and no joins; its grid is cut from the axis
//! length and height by the authored `u_spacing` (along the axis) and `v_spacing` (up).

use super::super::storey_levels::{vertical_of, StoreyLevel};
use super::super::wall_layout::axis_length;
use crate::{CurtainWall, ModelSnapshot};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

//#region 🔖️Values
/// 🪞️ Resolved layout of one curtain wall, in metres and square metres: `area` is `length * height` before mullions and openings; the grid has `u_panels * v_panels` equal panels of `panel_width` by `panel_height`.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct CurtainLayout {
    pub base_z: f64,
    pub top_z: f64,
    pub height: f64,
    pub length: f64,
    pub area: f64,
    pub u_panels: u32,
    pub v_panels: u32,
    pub panel_width: f64,
    pub panel_height: f64,
}
//#endregion 🔖️Values

//#region 🔖️Geometry
fn panels(extent: f64, spacing: f64) -> u32 {
    if spacing > 0.0 && extent > 0.0 {
        ((extent / spacing) - 1e-9).ceil().max(1.0) as u32
    } else {
        1
    }
}

/// 🧮️ The layout of a curtain wall from the levels of the storeys it is resolved by.
pub fn curtain_layout_of(curtain: &CurtainWall, own: &StoreyLevel, target: Option<&StoreyLevel>) -> CurtainLayout {
    let (base_z, top_z) = vertical_of(curtain.base_offset, &curtain.top, own, target);
    let (height, length) = (top_z - base_z, axis_length(&curtain.axis));
    let (u_panels, v_panels) = (panels(length, curtain.u_spacing), panels(height, curtain.v_spacing));
    CurtainLayout { base_z, top_z, height, length, area: length * height, u_panels, v_panels, panel_width: length / f64::from(u_panels), panel_height: height / f64::from(v_panels) }
}

/// 🔑️ What `curtain_layout_of` reads of a curtain wall.
pub fn dependency(curtain: &CurtainWall) -> DslValue {
    use semio_framework_value::ToValue;
    DslValue::object([
        ("axis".to_string(), curtain.axis.to_value()),
        ("base_offset".to_string(), curtain.base_offset.to_value()),
        ("top".to_string(), curtain.top.to_value()),
        ("u_spacing".to_string(), curtain.u_spacing.to_value()),
        ("v_spacing".to_string(), curtain.v_spacing.to_value()),
    ])
}
//#endregion 🔖️Geometry

//#region 🔖️Projection
/// 🪞️ The layout of every curtain wall (the `CurtainLayout` nodes of the model graph).
pub fn compute_curtain_layout(snapshot: &ModelSnapshot) -> BTreeMap<String, CurtainLayout> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::CURTAINS }>(snapshot).curtain_layout)
}
//#endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
