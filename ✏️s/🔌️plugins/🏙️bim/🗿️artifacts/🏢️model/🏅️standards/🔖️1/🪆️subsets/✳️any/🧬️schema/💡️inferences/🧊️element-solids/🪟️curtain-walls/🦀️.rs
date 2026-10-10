//! 🪟️ Curtain wall solids: a grid of cells cut by the grid rules of the wall (equal cells of about a spacing, or explicit interior lines), a mullion on every grid line (vertical mullions run the
//! full height, horizontal ones span between them, the outer ones sit inside the extent) and the panel of the cell in every cell. The border mullions (the four outer edges) use the border
//! section of the type, the interior ones the interior section, `across` along the wall and `depth` through it, centred on the axis. A panel is a thin flat pane of glass, an opaque panel of a
//! material, a door or a window that fills the cell with its frame, or nothing; the openings hosted by the curtain wall cut their rectangle out of every panel they overlap, the panel is
//! rebuilt from the rectangles that remain.

use super::super::super::curtain_layout::CurtainLayout;
use super::super::super::opening_frames::OpeningCut;
use super::super::fillers::{door_parts, window_parts};
use super::super::plan_kit::seg;
use crate::standards::v1::subsets::any::schema::authored::profile::{profile_extents, profile_polygon};
use super::super::{dep_object, dep_types, dep_value, parts, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::families::FamilyProfiles;
use crate::{CurtainPanel, CurtainWall, CurtainWallType, ModelSnapshot};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::mesh::{extrude, sweep_profile, TriMesh};
use semio_framework_geometry::placement::{Affine3, ZPlane};
use semio_framework_geometry::vector::{perp, unit};
use semio_framework_geometry::{Point, Vec2};
use semio_framework_value::DslValue;

/// 🪟️ Thickness of a glass panel of a curtain wall in metres.
pub const PANEL_THICKNESS: f64 = 0.024;

/// 🧱️ Thickness of an opaque panel of a curtain wall in metres (at most the depth of the mullions).
pub const SOLID_THICKNESS: f64 = 0.06;

const EPS: f64 = 1e-9;

//#region 🔖️Grid
/// 📏️ The `[start, end]` of the member centred on the grid line at `position` over `span`, kept inside `[0, span]`.
pub fn member(span: f64, position: f64, width: f64) -> (f64, f64) {
    let centre = position.clamp(width / 2.0, (span - width / 2.0).max(width / 2.0));
    (centre - width / 2.0, centre + width / 2.0)
}

/// ▭️ A rectangle `[s0, s1, z0, z1]` in the development of the wall: arc length along the axis and height above the base.
pub type Cell = [f64; 4];

/// ✂️ The rectangle without the part an opening cuts out of it: up to four rectangles (left, right, below and above the cut).
pub fn without(rect: Cell, cut: &OpeningCut) -> Vec<Cell> {
    let (s0, s1, z0, z1) = (cut.s_min.max(rect[0]), cut.s_max.min(rect[1]), cut.z_min.max(rect[2]), cut.z_max.min(rect[3]));
    if s1 - s0 <= EPS || z1 - z0 <= EPS {
        return vec![rect];
    }
    [[rect[0], s0, rect[2], rect[3]], [s1, rect[1], rect[2], rect[3]], [s0, s1, rect[2], z0], [s0, s1, z1, rect[3]]].into_iter().filter(|part| part[1] - part[0] > EPS && part[3] - part[2] > EPS).collect()
}

/// ✂️ The rectangle without every cut, in order.
pub fn remaining(rect: Cell, cuts: &[OpeningCut]) -> Vec<Cell> {
    cuts.iter().fold(vec![rect], |rects, cut| rects.into_iter().flat_map(|part| without(part, cut)).collect())
}

/// 🔎️ Whether any cut overlaps the rectangle.
pub fn touched(rect: Cell, cuts: &[OpeningCut]) -> bool {
    cuts.iter().any(|cut| cut.s_max.min(rect[1]) - cut.s_min.max(rect[0]) > EPS && cut.z_max.min(rect[3]) - cut.z_min.max(rect[2]) > EPS)
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

fn panel_mesh(axis: &BulgeSeg, length: f64, rect: Cell, thickness: f64, base_z: f64) -> TriMesh {
    let (a, b) = (axis.point_at(rect[0] / length), axis.point_at(rect[1] / length));
    let Some(direction) = unit(b - a, EPS) else { return TriMesh::new() };
    let half = perp(direction) * (thickness / 2.0);
    extrude(&[a - half, b - half, b + half, a + half], &[], ZPlane::flat(base_z + rect[2]), ZPlane::flat(base_z + rect[3]))
}

/// 🚪️ The placement of a door or window that fills the cell `rect`: the local frame (`x` across the cell, `y` through the wall, `z` up from the lower edge of the cell) at the middle of the cell.
fn cell_frame(axis: &BulgeSeg, length: f64, rect: Cell, base_z: f64) -> Affine3 {
    let (point, tangent, normal) = frame_at(axis, length, (rect[0] + rect[1]) / 2.0);
    Affine3::from_frame([point.x, point.y, base_z + rect[2]], [tangent.x, tangent.y, 0.0], [normal.x, normal.y, 0.0], [0.0, 0.0, 1.0])
}

fn add_panel(builder: &mut SolidBuilder, snapshot: &ModelSnapshot, kind: &CurtainWallType, panel: &CurtainPanel, axis: &BulgeSeg, length: f64, rect: Cell, cuts: &[OpeningCut], depth: f64, base_z: f64, layer: u32) {
    match panel {
        CurtainPanel::Empty => {}
        CurtainPanel::Glass => {
            for part in remaining(rect, cuts) {
                builder.add(parts::PANEL, &kind.panel_material, 0, &panel_mesh(axis, length, part, PANEL_THICKNESS.min(depth), base_z));
            }
        }
        CurtainPanel::Solid { material } => {
            for part in remaining(rect, cuts) {
                builder.add(parts::PANEL, material, 0, &panel_mesh(axis, length, part, SOLID_THICKNESS.min(depth), base_z));
            }
        }
        CurtainPanel::Window { window_type } => {
            if let (false, Some(window)) = (touched(rect, cuts), snapshot.window_types.get(window_type)) {
                let place = cell_frame(axis, length, rect, base_z);
                for (part, glazing, mesh) in window_parts(window, rect[1] - rect[0], rect[3] - rect[2], 0.0) {
                    builder.add(part, if glazing { &kind.panel_material } else { &window.material }, layer, &mesh.transformed(&place));
                }
            }
        }
        CurtainPanel::Door { door_type } => {
            if let (false, Some(door)) = (touched(rect, cuts), snapshot.door_types.get(door_type)) {
                let place = cell_frame(axis, length, rect, base_z);
                for (part, mesh) in door_parts(door, rect[1] - rect[0], rect[3] - rect[2], 0.0) {
                    builder.add(part, &door.material, layer, &mesh.transformed(&place));
                }
            }
        }
    }
}

/// 🔖️ The layer of the parts of the door or window that fills the cell `(u, v)`: one more than the row-major index of the cell, so every filler of a wall is one group family of its own (all other parts are layer `0`).
pub fn filler_layer(layout: &CurtainLayout, u: u32, v: u32) -> u32 {
    1 + v * layout.u_panels + u
}

/// 🔖️ The cell `(u, v)` a filler layer names, none for layer `0`.
pub fn filler_cell(layout: &CurtainLayout, layer: u32) -> Option<(u32, u32)> {
    let index = layer.checked_sub(1)?;
    (layout.u_panels > 0).then(|| (index % layout.u_panels, index / layout.u_panels))
}

/// 🧱️ The mullion members of a curtain wall: the clear interval `(start, end)` of the member on every grid edge along the wall (`verticals`) and up it (`horizontals`), the polygons and sizes of the
/// border and interior sections. The border members are the first and the last edge of each direction.
pub struct Members {
    pub verticals: Vec<(f64, f64)>,
    pub horizontals: Vec<(f64, f64)>,
    pub border: Vec<Point>,
    pub interior: Vec<Point>,
    pub border_size: (f64, f64),
    pub interior_size: (f64, f64),
}

impl Members {
    /// ▭️ The section polygon of the member on edge `k` of `last + 1` edges.
    pub fn outline(&self, k: usize, last: usize) -> &[Point] {
        if k == 0 || k == last {
            &self.border
        } else {
            &self.interior
        }
    }

    /// ▭️ The clear rectangle of the cell `(u, v)` between its members, none when a member eats it.
    pub fn cell(&self, u: usize, v: usize) -> Option<Cell> {
        let rect = [self.verticals.get(u)?.1, self.verticals.get(u + 1)?.0, self.horizontals.get(v)?.1, self.horizontals.get(v + 1)?.0];
        (rect[1] - rect[0] > EPS && rect[3] - rect[2] > EPS).then_some(rect)
    }
}

/// 🧱️ The members of a curtain wall from its type and layout; none when the wall has no extent or a section has no size.
pub fn members_of(kind: &CurtainWallType, layout: &CurtainLayout, profiles: &FamilyProfiles<'_>) -> Option<Members> {
    let (border, interior) = (profile_polygon(&profiles.resolve(&kind.border_mullion)), profile_polygon(&profiles.resolve(&kind.interior_mullion)));
    let (border_size, interior_size) = (profile_extents(&border), profile_extents(&interior));
    if layout.length <= EPS || layout.height <= EPS || [border_size, interior_size].iter().any(|(across, depth)| *across <= EPS || *depth <= EPS) {
        return None;
    }
    let across = |k: usize, last: usize| if k == 0 || k == last { border_size.0 } else { interior_size.0 };
    let (last_u, last_v) = (layout.u_edges.len() - 1, layout.v_edges.len() - 1);
    let verticals = layout.u_edges.iter().enumerate().map(|(k, edge)| member(layout.length, *edge, across(k, last_u))).collect();
    let horizontals = layout.v_edges.iter().enumerate().map(|(k, edge)| member(layout.height, *edge, across(k, last_v))).collect();
    Some(Members { verticals, horizontals, border, interior, border_size, interior_size })
}

/// 🧊️ The mullions and panels of one curtain wall from its layout and the cut rectangles of the openings it hosts; absent without a type.
pub fn curtain_solid(snapshot: &ModelSnapshot, curtain: &CurtainWall, layout: &CurtainLayout, cuts: &[OpeningCut]) -> ElementSolid {
    curtain_solid_in(snapshot, curtain, layout, cuts, &FamilyProfiles::new())
}

/// 🧊️ [`curtain_solid`] where a type whose mullions name a profile family gets that family's outline from `profiles`.
pub fn curtain_solid_in(snapshot: &ModelSnapshot, curtain: &CurtainWall, layout: &CurtainLayout, cuts: &[OpeningCut], profiles: &FamilyProfiles<'_>) -> ElementSolid {
    let (base_z, height) = (layout.base_z, layout.height);
    let mut builder = SolidBuilder::new(SolidFamily::CurtainWall);
    let Some(kind) = snapshot.curtain_wall_types.get(&curtain.curtain_wall_type) else { return builder.build() };
    let Some(members) = members_of(kind, layout, profiles) else { return builder.build() };
    let axis = seg(&curtain.axis);
    let length = layout.length;
    let (last_u, last_v) = (members.verticals.len() - 1, members.horizontals.len() - 1);
    for (k, (start, end)) in members.verticals.iter().enumerate() {
        builder.add(parts::MULLION, &kind.mullion_material, 0, &vertical_member(&axis, length, members.outline(k, last_u), (start + end) / 2.0, base_z, base_z + height));
    }
    for (k, (start, end)) in members.horizontals.iter().enumerate() {
        for pair in members.verticals.windows(2) {
            if pair[1].0 - pair[0].1 > EPS {
                builder.add(parts::MULLION, &kind.mullion_material, 0, &horizontal_member(&axis, length, members.outline(k, last_v), pair[0].1, pair[1].0, (start + end) / 2.0, base_z));
            }
        }
    }
    let depth = members.border_size.1.min(members.interior_size.1);
    for v in 0..layout.v_panels as usize {
        for u in 0..layout.u_panels as usize {
            if let (Some(rect), Some(panel)) = (members.cell(u, v), layout.panel_of(u as u32, v as u32)) {
                add_panel(&mut builder, snapshot, kind, panel, &axis, length, rect, cuts, depth, base_z, filler_layer(layout, u as u32, v as u32));
            }
        }
    }
    builder.build()
}

/// 🧮️ How many panels of one kind a curtain wall has and the clear area they cover.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PanelTakeoff {
    pub kind: &'static str,
    pub count: u32,
    pub area: f64,
}

/// 🧮️ How many mullion pieces of one section a curtain wall has and their total length.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MullionTakeoff {
    pub kind: &'static str,
    pub count: u32,
    pub length: f64,
}

/// 🧮️ The panels by kind (the clear cell area left by the members and cut by the hosted openings; a door or window panel overlapped by an opening is not drawn and not counted) and the mullions by section.
pub fn takeoff(snapshot: &ModelSnapshot, curtain: &CurtainWall, layout: &CurtainLayout, cuts: &[OpeningCut], profiles: &FamilyProfiles<'_>) -> (Vec<PanelTakeoff>, Vec<MullionTakeoff>) {
    let Some(members) = snapshot.curtain_wall_types.get(&curtain.curtain_wall_type).and_then(|kind| members_of(kind, layout, profiles)) else { return (Vec::new(), Vec::new()) };
    let mut panels: Vec<PanelTakeoff> = ["glass", "solid", "door", "window", "empty"].into_iter().map(|kind| PanelTakeoff { kind, count: 0, area: 0.0 }).collect();
    for v in 0..layout.v_panels as usize {
        for u in 0..layout.u_panels as usize {
            let (Some(rect), Some(panel)) = (members.cell(u, v), layout.panel_of(u as u32, v as u32)) else { continue };
            let whole = (rect[1] - rect[0]) * (rect[3] - rect[2]);
            let left = || remaining(rect, cuts).iter().map(|part| (part[1] - part[0]) * (part[3] - part[2])).sum::<f64>();
            let (slot, area) = match panel {
                CurtainPanel::Glass => (0, left()),
                CurtainPanel::Solid { .. } => (1, left()),
                CurtainPanel::Door { .. } if !touched(rect, cuts) => (2, whole),
                CurtainPanel::Window { .. } if !touched(rect, cuts) => (3, whole),
                CurtainPanel::Empty => (4, whole),
                CurtainPanel::Door { .. } | CurtainPanel::Window { .. } => continue,
            };
            panels[slot].count += 1;
            panels[slot].area += area;
        }
    }
    let (last_u, last_v) = (members.verticals.len() - 1, members.horizontals.len() - 1);
    let clear: f64 = members.verticals.windows(2).map(|pair| (pair[1].0 - pair[0].1).max(0.0)).sum();
    let gaps = members.verticals.windows(2).filter(|pair| pair[1].0 - pair[0].1 > EPS).count() as u32;
    let mut mullions = vec![MullionTakeoff { kind: "interior", count: 0, length: 0.0 }, MullionTakeoff { kind: "border", count: 0, length: 0.0 }];
    for k in 0..=last_u {
        let slot = usize::from(k == 0 || k == last_u);
        mullions[slot].count += 1;
        mullions[slot].length += layout.height;
    }
    for k in 0..=last_v {
        let slot = usize::from(k == 0 || k == last_v);
        mullions[slot].count += gaps;
        mullions[slot].length += clear;
    }
    (panels, mullions)
}

/// 🎨️ The materials a curtain wall names: its type's glass and mullion material, the materials of solid panels and of the door and window types its panels use.
pub fn materials_of(snapshot: &ModelSnapshot, id: &str, curtain: &CurtainWall) -> std::collections::BTreeSet<String> {
    let kind = snapshot.curtain_wall_types.get(&curtain.curtain_wall_type);
    let mut used: std::collections::BTreeSet<String> = kind.into_iter().flat_map(|kind| [kind.panel_material.clone(), kind.mullion_material.clone()]).collect();
    let panels = kind.map(|kind| &kind.panel).into_iter().chain(snapshot.curtain_panel_overrides.values().filter(|row| row.curtain == id).map(|row| &row.panel));
    for panel in panels {
        match panel {
            CurtainPanel::Solid { material } => {
                used.insert(material.clone());
            }
            CurtainPanel::Door { door_type } => used.extend(snapshot.door_types.get(door_type).map(|row| row.material.clone())),
            CurtainPanel::Window { window_type } => used.extend(snapshot.window_types.get(window_type).map(|row| row.material.clone())),
            CurtainPanel::Glass | CurtainPanel::Empty => {}
        }
    }
    used
}

/// 🔑️ What `curtain_solid` reads of a curtain wall besides its layout and the frames of its openings: the record, its type and the door and window types its panels name.
pub fn dependency(snapshot: &ModelSnapshot, id: &str, curtain: &CurtainWall) -> DslValue {
    let panels = snapshot.curtain_wall_types.get(&curtain.curtain_wall_type).map(|kind| &kind.panel).into_iter().chain(snapshot.curtain_panel_overrides.values().filter(|row| row.curtain == id).map(|row| &row.panel));
    let (mut doors, mut windows) = (Vec::new(), Vec::new());
    for panel in panels {
        match panel {
            CurtainPanel::Door { door_type } => doors.push(door_type),
            CurtainPanel::Window { window_type } => windows.push(window_type),
            _ => {}
        }
    }
    dep_object([
        ("curtain_wall", dep_value(curtain)),
        ("type", dep_value(&snapshot.curtain_wall_types.get(&curtain.curtain_wall_type).cloned())),
        ("door_types", dep_types(doors, &snapshot.door_types)),
        ("window_types", dep_types(windows, &snapshot.window_types)),
    ])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
