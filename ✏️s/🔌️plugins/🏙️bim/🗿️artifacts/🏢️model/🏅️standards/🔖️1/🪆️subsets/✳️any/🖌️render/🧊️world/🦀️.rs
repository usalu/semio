//! 🧊️ Shared World3d scene of the BIM surfaces: one inline mesh per `element-solids` entry keyed by the element id, one instance per element
//! (the solid placement is the instance transform) carrying that id as its interaction id, vertex colours from the materials of the solid
//! groups, the projection preset bank as the camera projection and the storey visibility filter. Editor and viewer call `scene` with their
//! own window configuration.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::{parts, ElementSolid, SolidFamily, SolidGroup};
use crate::ModelSnapshot;
use semio_framework_plugin::{world3d_camera_projection_json, world3d_scene, world3d_selection_json_with_granularity, MeshData, World3dScene, WorldProjectionConfig, WorldSunConfig};
use semio_framework_value::{DslValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Constants
/// 🕹️ The interaction domain of drawn elements: one granularity, the element, addressed by element id.
pub const ELEMENT_DOMAIN: &str = "elements";
pub const ELEMENT_GRANULARITY: &str = "element";
const GLASS_COLOR: [f32; 4] = [0.62, 0.78, 0.9, 0.35];
//#endregion 🔖️Constants

//#region 🔖️View
/// 👁️ What a window contributes to the scene: its stored orbit (none until the user moved the camera), its projection bank, the storeys it
/// hides and the framework selection and hover marks.
#[derive(Clone, Copy, Debug)]
pub struct WorldView<'a> {
    pub orbit: Option<&'a store::Viewport3dOrbit>,
    pub projection: &'a WorldProjectionConfig,
    pub hidden_storeys: &'a [String],
    pub selected: &'a [String],
    pub hovered: &'a [String],
}
//#endregion 🔖️View

//#region 🔖️Mesh
/// 🎨️ Colour of a family when a solid group names no known material.
pub fn family_color(family: SolidFamily) -> [f32; 4] {
    match family {
        SolidFamily::Wall | SolidFamily::CurtainWall => [0.82, 0.8, 0.76, 1.0],
        SolidFamily::Window => [0.35, 0.4, 0.45, 1.0],
        SolidFamily::Door => [0.55, 0.38, 0.24, 1.0],
        SolidFamily::Column | SolidFamily::Beam => [0.6, 0.6, 0.62, 1.0],
        SolidFamily::Slab => [0.72, 0.72, 0.7, 1.0],
        SolidFamily::Roof => [0.62, 0.3, 0.26, 1.0],
        SolidFamily::Stair | SolidFamily::Railing => [0.5, 0.5, 0.52, 1.0],
    }
}

/// 🎨️ The vertex colour of one solid group: the material colour, glass for glazing, otherwise the family colour.
pub fn group_color(snapshot: &ModelSnapshot, family: SolidFamily, group: &SolidGroup) -> [f32; 4] {
    match snapshot.materials.get(&group.material) {
        Some(material) => [material.color.r as f32, material.color.g as f32, material.color.b as f32, 1.0],
        None if group.part == parts::GLASS => GLASS_COLOR,
        None => family_color(family),
    }
}

/// 🥽️ The framework mesh of one solid: positions, normals, indices and face ids as authored, colours per vertex from the group materials.
pub fn mesh_data(snapshot: &ModelSnapshot, solid: &ElementSolid) -> MeshData {
    MeshData {
        positions: solid.positions_f32(),
        normals: solid.normals_f32(),
        colors: solid.vertex_colors(|group| group_color(snapshot, solid.family, group)),
        indices: solid.indices.clone(),
        face_ids: solid.face_ids(),
        ..MeshData::default()
    }
}
//#endregion 🔖️Mesh

//#region 🔖️Camera
type Bounds = ([f64; 3], [f64; 3]);

/// 🧭️ The corners of the solid bounds in world coordinates: rotated about `+Z` by the placement, then translated.
fn world_corners(solid: &ElementSolid) -> impl Iterator<Item = [f64; 3]> {
    let (placement, bounds) = (solid.placement, solid.bounds);
    let (sin, cos) = placement.rotation.sin_cos();
    (0..8).map(move |corner| {
        let (x, y, z) = (if corner & 1 == 0 { bounds.min.x } else { bounds.max.x }, if corner & 2 == 0 { bounds.min.y } else { bounds.max.y }, if corner & 4 == 0 { bounds.min.z } else { bounds.max.z });
        [placement.x + cos * x - sin * y, placement.y + sin * x + cos * y, placement.z + z]
    })
}

/// 📦️ World-space bounds of the drawn solids, `None` when nothing is drawn.
pub fn world_bounds<'a>(solids: impl IntoIterator<Item = &'a ElementSolid>) -> Option<Bounds> {
    solids.into_iter().flat_map(world_corners).fold(None, |bounds: Option<Bounds>, corner| {
        Some(match bounds {
            None => (corner, corner),
            Some((lo, hi)) => ([lo[0].min(corner[0]), lo[1].min(corner[1]), lo[2].min(corner[2])], [hi[0].max(corner[0]), hi[1].max(corner[1]), hi[2].max(corner[2])]),
        })
    })
}

/// 🎥️ The overview pose that frames the bounds of the drawn solids from the south-east above; the default when no pose is stored.
pub fn overview_orbit(bounds: Option<Bounds>) -> store::Viewport3dOrbit {
    let (min, max) = bounds.unwrap_or(([-5.0; 3], [5.0; 3]));
    let target = [(min[0] + max[0]) / 2.0, (min[1] + max[1]) / 2.0, (min[2] + max[2]) / 2.0];
    let extent = (max[0] - min[0]).hypot(max[1] - min[1]).hypot(max[2] - min[2]).max(1.0);
    store::Viewport3dOrbit { position: [target[0] + 0.9 * extent, target[1] - 1.1 * extent, target[2] + 0.8 * extent], target, zoom: 1.0, up: None }
}
//#endregion 🔖️Camera

//#region 🔖️Scene
fn instance(snapshot: &ModelSnapshot, id: &str, solid: &ElementSolid, view: &WorldView<'_>) -> DslValue {
    let half = solid.placement.rotation / 2.0;
    DslValue::object([
        ("id".to_string(), DslValue::String(id.to_string())),
        ("meshId".to_string(), DslValue::String(id.to_string())),
        ("position".to_string(), [solid.placement.x, solid.placement.y, solid.placement.z].to_value()),
        ("rotation".to_string(), [0.0, 0.0, half.sin(), half.cos()].to_value()),
        ("scale".to_string(), [1.0_f64; 3].to_value()),
        ("label".to_string(), DslValue::String(super::element_name(snapshot, id).to_string())),
        ("selected".to_string(), DslValue::Bool(view.selected.iter().any(|selected| selected == id))),
        ("hovered".to_string(), DslValue::Bool(view.hovered.iter().any(|hovered| hovered == id))),
        ("interactionId".to_string(), DslValue::String(id.to_string())),
        ("interactionGranularityId".to_string(), DslValue::String(ELEMENT_GRANULARITY.to_string())),
    ])
}

/// 🧊️ The solids a view draws, in element id order: empty solids and solids on hidden storeys are left out.
pub fn visible<'a>(solids: &'a BTreeMap<String, ElementSolid>, hidden_storeys: &[String]) -> Vec<(&'a String, &'a ElementSolid)> {
    solids.iter().filter(|(_, solid)| !solid.is_empty() && !hidden_storeys.iter().any(|hidden| *hidden == solid.storey)).collect()
}

/// 🧊️ The World3d scene of a model: meshes and instances of the visible solids, camera, projection, selection marks, the element
/// interaction domain and a one-shot fit while no orbit is stored.
pub fn scene(snapshot: &ModelSnapshot, solids: &BTreeMap<String, ElementSolid>, view: &WorldView<'_>) -> World3dScene {
    let drawn = visible(solids, view.hidden_storeys);
    let meshes: Vec<DslValue> = drawn
        .iter()
        .map(|(id, solid)| DslValue::object([("id".to_string(), DslValue::String((*id).clone())), ("data".to_string(), semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::Value::from(mesh_data(snapshot, solid))))]))
        .collect();
    let instances: Vec<DslValue> = drawn.iter().map(|(id, solid)| instance(snapshot, id, solid, view)).collect();
    let orbit = view.orbit.copied().unwrap_or_else(|| overview_orbit(world_bounds(drawn.iter().map(|(_, solid)| *solid))));
    let camera = world3d_camera_projection_json(orbit.position, orbit.target, Some(orbit.up.unwrap_or([0.0, 0.0, 1.0])), orbit.zoom, view.projection);
    let selection = world3d_selection_json_with_granularity("pick", view.selected, view.hovered.first().map(String::as_str), Some(ELEMENT_GRANULARITY));
    let mut scene = world3d_scene(camera, semio_framework_pack_json::to_json_string(&DslValue::Array(meshes)), semio_framework_pack_json::to_json_string(&DslValue::Array(instances)), selection, &WorldSunConfig::default());
    scene.fit_json = Some(semio_framework_pack_json::to_json_string(&DslValue::object([("enabled".to_string(), DslValue::Bool(view.orbit.is_none())), ("revision".to_string(), DslValue::int(0)), ("padding".to_string(), DslValue::float(1.25))])));
    scene.domain_id = Some(ELEMENT_DOMAIN.into());
    scene.domain_granularity_id = Some(ELEMENT_GRANULARITY.into());
    scene
}
//#endregion 🔖️Scene

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
