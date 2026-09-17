//! 🧪️ Tests for example `🏠️house`: the committed asset IS the generated house, every solid sits on
//! the 0.5 m module the mesh conformity rests on, the meshed solids genuinely share their interface
//! nodes (one connected structure, no floating part), and the engine solves it — balanced reactions,
//! slabs and roof sagging under gravity, ULS a linear superposition.
//!
//! 🔁️ `SEMIO_FEM3D_WRITE_HOUSE_ASSET=1` rewrites `🖼️assets/🏠️house/🗣️.dsl.semio` from `build()`
//! before the comparison runs — the one sanctioned way to regenerate the committed text.

use crate::fem3d_engine::meshing::{mesh_solids, NODE_MERGE_TOLERANCE};
use crate::fem3d_engine::fem3d_solve_all;
use crate::model::Dof;
use crate::standards::v1::subsets::any::schema::snapshot::text::{parse_dsl, print_dsl};
use crate::{FemAxis, Fem3dSnapshot};
use std::collections::{HashMap, HashSet};

const ASSET_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/🌐️any/🖼️assets/🏠️house/🗣️.dsl.semio");

fn on_module(value: f64) -> bool {
    (value / super::MODULE - (value / super::MODULE).round()).abs() < 1e-9
}

fn committed() -> Fem3dSnapshot {
    if std::env::var("SEMIO_FEM3D_WRITE_HOUSE_ASSET").as_deref() == Ok("1") {
        std::fs::write(ASSET_PATH, print_dsl(&super::build())).expect("the house asset is writable");
    }
    parse_dsl(&std::fs::read_to_string(ASSET_PATH).expect("the house asset is readable")).expect("house example dsl parses")
}

#[test]
fn primary_asset_is_the_generated_house() {
    let doc = committed();
    assert_eq!(doc, super::build(), "the committed asset must be exactly print_dsl(&build()) — set SEMIO_FEM3D_WRITE_HOUSE_ASSET=1 to regenerate it");
    let embedded = parse_dsl(super::PRIMARY_TEXT).expect("embedded house dsl parses");
    assert_eq!(embedded, doc, "the embedded PRIMARY_TEXT is the committed asset");
    let reparsed = parse_dsl(&print_dsl(&doc)).expect("printed dsl reparses");
    assert_eq!(reparsed, doc, "print → parse must be the identity");
}

#[test]
fn every_solid_is_drawn_on_the_module_with_openings_and_a_gable() {
    let doc = super::build();
    assert_eq!(doc.solids.len(), 8, "raft, two slabs, four walls, roof");
    assert!(doc.elements.is_empty() && doc.sections.is_empty(), "a masonry house is solids only");
    for solid in &doc.solids {
        assert!((solid.mesh_size - super::MESH_SIZE).abs() < 1e-12, "{}: meshed at √2·MODULE so the lattice lands on the module", solid.id);
        for point in solid.outline.iter().chain(solid.holes.iter().flatten()) {
            assert!(point.iter().all(|&coordinate| on_module(coordinate)) || solid.id == "roof", "{}: outline/hole corner {point:?} must sit on the {} m module", solid.id, super::MODULE);
        }
        assert!(on_module(solid.base_z) || solid.id == "roof", "{}: the extrusion offset sits on the module", solid.id);
        assert_eq!(solid.layers, (solid.height / super::MODULE).round().max(1.0) as usize, "{}: one layer per module along the extrusion", solid.id);
    }
    let by_id: HashMap<&str, &crate::FemSolid> = doc.solids.iter().map(|solid| (solid.id.as_str(), solid)).collect();
    assert_eq!(by_id["wall_west"].axis, FemAxis::X, "a long wall is drawn in elevation and extruded through its thickness");
    assert_eq!(by_id["wall_south"].axis, FemAxis::Y);
    assert_eq!(by_id["roof"].axis, FemAxis::Y, "the roof is its chevron section swept along the ridge");
    assert_eq!(by_id["raft"].axis, FemAxis::Z);
    assert_eq!(by_id["wall_west"].holes.len(), 2, "two windows per long wall");
    assert_eq!(by_id["wall_south"].holes.len(), 2, "a door and a window in the south gable");
    assert_eq!(by_id["wall_south"].outline.len(), 7, "the gable elevation is the storey rectangle topped by the roof underside");
    let ridge = by_id["wall_south"].outline.iter().map(|point| point[1]).fold(f64::NEG_INFINITY, f64::max);
    assert_eq!(ridge, super::RIDGE);
    assert!(by_id["roof"].outline.contains(&[super::WIDTH / 2.0, super::RIDGE]), "the roof underside starts on the gable's ridge");
    assert_eq!(doc.supports.len(), doc.nodes.len(), "every document node is a raft support");
    assert!(doc.nodes.iter().all(|node| node.z == -super::RAFT_THICKNESS && on_module(node.x) && on_module(node.y)), "supports sit on the raft underside lattice");
}

#[test]
fn meshed_solids_share_their_interface_nodes_as_one_structure() {
    let doc = super::build();
    let (nodes, meshes) = mesh_solids(&doc).expect("the house meshes");
    let ids_of = |solid_id: &str| -> HashSet<&str> { meshes.iter().find(|mesh| mesh.solid_id == solid_id).expect(solid_id).node_ids.iter().map(String::as_str).collect() };
    let shared = |a: &str, b: &str| ids_of(a).intersection(&ids_of(b)).count();
    assert!(shared("raft", "wall_west") >= 2 * 15, "the west wall's foot shares its full bottom edge with the raft top, got {}", shared("raft", "wall_west"));
    assert!(shared("raft", "wall_south") >= 2 * 13, "the south gable's foot shares its bottom edge with the raft, got {}", shared("raft", "wall_south"));
    assert!(shared("wall_west", "wall_south") >= 2 * 7, "the west wall's end sits on the south gable's inner face, got {}", shared("wall_west", "wall_south"));
    assert!(shared("attic_slab", "wall_west") >= 2 * 15, "the attic slab edge bears on the west wall, got {}", shared("attic_slab", "wall_west"));
    assert!(shared("roof", "wall_south") >= 8, "the roof rests on the gable's slanted top, got {}", shared("roof", "wall_south"));
    assert!(shared("roof", "wall_west") >= 2 * 15, "the roof seat rests on the west wall top, got {}", shared("roof", "wall_west"));
    assert!(shared("ground_slab", "raft") >= 11 * 15, "the ground slab is cast on the raft, got {}", shared("ground_slab", "raft"));
    let document_ids: HashSet<&str> = doc.nodes.iter().map(|node| node.id.as_str()).collect();
    let raft = ids_of("raft");
    assert!(document_ids.iter().all(|id| raft.contains(id)), "every support node is a raft mesh node");
    let mut positions: HashMap<[i64; 3], usize> = HashMap::new();
    for node in &nodes {
        let cell = [(node.pos[0] / NODE_MERGE_TOLERANCE).round() as i64, (node.pos[1] / NODE_MERGE_TOLERANCE).round() as i64, (node.pos[2] / NODE_MERGE_TOLERANCE).round() as i64];
        *positions.entry(cell).or_default() += 1;
    }
    assert!(positions.values().all(|&count| count == 1), "no two merged nodes sit on the same point");
    let mut union: Vec<usize> = (0..nodes.len()).collect();
    fn find(union: &mut [usize], index: usize) -> usize {
        let mut root = index;
        while union[root] != root {
            root = union[root];
        }
        let mut cursor = index;
        while union[cursor] != root {
            let next = union[cursor];
            union[cursor] = root;
            cursor = next;
        }
        root
    }
    let index_of: HashMap<&str, usize> = nodes.iter().enumerate().map(|(index, node)| (node.id.as_str(), index)).collect();
    for mesh in &meshes {
        for tet in &mesh.tets {
            let first = index_of[mesh.node_ids[tet[0] as usize].as_str()];
            for &corner in &tet[1..] {
                let (a, b) = (find(&mut union, first), find(&mut union, index_of[mesh.node_ids[corner as usize].as_str()]));
                union[a] = b;
            }
        }
    }
    let roots: HashSet<usize> = (0..nodes.len()).map(|index| find(&mut union, index)).collect();
    assert_eq!(roots.len(), 1, "raft, slabs, walls and roof mesh into ONE connected structure, got {} components", roots.len());
    let tets: usize = meshes.iter().map(|mesh| mesh.tets.len()).sum();
    assert!(tets > 4_000 && tets < 40_000, "the house is a real but tractable mesh: {tets} tets over {} nodes", nodes.len());
}

#[test]
fn deformation_solves_balanced_and_sags_under_gravity() {
    let doc = super::build();
    let results = fem3d_solve_all(&doc).expect("the house solves");
    for case_id in ["dead", "live", "snow", "uls", "sls"] {
        assert!(results.contains_key(case_id), "result for {case_id}");
    }
    let (_, meshes) = mesh_solids(&doc).expect("the house meshes");
    let gravity = 9.81;
    for (case_id, result) in &results {
        assert!(result.displacements.iter().all(|row| row.values.iter().all(|value| value.is_finite())), "{case_id}: finite displacements");
        assert!(result.checks.residual_norm < 1e-6, "{case_id}: residual {}", result.checks.residual_norm);
    }
    let footprint_area = |solid_id: &str| crate::fem3d_engine::meshing::footprint_area(meshes.iter().find(|mesh| mesh.solid_id == solid_id).expect(solid_id));
    let dead = &results["dead"];
    let reaction_z: f64 = dead.reactions.iter().filter(|reaction| reaction.dof == Dof::Tz).map(|reaction| reaction.value).sum();
    let self_weight: f64 = doc
        .solids
        .iter()
        .map(|solid| {
            let rho = doc.materials.iter().find(|material| material.id == solid.material_id).expect("material").rho;
            let mesh = meshes.iter().find(|mesh| mesh.solid_id == solid.id).expect("mesh");
            let volume: f64 = mesh
                .tets
                .iter()
                .map(|tet| {
                    let p = |index: u32| mesh.points[index as usize];
                    let (a, b, c, d) = (p(tet[0]), p(tet[1]), p(tet[2]), p(tet[3]));
                    let (ab, ac, ad) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - a[0], c[1] - a[1], c[2] - a[2]], [d[0] - a[0], d[1] - a[1], d[2] - a[2]]);
                    (ab[0] * (ac[1] * ad[2] - ac[2] * ad[1]) - ab[1] * (ac[0] * ad[2] - ac[2] * ad[0]) + ab[2] * (ac[0] * ad[1] - ac[1] * ad[0])).abs() / 6.0
                })
                .sum();
            -rho * gravity * volume
        })
        .sum();
    let applied = self_weight - 1000.0 * footprint_area("attic_slab");
    assert!(((reaction_z + applied) / applied).abs() < 1e-6, "dead: vertical reactions {reaction_z} balance the applied load {applied}");
    let node_id = |solid_id: &str, predicate: &dyn Fn([f64; 3]) -> bool| -> String {
        let mesh = meshes.iter().find(|mesh| mesh.solid_id == solid_id).expect(solid_id);
        let (index, _) = mesh.points.iter().enumerate().find(|(_, point)| predicate(**point)).unwrap_or_else(|| panic!("{solid_id}: no node matches"));
        mesh.node_ids[index].clone()
    };
    let tz = |case_id: &str, node: &str| results[case_id].displacements.iter().find(|row| row.node_id == node).unwrap_or_else(|| panic!("displacement {node}")).values[2];
    let attic_mid = node_id("attic_slab", &|point| (point[0] - 3.0).abs() < 1e-9 && (point[1] - 4.0).abs() < 1e-9 && (point[2] - 3.0).abs() < 1e-9);
    let attic_edge = node_id("attic_slab", &|point| (point[0] - 0.5).abs() < 1e-9 && (point[1] - 4.0).abs() < 1e-9 && (point[2] - 3.0).abs() < 1e-9);
    let ridge_mid = node_id("roof", &|point| (point[0] - 3.0).abs() < 1e-9 && (point[1] - 4.0).abs() < 1e-9 && (point[2] - super::RIDGE).abs() < 1e-9);
    let ridge_gable = node_id("roof", &|point| (point[0] - 3.0).abs() < 1e-9 && point[1].abs() < 1e-9 && (point[2] - super::RIDGE).abs() < 1e-9);
    assert!(tz("sls", &attic_mid) < tz("sls", &attic_edge) && tz("sls", &attic_edge) < 0.0, "the attic slab sags at mid-span below its supported edge under SLS");
    assert!(tz("snow", &ridge_mid) < tz("snow", &ridge_gable), "the ridge sags at mid-length under snow, held up at the gables");
    let peak = results["sls"].displacements.iter().map(|row| row.values[2].abs()).fold(0.0, f64::max);
    assert!(peak > 1e-5 && peak < 0.05, "SLS peak vertical deflection {peak} m is millimetres, not noise and not a mechanism");
    let expected = 1.35 * tz("dead", &attic_mid) + 1.5 * tz("live", &attic_mid) + 0.75 * tz("snow", &attic_mid);
    assert!((tz("uls", &attic_mid) - expected).abs() < 1e-12, "ULS superposes the solved cases linearly");
}
