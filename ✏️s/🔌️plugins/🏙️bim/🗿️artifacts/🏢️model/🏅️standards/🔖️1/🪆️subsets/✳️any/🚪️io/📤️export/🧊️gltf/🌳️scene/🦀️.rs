//! 🌳️ The glTF scene of a BIM model: site, building and storey nodes (translated and rotated like the model places them) and one node per element that has geometry.
//!
//! Frames: the model is Z-up (`x` east, `y` north), glTF is Y-up (`x` right, `y` up, `-z` forward); [`y_up`] is the proper rotation `(x, y, z) -> (x, z, -y)`, so winding and
//! handedness survive. The site node lifts by the site elevation, the building node moves to its origin, elevation and rotation about the vertical, the storey node lifts by its
//! elevation above the building datum; element vertices are expressed relative to their storey, so the chain of node transforms reproduces the world placement of the solid.
//! 📎 https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#coordinate-system-and-units

use super::document::{GltfMesh, GltfModel, GltfNode, GltfPrimitive};
use super::materials::Palette;
use crate::render::element_name;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{ElementSolid, SolidFamily};
use crate::{ModelInference, ModelSnapshot};
use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet};

/// 🧭️ A model point (Z-up) in glTF's Y-up frame; negative zero is normalised to zero.
pub fn y_up(x: f64, y: f64, z: f64) -> [f64; 3] {
    [x + 0.0, z + 0.0, -y + 0.0]
}

/// 🏷️ The kind an element node carries in its `extras`.
pub fn kind_of(family: SolidFamily) -> &'static str {
    match family {
        SolidFamily::Wall => "wall",
        SolidFamily::CurtainWall => "curtain-wall",
        SolidFamily::Window => "window",
        SolidFamily::Door => "door",
        SolidFamily::Column => "column",
        SolidFamily::Beam => "beam",
        SolidFamily::Slab => "slab",
        SolidFamily::Roof => "roof",
        SolidFamily::Stair => "stair",
        SolidFamily::Ramp => "ramp",
        SolidFamily::Railing => "railing",
        SolidFamily::Ceiling => "ceiling",
        SolidFamily::WallSweep => "wall-sweep",
        SolidFamily::Component => "component",
        SolidFamily::Mep => "mep",
    }
}

fn extras<const N: usize>(rows: [(&str, DslValue); N]) -> DslValue {
    DslValue::object(rows.into_iter().map(|(key, value)| (key.to_string(), value)))
}

fn text(value: &str) -> DslValue {
    DslValue::String(value.to_string())
}

fn label<'a>(name: &'a str, id: &'a str) -> &'a str {
    if name.is_empty() {
        id
    } else {
        name
    }
}

#[derive(Default)]
struct Primitive {
    primitive: GltfPrimitive,
    numbers: BTreeMap<u32, u32>,
}

fn mesh_of(snapshot: &ModelSnapshot, name: &str, solid: &ElementSolid, elevation: f64, palette: &mut Palette) -> GltfMesh {
    let mut primitives: BTreeMap<usize, Primitive> = BTreeMap::new();
    let vertex = |index: u32| index as usize * 3..index as usize * 3 + 3;
    for (triangle, corners) in solid.indices.chunks_exact(3).enumerate() {
        let group = solid.face_groups.get(triangle).and_then(|group| solid.groups.get(*group as usize)).cloned().unwrap_or_default();
        let material = palette.index_of(snapshot, solid.family, &group);
        let slot = primitives.entry(material).or_insert_with(|| Primitive { primitive: GltfPrimitive { material, ..GltfPrimitive::default() }, numbers: BTreeMap::new() });
        for corner in corners {
            let number = match slot.numbers.get(corner) {
                Some(number) => *number,
                None => {
                    let number = (slot.primitive.positions.len() / 3) as u32;
                    let (position, normal) = (&solid.positions[vertex(*corner)], &solid.normals[vertex(*corner)]);
                    let [x, y, z] = y_up(position[0], position[1], position[2] - elevation);
                    slot.primitive.positions.extend([x as f32, y as f32, z as f32]);
                    let [nx, ny, nz] = y_up(normal[0], normal[1], normal[2]);
                    slot.primitive.normals.extend([nx as f32, ny as f32, nz as f32]);
                    slot.numbers.insert(*corner, number);
                    number
                }
            };
            slot.primitive.indices.push(number);
        }
    }
    GltfMesh { name: name.to_string(), primitives: primitives.into_values().map(|slot| slot.primitive).collect() }
}

struct Tree {
    nodes: Vec<GltfNode>,
    meshes: Vec<GltfMesh>,
}

impl Tree {
    fn push(&mut self, parent: Option<usize>, node: GltfNode) -> usize {
        self.nodes.push(node);
        let index = self.nodes.len() - 1;
        if let Some(parent) = parent {
            self.nodes[parent].children.push(index);
        }
        index
    }
}

fn spatial(name: &str, translation: Option<[f64; 3]>, rotation: Option<[f64; 4]>, extras: DslValue) -> GltfNode {
    GltfNode { name: name.to_string(), translation, rotation, mesh: None, children: Vec::new(), extras }
}

/// 🌳️ The glTF model of `snapshot` from its inference plus a note per element that could not be placed in the site, building and storey hierarchy.
pub fn build(snapshot: &ModelSnapshot, inferred: &ModelInference) -> (GltfModel, Vec<String>) {
    let (solids, levels) = (&inferred.element_solids, &inferred.storey_levels);
    let mut by_storey: BTreeMap<&str, Vec<(&String, &ElementSolid)>> = BTreeMap::new();
    solids.iter().for_each(|(id, solid)| by_storey.entry(solid.storey.as_str()).or_default().push((id, solid)));
    let mut palette = Palette::default();
    let mut tree = Tree { nodes: Vec::new(), meshes: Vec::new() };
    let (mut roots, mut placed) = (Vec::new(), BTreeSet::new());
    for (site_id, site) in &snapshot.sites {
        let site_node = tree.push(None, spatial(label(&site.name, site_id), Some([0.0, site.elevation + 0.0, 0.0]), None, extras([("kind", text("site")), ("id", text(site_id)), ("name", text(&site.name)), ("latitude", DslValue::float(site.latitude)), ("longitude", DslValue::float(site.longitude)), ("trueNorth", DslValue::float(site.true_north))])));
        roots.push(site_node);
        for (building_id, building) in snapshot.buildings.iter().filter(|(_, building)| building.site == *site_id) {
            let (sin, cos) = (building.rotation * 0.5).sin_cos();
            let [x, y, z] = y_up(building.origin.x, building.origin.y, building.elevation);
            let building_node = tree.push(Some(site_node), spatial(label(&building.name, building_id), Some([x, y, z]), (building.rotation != 0.0).then_some([0.0, sin, 0.0, cos]), extras([("kind", text("building")), ("id", text(building_id)), ("name", text(&building.name)), ("rotation", DslValue::float(building.rotation))])));
            let mut storeys: Vec<_> = snapshot.storeys.iter().filter(|(_, storey)| storey.building == *building_id).collect();
            storeys.sort_by(|a, b| (a.1.level, a.0).cmp(&(b.1.level, b.0)));
            for (storey_id, storey) in storeys {
                let elevation = levels.get(storey_id).map_or(0.0, |level| level.elevation);
                let storey_node = tree.push(Some(building_node), spatial(label(&storey.name, storey_id), Some([0.0, elevation + 0.0, 0.0]), None, extras([("kind", text("storey")), ("id", text(storey_id)), ("name", text(&storey.name)), ("level", DslValue::int(i64::from(storey.level))), ("elevation", DslValue::float(elevation)), ("height", DslValue::float(storey.height))])));
                for (id, solid) in by_storey.get(storey_id.as_str()).into_iter().flatten() {
                    let name = element_name(snapshot, id);
                    let mesh = mesh_of(snapshot, label(name, id), solid, elevation, &mut palette);
                    tree.meshes.push(mesh);
                    let node = GltfNode {
                        name: label(name, id).to_string(),
                        translation: None,
                        rotation: None,
                        mesh: Some(tree.meshes.len() - 1),
                        children: Vec::new(),
                        extras: extras([("kind", text(kind_of(solid.family))), ("id", text(id)), ("name", text(name)), ("storey", text(storey_id))]),
                    };
                    tree.push(Some(storey_node), node);
                    placed.insert(id.as_str());
                }
            }
        }
    }
    let notes = solids.iter().filter(|(id, _)| !placed.contains(id.as_str())).map(|(id, solid)| format!("element {id}: storey {} is not part of a building on a site", solid.storey)).collect();
    let project = &snapshot.project;
    let model = GltfModel {
        name: label(&project.name, "BIM model").to_string(),
        extras: extras([("project", text(&project.name)), ("author", text(&project.author)), ("organization", text(&project.organization)), ("sourceUpAxis", text("Z")), ("unit", text("metre"))]),
        materials: palette.materials,
        meshes: tree.meshes,
        nodes: tree.nodes,
        roots,
    };
    (model, notes)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
