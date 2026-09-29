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
        eprintln!("mesh fixture {}: area={}", case.get("name").unwrap().as_str().unwrap(), read_channel_number(&report, "area").unwrap());
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
}

#[semio_framework_async_macros::async_test]
async fn mesh_inspection_widgets_match_portable_fixtures() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let fixture = pack::json::parse(include_str!("../../🧫️fixtures/🔎️inspection/🔣️.json")).unwrap();
    let mesh = decode_mesh(&pack::json::to_string(fixture.get("mesh").unwrap())).unwrap();
    let seed = neural_engine::ColdOwner::new(mesh_output(&mesh).unwrap());
    for case in fixture.get("cases").unwrap().as_array().unwrap() {
        let query = case.get("query").unwrap();
        let id = format!("brep.mesh.inspect{}", match query.get("kind").unwrap().as_str().unwrap() { "vertex" => "Vertex", "edge" => "Edge", _ => "Face" });
        let result = registry.dispatch_cold(&id, Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone()).insert("index", Value::Dictionary(number_dictionary(query.get("index").unwrap().as_f64().unwrap())))).unwrap();
        for (key, expected) in case.get("expected").unwrap().as_object().unwrap() {
            if key == "vertices" { assert_eq!(pack::json::parse(&read_text(&result, key).unwrap()).unwrap(), *expected); }
            else if let Some(values) = expected.as_array() {
                let actual = read_xyz(&result, key).unwrap();
                for axis in 0..3 { assert!((actual[axis] - values[axis].as_f64().unwrap()).abs() < 1e-5); }
            } else { assert_eq!(read_channel_number(&result, key).unwrap(), expected.as_f64().unwrap()); }
        }
        let info = registry.operator_info(&id).unwrap();
        for output in &info.outputs { assert!(result.get(&output.name).is_some(), "{id} missing {}", output.name); }
    }
    for query in fixture.get("invalid").unwrap().as_array().unwrap() {
        let Some(kind) = query.get("kind").unwrap().as_str().filter(|kind| ["vertex", "edge", "face"].contains(kind)) else { continue; };
        let id = format!("brep.mesh.inspect{}", match kind { "vertex" => "Vertex", "edge" => "Edge", _ => "Face" });
        assert!(registry.dispatch_cold(&id, Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone()).insert("index", Value::Dictionary(number_dictionary(query.get("index").unwrap().as_f64().unwrap())))).is_err());
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_modeling_widgets_match_kernel_portable_fixtures() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let fixtures = pack::json::parse(include_str!("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for (key, operation) in [("bevels", "bevel"), ("dissolutions", "dissolveVertices"), ("merges", "mergeVertices"), ("mirrors", "mirror"), ("decimations", "decimate")] {
        for fixture in fixtures.get(key).unwrap().as_array().unwrap() {
            let mut mesh = if let Some(mesh) = fixture.get("mesh") { decode_mesh(&pack::json::to_string(mesh)).unwrap() } else { HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap() };
            if let Some(translation) = fixture.get("translation") { mesh.translate(MeshVector([0, 1, 2].map(|axis| translation.as_array().unwrap()[axis].as_f64().unwrap() as f32))).unwrap(); }
            let seed = neural_engine::ColdOwner::new(mesh_output(&mesh).unwrap());
            let mut input = Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone());
            for parameter in ["amount", "segments", "ratio"] { if let Some(value) = fixture.get(parameter) { input = input.insert(parameter, Value::Dictionary(number_dictionary(value.as_f64().unwrap()))); } }
            for parameter in ["mode", "axis"] { if let Some(value) = fixture.get(parameter) { input = input.insert(parameter, Value::Dictionary(text_dictionary(value.as_str().unwrap()))); } }
            if let Some(value) = fixture.get("threshold") { input = input.insert("tolerance", Value::Dictionary(number_dictionary(value.as_f64().unwrap()))); }
            if let Some(value) = fixture.get("selection") { input = input.insert("selection", Value::Dictionary(text_dictionary(pack::json::to_string(value)))); }
            if let Some(pairs) = fixture.get("edges") {
                let ids: Vec<_> = pairs.as_array().unwrap().iter().map(|pair| {
                    let pair = pair.as_array().unwrap();
                    let a = pair[0].as_u64().unwrap() as u32; let b = pair[1].as_u64().unwrap() as u32;
                    (0..mesh.halfedge_count()).find(|&id| mesh.edge_endpoints(EdgeId(id as u32)).is_ok_and(|(x, y)| (x.0 == a && y.0 == b) || (x.0 == b && y.0 == a))).unwrap() as u32
                }).collect();
                input = input.insert("edges", Value::Dictionary(text_dictionary(pack::json::to_string(&pack::json::array(ids.into_iter().map(pack::json::Value::from))))));
            }
            let output = registry.dispatch_cold(&format!("brep.mesh.{operation}"), input).unwrap();
            let mesh = read_mesh(&output, "meshOut").unwrap();
            let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap());
            for parameter in ["vertices", "faces", "area", "volume"] {
                if let Some(expected) = fixture.get(parameter).and_then(|value| value.as_f64()) { let actual = read_channel_number(&report, parameter).unwrap(); assert!((actual - expected).abs() < 1e-5, "{operation} {parameter}: {actual} != {expected}"); }
            }
            if let Some(maximum) = fixture.get("maximumVertices") { assert!(mesh.vertex_count() as f64 <= maximum.as_f64().unwrap()); }
            if let Some(minimum) = fixture.get("minimumVolume") { assert!(read_channel_number(&report, "volume").unwrap() >= minimum.as_f64().unwrap()); }
        }
    }
    for (key, operation) in [("proportional", "moveProportional"), ("snapping", "snapVertices")] {
        let fixture = fixtures.get(key).unwrap();
        let source = decode_mesh(&pack::json::to_string(fixture.get("mesh").unwrap())).unwrap();
        let seed = neural_engine::ColdOwner::new(mesh_output(&source).unwrap());
        let mut input = Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone()).insert("selection", Value::Dictionary(text_dictionary(pack::json::to_string(fixture.get("selection").unwrap()))));
        for (parameter, target) in [("pivot", "center"), ("delta", "offset")] { if let Some(value) = fixture.get(parameter) { input = input.insert(target, Value::Dictionary(point_dictionary([0, 1, 2].map(|axis| value.as_array().unwrap()[axis].as_f64().unwrap())))); } }
        for parameter in ["radius", "grid"] { if let Some(value) = fixture.get(parameter) { input = input.insert(parameter, Value::Dictionary(number_dictionary(value.as_f64().unwrap()))); } }
        let output = registry.dispatch_cold(&format!("brep.mesh.{operation}"), input).unwrap();
        let mesh = read_mesh(&output, "meshOut").unwrap();
        for (id, expected) in fixture.get("expected").unwrap().as_array().unwrap().iter().enumerate() {
            let actual = mesh.vertex_position(VertexId(id as u32)).unwrap().0;
            for axis in 0..3 { assert!((actual[axis] as f64 - expected.as_array().unwrap()[axis].as_f64().unwrap()).abs() < 1e-6); }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_face_merge_widgets_preserve_fixture_surface() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let fixture = pack::json::parse(include_str!("../../🧫️fixtures/🧵️merge-faces/🔣️.json")).unwrap();
    let source = decode_mesh(&pack::json::to_string(fixture.get("mesh").unwrap())).unwrap();
    let seed = neural_engine::ColdOwner::new(mesh_output(&source).unwrap());
    for case in fixture.get("cases").unwrap().as_array().unwrap() {
        let mut input = Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone());
        if let Some(edges) = case.get("edges") { input = input.insert("edges", Value::Dictionary(text_dictionary(pack::json::to_string(edges)))); }
        let id = format!("brep.mesh.{}", case.get("operation").unwrap().as_str().unwrap());
        let output = registry.dispatch_cold(&id, input).unwrap();
        let result = read_mesh(&output, "meshOut").unwrap();
        let report = neural_engine::ColdOwner::new(analyze(&result).unwrap());
        for (key, expected) in case.get("expected").unwrap().as_object().unwrap() { assert!((read_channel_number(&report, key).unwrap() - expected.as_f64().unwrap()).abs() < 1e-6); }
    }
}

#[semio_framework_async_macros::async_test]
async fn reflected_preview_edge_ids_inspect_the_serialized_mesh() {
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let fixtures = pack::json::parse(include_str!("../../🧫️fixtures/🔎️inspection/🔣️.json")).unwrap();
    let source = neural_engine::ColdOwner::new(mesh_output(&HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap()).unwrap());
    for transform in fixtures.get("previewTransforms").unwrap().as_array().unwrap() {
        let factor = transform.get("factor").unwrap().as_array().unwrap();
        let output = registry.dispatch_cold("brep.mesh.scale", Dictionary::new().insert("mesh", source.get("meshOut").unwrap().clone()).insert("factor", Value::Dictionary(vector_dictionary([0, 1, 2].map(|axis| factor[axis].as_f64().unwrap()))))).unwrap();
        let preview_text = output.get("meshOut").unwrap().as_dictionary().unwrap().get("preview").unwrap().as_atom().unwrap().as_str().unwrap();
        let preview = decode_mesh_pack(&decode_base64(preview_text).unwrap()).unwrap();
        for (&id, endpoints) in preview.edge_ids.iter().zip(preview.edge_positions.chunks_exact(6)) {
            let inspected = registry.dispatch_cold("brep.mesh.inspectEdge", Dictionary::new().insert("mesh", output.get("meshOut").unwrap().clone()).insert("index", Value::Dictionary(number_dictionary(id as f64)))).unwrap();
            let start = read_xyz(&inspected, "start").unwrap(); let end = read_xyz(&inspected, "end").unwrap();
            let a = [endpoints[0] as f64, endpoints[1] as f64, endpoints[2] as f64]; let b = [endpoints[3] as f64, endpoints[4] as f64, endpoints[5] as f64];
            assert!((start == a && end == b) || (start == b && end == a), "preview edge {id}: {start:?} {end:?} != {a:?} {b:?}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_modeling_yields_and_cancels_through_graph_evaluation_handler() {
    let _serial = super::super::tests::test_serial().await;
    let registry = neural_engine::ColdOwner::new(super::super::module_registry().await);
    let fixture = pack::json::parse(include_str!("../../🧫️fixtures/⏱️modeling-budget/🔣️.json")).unwrap();
    let mesh = decode_mesh(&pack::json::to_string(fixture.get("mesh").unwrap())).unwrap();
    let source = neural_engine::ColdOwner::new(mesh_output(&mesh).unwrap());
    for (index, case) in fixture.get("cases").unwrap().as_array().unwrap().iter().enumerate() {
        let operator = case.get("operator").unwrap().as_str().unwrap();
        let mut input = Dictionary::new().insert("mesh", source.get("meshOut").unwrap().clone());
        for (key, value) in case.get("parameters").unwrap().as_object().unwrap() {
            let value = if let Some(number) = value.as_f64() { number_dictionary(number) } else { text_dictionary(value.as_str().unwrap()) };
            input = input.insert(key, Value::Dictionary(value));
        }
        let input = neural_engine::ColdOwner::new(input);
        let input_json = pack::json::to_json_string(&*input);
        let node_hash = 0x6D_00_00 + index as i64;
        let request = pack::json::to_string(&pack::json::object([
            ("operatorId".into(), pack::json::Value::from(operator)),
            ("inputJson".into(), pack::json::Value::from(input_json)),
            ("nodeHash".into(), pack::json::Value::from(node_hash)),
            ("budget".into(), pack::json::Value::from(1_i64)),
            ("wallMicros".into(), pack::json::Value::from(1_i64)),
        ]));
        let first = pack::json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(&registry, request.as_bytes()).unwrap()).unwrap();
        assert_eq!(first.get("done").and_then(|value| value.as_bool()), Some(false), "{operator} must yield");
        assert_eq!(first.get("cancellable").and_then(|value| value.as_bool()), Some(true));
        assert_eq!(first.get("outputJson").and_then(|value| value.as_str()), Some(""));
        assert!(flow_extension_sdk::evaluation_progress(operator, node_hash as u64).is_some());
        assert!(flow_extension_sdk::cancel_evaluation(operator, node_hash as u64));
        assert!(!flow_extension_sdk::cancel_evaluation(operator, node_hash as u64));
        assert!(flow_extension_sdk::evaluation_progress(operator, node_hash as u64).is_none());
        let mut trips = 0; let mut previous = 0;
        let output = loop {
            trips += 1; assert!(trips < 4096, "{operator} did not terminate");
            let result = pack::json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(&registry, request.as_bytes()).unwrap()).unwrap();
            let done = result.get("unitsDone").unwrap().as_u64().unwrap(); let total = result.get("unitsTotal").unwrap().as_u64().unwrap();
            assert!(done >= previous && done <= total); previous = done;
            if result.get("done").unwrap().as_bool() == Some(true) { break pack::json::parse(result.get("outputJson").unwrap().as_str().unwrap()).unwrap(); }
            assert_eq!(result.get("cancellable").and_then(|value| value.as_bool()), Some(true));
            assert_eq!(result.get("outputJson").and_then(|value| value.as_str()), Some(""));
        };
        assert!(trips >= case.get("minimumRoundTrips").unwrap().as_u64().unwrap() as usize);
        let expected = neural_engine::ColdOwner::new(registry.dispatch(operator, &input).unwrap());
        let expected = pack::json::to_json_string(&*expected);
        assert_eq!(output, pack::json::parse(&expected).unwrap());
        assert_eq!(encode_mesh(&mesh).unwrap(), source.get("meshOut").unwrap().as_dictionary().unwrap().get("data").unwrap().as_atom().unwrap().as_str().unwrap());
    }
}
