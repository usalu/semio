use super::*;
use crate::standards::v1::subsets::any::io::export::gltf::projection::through;
use crate::standards::v1::subsets::any::io::export::gltf::testkit::house;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::compute_element_solids;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry;
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels;

fn build(snapshot: &ModelSnapshot) -> (GltfModel, Vec<String>) {
    registry::with_inference(None, snapshot, |inferred| super::build(snapshot, inferred))
}

fn parent_of(model: &GltfModel, child: usize) -> Option<usize> {
    model.nodes.iter().position(|node| node.children.contains(&child))
}

fn element_nodes(model: &GltfModel) -> Vec<usize> {
    (0..model.nodes.len()).filter(|index| model.nodes[*index].mesh.is_some()).collect()
}

fn id_of(node: &GltfNode) -> &str {
    node.extras.get("id").and_then(|id| id.as_str()).expect("an id")
}

#[test]
fn the_house_has_one_node_per_element_with_geometry_and_nothing_is_skipped() {
    let snapshot = house();
    let (model, notes) = build(&snapshot);
    let solids = compute_element_solids(&snapshot);
    assert!(notes.is_empty(), "{notes:?}");
    assert_eq!(element_nodes(&model).len(), solids.len());
    let mut ids: Vec<&str> = element_nodes(&model).iter().map(|index| id_of(&model.nodes[*index])).collect();
    ids.sort();
    assert_eq!(ids, solids.keys().map(String::as_str).collect::<Vec<_>>());
    assert_eq!(model.nodes.len(), solids.len() + snapshot.storeys.len() + snapshot.buildings.len() + snapshot.sites.len());
    assert_eq!(model.meshes.len(), solids.len());
}

#[test]
fn the_nodes_form_the_site_building_storey_element_hierarchy() {
    let snapshot = house();
    let (model, _) = build(&snapshot);
    assert_eq!(model.roots.len(), snapshot.sites.len());
    for index in element_nodes(&model) {
        let element = &model.nodes[index];
        let storey = &model.nodes[parent_of(&model, index).expect("a storey")];
        let building = &model.nodes[parent_of(&model, parent_of(&model, index).unwrap()).expect("a building")];
        assert_eq!(storey.extras.get("kind").and_then(|kind| kind.as_str()), Some("storey"));
        assert_eq!(building.extras.get("kind").and_then(|kind| kind.as_str()), Some("building"));
        assert_eq!(element.extras.get("storey").and_then(|id| id.as_str()), storey.extras.get("id").and_then(|id| id.as_str()));
    }
    let site = &model.nodes[model.roots[0]];
    assert_eq!(site.extras.get("kind").and_then(|kind| kind.as_str()), Some("site"));
    assert_eq!(site.translation, Some([0.0, snapshot.sites.values().next().unwrap().elevation, 0.0]));
}

#[test]
fn storey_nodes_sit_at_their_inferred_elevation_in_stacking_order() {
    let snapshot = house();
    let (model, _) = build(&snapshot);
    let levels = compute_storey_levels(&snapshot);
    let building = &model.nodes[model.nodes[model.roots[0]].children[0]];
    let rows: Vec<(i64, f64)> = building.children.iter().map(|child| (model.nodes[*child].extras.get("level").and_then(|level| level.as_i64()).unwrap(), model.nodes[*child].translation.unwrap()[1])).collect();
    assert_eq!(rows.iter().map(|row| row.0).collect::<Vec<_>>(), [-1, 0, 1, 2]);
    for child in &building.children {
        let node = &model.nodes[*child];
        assert_eq!(node.translation.unwrap()[1], levels[id_of(node)].elevation);
    }
    assert!(rows.windows(2).all(|pair| pair[0].1 < pair[1].1));
}

#[test]
fn the_building_node_carries_origin_elevation_and_the_rotation_about_the_vertical() {
    let snapshot = house();
    let (model, _) = build(&snapshot);
    let (_, building) = snapshot.buildings.iter().next().unwrap();
    let node = &model.nodes[model.nodes[model.roots[0]].children[0]];
    assert_eq!(node.translation, Some([building.origin.x, building.elevation, -building.origin.y]));
    let [x, y, z, w] = node.rotation.expect("a rotation");
    assert_eq!((x, z), (0.0, 0.0));
    assert!((y - (building.rotation / 2.0).sin()).abs() < 1e-15 && (w - (building.rotation / 2.0).cos()).abs() < 1e-15);
    assert!((x * x + y * y + z * z + w * w - 1.0).abs() < 1e-15);
}

#[derive(Debug)]
struct Summary {
    min: [f64; 3],
    max: [f64; 3],
    centroids: [f64; 3],
    triangles: usize,
}

fn summarize(corners: &[[f64; 3]]) -> Summary {
    let mut summary = Summary { min: [f64::INFINITY; 3], max: [f64::NEG_INFINITY; 3], centroids: [0.0; 3], triangles: corners.len() / 3 };
    for corner in corners {
        for axis in 0..3 {
            summary.min[axis] = summary.min[axis].min(corner[axis]);
            summary.max[axis] = summary.max[axis].max(corner[axis]);
            summary.centroids[axis] += corner[axis] / 3.0;
        }
    }
    summary
}

#[test]
fn the_chain_of_node_transforms_places_every_vertex_where_the_model_places_the_solid() {
    let snapshot = house();
    let (model, _) = build(&snapshot);
    let solids = compute_element_solids(&snapshot);
    let mut checked = 0;
    for index in element_nodes(&model) {
        let element = &model.nodes[index];
        let solid = &solids[id_of(element)];
        let mut chain = vec![index];
        while let Some(parent) = parent_of(&model, *chain.last().unwrap()) {
            chain.push(parent);
        }
        let (sin, cos) = solid.placement.rotation.sin_cos();
        let mut exported = Vec::new();
        for primitive in &model.meshes[element.mesh.unwrap()].primitives {
            for corner in &primitive.indices {
                let point = &primitive.positions[*corner as usize * 3..*corner as usize * 3 + 3];
                exported.push(chain.iter().fold([f64::from(point[0]), f64::from(point[1]), f64::from(point[2])], |at, node| through(&model.nodes[*node], at)));
            }
        }
        let expected: Vec<[f64; 3]> = solid
            .indices
            .iter()
            .map(|corner| {
                let at = &solid.positions[*corner as usize * 3..*corner as usize * 3 + 3];
                y_up(solid.placement.x + cos * at[0] - sin * at[1], solid.placement.y + sin * at[0] + cos * at[1], solid.placement.z + at[2])
            })
            .collect();
        let (a, b) = (summarize(&exported), summarize(&expected));
        assert_eq!(a.triangles, b.triangles, "{}", id_of(element));
        for axis in 0..3 {
            assert!((a.min[axis] - b.min[axis]).abs() < 2e-4 && (a.max[axis] - b.max[axis]).abs() < 2e-4, "{}: {a:?} vs {b:?}", id_of(element));
            assert!((a.centroids[axis] - b.centroids[axis]).abs() < 1e-5 * (1.0 + a.triangles as f64) * 40.0, "{}: {a:?} vs {b:?}", id_of(element));
        }
        checked += 1;
    }
    assert!(checked > 20);
}

#[test]
fn every_triangle_of_a_solid_is_in_exactly_one_primitive_and_materials_are_not_repeated() {
    let snapshot = house();
    let (model, _) = build(&snapshot);
    let solids = compute_element_solids(&snapshot);
    for index in element_nodes(&model) {
        let element = &model.nodes[index];
        let mesh = &model.meshes[element.mesh.unwrap()];
        assert_eq!(mesh.primitives.iter().map(GltfPrimitive::triangle_count).sum::<usize>(), solids[id_of(element)].triangle_count(), "{}", id_of(element));
        let mut used: Vec<usize> = mesh.primitives.iter().map(|primitive| primitive.material).collect();
        used.sort();
        used.dedup();
        assert_eq!(used.len(), mesh.primitives.len(), "{}", id_of(element));
        assert!(mesh.primitives.iter().all(|primitive| primitive.material < model.materials.len() && primitive.indices.iter().all(|corner| (*corner as usize) < primitive.positions.len() / 3)));
    }
}

#[test]
fn normals_stay_unit_length_and_winding_still_agrees_with_them() {
    let snapshot = house();
    let (model, _) = build(&snapshot);
    for primitive in model.meshes.iter().flat_map(|mesh| &mesh.primitives) {
        for normal in primitive.normals.chunks_exact(3) {
            let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
            assert!((length - 1.0).abs() < 1e-5, "{normal:?}");
        }
        let at = |index: u32| {
            let i = index as usize * 3;
            [f64::from(primitive.positions[i]), f64::from(primitive.positions[i + 1]), f64::from(primitive.positions[i + 2])]
        };
        let mut agree = 0;
        let mut total = 0;
        for triangle in primitive.indices.chunks_exact(3) {
            let (a, b, c) = (at(triangle[0]), at(triangle[1]), at(triangle[2]));
            let (u, v) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - a[0], c[1] - a[1], c[2] - a[2]]);
            let face = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
            let n = &primitive.normals[triangle[0] as usize * 3..triangle[0] as usize * 3 + 3];
            if face.iter().map(|x| x * x).sum::<f64>() > 1e-12 {
                total += 1;
                agree += usize::from(face[0] * f64::from(n[0]) + face[1] * f64::from(n[1]) + face[2] * f64::from(n[2]) > 0.0);
            }
        }
        assert!(total == 0 || agree * 100 >= total * 99, "{agree} of {total} triangles wind against their normal");
    }
}

#[test]
fn glazing_is_translucent_and_the_materials_of_the_model_keep_their_colour() {
    let snapshot = house();
    let (model, _) = build(&snapshot);
    assert!(model.materials.iter().any(|material| material.name == "Glazing" && material.blend && material.color[3] < 1.0));
    for material in snapshot.materials.values() {
        if let Some(defined) = model.materials.iter().find(|defined| defined.name == material.name) {
            assert_eq!(defined.color[..3], [material.color.r as f32, material.color.g as f32, material.color.b as f32]);
        }
    }
}

#[test]
fn names_fall_back_to_the_id_and_extras_carry_id_kind_and_storey() {
    let snapshot = house();
    let (model, _) = build(&snapshot);
    let wall = model.nodes.iter().find(|node| node.extras.get("id").and_then(|id| id.as_str()) == Some("w-south")).expect("the south wall");
    assert_eq!(wall.extras.get("kind").and_then(|kind| kind.as_str()), Some("wall"));
    assert_eq!(wall.extras.get("storey").and_then(|id| id.as_str()), Some("st-ground"));
    let authored = snapshot.walls["w-south"].name.as_str();
    assert_eq!(wall.name, if authored.is_empty() { "w-south" } else { authored });
}

#[test]
fn y_up_is_the_proper_rotation_that_keeps_handedness() {
    assert_eq!(y_up(1.0, 2.0, 3.0), [1.0, 3.0, -2.0]);
    let (x, y, z) = (y_up(1.0, 0.0, 0.0), y_up(0.0, 1.0, 0.0), y_up(0.0, 0.0, 1.0));
    let det = x[0] * (y[1] * z[2] - y[2] * z[1]) - x[1] * (y[0] * z[2] - y[2] * z[0]) + x[2] * (y[0] * z[1] - y[1] * z[0]);
    assert_eq!(det, 1.0);
    assert_eq!(y_up(0.0, 0.0, 0.0).map(f64::is_sign_negative), [false; 3]);
}

#[test]
fn building_the_scene_twice_gives_the_same_document() {
    let snapshot = house();
    assert_eq!(build(&snapshot).0, build(&snapshot).0);
}

#[test]
fn an_empty_model_has_an_empty_scene() {
    let (model, notes) = build(&ModelSnapshot::default());
    assert!(notes.is_empty() && model.nodes.is_empty() && model.meshes.is_empty() && model.roots.is_empty());
}
