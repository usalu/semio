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
//! * roof: `gross_area` the eave outline (footprint grown by the overhang), `surface_area` the upward faces of the outermost layer of
//!   its solid, volumes and layers from the solid.
//! * column and beam: `length` the height or the axis length, `perimeter` and `gross_area` the profile, `net_volume = area * length`.
//! * window, door, void: `gross_area = width * height` of the opening, material rows from the filler solid.
//! * stair: `risers`, `length` the run, `height` the rise, volume from the solid. railing: `length` the path, material rows from the solid.
//! * space: `gross_area` the room, `net_area` without columns, `height` the clear height, `net_volume` the room volume.

use super::super::curtain_layout::CurtainLayout;
use super::super::element_solids::columns::profile_loop;
use super::super::element_solids::plan_kit::seg;
use super::super::element_solids::{dep_object, dep_value, ElementSolid};
use super::super::opening_frames::OpeningFrame;
use super::super::spaces::{SpaceRoom, SpaceStatus};
use super::super::stair_runs::StairRun;
use super::super::storey_levels::{vertical_of, StoreyLevel};
use super::super::wall_layout::WallLayout;
use super::super::ModelInference;
use crate::{Beam, Column, CurtainWall, Layer, ModelSnapshot, Opening, OpeningKind, Railing, Roof, Slab, Space, Stair, Vertex, Wall};
use semio_framework_geometry::loops;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet};

/// 🗺️ The snapshot collections the take-off reads, directly or through the fields it is computed from.
pub const READS: &[&str] = &["walls", "wall_types", "curtain_walls", "slabs", "slab_types", "roofs", "roof_types", "columns", "column_types", "beams", "beam_types", "openings", "window_types", "door_types", "stairs", "railings", "spaces", "materials", "storeys", "buildings", "sites"];

//#region 🔖️Values
/// 🗂️ Which kind of element a quantity row measures.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
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
    Space,
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
            Self::Space => "space",
        }
    }
}

/// 🍰️ One material row of an element: a layer of a layered type, or a material run of a solid. `thickness` is 0 for solid runs.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct LayerQuantity {
    pub material: String,
    pub thickness: f64,
    pub area: f64,
    pub volume: f64,
    pub mass: f64,
}

/// 🧮️ The quantities of one element; measures that do not apply to its kind are 0.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ElementQuantity {
    pub kind: QuantityKind,
    pub storey: String,
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
    pub layers: Vec<LayerQuantity>,
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
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Totals {
    pub count: u32,
    pub length: f64,
    pub area: f64,
    pub volume: f64,
    pub mass: f64,
}

/// ➕️ Totals per kind (`wall`, `slab`, …), per type (`wall:wt-300`) and per material id.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct QuantityTotals {
    pub kinds: BTreeMap<String, Totals>,
    pub types: BTreeMap<String, Totals>,
    pub materials: BTreeMap<String, Totals>,
}

/// 🧮️ The whole take-off: elements by id, totals per storey, per building and for the project.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    let volumes: Vec<f64> = layers.iter().enumerate().map(|(index, layer)| (areas[index] * layout.height - opening_area * layer.thickness * scale_at(&wall.axis, ratio(index))).max(0.0)).collect();
    let rows = layer_rows(snapshot, layers, &areas, &volumes);
    let net_volume = if rows.is_empty() { (layout.volume - opening_area * layout.thickness).max(0.0) } else { volumes.iter().sum() };
    ElementQuantity {
        kind: QuantityKind::Wall,
        storey: wall.storey.clone(),
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

/// 🪞️ The quantities of a curtain wall from its layout, the frames of its hosted openings and its solid (mullions and panels).
pub fn curtain_quantity(snapshot: &ModelSnapshot, curtain: &CurtainWall, layout: &CurtainLayout, frames: &[&OpeningFrame], solid: Option<&ElementSolid>) -> ElementQuantity {
    let opening_area = cut_area(frames.iter().copied());
    let rows = solid.map(|solid| solid_rows(snapshot, solid)).unwrap_or_default();
    let volume = solid.map_or(0.0, |solid| solid.volume);
    ElementQuantity {
        kind: QuantityKind::CurtainWall,
        storey: curtain.storey.clone(),
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
        storey: slab.storey.clone(),
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
        storey: roof.storey.clone(),
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
    let kind = snapshot.column_types.get(&column.column_type)?;
    let (base_z, top_z) = vertical_of(column.base_offset, &column.top, own, target);
    let outline = profile_loop(&kind.profile);
    let (area, height) = (loops::area(&outline), (top_z - base_z).max(0.0));
    let volume = area * height;
    let rows = vec![LayerQuantity { material: kind.material.clone(), thickness: 0.0, area, volume, mass: volume * density(snapshot, &kind.material) }];
    Some(ElementQuantity { kind: QuantityKind::Column, storey: column.storey.clone(), type_id: column.column_type.clone(), count: 1, length: height, height, perimeter: loops::perimeter(&outline), gross_area: area, net_area: area, gross_volume: volume, net_volume: volume, mass: mass_of(&rows), layers: rows, ..ElementQuantity::default() })
}

/// ➖️ The quantities of a beam; absent without a type.
pub fn beam_quantity(snapshot: &ModelSnapshot, beam: &Beam) -> Option<ElementQuantity> {
    let kind = snapshot.beam_types.get(&beam.beam_type)?;
    let outline = profile_loop(&kind.profile);
    let (area, length) = (loops::area(&outline), (beam.end.x - beam.start.x).hypot(beam.end.y - beam.start.y));
    let volume = area * length;
    let rows = vec![LayerQuantity { material: kind.material.clone(), thickness: 0.0, area, volume, mass: volume * density(snapshot, &kind.material) }];
    Some(ElementQuantity { kind: QuantityKind::Beam, storey: beam.storey.clone(), type_id: beam.beam_type.clone(), count: 1, length, perimeter: loops::perimeter(&outline), gross_area: area, net_area: area, gross_volume: volume, net_volume: volume, mass: mass_of(&rows), layers: rows, ..ElementQuantity::default() })
}

/// 🪟️ The quantities of an opening from its frame and its filler solid.
pub fn opening_quantity(snapshot: &ModelSnapshot, opening: &Opening, frame: &OpeningFrame, solid: Option<&ElementSolid>) -> ElementQuantity {
    let storey = snapshot.walls.get(&opening.host).map(|wall| wall.storey.clone()).or_else(|| snapshot.curtain_walls.get(&opening.host).map(|curtain| curtain.storey.clone())).unwrap_or_default();
    let (kind, type_id) = match &opening.kind {
        OpeningKind::Window { window_type } => (QuantityKind::Window, window_type.clone()),
        OpeningKind::Door { door_type } => (QuantityKind::Door, door_type.clone()),
        OpeningKind::Void { .. } => (QuantityKind::Void, String::new()),
    };
    let rows = solid.map(|solid| solid_rows(snapshot, solid)).unwrap_or_default();
    let volume = solid.map_or(0.0, |solid| solid.volume);
    let area = frame.width * frame.height;
    ElementQuantity { kind, storey, type_id, count: 1, width: frame.width, height: frame.height, perimeter: 2.0 * (frame.width + frame.height), gross_area: area, net_area: area, gross_volume: volume, net_volume: volume, mass: mass_of(&rows), layers: rows, ..ElementQuantity::default() }
}

/// 🪜️ The quantities of a stair from its run and solid.
pub fn stair_quantity(stair: &Stair, run: &StairRun, solid: Option<&ElementSolid>) -> ElementQuantity {
    let volume = solid.map_or(0.0, |solid| solid.volume);
    ElementQuantity { kind: QuantityKind::Stair, storey: stair.storey.clone(), count: 1, length: run.run_length, width: run.width, height: run.rise, gross_volume: volume, net_volume: volume, risers: run.riser_count, ..ElementQuantity::default() }
}

/// 🛤️ The quantities of a railing from its path and solid.
pub fn railing_quantity(snapshot: &ModelSnapshot, railing: &Railing, solid: Option<&ElementSolid>) -> ElementQuantity {
    let length: f64 = railing.path.windows(2).map(|pair| (pair[1].x - pair[0].x).hypot(pair[1].y - pair[0].y)).sum();
    let rows = solid.map(|solid| solid_rows(snapshot, solid)).unwrap_or_default();
    let volume = solid.map_or(0.0, |solid| solid.volume);
    ElementQuantity { kind: QuantityKind::Railing, storey: railing.storey.clone(), count: 1, length, height: railing.height, gross_volume: volume, net_volume: volume, mass: mass_of(&rows), layers: rows, ..ElementQuantity::default() }
}

/// 🏠️ The quantities of a space from its room; absent while the room is unresolved.
pub fn space_quantity(space: &Space, room: &SpaceRoom) -> Option<ElementQuantity> {
    matches!(room.status, SpaceStatus::Inferred | SpaceStatus::Explicit).then(|| ElementQuantity {
        kind: QuantityKind::Space,
        storey: space.storey.clone(),
        count: 1,
        height: room.clear_height,
        perimeter: room.perimeter,
        gross_area: room.area,
        net_area: room.net_floor_area,
        gross_volume: room.volume,
        net_volume: room.volume,
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
        (dep_value(&CurtainWall { name: String::new(), ..curtain.clone() }), DslValue::Null, [curtain.panel_material.clone(), curtain.mullion_material.clone()].into_iter().collect())
    } else if let Some(slab) = snapshot.slabs.get(id) {
        let kind = snapshot.slab_types.get(&slab.slab_type);
        (dep_value(&Slab { name: String::new(), ..slab.clone() }), dep_value(&kind.cloned()), kind.map(|row| materials(&row.layers)).unwrap_or_default())
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
        (dep_object([("storey", dep_value(&storey)), ("type", dep_value(&opening.kind))]), DslValue::Null, material.into_iter().collect())
    } else if let Some(stair) = snapshot.stairs.get(id) {
        (dep_value(&stair.storey), DslValue::Null, BTreeSet::new())
    } else if let Some(railing) = snapshot.railings.get(id) {
        (dep_value(&Railing { name: String::new(), ..railing.clone() }), DslValue::Null, BTreeSet::from([railing.material.clone()]))
    } else if let Some(space) = snapshot.spaces.get(id) {
        (dep_value(&space.storey), DslValue::Null, BTreeSet::new())
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
    }
}

/// ➕️ The totals of a set of element quantities, summed in the order given (the `Totals` nodes of the model graph pass them in element id order).
pub fn totals_of<'a>(elements: impl IntoIterator<Item = &'a ElementQuantity>) -> QuantityTotals {
    let mut totals = QuantityTotals::default();
    elements.into_iter().for_each(|element| totals.add(element));
    totals
}

/// 🧮️ The take-off of a model (the `Quantity` and `Totals` nodes of the model graph). The inference argument is ignored: the graph derives everything the take-off reads in one run.
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

/// 🧾️ The table the third-party oracle reproduces: the elements whose measures follow in closed form from the authored values (walls, slabs, columns, beams, spaces), summed again without the other kinds.
pub fn table_json(snapshot: &ModelSnapshot, quantities: &ModelQuantities) -> String {
    let closed_form = [QuantityKind::Wall, QuantityKind::Slab, QuantityKind::Column, QuantityKind::Beam, QuantityKind::Space];
    let elements = quantities.elements.iter().filter(|(_, element)| closed_form.contains(&element.kind)).map(|(id, element)| (id.clone(), element.clone())).collect();
    semio_framework_pack_json::to_json_string(&summarise(snapshot, elements))
}
//#endregion 🔖️Totals

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
