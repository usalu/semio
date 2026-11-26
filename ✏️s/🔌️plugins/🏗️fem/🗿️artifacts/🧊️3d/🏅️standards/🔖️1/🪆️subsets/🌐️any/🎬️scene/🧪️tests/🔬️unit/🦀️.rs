use super::*;
use semio_s_fem_receiving::cold::retire;

fn demo() -> Fem3dSnapshot {
    semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot().expect("demo snapshot")
}

fn instances(doc: &Fem3dSnapshot) -> Vec<Value> {
    let (_, instances_json) = fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None, semio_plugin::Locale::En).unwrap();
    semio_framework_pack_json::parse(&instances_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("instances json").as_array().cloned().expect("array")
}

fn find<'a>(instances: &'a [Value], id: &str) -> &'a Value {
    instances.iter().find(|instance| instance.get("id").and_then(Value::as_str) == Some(id)).unwrap_or_else(|| panic!("instance {id}"))
}

/// 🪪️ LAW: every document entity is one pickable instance keyed by its RAW id and tagged with its
/// granularity — the host's raycast hit IS the framework selection target.
#[test]
fn every_entity_is_an_instance_keyed_by_its_own_id_and_granularity() {
    let doc = demo();
    let all = instances(&doc);
    for node in doc.nodes.iter() {
        assert_eq!(find(&all, &fem3d_entity_target("node", &node.id).unwrap()).get("interactionGranularityId").and_then(Value::as_str), Some(FEM3D_GRANULARITY_NODE));
    }
    for element in &doc.elements {
        let instance = find(&all, &fem3d_entity_target("element", element_id(element)).unwrap());
        assert_eq!(instance.get("interactionGranularityId").and_then(Value::as_str), Some(FEM3D_GRANULARITY_ELEMENT));
        assert_eq!(instance.get("meshId").and_then(Value::as_str), Some("box"));
    }
    for solid in &doc.solids {
        let instance = find(&all, &fem3d_entity_target("solid", &solid.id).unwrap());
        assert_eq!(instance.get("interactionGranularityId").and_then(Value::as_str), Some(FEM3D_GRANULARITY_SOLID));
        assert_eq!(instance.get("meshId").and_then(Value::as_str), Some(format!("solid-{}", solid.id).as_str()));
    }
    for support in &doc.supports {
        let instance = find(&all, &fem3d_entity_target("support", &support.id).unwrap());
        assert_eq!(instance.get("interactionGranularityId").and_then(Value::as_str), Some(FEM3D_GRANULARITY_SUPPORT));
        assert_eq!(instance.get("meshId").and_then(Value::as_str), Some("cone"));
    }
    let ids: Vec<&str> = all.iter().filter_map(|instance| instance.get("id").and_then(Value::as_str)).collect();
    let mut unique = ids.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), ids.len(), "instance ids are unique: {ids:?}");
    retire(doc);
}

#[test]
fn bundled_solid_surfaces_publish_opaque_rgba_per_vertex() {
    let doc = demo();
    let stress = doc.nodes.iter().enumerate().map(|(index, node)| (node.id.as_str().to_owned(), index as f64)).collect::<HashMap<_, _>>();
    for stress in [None, Some(&stress)] {
        let (meshes, _) = fem3d_solid_mesh_entries(&doc, None, 1.0, stress, semio_plugin::Locale::En).expect("bundled solid mesh");
        assert!(!meshes.is_empty(), "the bundled demo exercises the custom solid mesh lane");
        for mesh in meshes {
            let data = mesh.get("data").expect("mesh data");
            let positions = data.get("positions").and_then(Value::as_array).expect("positions");
            let colors = data.get("colors").and_then(Value::as_array).expect("vertex colors");
            assert!(!positions.is_empty());
            assert_eq!(positions.len() % 3, 0);
            assert_eq!(colors.len(), positions.len() / 3 * 4);
            for rgba in colors.chunks_exact(4) {
                assert_eq!(rgba[3].as_f64(), Some(1.0));
            }
        }
    }
    retire(doc);
}

#[test]
fn solid_surfaces_share_positions_and_preserve_every_boundary_face() {
    let oriented_face = |points: [[f64; 3]; 3]| {
        let mut vertices = points.map(|point| point.map(f64::to_bits));
        let start = (0..vertices.len()).min_by_key(|&index| vertices[index]).unwrap();
        vertices.rotate_left(start);
        vertices
    };
    let doc = demo();
    let fixture = semio_framework_pack_json::parse(include_str!("🧫️boundary.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let original = mesh_preview::fem3d_mesh_preview(&doc).unwrap();
    let (scene, _) = fem3d_solid_mesh_entries(&doc, None, 1.0, None, semio_plugin::Locale::En).unwrap();
    assert_eq!(scene, fem3d_solid_mesh_entries(&doc, None, 1.0, None, semio_plugin::Locale::En).unwrap().0);
    assert_eq!(original.len(), scene.len());
    for (solid, mesh) in original.iter().zip(&scene) {
        let data = mesh.get("data").unwrap();
        let positions = data.get("positions").and_then(Value::as_array).unwrap();
        let indices = data.get("indices").and_then(Value::as_array).unwrap();
        let normal = data.get("attributes").and_then(|attributes| attributes.get("normal")).unwrap();
        assert_eq!(normal.get("domain").and_then(Value::as_str), Some("face"));
        assert_eq!(normal.get("values").and_then(Value::as_array).unwrap().len(), solid.boundary_tris.len());
        assert!(positions.len() < indices.len() * 3);
        assert_eq!(indices.len(), solid.boundary_tris.len() * 3);
        assert_eq!(Some(solid.solid_id.as_str()), fixture.get("solid").and_then(Value::as_str));
        assert_eq!(Some(positions.len() as u64 / 3), fixture.get("boundaryVertices").and_then(Value::as_u64));
        assert_eq!(Some(indices.len() as u64), fixture.get("expandedVertices").and_then(Value::as_u64));
        assert_eq!(Some(indices.len() as u64 / 3), fixture.get("faces").and_then(Value::as_u64));
        let mut expected: Vec<_> = solid.boundary_tris.iter().map(|face| oriented_face(face.map(|index| solid.points[index as usize]))).collect();
        let mut actual: Vec<_> = indices.chunks_exact(3).map(|face| oriented_face(std::array::from_fn(|corner| {
            let offset = face[corner].as_u64().unwrap() as usize * 3;
            std::array::from_fn(|axis| positions[offset + axis].as_f64().unwrap())
        }))).collect();
        expected.sort_unstable();
        actual.sort_unstable();
        assert_eq!(expected, actual);
    }
    retire(doc);
}

/// 🏋️ LAW: a load is drawn as arrow glyphs whose hits redirect onto the load id — a nodal load ends
/// at its node pointing along its DOF, a member load repeats along the member, a surface load
/// stands on the solid's top.
#[test]
fn load_glyphs_redirect_their_hit_onto_the_load() {
    let doc = demo();
    let all = instances(&doc);
    let glyphs: Vec<&Value> = all.iter().filter(|instance| instance.get("interactionId").and_then(Value::as_str) == Some(fem3d_load_target("live", "l2").unwrap().as_str())).collect();
    assert_eq!(glyphs.len(), 2, "one head and one shaft: {glyphs:?}");
    let head = find(&all, &fem3d_entity_target("load", &format!("{}:head", fem3d_load_target("live", "l2").unwrap())).unwrap());
    let position: Vec<f64> = head.get("position").and_then(Value::as_array).expect("position").iter().filter_map(Value::as_f64).collect();
    assert!((position[2] - (2.8 + LOAD_ARROW_HEAD_3D)).abs() < 1e-9, "a downward Tz load's head base sits above the node by the head length: {position:?}");
    let area: Vec<&Value> = all.iter().filter(|instance| instance.get("interactionId").and_then(Value::as_str) == Some(fem3d_load_target("dead", "l1").unwrap().as_str())).collect();
    assert_eq!(area.len(), 2);
    let udl_doc = {
        let mut doc = demo();
        doc.load_cases[0].loads.push(FemLoad::MemberUdl { id: "udl".into(), element_id: "fb1_0".into(), wx: 0.0, wy: 0.0, wz: -1000.0 });
        doc
    };
    let udl = instances(&udl_doc).into_iter().filter(|instance| instance.get("interactionId").and_then(Value::as_str) == Some(fem3d_load_target("dead", "udl").unwrap().as_str())).count();
    assert_eq!(udl, MEMBER_LOAD_ARROWS_3D * 2);
    retire(udl_doc);
    retire(doc);
}

/// 🌐️ LAW: the deformed scene moves nodes, their members and the glyphs riding them by the same
/// scaled displacement.
#[test]
fn displacements_move_nodes_members_supports_and_load_glyphs_together() {
    let doc = demo();
    let mut displacements = HashMap::new();
    displacements.insert("n20_l1".to_string(), [0.0, 0.0, -0.01, 0.0, 0.0, 0.0]);
    let (_, json) = fem3d_scene_parts(&doc, Some(&displacements), 100.0, None, semio_plugin::Locale::En).unwrap();
    let all: Vec<Value> = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("json").as_array().cloned().expect("array");
    let z = |id: &str| find(&all, id).get("position").and_then(Value::as_array).expect("position")[2].as_f64().expect("z");
    assert!((z(&fem3d_entity_target("node", "n20_l1").unwrap()) - 1.8).abs() < 1e-9, "the node dropped by one metre at scale 100");
    assert!((z(&fem3d_entity_target("load", &format!("{}:head", fem3d_load_target("live", "l2").unwrap())).unwrap()) - (1.8 + LOAD_ARROW_HEAD_3D)).abs() < 1e-9, "the load arrow rides its node");
    retire(doc);
}

/// 🧭️ LAW: the cone quaternion takes the mesh's +Y axis onto the requested direction.
#[test]
fn cone_orientation_takes_y_onto_the_direction() {
    let rotate = |q: [f64; 4], v: [f64; 3]| {
        let p = [v[0], v[1], v[2], 0.0];
        let conjugate = [-q[0], -q[1], -q[2], q[3]];
        let r = quat_mul(quat_mul(q, p), conjugate);
        [r[0], r[1], r[2]]
    };
    for direction in [[0.0, 0.0, 1.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.6, 0.0, 0.8]] {
        let rotated = rotate(quat_y_to(direction), [0.0, 1.0, 0.0]);
        for axis in 0..3 {
            assert!((rotated[axis] - direction[axis]).abs() < 1e-9, "{direction:?} → {rotated:?}");
        }
    }
}


/// 🚨️ LAW (F2): a solid the mesher refuses never takes the others with it: every meshable solid keeps its surface mesh and
/// instance, the refused one is still drawn as an error-tinted prism under its own id and its label is the diagnostic
/// naming it in the requested locale ONLY (English text for `En`, German text for `De`, never both).
#[test]
fn an_unmeshable_solid_is_marked_and_the_others_still_draw() {
    let mut doc = demo();
    let mut bad = doc.solids[0].clone();
    bad.id = "bad".into();
    bad.outline = vec![[0.0, 0.0], [1.0, 0.0]];
    doc.solids.push(bad);
    for (locale, present, absent) in [(semio_plugin::Locale::En, "Solid bad cannot be meshed", "Körper"), (semio_plugin::Locale::De, "Körper bad lässt sich nicht vernetzen", "Solid bad")] {
        let (meshes_json, instances_json) = fem3d_scene_parts(&doc, None, doc.analysis.deformation_scale, None, locale).unwrap();
        let meshes = semio_framework_pack_json::parse(&meshes_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let ids: Vec<&str> = meshes.as_array().unwrap().iter().filter_map(|mesh| mesh.get("id").and_then(Value::as_str)).collect();
        assert!(ids.contains(&"solid-sol1") && ids.contains(&"solid-bad"), "{ids:?}");
        let all = semio_framework_pack_json::parse(&instances_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap().as_array().cloned().unwrap();
        assert_eq!(find(&all, &fem3d_entity_target("solid", "sol1").unwrap()).get("label").and_then(Value::as_str), Some("sol1"));
        let label = find(&all, &fem3d_entity_target("solid", "bad").unwrap()).get("label").and_then(Value::as_str).unwrap().to_owned();
        assert!(label.starts_with(present) && !label.contains(absent) && !label.contains(" | "), "{locale:?}: {label}");
    }
    retire(doc);
}
