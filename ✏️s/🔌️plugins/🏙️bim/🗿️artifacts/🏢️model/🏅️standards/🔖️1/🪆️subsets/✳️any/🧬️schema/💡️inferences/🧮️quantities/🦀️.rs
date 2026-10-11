//! 🧮️ `quantities`: the quantity take-off (QTO) of the model. Everything is derived from the already inferred nodes of the model graph
//! (layouts, frames, rooms, runs, solids) and the authored types; nothing is stored. One [`ElementQuantity`] per element (a `Quantity` node),
//! summed per kind, per type and per material ([`QuantityTotals`], the `Totals` nodes) for every storey, every building and the project.
//!
//! Definitions. Lengths in metres, areas in square metres, volumes in cubic metres, masses in kilograms (`volume * density`).
//! * wall: `length` centreline, `width` thickness, `height`; `gross_side_area = length * height` (one side, centreline), `opening_area`
//!   the union of the cut rectangles of the **valid** hosted openings (exactly the holes the wall solid cuts), `net_side_area = gross - opening`; `gross_area` the join-trimmed footprint;
//!   `gross_volume` the footprint times the height, `net_volume` without the openings; `layers` split the net volume by layer.
//! * curtain wall: as a wall without layers; the volume and the material rows come from its solid (mullions and panels).
//! * slab: `gross_area` the boundary, `net_area` without holes, `surface_area` the sloped top, `width` the layer thickness (vertical),
//!   `net_volume = net_area * width`.
//! * ceiling: as a slab: `gross_area` the boundary, `net_area` without holes, `surface_area` the sloped underside, `width` the layer thickness (vertical),
//!   `net_volume = net_area * width`.
//! * roof: `gross_area` the eave outline (footprint grown by the overhang), `surface_area` the upward faces of the outermost layer of
//!   its solid, volumes and layers from the solid.
//! * column and beam: `length` the height or the axis length, `perimeter` and `gross_area` the profile, `net_volume = area * length`.
//! * window, door, void: `gross_area = width * height` of the opening, material rows from the filler solid.
//! * stair: `risers`, `length` the run, `height` the rise, volume from the solid (treads, risers, stringers and landings). railing: `length` the path, material rows from the solid, `surface_area` the area of one side of the infill
//!   (its volume over its thickness), `balusters` the number of balusters.
//! * space: `gross_area` the room, `net_area` without columns, `height` the clear height, `net_volume` the room volume, `finishes` the floor, wall and ceiling finish areas of the room (`finishes`).

use super::super::components::ComponentValue;
use super::super::curtain_layout::CurtainLayout;
use super::super::mep::{self, MepValue};
use super::super::element_solids::columns::profile_loop;
use super::super::element_solids::plan_kit::seg;
use super::super::element_solids::railings::baluster_count;
use super::super::element_solids::{curtain_walls, dep_object, dep_value, parts, rail_hosts, wall_sweeps, walls, ElementSolid};
use super::super::families::FamilyProfiles;
use super::super::finishes::{self, FinishQuantity};
use super::super::opening_frames::{OpeningCut, OpeningFrame};
use super::super::spaces::{SpaceRoom, SpaceStatus};
use super::super::ramp_runs::{strip_of, RampRun};
use super::super::stair_runs::StairRun;
use super::super::storey_levels::{vertical_of, StoreyLevel};
use super::super::wall_layout::WallLayout;
use super::super::ModelInference;
use crate::{Beam, Ceiling, Column, Component, CurtainWall, Infill, Layer, ModelSnapshot, Opening, OpeningKind, Phase, Railing, Ramp, Roof, Slab, Space, Stair, Vertex, Wall, WallSweep};
use semio_framework_geometry::loops;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet};

/// 🗺️ The snapshot collections the take-off reads, directly or through the fields it is computed from.
pub const READS: &[&str] = &["components", "component_overrides", "mep_elements", "families", "family_parameters", "family_solids", "walls","wall_sweeps", "wall_types", "curtain_walls", "curtain_wall_types", "curtain_panel_overrides", "slabs", "slab_types", "ceilings", "ceiling_types", "roofs", "roof_types", "columns", "column_types", "beams", "beam_types", "openings", "window_types", "door_types", "stairs", "ramps", "railings", "spaces", "materials", "storeys", "buildings", "sites"];

//#region 🔖️Values
/// 🗂️ Which kind of element a quantity row measures.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum QuantityKind {
    #[default]
    Wall,
    CurtainWall,
    Slab,
    Roof,
    Column,
    Beam,
    Window,
    Door,
    Void,
    Stair,
    Railing,
    Ramp,
    Space,
    Ceiling,
    WallSweep,
    Component,
    Mep,
}

impl QuantityKind {
    /// 🏷️ The stable key of the kind in the totals tables.
    pub fn key(self) -> &'static str {
        match self {
            Self::Wall => "wall",
            Self::CurtainWall => "curtain-wall",
            Self::Slab => "slab",
            Self::Roof => "roof",
            Self::Column => "column",
            Self::Beam => "beam",
            Self::Window => "window",
            Self::Door => "door",
            Self::Void => "void",
            Self::Stair => "stair",
            Self::Railing => "railing",
            Self::Ramp => "ramp",
            Self::Space => "space",
            Self::Ceiling => "ceiling",
            Self::WallSweep => "wall-sweep",
            Self::Component => "component",
            Self::Mep => "mep",
        }
    }
}

/// 🍰️ One material row of an element: a layer of a layered type, or a material run of a solid. `thickness` is 0 for solid runs.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct LayerQuantity {
    pub material: String,
    pub thickness: f64,
    pub area: f64,
    pub volume: f64,
    pub mass: f64,
}

/// 🪟️ The panels of one kind of a curtain wall: how many cells hold one and the clear area they cover (`glass`, `solid`, `door`, `window`, `empty`).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct PanelQuantity {
    pub kind: String,
    pub count: u32,
    pub area: f64,
}

/// 🪛️ The mullion pieces of one section of a curtain wall: how many pieces and their total length (`interior` and `border`).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct MullionQuantity {
    pub kind: String,
    pub count: u32,
    pub length: f64,
}

/// 🧮️ The quantities of one element; measures that do not apply to its kind are 0.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct ElementQuantity {
    pub kind: QuantityKind,
    pub storey: String,
    pub phase: Phase,
    pub type_id: String,
    pub count: u32,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub perimeter: f64,
    pub gross_side_area: f64,
    pub opening_area: f64,
    pub net_side_area: f64,
    pub gross_area: f64,
    pub net_area: f64,
    pub surface_area: f64,
    pub gross_volume: f64,
    pub net_volume: f64,
    pub mass: f64,
    pub risers: u32,
    #[value(default, skip_serializing_if = "no_balusters")]
    pub balusters: u32,
    pub layers: Vec<LayerQuantity>,
    pub finishes: Vec<FinishQuantity>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub panels: Vec<PanelQuantity>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mullions: Vec<MullionQuantity>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
}

fn no_balusters(count: &u32) -> bool {
    *count == 0
}

impl ElementQuantity {
    /// 📐️ The primary area of the element in a total: the net side area of a wall, the net plan area of everything else.
    pub fn area(&self) -> f64 {
        match self.kind {
            QuantityKind::Wall | QuantityKind::CurtainWall => self.net_side_area,
            _ => self.net_area,
        }
    }
}

/// ➕️ Sums of element quantities: `area` is [`ElementQuantity::area`], `volume` the net volume; material totals sum the layer rows.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct Totals {
    pub count: u32,
    pub length: f64,
    pub area: f64,
    pub volume: f64,
    pub mass: f64,
}

/// ➕️ Totals per kind (`wall`, `slab`, …), per type (`wall:wt-300`), per material id, per finish (`wall:m-paint`: the area of one surface finished with one material) and per construction phase (`new`; `phase_kinds` splits each phase by kind, `demolished:wall`).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct QuantityTotals {
    pub kinds: BTreeMap<String, Totals>,
    pub types: BTreeMap<String, Totals>,
    pub materials: BTreeMap<String, Totals>,
    pub finishes: BTreeMap<String, Totals>,
    pub phases: BTreeMap<String, Totals>,
    pub phase_kinds: BTreeMap<String, Totals>,
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub groups: BTreeMap<String, Totals>,
}

/// 🧮️ The whole take-off: elements by id, totals per storey, per building and for the project.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ModelQuantities {
    pub elements: BTreeMap<String, ElementQuantity>,
    pub storeys: BTreeMap<String, QuantityTotals>,
    pub buildings: BTreeMap<String, QuantityTotals>,
    pub project: QuantityTotals,
}
//#endregion 🔖️Values

//#region 🔖️Measures
fn density(snapshot: &ModelSnapshot, material: &str) -> f64 {
    snapshot.materials.get(material).map_or(0.0, |row| row.density)
}

fn plan(vertices: &[Vertex]) -> Vec<loops::Vertex> {
    vertices.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect()
}

fn layer_rows(snapshot: &ModelSnapshot, layers: &[Layer], areas: &[f64], volumes: &[f64]) -> Vec<LayerQuantity> {
    layers.iter().enumerate().map(|(index, layer)| LayerQuantity { material: layer.material.clone(), thickness: layer.thickness, area: areas[index], volume: volumes[index], mass: volumes[index] * density(snapshot, &layer.material) }).collect()
}

fn mass_of(rows: &[LayerQuantity]) -> f64 {
    rows.iter().map(|row| row.mass).sum()
}

/// 🧊️ The volume of every group of a solid: the signed tetrahedron sum over its triangles, exact for closed parts.
pub fn group_volumes(solid: &ElementSolid) -> Vec<f64> {
    let vertex = |index: u32| {
        let at = 3 * index as usize;
        [solid.positions[at], solid.positions[at + 1], solid.positions[at + 2]]
    };
    let mut volumes = vec![0.0; solid.groups.len()];
    for (triangle, group) in solid.face_groups.iter().enumerate() {
        let [a, b, c] = [vertex(solid.indices[3 * triangle]), vertex(solid.indices[3 * triangle + 1]), vertex(solid.indices[3 * triangle + 2])];
        volumes[*group as usize] += (a[0] * (b[1] * c[2] - b[2] * c[1]) + a[1] * (b[2] * c[0] - b[0] * c[2]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0;
    }
    volumes
}

/// 🧊️ The area of the upward faces (normal `z > 0`) of one layer of a solid.
pub fn upward_area(solid: &ElementSolid, layer: u32) -> f64 {
    let vertex = |index: u32| {
        let at = 3 * index as usize;
        [solid.positions[at], solid.positions[at + 1], solid.positions[at + 2]]
    };
    solid
        .face_groups
        .iter()
        .enumerate()
        .filter(|(_, group)| solid.groups[**group as usize].layer == layer)
        .map(|(triangle, _)| {
            let [a, b, c] = [vertex(solid.indices[3 * triangle]), vertex(solid.indices[3 * triangle + 1]), vertex(solid.indices[3 * triangle + 2])];
            let (u, v) = ([b[0] - a[0], b[1] - a[1]], [c[0] - a[0], c[1] - a[1]]);
            (u[0] * v[1] - u[1] * v[0]) / 2.0
        })
        .filter(|area| *area > 0.0)
        .sum()
}

fn solid_rows(snapshot: &ModelSnapshot, solid: &ElementSolid) -> Vec<LayerQuantity> {
    let volumes = group_volumes(solid);
    let mut rows: BTreeMap<(u32, String), f64> = BTreeMap::new();
    for (group, volume) in solid.groups.iter().zip(volumes) {
        *rows.entry((group.layer, group.material.clone())).or_default() += volume;
    }
    rows.into_iter().map(|((_, material), volume)| LayerQuantity { mass: volume * density(snapshot, &material), material, thickness: 0.0, area: 0.0, volume }).collect()
}

fn scale_at(axis: &crate::Axis, offset: f64) -> f64 {
    let segment = seg(axis);
    if segment.bulge == 0.0 || !segment.radius().is_finite() {
        1.0
    } else {
        1.0 - segment.bulge.signum() * offset / segment.radius()
    }
}

fn face_length(layout: &WallLayout, offset: f64) -> f64 {
    let span = layout.offset_left + layout.offset_right;
    if span <= 0.0 {
        layout.left_length
    } else {
        layout.right_length + (layout.left_length - layout.right_length) * (offset + layout.offset_right) / span
    }
}

/// ✂️ The area of the union of rectangles `(s_min, s_max, z_min, z_max)`: overlaps count once.
pub fn union_area(rects: &[(f64, f64, f64, f64)]) -> f64 {
    let mut marks: Vec<f64> = rects.iter().flat_map(|rect| [rect.0, rect.1]).collect();
    marks.sort_by(f64::total_cmp);
    marks.dedup();
    marks
        .windows(2)
        .map(|pair| {
            let mut spans: Vec<(f64, f64)> = rects.iter().filter(|rect| rect.0 <= pair[0] && rect.1 >= pair[1]).map(|rect| (rect.2, rect.3)).collect();
            spans.sort_by(|a, b| a.0.total_cmp(&b.0));
            let (mut covered, mut end) = (0.0, f64::NEG_INFINITY);
            for (from, to) in spans {
                let from = from.max(end);
                if to > from {
                    covered += to - from;
                }
                end = end.max(to);
            }
            covered * (pair[1] - pair[0])
        })
        .sum()
}

/// ✂️ The area the host solid cuts out of its side: the union of the cut rectangles of the **valid** frames, exactly the holes and notches `element-solids` cuts.
pub fn cut_area<'a>(frames: impl IntoIterator<Item = &'a OpeningFrame>) -> f64 {
    union_area(&frames.into_iter().filter(|frame| frame.valid).map(|frame| (frame.cut.s_min, frame.cut.s_max, frame.cut.z_min, frame.cut.z_max)).collect::<Vec<_>>())
}
//#endregion 🔖️Measures

//#region 🔖️Elements
/// 🧱️ The quantities of a wall from its layout and the frames of its hosted openings.
pub fn wall_quantity(snapshot: &ModelSnapshot, wall: &Wall, layout: &WallLayout, frames: &[&OpeningFrame]) -> ElementQuantity {
    let opening_area = cut_area(frames.iter().copied());
    let kind = snapshot.wall_types.get(&wall.wall_type);
    let layers: &[Layer] = kind.map_or(&[], |kind| &kind.layers);
    let ratio = |index: usize| -> f64 { (layout.layer_offsets.get(index).copied().unwrap_or(0.0) + layout.layer_offsets.get(index + 1).copied().unwrap_or(0.0)) / 2.0 };
    let raw: Vec<f64> = layers.iter().enumerate().map(|(index, layer)| (face_length(layout, layout.layer_offsets.get(index).copied().unwrap_or(0.0)) + face_length(layout, layout.layer_offsets.get(index + 1).copied().unwrap_or(0.0))) / 2.0 * layer.thickness).collect();
    let normal = if raw.iter().sum::<f64>() > 0.0 { layout.footprint_area / raw.iter().sum::<f64>() } else { 0.0 };
    let areas: Vec<f64> = raw.iter().map(|area| area * normal).collect();
    let height = if layout.top_profile.is_empty() && layout.base_profile.is_empty() || layout.footprint_area <= 0.0 { layout.height } else { layout.volume / layout.footprint_area };
    let volumes: Vec<f64> = layers.iter().enumerate().map(|(index, layer)| (areas[index] * height - opening_area * layer.thickness * scale_at(&wall.axis, ratio(index))).max(0.0)).collect();
    let rows = layer_rows(snapshot, layers, &areas, &volumes);
    let net_volume = if rows.is_empty() { (layout.volume - opening_area * layout.thickness).max(0.0) } else { volumes.iter().sum() };
    ElementQuantity {
        kind: QuantityKind::Wall,
        storey: wall.storey.clone(), phase: wall.phase,
        type_id: wall.wall_type.clone(),
        count: 1,
        length: layout.length,
        width: layout.thickness,
        height: layout.height,
        perimeter: loops::perimeter(&plan(&layout.footprint)),
        gross_side_area: layout.side_area,
        opening_area,
        net_side_area: (layout.side_area - opening_area).max(0.0),
        gross_area: layout.footprint_area,
        net_area: layout.footprint_area,
        gross_volume: layout.volume,
        net_volume,
        mass: mass_of(&rows),
        layers: rows,
        ..ElementQuantity::default()
    }
}

/// 🧷️ The quantities of a wall sweep: `length` the path along the face less the stretches its openings interrupt, `width` and `height` the extents of the profile (out of the wall and up), `perimeter` the visible outline of the
/// profile, `gross_area` the cross-section, `net_area` and `surface_area` the visible surface (outline times length), `gross_volume` the cross-section times the length and `net_volume` the volume of the solid.
pub fn sweep_quantity(snapshot: &ModelSnapshot, sweep: &WallSweep, wall: &Wall, layout: &WallLayout, frames: &[&OpeningFrame], solid: Option<&ElementSolid>) -> ElementQuantity {
    let length = wall_sweeps::path_length(sweep, wall, layout, &walls::cuts_of(frames.iter().copied()));
    let (protrusion, rise) = wall_sweeps::extents_of(sweep);
    let (section, outline) = (wall_sweeps::section_area(sweep), wall_sweeps::visible_perimeter(sweep));
    let volume = solid.map_or(section * length, |solid| solid.volume);
    let rows = vec![LayerQuantity { material: sweep.material.clone(), thickness: protrusion, area: outline * length, volume, mass: volume * density(snapshot, &sweep.material) }];
    ElementQuantity {
        kind: QuantityKind::WallSweep,
        storey: wall.storey.clone(),
        phase: wall.phase,
        count: 1,
        length,
        width: protrusion,
        height: rise,
        perimeter: outline,
        gross_area: section,
        net_area: outline * length,
        surface_area: outline * length,
        gross_volume: section * length,
        net_volume: volume,
        mass: mass_of(&rows),
        layers: rows,
        ..ElementQuantity::default()
    }
}

/// 🪞️ The quantities of a curtain wall from its layout, the frames of its hosted openings and its solid (mullions and panels).
pub fn curtain_quantity(snapshot: &ModelSnapshot, curtain: &CurtainWall, layout: &CurtainLayout, frames: &[&OpeningFrame], solid: Option<&ElementSolid>, profiles: &FamilyProfiles<'_>) -> ElementQuantity {
    let opening_area = cut_area(frames.iter().copied());
    let rows = solid.map(|solid| solid_rows(snapshot, solid)).unwrap_or_default();
    let volume = solid.map_or(0.0, |solid| solid.volume);
    let cuts: Vec<OpeningCut> = frames.iter().filter(|frame| frame.valid).map(|frame| frame.cut).collect();
    let (panels, mullions) = curtain_walls::takeoff(snapshot, curtain, layout, &cuts, profiles);
    ElementQuantity {
        kind: QuantityKind::CurtainWall,
        type_id: curtain.curtain_wall_type.clone(),
        panels: panels.into_iter().filter(|row| row.count > 0).map(|row| PanelQuantity { kind: row.kind.to_string(), count: row.count, area: row.area }).collect(),
        mullions: mullions.into_iter().filter(|row| row.count > 0).map(|row| MullionQuantity { kind: row.kind.to_string(), count: row.count, length: row.length }).collect(),
        storey: curtain.storey.clone(), phase: curtain.phase,
        count: 1,
        length: layout.length,
        height: layout.height,
        gross_side_area: layout.area,
        opening_area,
        net_side_area: (layout.area - opening_area).max(0.0),
        gross_volume: volume,
        net_volume: volume,
        mass: mass_of(&rows),
        layers: rows,
        ..ElementQuantity::default()
    }
}

/// ⬜️ The quantities of a slab from its boundary, holes and type.
pub fn slab_quantity(snapshot: &ModelSnapshot, slab: &Slab) -> ElementQuantity {
    let outer = plan(&slab.boundary);
    let holes: Vec<Vec<loops::Vertex>> = slab.holes.iter().map(|hole| plan(hole)).collect();
    let gross = loops::area(&outer);
    let net = (gross - holes.iter().map(|hole| loops::area(hole)).sum::<f64>()).max(0.0);
    let layers: &[Layer] = snapshot.slab_types.get(&slab.slab_type).map_or(&[], |kind| &kind.layers);
    let thickness: f64 = layers.iter().map(|layer| layer.thickness.max(0.0)).sum();
    let slope = slab.slope.map_or(1.0, |slope| slope.angle.cos().abs().max(1e-9));
    let areas = vec![net; layers.len()];
    let volumes: Vec<f64> = layers.iter().map(|layer| net * layer.thickness.max(0.0)).collect();
    let rows = layer_rows(snapshot, layers, &areas, &volumes);
    ElementQuantity {
        kind: QuantityKind::Slab,
        storey: slab.storey.clone(), phase: slab.phase,
        type_id: slab.slab_type.clone(),
        count: 1,
        width: thickness,
        perimeter: loops::perimeter(&outer) + holes.iter().map(|hole| loops::perimeter(hole)).sum::<f64>(),
        gross_area: gross,
        net_area: net,
        surface_area: net / slope,
        gross_volume: gross * thickness,
        net_volume: net * thickness,
        mass: mass_of(&rows),
        layers: rows,
        ..ElementQuantity::default()
    }
}

/// 🔲️ The quantities of a ceiling from its boundary, holes and type.
pub fn ceiling_quantity(snapshot: &ModelSnapshot, ceiling: &Ceiling) -> ElementQuantity {
    let outer = plan(&ceiling.boundary);
    let holes: Vec<Vec<loops::Vertex>> = ceiling.holes.iter().map(|hole| plan(hole)).collect();
    let gross = loops::area(&outer);
    let net = (gross - holes.iter().map(|hole| loops::area(hole)).sum::<f64>()).max(0.0);
    let layers: &[Layer] = snapshot.ceiling_types.get(&ceiling.ceiling_type).map_or(&[], |kind| &kind.layers);
    let thickness: f64 = layers.iter().map(|layer| layer.thickness.max(0.0)).sum();
    let slope = ceiling.slope.map_or(1.0, |slope| slope.angle.cos().abs().max(1e-9));
    let areas = vec![net; layers.len()];
    let volumes: Vec<f64> = layers.iter().map(|layer| net * layer.thickness.max(0.0)).collect();
    let rows = layer_rows(snapshot, layers, &areas, &volumes);
    ElementQuantity {
        kind: QuantityKind::Ceiling,
        storey: ceiling.storey.clone(), phase: Phase::New,
        type_id: ceiling.ceiling_type.clone(),
        count: 1,
        width: thickness,
        perimeter: loops::perimeter(&outer) + holes.iter().map(|hole| loops::perimeter(hole)).sum::<f64>(),
        gross_area: gross,
        net_area: net,
        surface_area: net / slope,
        gross_volume: gross * thickness,
        net_volume: net * thickness,
        mass: mass_of(&rows),
        layers: rows,
        ..ElementQuantity::default()
    }
}

/// 🏠️ The quantities of a roof from its footprint, overhang and solid.
pub fn roof_quantity(snapshot: &ModelSnapshot, roof: &Roof, solid: Option<&ElementSolid>) -> ElementQuantity {
    let footprint = plan(&roof.footprint);
    let eave = if roof.overhang > 0.0 { loops::offset(&footprint, roof.overhang, 4.0).unwrap_or_else(|| footprint.clone()) } else { footprint };
    let area = loops::area(&eave);
    let rows = solid.map(|solid| solid_rows(snapshot, solid)).unwrap_or_default();
    let volume = solid.map_or(0.0, |solid| solid.volume);
    let thickness = snapshot.roof_types.get(&roof.roof_type).map_or(0.0, |kind| kind.layers.iter().map(|layer| layer.thickness.max(0.0)).sum());
    ElementQuantity {
        kind: QuantityKind::Roof,
        storey: roof.storey.clone(), phase: roof.phase,
        type_id: roof.roof_type.clone(),
        count: 1,
        width: thickness,
        perimeter: loops::perimeter(&eave),
        gross_area: area,
        net_area: area,
        surface_area: solid.map_or(area, |solid| upward_area(solid, 0)),
        gross_volume: volume,
        net_volume: volume,
        mass: mass_of(&rows),
        layers: rows,
        ..ElementQuantity::default()
    }
}

/// 🏛️ The quantities of a column from the levels it is resolved by; absent without a type.
pub fn column_quantity(snapshot: &ModelSnapshot, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>) -> Option<ElementQuantity> {
    column_quantity_in(snapshot, column, own, target, &FamilyProfiles::new())
}

/// 🏛️ [`column_quantity`] where a type that names a profile family gets that family's outline from `profiles`.
pub fn column_quantity_in(snapshot: &ModelSnapshot, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>, profiles: &FamilyProfiles<'_>) -> Option<ElementQuantity> {
    let kind = snapshot.column_types.get(&column.column_type)?;
    let (base_z, top_z) = vertical_of(column.base_offset, &column.top, own, target);
    let outline = profile_loop(&profiles.resolve(&kind.profile));
    let (area, height) = (loops::area(&outline), (top_z - base_z).max(0.0));
    let length = column.tilt.filter(|tilt| tilt.angle.is_finite() && tilt.angle.abs() < std::f64::consts::FRAC_PI_2).map_or(height, |tilt| height / tilt.angle.cos());
    let volume = area * length;
    let rows = vec![LayerQuantity { material: kind.material.clone(), thickness: 0.0, area, volume, mass: volume * density(snapshot, &kind.material) }];
    Some(ElementQuantity { kind: QuantityKind::Column, storey: column.storey.clone(), phase: column.phase, type_id: column.column_type.clone(), count: 1, length, height, perimeter: loops::perimeter(&outline), gross_area: area, net_area: area, gross_volume: volume, net_volume: volume, mass: mass_of(&rows), layers: rows, ..ElementQuantity::default() })
}

/// ➖️ The quantities of a beam; absent without a type. The length is the length of the axis (along the arc, along the incline), the gross volume the profile area times it, the net volume the volume of the solid
/// once the joins with columns cut the ends back (the gross volume while there is no solid).
pub fn beam_quantity(snapshot: &ModelSnapshot, beam: &Beam, solid: Option<&ElementSolid>) -> Option<ElementQuantity> {
    beam_quantity_in(snapshot, beam, solid, &FamilyProfiles::new())
}

/// ➖️ [`beam_quantity`] where a type that names a profile family gets that family's outline from `profiles`.
pub fn beam_quantity_in(snapshot: &ModelSnapshot, beam: &Beam, solid: Option<&ElementSolid>, profiles: &FamilyProfiles<'_>) -> Option<ElementQuantity> {
    let kind = snapshot.beam_types.get(&beam.beam_type)?;
    let outline = profile_loop(&profiles.resolve(&kind.profile));
    let rise = beam.end_top_offset.map_or(0.0, |end| end - beam.top_offset);
    let (area, length) = (loops::area(&outline), seg(&beam.axis).length().hypot(rise));
    let gross = area * length;
    let net = solid.filter(|solid| !solid.is_empty()).map_or(gross, |solid| solid.volume);
    let rows = vec![LayerQuantity { material: kind.material.clone(), thickness: 0.0, area, volume: net, mass: net * density(snapshot, &kind.material) }];
    Some(ElementQuantity { kind: QuantityKind::Beam, storey: beam.storey.clone(), phase: beam.phase, type_id: beam.beam_type.clone(), count: 1, length, perimeter: loops::perimeter(&outline), gross_area: area, net_area: area, gross_volume: gross, net_volume: net, mass: mass_of(&rows), layers: rows, ..ElementQuantity::default() })
}

/// 🕰️ The phase an opening takes: the one of its host wall or curtain wall, new construction without a host.
fn host_phase(snapshot: &ModelSnapshot, host: &str) -> Phase {
    snapshot.walls.get(host).map(|wall| wall.phase).or_else(|| snapshot.curtain_walls.get(host).map(|curtain| curtain.phase)).unwrap_or_default()
}

/// 🪟️ The quantities of an opening from its frame and its filler solid.
pub fn opening_quantity(snapshot: &ModelSnapshot, opening: &Opening, frame: &OpeningFrame, solid: Option<&ElementSolid>) -> ElementQuantity {
    let storey = snapshot.walls.get(&opening.host).map(|wall| wall.storey.clone()).or_else(|| snapshot.curtain_walls.get(&opening.host).map(|curtain| curtain.storey.clone())).unwrap_or_default();
    let phase = host_phase(snapshot, &opening.host);
    let (kind, type_id) = match &opening.kind {
        OpeningKind::Window { window_type } => (QuantityKind::Window, window_type.clone()),
        OpeningKind::Door { door_type } => (QuantityKind::Door, door_type.clone()),
        OpeningKind::Void { .. } => (QuantityKind::Void, String::new()),
    };
    let rows = solid.map(|solid| solid_rows(snapshot, solid)).unwrap_or_default();
    let volume = solid.map_or(0.0, |solid| solid.volume);
    let area = frame.width * frame.height;
    ElementQuantity { kind, storey, phase, type_id, count: 1, width: frame.width, height: frame.height, perimeter: 2.0 * (frame.width + frame.height), gross_area: area, net_area: area, gross_volume: volume, net_volume: volume, mass: mass_of(&rows), layers: rows, ..ElementQuantity::default() }
}

/// 🪜️ The quantities of a stair from its run and solid.
pub fn stair_quantity(stair: &Stair, run: &StairRun, solid: Option<&ElementSolid>) -> ElementQuantity {
    let volume = solid.map_or(0.0, |solid| solid.volume);
    ElementQuantity { kind: QuantityKind::Stair, storey: stair.storey.clone(), phase: stair.phase, count: 1, length: run.run_length, width: run.width, height: run.rise, gross_volume: volume, net_volume: volume, risers: run.riser_count, ..ElementQuantity::default() }
}

/// 🛝️ The quantities of a ramp from its run, its plan strip and its solid: `length` the path, `width`, `height` the rise, the plan area of the strip as gross and net area, the walking surface as `surface_area`, the slab (not the side railings) as volume.
pub fn ramp_quantity(snapshot: &ModelSnapshot, ramp: &Ramp, run: &RampRun, solid: Option<&ElementSolid>) -> ElementQuantity {
    let outline = strip_of(ramp).outline();
    let (area, perimeter) = (loops::area(&outline), loops::perimeter(&outline));
    let volume = solid.map_or(0.0, |solid| group_volumes(solid).iter().zip(&solid.groups).filter(|(_, group)| group.part == parts::BODY).map(|(volume, _)| *volume).sum());
    let rows = vec![LayerQuantity { material: ramp.material.clone(), thickness: ramp.thickness, area, volume, mass: volume * density(snapshot, &ramp.material) }];
    ElementQuantity {
        kind: QuantityKind::Ramp,
        storey: ramp.storey.clone(), phase: Phase::New,
        count: 1,
        length: run.length,
        width: ramp.width,
        height: run.rise,
        perimeter,
        gross_area: area,
        net_area: area,
        surface_area: solid.map_or(area, |solid| upward_area(solid, 0)),
        gross_volume: volume,
        net_volume: volume,
        mass: mass_of(&rows),
        layers: rows,
        ..ElementQuantity::default()
    }
}

fn category_key(category: crate::FamilyCategory) -> &'static str {
    match category {
        crate::FamilyCategory::Furniture => "furniture",
        crate::FamilyCategory::Equipment => "equipment",
        crate::FamilyCategory::Casework => "casework",
        crate::FamilyCategory::Plumbing => "plumbing",
        crate::FamilyCategory::Lighting => "lighting",
        crate::FamilyCategory::Mechanical => "mechanical",
        crate::FamilyCategory::Electrical => "electrical",
        crate::FamilyCategory::Generic => "generic",
        crate::FamilyCategory::Profile => "profile",
    }
}

/// 🪑️ The quantities of a component: `type_id` is its family, `width` and `length` the extents of the footprint across and along the family frame (`x` and `y`), `height` the height of its solids, `gross_area` and `net_area` the area of the footprint, `gross_volume` the sum of the volumes of the visible family solids and `net_volume` the volume of its solid; the groups
/// `component-category:<category>` and, for a terminal, `component-system:<system>` sum it per category and per system.
pub fn component_quantity(snapshot: &ModelSnapshot, component: &Component, value: &ComponentValue, solid: Option<&ElementSolid>) -> ElementQuantity {
    let rows = solid.map(|solid| solid_rows(snapshot, solid)).unwrap_or_default();
    let volume = solid.map_or(value.volume, |solid| solid.volume);
    let (across, along) = (value.placement.rotate([1.0, 0.0, 0.0]), value.placement.rotate([0.0, 1.0, 0.0]));
    let extent = |axis: [f64; 3]| {
        let projected = value.footprint.iter().map(|corner| corner.x * axis[0] + corner.y * axis[1]);
        projected.clone().fold(f64::NEG_INFINITY, f64::max) - projected.fold(f64::INFINITY, f64::min)
    };
    let (width, length) = if value.solid() { (extent(across), extent(along)) } else { (0.0, 0.0) };
    let mut groups: Vec<String> = value.category.map(|category| format!("component-category:{}", category_key(category))).into_iter().collect();
    groups.extend(value.connector.as_ref().map(|connector| format!("component-system:{}", mep::key(connector.system))));
    ElementQuantity {
        kind: QuantityKind::Component,
        storey: value.storey.clone(),
        phase: Phase::New,
        type_id: component.family.clone(),
        count: 1,
        length,
        width,
        height: (value.bounds.max.z - value.bounds.min.z).max(0.0),
        perimeter: 2.0 * (width + length),
        gross_area: value.footprint_area,
        net_area: value.footprint_area,
        gross_volume: value.volume,
        net_volume: volume,
        mass: mass_of(&rows),
        layers: rows,
        groups,
        ..ElementQuantity::default()
    }
}

/// 🌀️ The quantities of a MEP element: `type_id` is its system key, `length` the centre line, `width` and `height` the section, `perimeter` its perimeter, `gross_area` the cross-section, `net_area` and `surface_area` the lateral surface, `gross_volume` the closed form
/// (section times length) and `net_volume` the volume of the tessellated solid; the groups `mep-system:<system>` and `mep-size:<label>` sum it per system and per size.
pub fn mep_quantity(_snapshot: &ModelSnapshot, _id: &str, value: &MepValue, solid: Option<&ElementSolid>) -> ElementQuantity {
    let volume = solid.filter(|solid| !solid.is_empty()).map_or(value.volume, |solid| solid.volume);
    ElementQuantity {
        kind: QuantityKind::Mep,
        storey: value.storey.clone(),
        phase: Phase::New,
        type_id: mep::key(value.system).to_string(),
        count: 1,
        length: value.length,
        width: value.section.width,
        height: value.section.height,
        perimeter: value.section.perimeter,
        gross_area: value.section.area,
        net_area: value.surface_area,
        surface_area: value.surface_area,
        gross_volume: value.volume,
        net_volume: volume,
        groups: vec![format!("mep-system:{}", mep::key(value.system)), format!("mep-size:{}", value.section.label)],
        ..ElementQuantity::default()
    }
}

/// 🪟️ The area of one side of the infill of a railing: the volume of the infill part of its solid over the thickness of the infill.
fn infill_area(railing: &Railing, solid: &ElementSolid) -> f64 {
    let (Infill::Glass { thickness } | Infill::Panel { thickness }) = railing.infill else { return 0.0 };
    let volumes = group_volumes(solid);
    solid.groups.iter().zip(volumes).filter(|(group, _)| group.part == parts::INFILL).map(|(_, volume)| volume).sum::<f64>() / thickness
}

/// 🛤️ The quantities of a railing from its path and solid.
pub fn railing_quantity(snapshot: &ModelSnapshot, railing: &Railing, solid: Option<&ElementSolid>) -> ElementQuantity {
    let length: f64 = railing.path.windows(2).map(|pair| (pair[1].x - pair[0].x).hypot(pair[1].y - pair[0].y)).sum();
    railing_quantity_of(snapshot, railing, length, solid)
}

/// 🪝️ The quantities of a hosted railing: `length` is the rail along the paths its host yields.
pub fn hosted_railing_quantity(snapshot: &ModelSnapshot, railing: &Railing, length: f64, solid: Option<&ElementSolid>) -> ElementQuantity {
    railing_quantity_of(snapshot, railing, length, solid)
}

fn railing_quantity_of(snapshot: &ModelSnapshot, railing: &Railing, length: f64, solid: Option<&ElementSolid>) -> ElementQuantity {
    let rows = solid.map(|solid| solid_rows(snapshot, solid)).unwrap_or_default();
    let volume = solid.map_or(0.0, |solid| solid.volume);
    let (infill, balusters) = solid.map_or((0.0, 0), |solid| (infill_area(railing, solid), if solid.groups.iter().any(|group| group.part == parts::BALUSTER) { baluster_count(railing) } else { 0 }));
    ElementQuantity { kind: QuantityKind::Railing, storey: railing.storey.clone(), phase: railing.phase, count: 1, length, height: railing.height, surface_area: infill, gross_volume: volume, net_volume: volume, mass: mass_of(&rows), balusters, layers: rows, ..ElementQuantity::default() }
}

/// 🏠️ The quantities of a space from its room and the frames of the openings on the walls of its storey; absent while the room is unresolved.
pub fn space_quantity(snapshot: &ModelSnapshot, space: &Space, room: &SpaceRoom, frames: &[&OpeningFrame]) -> Option<ElementQuantity> {
    matches!(room.status, SpaceStatus::Inferred | SpaceStatus::Explicit).then(|| ElementQuantity {
        kind: QuantityKind::Space,
        storey: space.storey.clone(), phase: space.phase,
        count: 1,
        height: room.clear_height,
        perimeter: room.perimeter,
        gross_area: room.area,
        net_area: room.net_floor_area,
        gross_volume: room.volume,
        net_volume: room.volume,
        finishes: finishes::finish_rows(snapshot, space, room, frames),
        ..ElementQuantity::default()
    })
}

/// 🔑️ What the quantity of element `id` reads besides the nodes it is computed from: the element record (without its name), its type and the density of every material it names.
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let materials = |layers: &[Layer]| layers.iter().map(|layer| layer.material.clone()).collect::<BTreeSet<String>>();
    let (element, kind, used): (DslValue, DslValue, BTreeSet<String>) = if let Some(wall) = snapshot.walls.get(id) {
        let kind = snapshot.wall_types.get(&wall.wall_type);
        (dep_value(&Wall { name: String::new(), ..wall.clone() }), dep_value(&kind.cloned()), kind.map(|row| materials(&row.layers)).unwrap_or_default())
    } else if let Some(curtain) = snapshot.curtain_walls.get(id) {
        (dep_value(&CurtainWall { name: String::new(), ..curtain.clone() }), dep_value(&snapshot.curtain_wall_types.get(&curtain.curtain_wall_type).cloned()), curtain_walls::materials_of(snapshot, id, curtain))
    } else if let Some(slab) = snapshot.slabs.get(id) {
        let kind = snapshot.slab_types.get(&slab.slab_type);
        (dep_value(&Slab { name: String::new(), ..slab.clone() }), dep_value(&kind.cloned()), kind.map(|row| materials(&row.layers)).unwrap_or_default())
    } else if let Some(ceiling) = snapshot.ceilings.get(id) {
        let kind = snapshot.ceiling_types.get(&ceiling.ceiling_type);
        (dep_value(&Ceiling { name: String::new(), ..ceiling.clone() }), dep_value(&kind.cloned()), kind.map(|row| materials(&row.layers)).unwrap_or_default())
    } else if let Some(roof) = snapshot.roofs.get(id) {
        let kind = snapshot.roof_types.get(&roof.roof_type);
        (dep_value(&Roof { name: String::new(), ..roof.clone() }), dep_value(&kind.cloned()), kind.map(|row| materials(&row.layers)).unwrap_or_default())
    } else if let Some(column) = snapshot.columns.get(id) {
        let kind = snapshot.column_types.get(&column.column_type);
        (dep_value(&Column { name: String::new(), ..column.clone() }), dep_value(&kind.cloned()), kind.map(|row| BTreeSet::from([row.material.clone()])).unwrap_or_default())
    } else if let Some(beam) = snapshot.beams.get(id) {
        let kind = snapshot.beam_types.get(&beam.beam_type);
        (dep_value(&Beam { name: String::new(), ..beam.clone() }), dep_value(&kind.cloned()), kind.map(|row| BTreeSet::from([row.material.clone()])).unwrap_or_default())
    } else if let Some(opening) = snapshot.openings.get(id) {
        let material = match &opening.kind {
            OpeningKind::Window { window_type } => snapshot.window_types.get(window_type).map(|row| row.material.clone()),
            OpeningKind::Door { door_type } => snapshot.door_types.get(door_type).map(|row| row.material.clone()),
            OpeningKind::Void { .. } => None,
        };
        let storey = snapshot.walls.get(&opening.host).map(|wall| wall.storey.clone()).or_else(|| snapshot.curtain_walls.get(&opening.host).map(|curtain| curtain.storey.clone()));
        (dep_object([("storey", dep_value(&storey)), ("phase", dep_value(&host_phase(snapshot, &opening.host))), ("type", dep_value(&opening.kind))]), DslValue::Null, material.into_iter().collect())
    } else if let Some(sweep) = snapshot.wall_sweeps.get(id) {
        let host = snapshot.walls.get(&sweep.host);
        (dep_object([("sweep", dep_value(&WallSweep { name: String::new(), ..sweep.clone() })), ("storey", dep_value(&host.map(|wall| wall.storey.clone()))), ("phase", dep_value(&host.map(|wall| wall.phase)))]), DslValue::Null, BTreeSet::from([sweep.material.clone()]))
    } else if let Some(stair) = snapshot.stairs.get(id) {
        (dep_object([("storey", dep_value(&stair.storey)), ("phase", dep_value(&stair.phase))]), DslValue::Null, BTreeSet::new())
    } else if let Some(railing) = snapshot.railings.get(id) {
        (dep_object([("railing", dep_value(&Railing { name: String::new(), ..railing.clone() })), ("host", rail_hosts::dependency(snapshot, railing))]), DslValue::Null, BTreeSet::from([railing.material.clone()]))
    } else if let Some(ramp) = snapshot.ramps.get(id) {
        (dep_value(&Ramp { name: String::new(), ..ramp.clone() }), DslValue::Null, BTreeSet::from([ramp.material.clone()]))
    } else if snapshot.components.contains_key(id) {
        (DslValue::Null, DslValue::Null, snapshot.materials.keys().cloned().collect())
    } else if snapshot.mep_elements.contains_key(id) {
        (DslValue::Null, DslValue::Null, BTreeSet::new())
    } else if let Some(space) = snapshot.spaces.get(id) {
        (dep_object([("storey", dep_value(&space.storey)), ("phase", dep_value(&space.phase)), ("finishes", finishes::dependency(snapshot, space))]), DslValue::Null, BTreeSet::new())
    } else {
        (DslValue::Null, DslValue::Null, BTreeSet::new())
    };
    let densities = DslValue::object(used.into_iter().map(|material| (material.clone(), dep_value(&snapshot.materials.get(&material).map(|row| row.density)))));
    dep_object([("element", element), ("type", kind), ("densities", densities)])
}
//#endregion 🔖️Elements

//#region 🔖️Totals
impl Totals {
    fn add_element(&mut self, element: &ElementQuantity) {
        self.count += element.count;
        self.length += element.length;
        self.area += element.area();
        self.volume += element.net_volume;
        self.mass += element.mass;
    }

    fn add_finish(&mut self, row: &FinishQuantity) {
        self.count += 1;
        self.area += row.area;
    }

    fn add_layer(&mut self, row: &LayerQuantity) {
        self.count += 1;
        self.area += row.area;
        self.volume += row.volume;
        self.mass += row.mass;
    }
}

impl QuantityTotals {
    fn add(&mut self, element: &ElementQuantity) {
        self.kinds.entry(element.kind.key().to_string()).or_default().add_element(element);
        if !element.type_id.is_empty() {
            self.types.entry(format!("{}:{}", element.kind.key(), element.type_id)).or_default().add_element(element);
        }
        for row in element.layers.iter().filter(|row| !row.material.is_empty()) {
            self.materials.entry(row.material.clone()).or_default().add_layer(row);
        }
        for row in element.finishes.iter().filter(|row| !row.material.is_empty()) {
            self.finishes.entry(format!("{}:{}", row.surface.key(), row.material)).or_default().add_finish(row);
        }
        self.phases.entry(element.phase.key().to_string()).or_default().add_element(element);
        self.phase_kinds.entry(format!("{}:{}", element.phase.key(), element.kind.key())).or_default().add_element(element);
        for group in &element.groups {
            self.groups.entry(group.clone()).or_default().add_element(element);
        }
    }
}

/// ➕️ The totals of a set of element quantities, summed in the order given (the `Totals` nodes of the model graph pass them in element id order).
pub fn totals_of<'a>(elements: impl IntoIterator<Item = &'a ElementQuantity>) -> QuantityTotals {
    let mut totals = QuantityTotals::default();
    elements.into_iter().for_each(|element| totals.add(element));
    totals
}

/// 🧮️ The take-off of a model (the `Quantity` and `Totals` nodes of the model graph). The inference argument is ignored: the graph derives everything the take-off reads in one run.
#[cfg(test)]
pub fn compute_quantities(snapshot: &ModelSnapshot, _inferred: &ModelInference) -> ModelQuantities {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::QUANTITIES }>(snapshot).quantities)
}

/// ➕️ The totals of a set of element quantities: per storey, per building and for the project.
pub fn summarise(snapshot: &ModelSnapshot, elements: BTreeMap<String, ElementQuantity>) -> ModelQuantities {
    let mut quantities = ModelQuantities { elements, ..ModelQuantities::default() };
    for element in quantities.elements.values() {
        quantities.project.add(element);
        if let Some(storey) = snapshot.storeys.get(&element.storey) {
            quantities.storeys.entry(element.storey.clone()).or_default().add(element);
            quantities.buildings.entry(storey.building.clone()).or_default().add(element);
        }
    }
    quantities
}


//#endregion 🔖️Totals

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
