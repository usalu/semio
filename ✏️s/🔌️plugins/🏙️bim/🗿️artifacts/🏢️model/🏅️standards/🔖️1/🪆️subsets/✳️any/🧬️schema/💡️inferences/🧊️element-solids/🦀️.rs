//! 🧊️ `element-solids`: the owned triangle meshes of every building element, keyed by element id.
//!
//! B-Rep handles cannot be cached as values, so every element is tessellated into an owned [`ElementSolid`] (flat `f64` positions and normals, indexed triangles,
//! per-triangle material/layer groups, bounds, volume, area). Solids live in building-local coordinates (the coordinates of the snapshot, `z` up from the building
//! datum); [`SolidPlacement`] is the instance transform into the world. Each family module is the pure part of its `Solid` nodes in the model graph: a free function from the
//! element, its type and the already inferred values it is built from (layouts, frames, runs, levels) to an [`ElementSolid`], and the `dependency` it reads besides them.
//! The pure geometry comes from `semio_framework_geometry`. See `r4-api-element-solids.md` and `r7-design-model-graph.md` in the BIM-PLUGIN ticket.

use super::super::storey_levels::StoreyLevel;
use crate::{ModelSnapshot, Profile};
use semio_framework_geometry::loops;
use semio_framework_geometry::mesh::TriMesh;
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

//#region 🔖️Values
/// 📏️ Sagitta of every tessellated arc, in metres: 0.1 mm keeps the volume error of an arc wall near `2e-5` relative.
pub const CHORD_TOLERANCE: f64 = 1e-4;

/// 🏷️ Well-known `part` names of a [`SolidGroup`].
pub mod parts {
    pub const BODY: &str = "body";
    pub const LAYER: &str = "layer";
    pub const PANEL: &str = "panel";
    pub const MULLION: &str = "mullion";
    pub const FRAME: &str = "frame";
    pub const MUNTIN: &str = "muntin";
    pub const GLASS: &str = "glass";
    pub const LEAF: &str = "leaf";
    pub const STEP: &str = "step";
    pub const POST: &str = "post";
    pub const RAIL: &str = "rail";
}

/// 🧩️ What a solid represents.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
pub enum SolidFamily {
    #[default]
    Wall,
    CurtainWall,
    Window,
    Door,
    Column,
    Beam,
    Slab,
    Roof,
    Stair,
    Railing,
}

/// 🔑️ A `Solid` node of the model graph: the family and the element id (a filler is `Window` or `Door` by the kind of its opening; voids have no solid).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
pub struct SolidKey {
    pub family: SolidFamily,
    pub id: String,
}

impl SolidKey {
    /// 🔑️ The node of the element `id` of `family`.
    pub fn of(family: SolidFamily, id: &str) -> Self {
        Self { family, id: id.to_string() }
    }
}

/// 📍️ A point in metres.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct SolidPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 📦️ Axis-aligned bounds of a solid.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct SolidBounds {
    pub min: SolidPoint,
    pub max: SolidPoint,
}

/// 🧭️ Instance transform of a building-local solid into the world: rotate about `+Z` by `rotation`, then translate by `(x, y, z)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct SolidPlacement {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rotation: f64,
}

/// 🎨️ A run of faces that share a part, a material and a layer.
#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub struct SolidGroup {
    pub part: String,
    pub material: String,
    pub layer: u32,
}

/// 🧊️ The owned tessellation of one element: counter-clockwise outward triangles, one vertex triple per triangle.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ElementSolid {
    pub family: SolidFamily,
    pub storey: String,
    pub placement: SolidPlacement,
    pub groups: Vec<SolidGroup>,
    pub positions: Vec<f64>,
    pub normals: Vec<f64>,
    pub indices: Vec<u32>,
    pub face_groups: Vec<u32>,
    pub bounds: SolidBounds,
    pub volume: f64,
    pub area: f64,
}

impl ElementSolid {
    /// 🕳️ `true` when there is no triangle.
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
    /// 🔢️ Number of triangles.
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
    /// 🔢️ Number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }
    /// 🕸️ The geometry crate mesh again, for sections and measures.
    pub fn mesh(&self) -> TriMesh {
        let triples = |flat: &[f64]| flat.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect::<Vec<[f64; 3]>>();
        TriMesh { positions: triples(&self.positions), normals: triples(&self.normals), indices: self.indices.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect() }
    }
    /// 📤️ Flat `f32` positions for `MeshData.positions`.
    pub fn positions_f32(&self) -> Vec<f32> {
        self.positions.iter().map(|v| *v as f32).collect()
    }
    /// 📤️ Flat `f32` normals for `MeshData.normals`.
    pub fn normals_f32(&self) -> Vec<f32> {
        self.normals.iter().map(|v| *v as f32).collect()
    }
    /// 📤️ One group index per triangle for `MeshData.faceIds`.
    pub fn face_ids(&self) -> Vec<u32> {
        self.face_groups.clone()
    }
    /// 🎨️ RGBA per vertex for `MeshData.colors`, coloured per group.
    pub fn vertex_colors(&self, colour: impl Fn(&SolidGroup) -> [f32; 4]) -> Vec<f32> {
        let palette: Vec<[f32; 4]> = self.groups.iter().map(colour).collect();
        let mut colours = Vec::with_capacity(self.face_groups.len() * 12);
        for group in &self.face_groups {
            for _ in 0..3 {
                colours.extend_from_slice(&palette[*group as usize]);
            }
        }
        colours
    }
    /// 📍️ The same solid on a storey with its world placement.
    pub fn placed(mut self, storey: &str, placement: SolidPlacement) -> Self {
        self.storey = storey.to_string();
        self.placement = placement;
        self
    }
    /// ⚖️ Bytes the solid accounts for in a cache budget.
    pub fn byte_size(&self) -> usize {
        std::mem::size_of::<Self>() + 8 * (self.positions.len() + self.normals.len()) + 4 * (self.indices.len() + self.face_groups.len())
    }
}

/// 🧱️ Collects parts into one [`ElementSolid`]; bounds, volume and area are computed by [`SolidBuilder::build`].
pub struct SolidBuilder {
    family: SolidFamily,
    groups: Vec<SolidGroup>,
    mesh: TriMesh,
    face_groups: Vec<u32>,
}

impl SolidBuilder {
    /// 🆕️ An empty builder of `family`.
    pub fn new(family: SolidFamily) -> Self {
        Self { family, groups: Vec::new(), mesh: TriMesh::new(), face_groups: Vec::new() }
    }
    /// ➕️ Appends `mesh` as part `part` of `material` in `layer`.
    pub fn add(&mut self, part: &str, material: &str, layer: u32, mesh: &TriMesh) -> &mut Self {
        if mesh.indices.is_empty() {
            return self;
        }
        let group = SolidGroup { part: part.to_string(), material: material.to_string(), layer };
        let index = self.groups.iter().position(|row| *row == group).unwrap_or_else(|| {
            self.groups.push(group);
            self.groups.len() - 1
        });
        self.face_groups.extend(std::iter::repeat_n(index as u32, mesh.indices.len()));
        self.mesh.append(mesh);
        self
    }
    /// 🧊️ The solid with its measures.
    pub fn build(self) -> ElementSolid {
        let flat = |rows: &[[f64; 3]]| rows.iter().flatten().copied().collect::<Vec<f64>>();
        let bounds = self.mesh.bounds().map_or_else(SolidBounds::default, |(lo, hi)| SolidBounds { min: SolidPoint { x: lo[0], y: lo[1], z: lo[2] }, max: SolidPoint { x: hi[0], y: hi[1], z: hi[2] } });
        ElementSolid {
            family: self.family,
            storey: String::new(),
            placement: SolidPlacement::default(),
            groups: self.groups,
            positions: flat(&self.mesh.positions),
            normals: flat(&self.mesh.normals),
            indices: self.mesh.indices.iter().flatten().copied().collect(),
            face_groups: self.face_groups,
            bounds,
            volume: self.mesh.signed_volume(),
            area: self.mesh.surface_area(),
        }
    }
}

/// 📦️ The value of a `Solid` node of the model graph: the solid and, for a roof that fell back to a flat roof, why.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SolidEntry {
    pub solid: ElementSolid,
    pub fallback: Option<super::roofs::RoofFallback>,
}
//#endregion 🔖️Values

//#region 🔖️Helpers
/// 🔑️ A dependency object from named values.
pub fn dep_object<const N: usize>(rows: [(&str, DslValue); N]) -> DslValue {
    DslValue::object(rows.map(|(name, value)| (name.to_string(), value)))
}

/// 🔑️ A dependency value of anything with a value form.
pub fn dep_value<T: semio_framework_value::ToValue>(item: &T) -> DslValue {
    semio_framework_value::ToValue::to_value(item)
}

/// 🏷️ An element record without its display name: no inference reads a name, so renaming an element never invalidates a node that depends on it.
pub trait Anonymous: Clone {
    fn anonymous(&self) -> Self;
}

macro_rules! anonymous {
    ($($record:ty),+) => {
        $(impl Anonymous for $record {
            fn anonymous(&self) -> Self {
                Self { name: String::new(), ..self.clone() }
            }
        })+
    };
}

anonymous!(crate::Wall, crate::CurtainWall, crate::Column, crate::Beam, crate::Slab, crate::Roof, crate::Stair, crate::Railing, crate::Opening);

/// 🔑️ A dependency value of keyed records, names left out.
pub fn dep_records<'a, T: Anonymous + semio_framework_value::ToValue + 'a>(rows: impl IntoIterator<Item = (&'a String, &'a T)>) -> DslValue {
    DslValue::object(rows.into_iter().map(|(id, row)| (id.clone(), dep_value(&row.anonymous()))))
}

/// 🔑️ A dependency value of the types of a library that the given ids name (a missing type is a null, so adding it changes the dependency).
pub fn dep_types<'a, T: Clone + semio_framework_value::ToValue>(ids: impl IntoIterator<Item = &'a String>, library: &std::collections::BTreeMap<String, T>) -> DslValue {
    let ids: std::collections::BTreeSet<&String> = ids.into_iter().collect();
    DslValue::object(ids.into_iter().map(|id| (id.clone(), dep_value(&library.get(id).cloned()))))
}

/// ▭️ The outline of a profile in `(a, b)` = (across, depth) coordinates centred on the origin, counter-clockwise; circles and custom arcs are flattened within [`CHORD_TOLERANCE`].
pub fn profile_polygon(profile: &Profile) -> Vec<Point> {
    match profile {
        Profile::Rectangle { width, depth } => vec![Point::new(-width / 2.0, -depth / 2.0), Point::new(width / 2.0, -depth / 2.0), Point::new(width / 2.0, depth / 2.0), Point::new(-width / 2.0, depth / 2.0)],
        Profile::Circle { diameter } => {
            let radius = diameter / 2.0;
            let step = if radius > CHORD_TOLERANCE { 2.0 * (1.0 - CHORD_TOLERANCE / radius).acos() } else { std::f64::consts::FRAC_PI_2 };
            let count = ((std::f64::consts::TAU / step).ceil() as usize).clamp(16, 512);
            (0..count).map(|k| k as f64 * std::f64::consts::TAU / count as f64).map(|angle| Point::new(radius * angle.cos(), radius * angle.sin())).collect()
        }
        Profile::IShape { width, depth, web, flange } => {
            let (w, d, t, f) = (width / 2.0, depth / 2.0, web / 2.0, *flange);
            [(-w, -d), (w, -d), (w, -d + f), (t, -d + f), (t, d - f), (w, d - f), (w, d), (-w, d), (-w, d - f), (-t, d - f), (-t, -d + f), (-w, -d + f)].into_iter().map(|(a, b)| Point::new(a, b)).collect()
        }
        Profile::Custom { outline } => loops::flatten(&outline.iter().map(|vertex| loops::Vertex { point: Point::new(vertex.point.x, vertex.point.y), bulge: vertex.bulge }).collect::<Vec<_>>(), CHORD_TOLERANCE),
    }
}

/// ▭️ Extents `(across, depth)` of a profile outline.
pub fn profile_extents(outline: &[Point]) -> (f64, f64) {
    let span = |value: fn(&Point) -> f64| outline.iter().map(value).fold(f64::NEG_INFINITY, f64::max) - outline.iter().map(value).fold(f64::INFINITY, f64::min);
    if outline.is_empty() {
        (0.0, 0.0)
    } else {
        (span(|p| p.x), span(|p| p.y))
    }
}

/// 🧭️ The instance transform of the elements of `storey`: the placement of its building and the datum of its level.
pub fn placement_of(snapshot: &ModelSnapshot, storey: &str, own: Option<&StoreyLevel>) -> SolidPlacement {
    let building = snapshot.storeys.get(storey).and_then(|row| snapshot.buildings.get(&row.building));
    let datum = own.map_or(0.0, |level| level.absolute_elevation - level.elevation);
    building.map_or(SolidPlacement { z: datum, ..SolidPlacement::default() }, |row| SolidPlacement { x: row.origin.x, y: row.origin.y, z: datum, rotation: row.rotation })
}
//#endregion 🔖️Helpers

//#region 🔖️Families
/// 🗺️ The snapshot collections the field reads (material colours are deliberately absent: a solid names its materials by id).
pub const READS: &[&str] = &["walls", "wall_types", "curtain_walls", "openings", "window_types", "door_types", "columns", "column_types", "beams", "beam_types", "slabs", "slab_types", "roofs", "roof_types", "stairs", "railings", "storeys", "buildings", "sites"];
//#endregion 🔖️Families

//#region 🔖️Projection
/// 🧊️ The solid of every element that has geometry, keyed by element id (the `Solid` nodes of the model graph).
pub fn compute_element_solids(snapshot: &ModelSnapshot) -> BTreeMap<String, ElementSolid> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::SOLIDS }>(snapshot).element_solids)
}
//#endregion 🔖️Projection

//#region 🔖️Fixtures
/// 🧫️ The committed fixture cases `🧫️fixtures/💡️inferences/🧊️element-solids/<case>/🔣️.json`: an authored `snapshot`, the oracle-written `expected` table and the blessed `meshes`.
#[cfg(test)]
pub(crate) mod fixtures {
    use crate::ModelSnapshot;
    use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

    const STRAIGHT_OPENINGS: &str = include_str!("../../../🧫️fixtures/💡️inferences/🧊️element-solids/🚪️straight-openings/🔣️.json");
    const ARC_WINDOW: &str = include_str!("../../../🧫️fixtures/💡️inferences/🧊️element-solids/🌀️arc-window/🔣️.json");
    const ROOM_JOINS: &str = include_str!("../../../🧫️fixtures/💡️inferences/🧊️element-solids/🧩️room-joins/🔣️.json");
    const CURTAIN_GRID: &str = include_str!("../../../🧫️fixtures/💡️inferences/🧊️element-solids/🏬️curtain-grid/🔣️.json");

    /// 🧫️ Every case name with its committed text.
    pub const CASES: [(&str, &str); 4] = [("straight-openings", STRAIGHT_OPENINGS), ("arc-window", ARC_WINDOW), ("room-joins", ROOM_JOINS), ("curtain-grid", CURTAIN_GRID)];

    /// 🧫️ The decoded snapshot and the whole committed document of a case.
    pub fn case(name: &str) -> (ModelSnapshot, serde_json::Value) {
        let text = CASES.iter().find(|(case, _)| *case == name).map(|(_, text)| *text).expect("a committed case");
        let document: serde_json::Value = serde_json::from_str(text).expect("the case is JSON");
        let snapshot = from_json_str(&document["snapshot"].to_string(), JsonMemberPolicy::Reject).expect("the case snapshot decodes");
        (snapshot, document)
    }
}
//#endregion 🔖️Fixtures

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
