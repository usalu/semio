
use super::*;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::Brep;

fn mesh_triangle_area(mesh: &MeshData, triangle_index: usize) -> f32 {
    let i0 = mesh.indices[triangle_index * 3] as usize;
    let i1 = mesh.indices[triangle_index * 3 + 1] as usize;
    let i2 = mesh.indices[triangle_index * 3 + 2] as usize;
    let p0 = [mesh.positions[i0 * 3], mesh.positions[i0 * 3 + 1], mesh.positions[i0 * 3 + 2]];
    let p1 = [mesh.positions[i1 * 3], mesh.positions[i1 * 3 + 1], mesh.positions[i1 * 3 + 2]];
    let p2 = [mesh.positions[i2 * 3], mesh.positions[i2 * 3 + 1], mesh.positions[i2 * 3 + 2]];
    let e0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let e1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let cross = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
    0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt()
}

#[semio_framework_async_macros::async_test]
async fn forest_wire_chains_reversed_edges_by_vertex_id() {
    let source = include_str!("../../../../📚️examples/🖼️assets/🎮️play/🔣️.json");
    let root: protocol::os_pack::json::Value = protocol::json::parse(source).expect("fixture");
    let geometry_value = root.pointer("/models/0/model/geometry").map(protocol::json::to_dsl_value);
    let geometry = parse_geometry(geometry_value.as_ref());
    let edges = edge_map(&geometry);
    let wire = geometry.wires.iter().find(|wire| wire.id == "hexagonal-cut-concrete-forest-left-wire-103").expect("wire");
    let chain = wire_vertex_chain(wire, &edges);
    assert_eq!(
        chain,
        vec![
            "hexagonal-cut-concrete-forest-left-vertex-84".to_string(),
            "hexagonal-cut-concrete-forest-left-vertex-96".to_string(),
            "hexagonal-cut-concrete-forest-left-vertex-94".to_string(),
            "hexagonal-cut-concrete-forest-left-vertex-83".to_string(),
            "hexagonal-cut-concrete-forest-left-vertex-84".to_string(),
        ]
    );
}

#[semio_framework_async_macros::async_test]
async fn forest_shape_geometry_imports_solid_handle() {
    let source = include_str!("../../../../📚️examples/🖼️assets/🎮️play/🔣️.json");
    let root: protocol::os_pack::json::Value = protocol::json::parse(source).expect("fixture");
    let geometry_value = root.pointer("/models/0/model/geometry").map(protocol::json::to_dsl_value);
    let geometry = parse_geometry(geometry_value.as_ref());
    let objects: Vec<DslValue> = root.pointer("/models/0/model/objects").and_then(|value| value.as_array()).map(|entries| entries.iter().map(protocol::json::to_dsl_value).collect()).unwrap_or_default();
    let mut kernel = Brep::new();
    let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
    assert_eq!(imported.len(), 1);
    assert!(imported[0].solid_handle.is_some());
    let mesh = tessellate_geometry_handle(&mut kernel, imported[0].solid_handle.as_ref().expect("handle"), "solid").expect("mesh");
    assert!(mesh.positions.len() > 12);
    assert!(mesh.edge_positions.len() >= 6);
    assert_eq!(mesh.edge_positions.len() % 6, 0);
    for triangle_index in 0..mesh.triangle_count() {
        assert!(mesh_triangle_area(&mesh, triangle_index) > 1e-10, "triangle {triangle_index} is degenerate");
    }
}

#[semio_framework_async_macros::async_test]
async fn forest_energy_surface_tessellates_at_authored_height() {
    let source = include_str!("../../../../📚️examples/🖼️assets/🎮️play/🔣️.json");
    let root: protocol::os_pack::json::Value = protocol::json::parse(source).expect("fixture");
    let geometry_value = root.pointer("/models/2/model/geometry").map(protocol::json::to_dsl_value);
    let geometry = parse_geometry(geometry_value.as_ref());
    let objects: Vec<DslValue> = root.pointer("/models/2/model/objects").and_then(|value| value.as_array()).map(|entries| entries.iter().map(protocol::json::to_dsl_value).collect()).unwrap_or_default();
    let mut kernel = Brep::new();
    let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
    assert_eq!(imported.len(), 1);
    assert!(imported[0].solid_handle.is_some(), "energy face handle");
    let handle_id = imported[0].solid_handle.as_ref().expect("handle");
    let mesh = tessellate_geometry_handle(&mut kernel, handle_id, "surface").expect("surface mesh");
    let min_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
    let max_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::NEG_INFINITY, f32::max);
    assert!(min_z > 2.5, "energy surface min z {min_z}");
    assert!(max_z < 3.5, "energy surface max z {max_z}");
}

#[semio_framework_async_macros::async_test]
async fn forest_structure_surface_tessellates_at_authored_height() {
    let source = include_str!("../../../../📚️examples/🖼️assets/🎮️play/🔣️.json");
    let root: protocol::os_pack::json::Value = protocol::json::parse(source).expect("fixture");
    let geometry_value = root.pointer("/models/3/model/geometry").map(protocol::json::to_dsl_value);
    let geometry = parse_geometry(geometry_value.as_ref());
    let objects: Vec<DslValue> = root.pointer("/models/3/model/objects").and_then(|value| value.as_array()).map(|entries| entries.iter().map(protocol::json::to_dsl_value).collect()).unwrap_or_default();
    let mut kernel = Brep::new();
    let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
    let slab = imported.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("surface object");
    let mesh = tessellate_geometry_handle(&mut kernel, slab.solid_handle.as_ref().expect("handle"), "surface").expect("surface mesh");
    let min_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
    assert!(min_z > 2.5, "structure slab min z {min_z}");
}

#[semio_framework_async_macros::async_test]
async fn forest_structure_curve_wires_tessellate_as_centerlines() {
    let source = include_str!("../../../../📚️examples/🖼️assets/🎮️play/🔣️.json");
    let root: protocol::os_pack::json::Value = protocol::json::parse(source).expect("fixture");
    let geometry_value = root.pointer("/models/3/model/geometry").map(protocol::json::to_dsl_value);
    let geometry = parse_geometry(geometry_value.as_ref());
    let objects: Vec<DslValue> = root.pointer("/models/3/model/objects").and_then(|value| value.as_array()).map(|entries| entries.iter().map(protocol::json::to_dsl_value).collect()).unwrap_or_default();
    let mut kernel = Brep::new();
    let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
    assert!(!imported.is_empty());
    let curve_object = imported.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "curve")).expect("curve object");
    let handle = curve_object.solid_handle.as_ref().expect("curve handle");
    let mesh = tessellate_geometry_handle(&mut kernel, handle, "curve").expect("curve mesh");
    assert!(mesh.edge_positions.len() >= 6);
    assert_eq!(mesh.edge_positions.len() % 6, 0);
    assert!(mesh.indices.is_empty());
}

//#region 🧪️ModelBridgeLaws
#[semio_framework_async_macros::async_test]
async fn cad_object_model_element_round_trip_preserves_identity_placement_and_geometry() {
    let object = CadObject {
        id: "object-7".into(),
        label: "ignored on the way in — restored from the id on the way out".into(),
        typology: "building.building.column".into(),
        visible: false,
        locked: true,
        origin: [1.5, -2.25, 3.0],
        orientation: Some([0.0, 0.707, 0.0, 0.707]),
        scale: Some([1.0, 2.0, 1.0]),
        mesh_url: None,
        extent: Some([0.5, 0.5, 3.0]),
        solid_handle: Some("brep-handle-42".into()),
        primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: "brep-handle-42".into(), kind: "solid".into() }],
    };
    let element = model_element_from_cad_object(&object);
    assert_eq!(element.id, object.id);
    assert_eq!(element.geometry, GeometryRef::Brep { brep_id: "brep-handle-42".into() });
    let restored = cad_object_from_model_element(&element);
    assert_eq!(restored.id, object.id);
    assert_eq!(restored.typology, object.typology, "typology round-trips through ElementClass::Other");
    assert_eq!(restored.origin, object.origin);
    assert_eq!(restored.orientation, object.orientation);
    assert_eq!(restored.scale, object.scale);
    assert_eq!(restored.solid_handle, object.solid_handle);
}

#[semio_framework_async_macros::async_test]
async fn semio_model_snapshot_from_objects_round_trips_via_objects_from_model_snapshot() {
    let objects = vec![
        CadObject {
            id: "object-a".into(),
            label: "A".into(),
            typology: "spatial.shape.primitive.box".into(),
            visible: true,
            locked: false,
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            extent: None,
            solid_handle: Some("h1".into()),
            primitives: Vec::new(),
        },
        CadObject {
            id: "object-b".into(),
            label: "B".into(),
            typology: "building.building.slab".into(),
            visible: true,
            locked: false,
            origin: [1.0, 2.0, 3.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            extent: None,
            solid_handle: None,
            primitives: Vec::new(),
        },
    ];
    let model = semio_model_snapshot_from_objects(&objects);
    assert_eq!(model.elements.len(), 2);
    let restored = objects_from_model_snapshot(&model);
    assert_eq!(restored.len(), 2);
    assert_eq!(restored[0].id, "object-a");
    assert_eq!(restored[0].typology, "spatial.shape.primitive.box");
    assert_eq!(restored[0].solid_handle, Some("h1".into()));
    assert_eq!(restored[1].id, "object-b");
    assert_eq!(restored[1].solid_handle, None);
}
//#endregion 🧪️ModelBridgeLaws
