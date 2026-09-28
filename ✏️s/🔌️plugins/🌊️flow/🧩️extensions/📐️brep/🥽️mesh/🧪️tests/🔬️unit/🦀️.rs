//! 🧪️ The mesh contract is shared with the independent Three.js oracle.
use super::*;

#[semio_framework_async_macros::async_test]
async fn mesh_knife_widget_preserves_shared_fixture_surfaces() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let fixtures = pack::json::parse(include_str!("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/✂️knife-cut/🔣️.json")).unwrap();
    let info = registry.operator_info("brep.mesh.knifeCut").unwrap();
    assert!(info.inputs.iter().any(|channel| channel.name == "start" && channel.value_types.iter().any(|kind| kind == "point")));
    assert!(info.outputs.iter().any(|channel| channel.name == "meshOut" && channel.value_types.iter().any(|kind| kind == "mesh")));
    for case in fixtures.get("cases").unwrap().as_array().unwrap() {
        let source = decode_mesh(&pack::json::to_string(case.get("mesh").unwrap())).unwrap();
        let seed = neural_engine::ColdOwner::new(mesh_output(&source).unwrap());
        let cut = case.get("cut").unwrap();
        let coordinates = |key: &str| [0, 1, 2].map(|axis| cut.get(key).unwrap().as_array().unwrap()[axis].as_f64().unwrap());
        let output = registry.dispatch_cold("brep.mesh.knifeCut", Dictionary::new()
            .insert("mesh", seed.get("meshOut").unwrap().clone())
            .insert("face", Value::Dictionary(number_dictionary(cut.get("face").unwrap().as_f64().unwrap())))
            .insert("start", Value::Dictionary(point_dictionary(coordinates("start"))))
            .insert("end", Value::Dictionary(point_dictionary(coordinates("end"))))).unwrap();
        let mesh = read_mesh(&output, "meshOut").unwrap();
        let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap());
        for (key, expected) in case.get("expected").unwrap().as_object().unwrap() {
            let expected = expected.as_f64().unwrap();
            if key == "minimumFaces" { assert!(mesh.face_count() as f64 >= expected); continue; }
            let actual = read_channel_number(&report, key).unwrap();
            if expected == 0.0 { assert_eq!(actual, 0.0); } else { assert!((actual / expected - 1.0).abs() < 1e-6, "{key}: {actual} != {expected}"); }
        }
        eprintln!("[DEBUG] knife widget {}: {} vertices, {} faces", case.get("name").unwrap().as_str().unwrap(), mesh.vertex_count(), mesh.face_count());
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_component_transforms_match_shared_fixtures() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let fixture = pack::json::parse(include_str!("../../🧫️fixtures/🧭️component-transform/🔣️.json")).unwrap();
    let source = decode_mesh(&pack::json::to_string(fixture.get("mesh").unwrap())).unwrap();
    for case in fixture.get("cases").unwrap().as_array().unwrap() {
        let transform = case.get("transform").unwrap();
        let operation = transform.get("operation").unwrap().as_str().unwrap();
        let coordinates = |key: &str| [0, 1, 2].map(|axis| transform.get(key).unwrap().as_array().unwrap()[axis].as_f64().unwrap());
        let seed = neural_engine::ColdOwner::new(mesh_output(&source).unwrap());
        let pivot = transform.get("pivot").unwrap();
        let input = Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone())
            .insert("mode", Value::Dictionary(text_dictionary(transform.get("mode").unwrap().as_str().unwrap())))
            .insert("selection", Value::Dictionary(text_dictionary(pack::json::to_string(transform.get("selection").unwrap()))))
            .insert(match operation { "translate" => "offset", "rotate" => "axis", _ => "factor" }, Value::Dictionary(vector_dictionary(coordinates("vector"))))
            .insert("angle", Value::Dictionary(number_dictionary(transform.get("angle").unwrap().as_f64().unwrap())))
            .insert("pivot", Value::Dictionary(text_dictionary(if pivot.as_str() == Some("selection") { "selection" } else { "point" })))
            .insert("center", Value::Dictionary(point_dictionary(if pivot.as_array().is_some() { coordinates("pivot") } else { [0.0; 3] })));
        let output = registry.dispatch_cold(&format!("brep.mesh.{operation}Components"), input).unwrap();
        let mesh = read_mesh(&output, "meshOut").unwrap();
        assert_eq!(mesh.face_count(), source.face_count());
        for id in 0..mesh.face_count() { assert_eq!(mesh.face_vertex_ids(FaceId(id as u32)).unwrap(), source.face_vertex_ids(FaceId(id as u32)).unwrap()); }
        for (id, expected) in case.get("expected").unwrap().as_array().unwrap().iter().enumerate() {
            let actual = mesh.vertex_position(VertexId(id as u32)).unwrap().0;
            for axis in 0..3 { assert!((actual[axis] as f64 - expected.as_array().unwrap()[axis].as_f64().unwrap()).abs() < 1e-5); }
        }
        eprintln!("[DEBUG] mesh component transform: {}, {} vertices", case.get("name").unwrap().as_str().unwrap(), mesh.vertex_count());
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_loop_cut_consumes_preview_halfedge_ids() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let source = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let preview = source.tessellate().unwrap();
    let edge = *preview.edge_ids.iter().find(|&&id| id as usize >= source.edge_count()).expect("preview has sparse halfedge ids");
    let seed = neural_engine::ColdOwner::new(mesh_output(&source).unwrap());
    let output = registry.dispatch_cold("brep.mesh.loopCut", Dictionary::new()
        .insert("mesh", seed.get("meshOut").unwrap().clone())
        .insert("edges", Value::Dictionary(text_dictionary(format!("[{edge}]"))))
        .insert("cuts", Value::Dictionary(number_dictionary(2.0)))).unwrap();
    let mesh = read_mesh(&output, "meshOut").unwrap();
    assert_eq!(mesh.vertex_count(), 16);
    assert_eq!(mesh.face_count(), 14);
    let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap());
    assert_eq!(read_channel_number(&report, "boundaryEdges").unwrap(), 0.0);
    assert!((read_channel_number(&report, "volume").unwrap() - 1.0).abs() < 1e-6);
    eprintln!("[DEBUG] loop-cut widget: preview edge={edge}, vertices={}, faces={}", mesh.vertex_count(), mesh.face_count());
}

#[semio_framework_async_macros::async_test]
async fn brep_scale_preserves_each_axis_and_explicit_center() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let fixture = pack::json::parse(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture.get("brepScales").unwrap().as_array().unwrap() {
        let coordinates = |key: &str| [0, 1, 2].map(|axis| case.get(key).unwrap().as_array().unwrap()[axis].as_f64().unwrap());
        let input = Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0)));
        let source = registry.dispatch_cold("brep.prim3d.box", input).unwrap();
        let output = registry.dispatch_cold("brep.xform.scale", Dictionary::new().insert("geometry", source.get("solid").unwrap().clone()).insert("factor", Value::Dictionary(vector_dictionary(coordinates("factors")))).insert("center", Value::Dictionary(point_dictionary(coordinates("center"))))).unwrap();
        let handle = read_geometry(&output, "geometryOut").unwrap();
        with_kernel_read(|kernel| {
            let mesh = kernel.tessellate(&handle, 0.1).map_err(|error| map_kernel_error(&error))?;
            let minimum = [0, 1, 2].map(|axis| mesh.position.chunks_exact(3).map(|point| point[axis] as f64).fold(f64::INFINITY, f64::min));
            let maximum = [0, 1, 2].map(|axis| mesh.position.chunks_exact(3).map(|point| point[axis] as f64).fold(f64::NEG_INFINITY, f64::max));
            let volume = kernel.volume(&handle).map_err(|error| map_kernel_error(&error))?;
            for axis in 0..3 {
                assert!((minimum[axis] - coordinates("minimum")[axis]).abs() < 1e-6);
                assert!((maximum[axis] - coordinates("maximum")[axis]).abs() < 1e-6);
            }
            assert!((volume - case.get("volume").unwrap().as_f64().unwrap()).abs() < 1e-6);
            eprintln!("[DEBUG] B-Rep axis scale: factors={:?}, volume={volume}", coordinates("factors"));
            Ok(())
        }).unwrap();
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_reflections_preserve_outward_winding_at_every_scale() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let fixture = pack::json::parse(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture.get("reflections").unwrap().as_array().unwrap() {
        let factors = case.get("factors").unwrap().as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>();
        let seed = neural_engine::ColdOwner::new(mesh_output(&HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap()).unwrap());
        let factor = Dictionary::with_schema("vector").insert("x", Value::Atom(Atom::Decimal(factors[0]))).insert("y", Value::Atom(Atom::Decimal(factors[1]))).insert("z", Value::Atom(Atom::Decimal(factors[2])));
        let output = registry.dispatch_cold("brep.mesh.scale", Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone()).insert("factor", Value::Dictionary(factor))).unwrap();
        let mesh = read_mesh(&output, "meshOut").unwrap();
        let triangles = mesh.tessellate().unwrap();
        let mut volume = 0.0;
        for face in triangles.indices.chunks_exact(3) {
            let point = |index: u32| [0, 1, 2].map(|axis| triangles.positions[index as usize * 3 + axis] as f64);
            let [a, b, c] = [point(face[0]), point(face[1]), point(face[2])];
            volume += (a[0] * (b[1] * c[2] - b[2] * c[1]) + a[1] * (b[2] * c[0] - b[0] * c[2]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0;
        }
        let expected = case.get("volume").unwrap().as_f64().unwrap();
        assert!((volume / expected - 1.0).abs() < 1e-6, "{}: signed volume {volume}", case.get("name").unwrap().as_str().unwrap());
    }
}

#[test]
fn indexed_mesh_contract_fixtures() {
    let fixture = pack::json::parse(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture.get("meshes").unwrap().as_array().unwrap() {
        let mesh = decode_mesh(&pack::json::to_string(case.get("mesh").unwrap())).unwrap();
        let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap());
        for (key, expected) in case.get("expected").unwrap().as_object().unwrap() {
            if let Some(number) = expected.as_f64() {
                let actual = report.get(key).unwrap().as_dictionary().unwrap().get("value").unwrap().as_atom().unwrap().as_f64().unwrap();
                assert!((actual - number).abs() <= if number == 0.0 { 1e-12 } else { number.abs() * 1e-5 }, "{key}: {actual} != {number}");
            } else { assert!(report.get(key).is_none(), "open surfaces cannot report enclosed volume"); }
        }
        eprintln!("[DEBUG] mesh fixture {}: area={}", case.get("name").unwrap().as_str().unwrap(), read_channel_number(&report, "area").unwrap());
        assert_eq!(mesh.to_obj().unwrap(), decode_mesh(&encode_mesh(&mesh).unwrap()).unwrap().to_obj().unwrap());
    }
    for case in fixture.get("invalid").unwrap().as_array().unwrap() {
        assert!(decode_mesh(&pack::json::to_string(case.get("mesh").unwrap())).is_err());
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_widgets_are_registered_and_typed() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    for id in ["brep.mesh.box", "brep.mesh.construct", "brep.mesh.fromBrep", "brep.mesh.extrude", "brep.mesh.inset", "brep.mesh.loopCut", "brep.mesh.knifeCut", "brep.mesh.analyze", "brep.mesh.exportObj"] {
        let info = registry.operator_info(id).expect(id);
        assert!(info.group.iter().any(|group| group.starts_with("Mesh")));
        assert!(!info.outputs.is_empty());
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_widget_workflow_fixtures_execute() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let fixture = pack::json::parse(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture.get("workflows").unwrap().as_array().unwrap() {
        let operation = case.get("operation").unwrap().as_str().unwrap();
        let id = format!("brep.mesh.{operation}");
        let info = registry.operator_info(&id).unwrap();
        let mut input = Dictionary::new();
        for channel in &info.inputs {
            if let Some(value) = &channel.default { input = input.insert(channel.name.clone(), value.clone()); }
        }
        if info.inputs.iter().any(|channel| channel.name == "mesh") {
            let seed = neural_engine::ColdOwner::new(mesh_output(&HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap()).unwrap());
            input = input.insert("mesh", seed.get("meshOut").unwrap().clone());
        }
        let output = registry.dispatch_cold(&id, input).unwrap_or_else(|error| panic!("{id}: {error:?}"));
        let mesh = output.get("meshOut").unwrap().as_dictionary().unwrap();
        let mesh = decode_mesh(mesh.get("data").unwrap().as_atom().unwrap().as_str().unwrap()).unwrap();
        let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap());
        for (key, expected) in case.get("expected").unwrap().as_object().unwrap() {
            let actual = report.get(key).unwrap().as_dictionary().unwrap().get("value").unwrap().as_atom().unwrap().as_f64().unwrap();
            assert!((actual - expected.as_f64().unwrap()).abs() < 1e-5, "{id} {key}: {actual} != {expected:?}");
        }
        eprintln!("[DEBUG] {id}: {} vertices, {} faces", mesh.vertex_count(), mesh.face_count());
    }
}

#[test]
fn mesh_preview_preserves_topology_identifiers() {
    let output = neural_engine::ColdOwner::new(mesh_output(&HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap()).unwrap());
    let dictionary = output.get("meshOut").unwrap().as_dictionary().unwrap();
    let body = dictionary.get("preview").unwrap().as_atom().unwrap().as_str().unwrap();
    let preview = decode_mesh_pack(&decode_base64(body).unwrap()).unwrap();
    assert_eq!(preview.indices.len(), 36);
    assert_eq!(preview.face_ids.len(), 12);
    assert!(preview.face_ids.iter().all(|id| *id < 6));
    assert!(preview.vertex_ids.iter().all(|id| *id < 8));
    eprintln!("[DEBUG] native mesh preview: {} triangles, {} face ids", preview.indices.len() / 3, preview.face_ids.len());
}

#[test]
fn brep_tessellation_seams_become_shared_mesh_vertices() {
    let source = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap().tessellate().unwrap();
    let welded = indexed_triangle_mesh(&source.positions, &source.indices).unwrap();
    assert_eq!(welded.vertex_count(), 8);
    let report = neural_engine::ColdOwner::new(analyze(&welded).unwrap());
    assert_eq!(read_channel_number(&report, "boundaryEdges").unwrap(), 0.0);
    assert!((read_channel_number(&report, "volume").unwrap() - 1.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn mesh_workbench_creates_edits_analyzes_and_converts() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let defaults = |id: &str| registry.operator_info(id).unwrap().inputs.iter().fold(Dictionary::new(), |input, channel| match &channel.default {
        Some(value) => input.insert(channel.name.clone(), value.clone()),
        None => input,
    });
    let mut output = registry.dispatch_cold("brep.mesh.box", defaults("brep.mesh.box")).unwrap();
    for (id, key, value) in [("brep.mesh.inset", "amount", 0.1), ("brep.mesh.extrude", "distance", 0.5)] {
        let input = defaults(id).insert("mesh", output.get("meshOut").unwrap().clone()).insert(key, Value::Dictionary(number_dictionary(value)));
        output = registry.dispatch_cold(id, input).unwrap();
    }
    let analyzed = registry.dispatch_cold("brep.mesh.analyze", Dictionary::new().insert("mesh", output.get("meshOut").unwrap().clone())).unwrap();
    let area = read_channel_number(&analyzed, "area").unwrap();
    let volume = read_channel_number(&analyzed, "volume").unwrap();
    assert!((area - 7.6).abs() < 1e-5);
    assert!((volume - 1.32).abs() < 1e-5);
    let brep = registry.dispatch_cold("brep.mesh.toBrep", defaults("brep.mesh.toBrep").insert("mesh", output.get("meshOut").unwrap().clone())).unwrap();
    let handle = read_geometry(&brep, "geometry").unwrap();
    let transfer = with_kernel_read(|kernel| kernel.tessellate(&handle, 0.1).map_err(|error| map_kernel_error(&error))).unwrap();
    assert!(!transfer.index.is_empty());
    eprintln!("[DEBUG] mesh workbench: area={area}, volume={volume}, B-Rep preview triangles={}", transfer.index.len() / 3);
}
