//! 🌡️ The envelope overlay of the 3D world: the `EnvelopeSurface` polygons of the conditioned spaces as one coloured mesh per building placement. The colouring is pure and shared by editor and viewer.
//!
//! Modes. [`Mode::UValue`] colours a surface by its U-value on a diverging scale: the `coolwarm` ramp of the framework over `0.0` to `2.0` W/(m²·K), so 0 is blue (well insulated), `1.0` the light grey mid-point and `2.0`
//! or more red (poor); a surface without a U-value is [`NEUTRAL`], the mid grey of the framework heatmap that no step of the ramp reaches. [`Mode::Boundary`] colours by what lies behind: exterior, ground, adjacent space or adiabatic, four
//! hues of the colour-blind safe Okabe-Ito palette. A polygon is drawn `LIFT` metres towards the inside of its space so it does not fight the wall face it lies on.
//!
//! Related: Okabe and Ito <https://jfly.uni-koeln.de/color/>, gbXML surface types <https://gbxml.org/schema_doc/7.03/GreenBuildingXML_Ver7.03.html>.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::{placement_of, SolidPlacement};
use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::{Boundary, EnvelopeSpace, EnvelopeSurface};
use crate::standards::v1::subsets::any::schema::inferences::opening_frames::Vec3;
use crate::{ModelInference, ModelSnapshot};
use semio_framework_geometry::triangulation::triangulate;
use semio_framework_geometry::Point;
use semio_framework_plugin::{MeshData, World3dColorRamp};
use std::collections::BTreeMap;

//#region 🔖️Constants
/// 🌡️ The U-value at the cold end of the scale, in watts per square metre and kelvin.
pub const U_MIN: f64 = 0.0;
/// 🌡️ The U-value at the warm end of the scale; the mid-point of the scale is half of it.
pub const U_MAX: f64 = 2.0;
/// 🩶 The colour of a surface that states no U-value: the no-data grey of the framework heatmap, which no step of the scale reaches.
pub const NEUTRAL: [u8; 3] = [0x80, 0x80, 0x80];
/// 🪜️ How far a polygon is moved towards the inside of its space, in metres.
pub const LIFT: f64 = 0.01;
/// 🔢️ The number of labelled steps of the U-value legend.
pub const TICKS: usize = 5;
//#endregion 🔖️Constants

//#region 🔖️Mode
/// 🎨️ What the overlay colours by.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    UValue,
    Boundary,
}

impl Mode {
    /// 🔢️ Every mode, in the order the window offers them.
    pub const ALL: [Mode; 2] = [Self::UValue, Self::Boundary];

    /// 🏷️ The stable key of the mode, as the window config stores it.
    pub const fn key(self) -> &'static str {
        match self {
            Self::UValue => "u_value",
            Self::Boundary => "boundary",
        }
    }

    /// 🔎️ The mode a key names.
    pub fn parse(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|mode| mode.key() == key.trim())
    }
}
//#endregion 🔖️Mode

//#region 🔖️Colours
/// 🌡️ The colour of a U-value on the diverging scale; no U-value is [`NEUTRAL`].
pub fn u_value_rgb(u_value: Option<f64>) -> [u8; 3] {
    u_value.filter(|value| value.is_finite()).map_or(NEUTRAL, |value| World3dColorRamp::Coolwarm.sample((value - U_MIN) / (U_MAX - U_MIN)))
}

/// 🔭️ The colour of a boundary condition.
pub const fn boundary_rgb(boundary: Boundary) -> [u8; 3] {
    match boundary {
        Boundary::Exterior => [0x00, 0x72, 0xb2],
        Boundary::Ground => [0xe6, 0x9f, 0x00],
        Boundary::Adjacent => [0x00, 0x9e, 0x73],
        Boundary::Adiabatic => [0xcc, 0x79, 0xa7],
    }
}

/// 🎨️ The colour of a surface in a mode.
pub fn rgb(mode: Mode, surface: &EnvelopeSurface) -> [u8; 3] {
    match mode {
        Mode::UValue => u_value_rgb(surface.u_value),
        Mode::Boundary => boundary_rgb(surface.boundary),
    }
}

/// 🏷️ The steps of the U-value legend: `TICKS` evenly spaced U-values from [`U_MIN`] to [`U_MAX`] with their colours.
pub fn u_value_ticks() -> Vec<(f64, [u8; 3])> {
    (0..TICKS).map(|step| U_MIN + (U_MAX - U_MIN) * step as f64 / (TICKS - 1) as f64).map(|value| (value, u_value_rgb(Some(value)))).collect()
}

/// 🏷️ The boundary conditions of the boundary legend with their colours.
pub fn boundary_steps() -> [(Boundary, [u8; 3]); 4] {
    [Boundary::Exterior, Boundary::Ground, Boundary::Adjacent, Boundary::Adiabatic].map(|boundary| (boundary, boundary_rgb(boundary)))
}
//#endregion 🔖️Colours

//#region 🔖️Parts
/// 🧱️ One surface of the overlay with the space it bounds and the placement of its building.
#[derive(Clone, Copy, Debug)]
pub struct Part<'a> {
    pub space: &'a EnvelopeSpace,
    pub surface: &'a EnvelopeSurface,
    pub placement: SolidPlacement,
}

/// 🧱️ The surfaces of the conditioned spaces whose storey `shown` accepts, in space and surface order.
pub fn parts<'a>(snapshot: &ModelSnapshot, inference: &'a ModelInference, shown: &dyn Fn(&str) -> bool) -> Vec<Part<'a>> {
    inference
        .energy_envelopes
        .values()
        .filter(|space| space.conditioned && shown(&space.storey))
        .flat_map(|space| {
            let placement = placement_of(snapshot, &space.storey, inference.storey_levels.get(&space.storey));
            space.surfaces.iter().map(move |surface| Part { space, surface, placement })
        })
        .collect()
}

/// 🧭️ A building-local point in world coordinates: rotated about `+Z` by the placement, then translated.
pub fn to_world(placement: SolidPlacement, at: [f64; 3]) -> [f64; 3] {
    let (sin, cos) = placement.rotation.sin_cos();
    [placement.x + cos * at[0] - sin * at[1], placement.y + sin * at[0] + cos * at[1], placement.z + at[2]]
}

/// 📍️ The centre of a polygon in building coordinates: the mean of its corners.
pub fn centre(surface: &EnvelopeSurface) -> [f64; 3] {
    let count = surface.polygon.len().max(1) as f64;
    let sum = surface.polygon.iter().fold([0.0; 3], |sum, corner| [sum[0] + corner.x, sum[1] + corner.y, sum[2] + corner.z]);
    [sum[0] / count, sum[1] / count, sum[2] / count]
}

/// 🧭️ Groups the parts by the placement of their building, so each group is one instance.
pub fn by_placement<'a>(parts: &[Part<'a>]) -> Vec<(SolidPlacement, Vec<Part<'a>>)> {
    let mut groups: BTreeMap<[u64; 4], (SolidPlacement, Vec<Part<'a>>)> = BTreeMap::new();
    for part in parts {
        let key = [part.placement.x.to_bits(), part.placement.y.to_bits(), part.placement.z.to_bits(), part.placement.rotation.to_bits()];
        groups.entry(key).or_insert_with(|| (part.placement, Vec::new())).1.push(*part);
    }
    groups.into_values().collect()
}
//#endregion 🔖️Parts

//#region 🔖️Mesh
/// 🧭️ The unit normal of a polygon by Newell's method, `None` for a degenerate one.
pub fn normal_of(polygon: &[Vec3]) -> Option<[f64; 3]> {
    let mut sum = [0.0; 3];
    for (index, a) in polygon.iter().enumerate() {
        let b = &polygon[(index + 1) % polygon.len()];
        sum[0] += (a.y - b.y) * (a.z + b.z);
        sum[1] += (a.z - b.z) * (a.x + b.x);
        sum[2] += (a.x - b.x) * (a.y + b.y);
    }
    let length = (sum[0] * sum[0] + sum[1] * sum[1] + sum[2] * sum[2]).sqrt();
    (length > 1e-12).then(|| [sum[0] / length, sum[1] / length, sum[2] / length])
}

/// 🔺️ The triangles of a polygon as corner indices, wound counter-clockwise around its normal.
pub fn triangles_of(polygon: &[Vec3], normal: [f64; 3]) -> Vec<[usize; 3]> {
    let axis = (0..3).max_by(|a, b| normal[*a].abs().total_cmp(&normal[*b].abs())).unwrap_or(2);
    let project = |corner: &Vec3| {
        let at = [corner.x, corner.y, corner.z];
        Point { x: at[(axis + 1) % 3], y: at[(axis + 2) % 3] }
    };
    let ring: Vec<Point> = polygon.iter().map(project).collect();
    let flipped = normal[axis] < 0.0;
    triangulate(&ring, &[]).triangles.into_iter().map(|triangle| if flipped { [triangle[0] as usize, triangle[2] as usize, triangle[1] as usize] } else { [triangle[0] as usize, triangle[1] as usize, triangle[2] as usize] }).collect()
}

/// 📐️ The area of a polygon in square metres, from its normal: half the length of the vector area.
pub fn polygon_area(polygon: &[Vec3]) -> f64 {
    let mut sum = [0.0; 3];
    for (index, a) in polygon.iter().enumerate() {
        let b = &polygon[(index + 1) % polygon.len()];
        sum[0] += a.y * b.z - a.z * b.y;
        sum[1] += a.z * b.x - a.x * b.z;
        sum[2] += a.x * b.y - a.y * b.x;
    }
    (sum[0] * sum[0] + sum[1] * sum[1] + sum[2] * sum[2]).sqrt() / 2.0
}

/// 🥽️ The mesh of the parts as a triangle soup: three vertices per triangle with the normal of the surface, the colour of the mode, and one face id per triangle that is the index of the part. A degenerate polygon is left out.
pub fn mesh_of(parts: &[Part<'_>], mode: Mode) -> MeshData {
    let mut mesh = MeshData::default();
    for (index, part) in parts.iter().enumerate() {
        let polygon = &part.surface.polygon;
        let Some(normal) = normal_of(polygon) else { continue };
        let [r, g, b] = rgb(mode, part.surface);
        let colour = [f32::from(r) / 255.0, f32::from(g) / 255.0, f32::from(b) / 255.0, 1.0];
        for triangle in triangles_of(polygon, normal) {
            for corner in triangle {
                let at = &polygon[corner];
                mesh.positions.extend([(at.x - normal[0] * LIFT) as f32, (at.y - normal[1] * LIFT) as f32, (at.z - normal[2] * LIFT) as f32]);
                mesh.normals.extend(normal.map(|component| component as f32));
                mesh.colors.extend(colour);
                mesh.indices.push(mesh.indices.len() as u32);
            }
            mesh.face_ids.push(index as u32);
        }
    }
    mesh
}
/// 🌡️ The U-value of the surface behind every triangle of `mesh`, in triangle order: the values of the framework heatmap that paints the same scale and draws its legend.
pub fn u_values(parts: &[Part<'_>], mesh: &MeshData) -> Vec<Option<f64>> {
    mesh.face_ids.iter().map(|index| parts.get(*index as usize).and_then(|part| part.surface.u_value)).collect()
}
//#endregion 🔖️Mesh

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
