use super::*;

fn demo() -> Fem3dSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_demo_snapshot()
}

fn instances(doc: &Fem3dSnapshot) -> Vec<Value> {
    let (_, instances_json) = fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None);
    dsl::json::parse(&instances_json).expect("instances json").as_array().cloned().expect("array")
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
    for node in &doc.nodes {
        assert_eq!(find(&all, &node.id).get("interactionGranularityId").and_then(Value::as_str), Some(FEM3D_GRANULARITY_NODE));
    }
    for element in &doc.elements {
        let instance = find(&all, element_id(element));
        assert_eq!(instance.get("interactionGranularityId").and_then(Value::as_str), Some(FEM3D_GRANULARITY_ELEMENT));
        assert_eq!(instance.get("meshId").and_then(Value::as_str), Some("box"));
    }
    for solid in &doc.solids {
        let instance = find(&all, &solid.id);
        assert_eq!(instance.get("interactionGranularityId").and_then(Value::as_str), Some(FEM3D_GRANULARITY_SOLID));
        assert_eq!(instance.get("meshId").and_then(Value::as_str), Some(format!("solid-{}", solid.id).as_str()));
    }
    for support in &doc.supports {
        let instance = find(&all, &support.id);
        assert_eq!(instance.get("interactionGranularityId").and_then(Value::as_str), Some(FEM3D_GRANULARITY_SUPPORT));
        assert_eq!(instance.get("meshId").and_then(Value::as_str), Some("cone"));
    }
    let ids: Vec<&str> = all.iter().filter_map(|instance| instance.get("id").and_then(Value::as_str)).collect();
    let mut unique = ids.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), ids.len(), "instance ids are unique: {ids:?}");
}

/// 🏋️ LAW: a load is drawn as arrow glyphs whose hits redirect onto the load id — a nodal load ends
/// at its node pointing along its DOF, a member load repeats along the member, a surface load
/// stands on the solid's top.
#[test]
fn load_glyphs_redirect_their_hit_onto_the_load() {
    let doc = demo();
    let all = instances(&doc);
    let glyphs: Vec<&Value> = all.iter().filter(|instance| instance.get("interactionId").and_then(Value::as_str) == Some("l2")).collect();
    assert_eq!(glyphs.len(), 2, "one head and one shaft: {glyphs:?}");
    let head = find(&all, "l2:head");
    let position: Vec<f64> = head.get("position").and_then(Value::as_array).expect("position").iter().filter_map(Value::as_f64).collect();
    assert!((position[2] - (2.8 + LOAD_ARROW_HEAD_3D)).abs() < 1e-9, "a downward Tz load's head base sits above the node by the head length: {position:?}");
    let area: Vec<&Value> = all.iter().filter(|instance| instance.get("interactionId").and_then(Value::as_str) == Some("l1")).collect();
    assert_eq!(area.len(), 2);
    let udl_doc = {
        let mut doc = demo();
        doc.load_cases[0].loads.push(FemLoad::MemberUdl { id: "udl".into(), element_id: "fb1_0".into(), wx: 0.0, wy: 0.0, wz: -1000.0 });
        doc
    };
    let udl = instances(&udl_doc).into_iter().filter(|instance| instance.get("interactionId").and_then(Value::as_str) == Some("udl")).count();
    assert_eq!(udl, MEMBER_LOAD_ARROWS_3D * 2);
}

/// 🌐️ LAW: the deformed scene moves nodes, their members and the glyphs riding them by the same
/// scaled displacement.
#[test]
fn displacements_move_nodes_members_supports_and_load_glyphs_together() {
    let doc = demo();
    let mut displacements = HashMap::new();
    displacements.insert("n20_l1".to_string(), [0.0, 0.0, -0.01, 0.0, 0.0, 0.0]);
    let (_, json) = fem3d_scene_parts(&doc, Some(&displacements), 100.0, None);
    let all: Vec<Value> = dsl::json::parse(&json).expect("json").as_array().cloned().expect("array");
    let z = |id: &str| find(&all, id).get("position").and_then(Value::as_array).expect("position")[2].as_f64().expect("z");
    assert!((z("n20_l1") - 1.8).abs() < 1e-9, "the node dropped by one metre at scale 100");
    assert!((z("l2:head") - (1.8 + LOAD_ARROW_HEAD_3D)).abs() < 1e-9, "the load arrow rides its node");
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

