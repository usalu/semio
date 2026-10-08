//! 🪟️ Curtain wall solids: a regular grid of `u_spacing × v_spacing` cells (the spacings are nominal; each axis is divided into equal cells), a mullion on every grid line
//! (vertical mullions run the full height, horizontal ones span between them, the outer ones sit inside the extent) and a thin flat panel in every cell. Mullions use the
//! authored profile, `across` along the wall and `depth` through it, centred on the axis; panels are [`PANEL_THICKNESS`] thick and centred as well.

use super::super::super::curtain_layout::CurtainLayout;
use super::super::plan_kit::seg;
use super::super::{dep_object, dep_value, parts, profile_extents, profile_polygon, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::CurtainWall;
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::mesh::{extrude, sweep_profile, TriMesh};
use semio_framework_geometry::placement::ZPlane;
use semio_framework_geometry::vector::{perp, unit};
use semio_framework_geometry::{Point, Vec2};
use semio_framework_value::DslValue;

/// 🪟️ Thickness of a curtain wall panel in metres.
pub const PANEL_THICKNESS: f64 = 0.024;

const EPS: f64 = 1e-9;

//#region 🔖️Grid
/// 📏️ The `[start, end]` of the member on grid line `k` of `n` over `span`: centred on the line, kept inside `[0, span]`.
pub fn member(span: f64, n: usize, k: usize, width: f64) -> (f64, f64) {
    let centre = (span * k as f64 / n as f64).clamp(width / 2.0, (span - width / 2.0).max(width / 2.0));
    (centre - width / 2.0, centre + width / 2.0)
}
//#endregion 🔖️Grid

//#region 🔖️Solid
fn frame_at(axis: &BulgeSeg, length: f64, s: f64) -> (Point, Vec2, Vec2) {
    let t = (s / length).clamp(0.0, 1.0);
    let tangent = axis.tangent_at(t);
    (axis.point_at(t), tangent, perp(tangent))
}

fn vertical_member(axis: &BulgeSeg, length: f64, outline: &[Point], s: f64, bottom: f64, top: f64) -> TriMesh {
    let (point, tangent, normal) = frame_at(axis, length, s);
    let plan: Vec<Point> = outline.iter().map(|p| point + tangent * p.x + normal * p.y).collect();
    extrude(&plan, &[], ZPlane::flat(bottom), ZPlane::flat(top))
}

fn horizontal_member(axis: &BulgeSeg, length: f64, outline: &[Point], from: f64, to: f64, centre: f64, base_z: f64) -> TriMesh {
    let profile: Vec<Point> = outline.iter().map(|p| Point::new(p.y, centre + p.x)).collect();
    sweep_profile(&profile, &[], &[axis.subsegment(from / length, to / length)], base_z, CHORD_TOLERANCE)
}

fn panel(axis: &BulgeSeg, length: f64, from: f64, to: f64, thickness: f64, bottom: f64, top: f64) -> TriMesh {
    let (a, b) = (axis.point_at(from / length), axis.point_at(to / length));
    let Some(direction) = unit(b - a, EPS) else { return TriMesh::new() };
    let half = perp(direction) * (thickness / 2.0);
    extrude(&[a - half, b - half, b + half, a + half], &[], ZPlane::flat(bottom), ZPlane::flat(top))
}

/// 🧊️ The mullions and panels of one curtain wall from its layout.
pub fn curtain_solid(curtain: &CurtainWall, layout: &CurtainLayout) -> ElementSolid {
    let (base_z, height) = (layout.base_z, layout.height);
    let mut builder = SolidBuilder::new(SolidFamily::CurtainWall);
    let axis = seg(&curtain.axis);
    let length = layout.length;
    let outline = profile_polygon(&curtain.mullion);
    let (across, depth) = profile_extents(&outline);
    if length <= EPS || height <= EPS || across <= EPS || depth <= EPS {
        return builder.build();
    }
    let (columns, rows) = (layout.u_panels as usize, layout.v_panels as usize);
    let verticals: Vec<(f64, f64)> = (0..=columns).map(|k| member(length, columns, k, across)).collect();
    let horizontals: Vec<(f64, f64)> = (0..=rows).map(|k| member(height, rows, k, across)).collect();
    let (mullion, panel_material) = (&curtain.mullion_material, &curtain.panel_material);
    for (start, end) in &verticals {
        builder.add(parts::MULLION, mullion, 0, &vertical_member(&axis, length, &outline, (start + end) / 2.0, base_z, base_z + height));
    }
    for (start, end) in &horizontals {
        for pair in verticals.windows(2) {
            if pair[1].0 - pair[0].1 > EPS {
                builder.add(parts::MULLION, mullion, 0, &horizontal_member(&axis, length, &outline, pair[0].1, pair[1].0, (start + end) / 2.0, base_z));
            }
        }
    }
    let thickness = PANEL_THICKNESS.min(depth);
    for columns in verticals.windows(2) {
        for rows in horizontals.windows(2) {
            let (from, to, bottom, top) = (columns[0].1, columns[1].0, rows[0].1, rows[1].0);
            if to - from > EPS && top - bottom > EPS {
                builder.add(parts::PANEL, panel_material, 0, &panel(&axis, length, from, to, thickness, base_z + bottom, base_z + top));
            }
        }
    }
    builder.build()
}

/// 🔑️ What `curtain_solid` reads of a curtain wall besides its layout: the record (profile, materials, axis).
pub fn dependency(curtain: &CurtainWall) -> DslValue {
    dep_object([("curtain_wall", dep_value(curtain))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
