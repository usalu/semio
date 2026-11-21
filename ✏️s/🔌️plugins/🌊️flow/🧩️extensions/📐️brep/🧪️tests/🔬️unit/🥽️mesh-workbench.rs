//! 🥽️ Exercises contributed mesh operations through the original Flow extension request and result boundary.

use flow_extension_sdk::{ExtensionEvaluationResources, evaluate_invoke_json};
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant;
use semio_plugin::ExtensionResourceOwner;
use semio_s_spatial_kernel_semio_session::Session;
use serde_json::{Value, json};

const GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 16,
    maximum_copy_bytes: 65_536,
    maximum_capacity_bytes: 1_048_576,
    maximum_release_bytes: 268_435_456,
    maximum_depth: 1_024,
};

fn number(value: f64) -> Value {
    json!({"$schema": "number", "value": value})
}

fn text(value: &str) -> Value {
    json!({"$schema": "text", "value": value})
}

fn boolean(value: bool) -> Value {
    json!({"$schema": "boolean", "value": value})
}

fn mesh_source(output: &Value) -> Value {
    serde_json::from_str(output["meshOut"]["data"].as_str().unwrap()).unwrap()
}

fn evaluate(owner: &mut ExtensionEvaluationResources, caller: &mut OriginalJobFixtureCaller, operator: &str, input: Value, limit: u64) -> Value {
    let request = serde_json::to_vec(&json!({"neuronId": operator, "operatorId": operator, "inputJson": input.to_string()})).unwrap();
    for _ in 0..limit {
        let reply = caller.turn(GRANT, |cx| evaluate_invoke_json(owner, &request, cx).map_err(Into::into)).unwrap();
        assert_eq!(reply.retained_progress, caller.receipt());
        assert!(reply.refusal.is_none(), "{operator} refused evaluation");
        if let Some(bytes) = reply.payload {
            let output = neural_engine::ColdOwner::new(pack::record::decode_value_record_body_exact(&bytes, 1, &pack::record::DecodeOptions::default().limits).unwrap());
            let logical: Value = serde_json::from_str(&semio_framework_pack_json::from_dsl_value(&output).to_string()).unwrap();
            assert_eq!(logical["done"], true, "{operator}: {logical}");
            return logical["output"].clone();
        }
    }
    panic!("{operator} did not publish its result");
}

#[test]
fn mesh_workbench_finishes_through_the_original_contributed_evaluator() {
    let fixture: Value = serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🥽️retained-workbench/🔣️.json")).unwrap();
    let limit = fixture["maximumTurns"].as_u64().unwrap();
    let stock = RetainedCloneGrant {
        maximum_items: 1_000_000,
        maximum_copy_bytes: 268_435_456,
        maximum_capacity_bytes: 536_870_912,
        maximum_release_bytes: 536_870_912,
        maximum_depth: 1_024,
    };
    let session = Session::new();
    let mut owner = ExtensionEvaluationResources::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(0x6D6000), Generation(7), stock, || Some(1_000), 1_500, 1_000_000);
    let ready = (0..limit).any(|_| caller.turn(GRANT, |cx| owner.admit_registry_on_original(cx)).unwrap() == Some(true));
    assert!(ready, "contributed mesh registry must settle");

    for grid in fixture["parametricPrimitives"].as_array().unwrap() {
        let parameters = grid["inputs"].as_object().unwrap().iter()
            .map(|(key, value)| (key.clone(), number(value.as_f64().unwrap())))
            .collect::<serde_json::Map<String, Value>>();
        let mesh = evaluate(&mut owner, &mut caller, grid["operator"].as_str().unwrap(), Value::Object(parameters), limit);
        let analyzed = evaluate(&mut owner, &mut caller, "brep.mesh.analyze", json!({"mesh": mesh["meshOut"]}), limit);
        for field in ["vertices", "faces", "triangles"] {
            assert_eq!(analyzed[field]["value"].as_f64(), grid[field].as_f64(), "{field}");
        }
        assert_eq!(analyzed["boundaryEdges"]["value"].as_f64(), Some(0.0));
        assert!(analyzed["volume"]["value"].as_f64().unwrap() > 0.0);
    }

    let box_mesh = evaluate(&mut owner, &mut caller, "brep.mesh.box", json!({ "width": number(1.0), "height": number(1.0), "depth": number(1.0) }), limit);
    let binary = &fixture["binaryInterchange"];
    for format in binary["formats"].as_array().unwrap() {
        let suffix = format["suffix"].as_str().unwrap();
        let channel = format["output"].as_str().unwrap();
        let exported = evaluate(&mut owner, &mut caller, &format!("brep.mesh.export{suffix}"), json!({"mesh": box_mesh["meshOut"]}), limit);
        let data = exported[channel]["value"].as_str().unwrap();
        let imported = evaluate(&mut owner, &mut caller, &format!("brep.mesh.import{suffix}"), json!({"data": text(data)}), limit);
        let analyzed = evaluate(&mut owner, &mut caller, "brep.mesh.analyze", json!({"mesh": imported["meshOut"]}), limit);
        for key in ["vertices", "faces", "area", "volume"] {
            assert!((analyzed[key]["value"].as_f64().unwrap() - binary[key].as_f64().unwrap()).abs() < 1e-5, "{suffix} {key}");
        }
    }
    let glb: Value = serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧊️glb-surface/🔣️.json")).expect("GLB surface fixture");
    let imported = evaluate(&mut owner, &mut caller, "brep.mesh.importGlb", json!({"data": text(glb["source"].as_str().unwrap())}), limit);
    let exported = evaluate(&mut owner, &mut caller, "brep.mesh.exportGlb", json!({"mesh": imported["meshOut"]}), limit);
    let returned = evaluate(&mut owner, &mut caller, "brep.mesh.importGlb", json!({"data": exported["glb"]}), limit);
    for output in [imported, returned] {
        let source = mesh_source(&output);
        let positions = source["vertices"].as_array().unwrap();
        assert_eq!(positions.len() as u64, glb["weldedVertices"].as_u64().unwrap());
        let normals = source["attributes"]["normal"]["values"].as_array().unwrap();
        let surface = source["surface"].as_array().unwrap();
        let mut corner = 0;
        for face in source["faces"].as_array().unwrap() {
            for id in face.as_array().unwrap() {
                let coordinates = |value: &Value| value.as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>();
                assert_eq!(coordinates(&positions[id.as_u64().unwrap() as usize]), coordinates(&glb["positions"][corner]));
                assert_eq!(coordinates(&normals[corner]), coordinates(&glb["normals"][corner]));
                let uv = surface.iter().find(|sample| sample["kind"] == "cornerUv" && sample["index"].as_u64() == Some(corner as u64)).unwrap();
                assert_eq!(coordinates(&uv["uv"]), coordinates(&glb["uvs"][corner]));
                corner += 1;
            }
        }
        assert_eq!(corner, glb["positions"].as_array().unwrap().len());
    }
    let sparse_glb = sparse_glb_fixture();
    let uri_glb = uri_glb_fixture();
    let wide_glb = wide_glb_source();
    for (source, source_limit) in [
        (sparse_glb["source"].as_str().unwrap(), limit),
        (uri_glb["source"].as_str().unwrap(), limit),
        (wide_glb.as_str(), limit * 8),
    ] {
        let imported = evaluate(&mut owner, &mut caller, "brep.mesh.importGlb", json!({"data": text(source)}), source_limit);
        let polygon = mesh_source(&imported);
        let numeric_positions = |value: &Value| value.as_array().unwrap().iter()
            .map(|position| [0, 1, 2].map(|axis| position[axis].as_f64().unwrap())).collect::<Vec<_>>();
        assert_eq!(numeric_positions(&polygon["vertices"]), numeric_positions(&sparse_glb["expectedPositions"]));
        assert_eq!(polygon["faces"], json!([[0, 1, 2], [0, 2, 3]]));
        let sparse_corners = polygon["surface"].as_array().unwrap();
        for (corner, index) in sparse_glb["expectedIndices"].as_array().unwrap().iter().enumerate() {
            let vertex = index.as_u64().unwrap() as usize;
            let uv = sparse_corners.iter().find(|sample| sample["kind"] == "cornerUv" && sample["index"].as_u64() == Some(corner as u64));
            for axis in 0..2 {
                let actual = uv.map_or(0.0, |sample| sample["uv"][axis].as_f64().unwrap());
                let expected = sparse_glb["expectedUvs"][vertex][axis].as_f64().unwrap();
                assert!((actual - expected).abs() < 1e-6);
            }
        }
    }
    let inset = evaluate(&mut owner, &mut caller, "brep.mesh.inset", json!({ "mesh": box_mesh["meshOut"], "faces": text("[0]"), "amount": number(fixture["amount"].as_f64().unwrap()) }), limit);
    let extruded = evaluate(&mut owner, &mut caller, "brep.mesh.extrude", json!({ "mesh": inset["meshOut"], "faces": text("[0]"), "distance": number(fixture["distance"].as_f64().unwrap()) }), limit);
    let analyzed = evaluate(&mut owner, &mut caller, "brep.mesh.analyze", json!({ "mesh": extruded["meshOut"] }), limit);
    for key in ["area", "volume"] {
        assert!((analyzed[key]["value"].as_f64().unwrap() - fixture[key].as_f64().unwrap()).abs() < 1e-5, "{key}");
    }
    let faceted = evaluate(&mut owner, &mut caller, "brep.mesh.toBrep", json!({ "mesh": extruded["meshOut"], "tolerance": number(0.001) }), limit);
    assert_eq!(faceted["geometry"]["kind"], "solid");
    assert!(faceted["geometry"]["handle"].as_str().is_some());

    let dissolve = &fixture["vertexDissolve"];
    let grid = evaluate(&mut owner, &mut caller, "brep.mesh.construct", json!({ "data": text(&dissolve["mesh"].to_string()) }), limit);
    let expected = &dissolve["inspection"];
    for (suffix, key) in [("Vertex", "vertex"), ("Edge", "edge"), ("Face", "face")] {
        let operator = format!("brep.mesh.inspect{suffix}");
        let output = evaluate(&mut owner, &mut caller, &operator, json!({ "mesh": grid["meshOut"], "index": number(expected[key].as_f64().unwrap()) }), limit);
        let axes = |field: &str| ["x", "y", "z"].map(|axis| output[field][axis].as_f64().unwrap());
        match key {
            "vertex" => assert_eq!(axes("point"), std::array::from_fn(|axis| expected["position"][axis].as_f64().unwrap())),
            "edge" => {
                assert_eq!(axes("start"), std::array::from_fn(|axis| expected["start"][axis].as_f64().unwrap()));
                assert_eq!(axes("end"), std::array::from_fn(|axis| expected["end"][axis].as_f64().unwrap()));
                assert_eq!(output["length"]["value"].as_f64(), expected["length"].as_f64());
            }
            _ => {
                assert_eq!(axes("normal"), std::array::from_fn(|axis| expected["normal"][axis].as_f64().unwrap()));
                assert_eq!(axes("center"), std::array::from_fn(|axis| expected["center"][axis].as_f64().unwrap()));
                assert_eq!(serde_json::from_str::<Value>(output["vertices"]["value"].as_str().unwrap()).unwrap(), expected["vertices"]);
            }
        }
    }
    let dissolved = evaluate(&mut owner, &mut caller, "brep.mesh.dissolveVertices", json!({ "mesh": grid["meshOut"], "selection": text(&dissolve["selection"].to_string()) }), limit);
    let inspected = evaluate(&mut owner, &mut caller, "brep.mesh.analyze", json!({ "mesh": dissolved["meshOut"] }), limit);
    assert_eq!(inspected["faces"]["value"].as_f64(), dissolve["expectedFaces"].as_f64());
    assert_eq!(inspected["area"]["value"].as_f64(), dissolve["area"].as_f64());

    let obj = &fixture["objImport"];
    let imported = evaluate(&mut owner, &mut caller, "brep.mesh.importObj", json!({ "data": text(obj["text"].as_str().unwrap()) }), limit);
    let analyzed = evaluate(&mut owner, &mut caller, "brep.mesh.analyze", json!({ "mesh": imported["meshOut"] }), limit);
    for key in ["vertices", "faces", "area"] {
        assert_eq!(analyzed[key]["value"].as_f64(), obj[key].as_f64(), "{key}");
    }

    let surface = &fixture["surfacePipeline"];
    let source = &surface["polygon"];
    let construct = evaluate(&mut owner, &mut caller, "brep.mesh.construct", json!({"data": text(&source.to_string())}), limit);
    let parsed = mesh_source(&construct);
    let records = parsed["surface"].as_array().unwrap();
    for sample in source["surface"].as_array().unwrap() {
        let roundtrip = records.iter().find(|record| record["kind"] == sample["kind"] && record["index"] == sample["index"])
            .unwrap_or_else(|| panic!("missing native surface sample {sample}"));
        for key in ["normal", "uv"] {
            let coordinates = |value: &Value| value[key].as_array().map(|coordinates| coordinates.iter().map(Value::as_f64).collect::<Vec<_>>());
            assert_eq!(coordinates(roundtrip), coordinates(sample));
        }
    }
    let flat = evaluate(&mut owner, &mut caller, "brep.mesh.setShading", json!({"mesh":construct["meshOut"],"faces":text("[0]"),"smooth": boolean(false)}), limit);
    let parsed = mesh_source(&flat);
    assert!(parsed["surface"].as_array().unwrap().iter().all(|sample| sample["kind"] != "smoothFace"));
    let smooth = evaluate(&mut owner, &mut caller, "brep.mesh.setShading", json!({"mesh":flat["meshOut"],"faces":text("[0]"),"smooth": boolean(true)}), limit);
    let normals = evaluate(&mut owner, &mut caller, "brep.mesh.recomputeNormals", json!({"mesh":smooth["meshOut"]}), limit);
    let marked = evaluate(&mut owner, &mut caller, "brep.mesh.markSeams", json!({"mesh":normals["meshOut"],"edges":text("[2]"),"seam":boolean(true)}), limit);
    let unwrapped = evaluate(&mut owner, &mut caller, "brep.mesh.unwrapUv", json!({"mesh":marked["meshOut"]}), limit);
    let exported = evaluate(&mut owner, &mut caller, "brep.mesh.exportJson", json!({"mesh":unwrapped["meshOut"]}), limit);
    let parsed: Value = serde_json::from_str(exported["text"]["value"].as_str().unwrap()).unwrap();
    let records = parsed["surface"].as_array().unwrap();
    assert!(records.iter().any(|sample| sample["kind"] == "smoothFace"));
    assert!(records.iter().any(|sample| sample["kind"] == "seamEdge" && sample["index"] == 2));
    let normals = records.iter().filter(|sample| sample["kind"] == "vertexNormal").collect::<Vec<_>>();
    assert_eq!(normals.len(), 4);
    assert!(normals.iter().all(|sample| (0..3).all(|axis| sample["normal"][axis].as_f64() == surface["normal"][axis].as_f64())));
    let coordinates = records.iter().filter(|sample| sample["kind"] == "cornerUv").collect::<Vec<_>>();
    assert!(coordinates.len() >= 3);
    assert!(coordinates.iter().all(|sample| sample["uv"].as_array().unwrap().iter().all(|value| (0.0..=1.0).contains(&value.as_f64().unwrap()))));

    owner.begin_close();
    for _ in 0..limit {
        if owner.terminal_is_empty() {
            break;
        }
        caller.turn(GRANT, |cx| owner.close_step(cx)).unwrap();
    }
    assert!(owner.terminal_is_empty());
    let obj: Value = serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧵️obj-corners.json")).unwrap();
    let mesh = evaluate(&mut owner, &mut caller, "brep.mesh.construct", json!({"data":text(&obj["source"].to_string())}), limit);
    let export = evaluate(&mut owner, &mut caller, "brep.mesh.exportObj", json!({"mesh":mesh["meshOut"]}), limit);
    let output = export["text"]["value"].as_str().unwrap();
    assert_eq!(output.lines().filter(|line| line.starts_with("vn ")).collect::<Vec<_>>(), ["vn 0 0 1","vn 1 0 0"]);
    assert_eq!(output.lines().filter(|line| *line=="vt 0 0").count(),6);
    let cited_normals = output.lines().filter_map(|line| line.strip_prefix("f "))
        .flat_map(str::split_whitespace).map(|token| {
            let fields=token.split("/").collect::<Vec<_>>();
            assert_eq!(fields.len(),3);
            assert!(!fields[1].is_empty());
            fields[2].parse::<usize>().unwrap()
        }).collect::<Vec<_>>();
    assert_eq!(cited_normals,[1,1,1,2,2,2]);

    for input in [output,obj["relativeObj"].as_str().unwrap()] {
        let imported=evaluate(&mut owner,&mut caller,"brep.mesh.importObj",json!({"data":text(input)}),limit);
        let imported=mesh_source(&imported);
        assert_eq!(imported["faces"],obj["source"]["faces"]);
        let normals=imported["attributes"]["normal"]["values"].as_array().unwrap();
        assert_eq!(normals.len(),2);
        for (row,expected) in normals.iter().zip([[0.0,0.0,1.0],[1.0,0.0,0.0]]) {
            assert_eq!([0,1,2].map(|axis|row[axis].as_f64().unwrap()),expected);
        }
        assert_eq!(imported["attributes"]["normal"]["indices"],json!([0,0,0,1,1,1]));
        let uv=&imported["attributes"]["uv"];
        for index in uv["indices"].as_array().unwrap() {
            let sample=&uv["values"][index.as_u64().unwrap() as usize];
            assert_eq!([sample[0].as_f64(),sample[1].as_f64()],[Some(0.0),Some(0.0)]);
        }
    }
    let quad=evaluate(&mut owner,&mut caller,"brep.mesh.importObj",json!({"data":text(obj["quadObj"].as_str().unwrap())}),limit);
    let source=mesh_source(&quad);
    assert_eq!(source["faces"],obj["quad"]["faces"]);
    assert_eq!(source["attributes"]["uv"]["indices"],json!([0,1,2,3]));
    assert!(source["attributes"].get("normal").is_none());
    let measured=evaluate(&mut owner,&mut caller,"brep.mesh.analyze",json!({"mesh":quad["meshOut"]}),limit);
    assert_eq!(measured["triangles"]["value"].as_f64(),obj["quad"]["triangles"].as_f64());


    let winding: Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧭️orient-shells.json")).unwrap();
    let shell=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&winding["closed"].to_string())}),limit);
    let oriented=evaluate(&mut owner,&mut caller,"brep.mesh.orient",json!({"mesh":shell["meshOut"]}),limit);
    let polygon=mesh_source(&oriented);
    assert_eq!(polygon["faces"],winding["outward"]["faces"]);
    assert_eq!(polygon["attributes"]["submittedNormal"]["values"].as_array().unwrap().len(),8);
    let analysis=evaluate(&mut owner,&mut caller,"brep.mesh.analyze",json!({"mesh":oriented["meshOut"]}),limit);
    assert!((analysis["volume"]["value"].as_f64().unwrap()-8.0/3.0).abs()<1e-6);

    let raw_surface:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🎨️triangulate-surface.json")).unwrap();
    let polygon=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&raw_surface["source"].to_string())}),limit);
    let triangulated=evaluate(&mut owner,&mut caller,"brep.mesh.triangulate",json!({"mesh":polygon["meshOut"]}),limit);
    let triangle_mesh=mesh_source(&triangulated);
    assert_eq!(triangle_mesh["faces"].as_array().unwrap().len(),3);
    assert_eq!(triangle_mesh["attributes"]["design"]["indices"],json!([0,0,1]));
    let surface=triangle_mesh["surface"].as_array().unwrap();
    assert_eq!(surface.iter().filter(|row|row["kind"]=="smoothFace").map(|row|row["index"].as_u64().unwrap()).collect::<Vec<_>>(),[0,1]);
    let positions=triangle_mesh["vertices"].as_array().unwrap();
    for normal in surface.iter().filter(|row|row["kind"]=="vertexNormal") {
        let position=&positions[normal["index"].as_u64().unwrap() as usize];
        let index=raw_surface["source"]["vertices"].as_array().unwrap().iter().position(|source|[0,1,2].iter().all(|axis|source[*axis].as_f64()==position[*axis].as_f64())).unwrap();
        let expected=&raw_surface["expected"]["vertexNormals"][index];
        for axis in 0..3 {assert!((normal["normal"][axis].as_f64().unwrap()-expected[axis].as_f64().unwrap()).abs()<1e-6);}
    }
    assert_eq!(surface.iter().filter(|row|row["kind"]=="vertexNormal").count(),5);
    assert_eq!(surface.iter().filter(|row|row["kind"]=="cornerUv").count(),9);
    let glb=evaluate(&mut owner,&mut caller,"brep.mesh.exportGlb",json!({"mesh":triangulated["meshOut"]}),limit);
    assert!(glb["glb"]["value"].as_str().is_some_and(|value|!value.is_empty()));

    let refined=evaluate(&mut owner,&mut caller,"brep.mesh.subdivide",json!({"mesh":polygon["meshOut"],"faces":text("[0]")}),limit);
    let refined=mesh_source(&refined);
    assert_eq!(refined["faces"].as_array().unwrap().len(),5);
    let center=refined["vertices"].as_array().unwrap().iter().position(|vertex|[0,1,2].map(|axis|vertex[axis].as_f64().unwrap())==[1.0,1.0,0.0]).unwrap();
    let records=refined["surface"].as_array().unwrap();
    let normal=records.iter().find(|row|row["kind"]=="vertexNormal" && row["index"].as_u64()==Some(center as u64)).unwrap();
    assert_eq!([0,1,2].map(|axis|normal["normal"][axis].as_f64().unwrap()),[0.0,0.0,1.0]);
    assert_eq!(records.iter().filter(|row|row["kind"]=="smoothFace").count(),4);
    let center_corners=refined["faces"].as_array().unwrap().iter().flat_map(|face|face.as_array().unwrap()).enumerate().filter_map(|(index,vertex)|(vertex.as_u64()==Some(center as u64)).then_some(index)).collect::<Vec<_>>();
    assert_eq!(center_corners.len(),4);
    for corner in center_corners {
        let uv=records.iter().find(|row|row["kind"]=="cornerUv" && row["index"].as_u64()==Some(corner as u64)).unwrap();
        assert_eq!([uv["uv"][0].as_f64(),uv["uv"][1].as_f64()],[Some(0.5),Some(0.5)]);
    }

    let weld:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧲️weld-surface.json")).unwrap();
    let original=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&weld["source"].to_string())}),limit);
    let welded=evaluate(&mut owner,&mut caller,"brep.mesh.weld",json!({"mesh":original["meshOut"],"tolerance":number(weld["tolerance"].as_f64().unwrap())}),limit);
    let records=mesh_source(&welded);
    assert_eq!(records["vertices"].as_array().unwrap().len(),4);
    assert_eq!(records["faces"].as_array().unwrap().len(),2);
    assert_eq!(records["attributes"]["hardNormal"]["indices"],json!([0,0,0,1,1,1]));
    let surface=records["surface"].as_array().unwrap();
    assert_eq!(surface.iter().filter(|row|row["kind"]=="smoothFace").count(),1);
    assert_eq!(surface.iter().filter(|row|row["kind"]=="seamEdge").count(),2);
    let uvs=surface.iter().filter(|row|row["kind"]=="cornerUv").collect::<Vec<_>>();
    assert_eq!(uvs.len(),6);
    for row in uvs {
        let index=row["index"].as_u64().unwrap() as usize;
        for axis in 0..2 {assert_eq!(row["uv"][axis].as_f64(),weld["expected"]["cornerUvs"][index][axis].as_f64());}
    }
    let glb=evaluate(&mut owner,&mut caller,"brep.mesh.exportGlb",json!({"mesh":welded["meshOut"]}),limit);
    assert!(glb["glb"]["value"].as_str().is_some_and(|value|!value.is_empty()));

    let collapse:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🪶️decimate-surface.json")).unwrap();
    let input=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&collapse["source"].to_string())}),limit);
    let reduced=evaluate(&mut owner,&mut caller,"brep.mesh.decimate",json!({"mesh":input["meshOut"],"ratio":number(collapse["ratio"].as_f64().unwrap())}),limit);
    let reduced=mesh_source(&reduced);
    assert_eq!(reduced["faces"].as_array().unwrap().len(),4);
    let midpoint=reduced["vertices"].as_array().unwrap().iter().position(|row|row[0].as_f64()==Some(0.125)).unwrap();
    let data=reduced["surface"].as_array().unwrap();
    assert_eq!(data.iter().filter(|row|row["kind"]=="smoothFace").count(),3);
    assert_eq!(data.iter().filter(|row|row["kind"]=="seamEdge").count(),2);
    let moved:Vec<_>=reduced["faces"].as_array().unwrap().iter().flat_map(|row|row.as_array().unwrap()).enumerate().filter_map(|(corner,vertex)|(vertex.as_u64()==Some(midpoint as u64)).then_some(corner)).collect();
    assert_eq!(moved.len(),3);
    for (expected,corner) in moved.into_iter().enumerate() {
        let uv=data.iter().find(|row|row["kind"]=="cornerUv"&&row["index"].as_u64()==Some(corner as u64)).unwrap();
        for axis in 0..2 {assert_eq!(uv["uv"][axis].as_f64(),collapse["expected"]["midpointUv"][expected][axis].as_f64());}
    }

    let authored_transform:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🔄️transform-normals.json")).unwrap();
    let imported=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&authored_transform["source"].to_string())}),limit);
    let matrix=authored_transform["affine"]["matrix"].as_array().unwrap().iter().enumerate()
        .map(|(index,entry)|(index.to_string(),number(entry.as_f64().unwrap()))).collect::<serde_json::Map<_,_>>();
    let mut matrix=Value::Object(matrix);
    matrix["$schema"]=json!("list");
    let output=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.transform",json!({"mesh":imported["meshOut"],"matrix":matrix}),limit));
    let normal=output["surface"].as_array().unwrap().iter().find(|row|row["kind"]=="vertexNormal"&&row["index"]==0).unwrap();
    let expected=[-0.3_f64,0.05,1.5];
    let length=expected.into_iter().fold(0.0,f64::hypot);
    for axis in 0..3 {assert!((normal["normal"][axis].as_f64().unwrap()-expected[axis]/length).abs()<1e-6);}

    let about_origin=json!({"mesh":imported["meshOut"],"mode":text("vertex"),"selection":text("[0]"),"pivot":text("point"),
        "center":{"$schema":"point","x":0,"y":0,"z":0},"axis":{"$schema":"vector","x":0,"y":0,"z":1},"angle":number(std::f64::consts::FRAC_PI_2)});
    let rotated=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.rotateComponents",about_origin,limit));
    let normals=rotated["surface"].as_array().unwrap();
    for (index,expected) in [[0.0,0.6,0.8],[0.1,0.9,0.4]].into_iter().enumerate() {
        let sample=normals.iter().find(|row|row["kind"]=="vertexNormal"&&row["index"].as_u64()==Some(index as u64)).unwrap();
        for axis in 0..3 {assert!((sample["normal"][axis].as_f64().unwrap()-expected[axis]).abs()<1e-6);}
    }

    let bevel:Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/🧫️fixtures/🛠️modeling/🪚️bevel.json")).unwrap();
    let concave=&bevel["concave"];
    let prism=json!({"vertices":concave["positions"],"faces":concave["faces"]});
    let prepared=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&prism.to_string())}),limit);
    let beveled=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.bevel",
        json!({"mesh":prepared["meshOut"],"edges":text(&concave["selection"].to_string()),"amount":number(concave["amount"].as_f64().unwrap()),"segments":number(1.0)}),limit));
    assert_eq!(beveled["faces"].as_array().unwrap().len() as u64,concave["expected"]["faces"].as_u64().unwrap());
    assert_eq!(beveled["vertices"].as_array().unwrap().len() as u64,concave["expected"]["vertices"].as_u64().unwrap());

    let boolean:Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/🧫️fixtures/🛠️modeling/⚗️authored-intersection.json")).unwrap();
    let box_source=|bounds:&Value|{
        let [a,b]:[[f32;3];2]=serde_json::from_value(bounds.clone()).unwrap();
        let positions=[[a[0],a[1],a[2]],[b[0],a[1],a[2]],[b[0],b[1],a[2]],[a[0],b[1],a[2]],
            [a[0],a[1],b[2]],[b[0],a[1],b[2]],[b[0],b[1],b[2]],[a[0],b[1],b[2]]];
        json!({"vertices":positions,"faces":boolean["faces"]})
    };
    let left=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&boolean["left"]["bounds"]).to_string())}),limit);
    let right=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&boolean["right"]["bounds"]).to_string())}),limit);
    let cut=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.intersect",
        json!({"mesh":left["meshOut"],"other":right["meshOut"]}),limit));
    for part in ["faces","vertices"]{assert_eq!(cut[part].as_array().unwrap().len() as u64,boolean["expected"][part].as_u64().unwrap());}

    let difference:Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/🧫️fixtures/🛠️modeling/➖️convex-difference.json")).unwrap();
    let left=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&difference["left"]).to_string())}),limit);
    let right=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&difference["cases"][0]["right"]).to_string())}),limit);
    let slice=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.difference",
        json!({"mesh":left["meshOut"],"other":right["meshOut"]}),limit));
    assert_eq!(slice["faces"].as_array().unwrap().len(),6);
    let points=slice["vertices"].as_array().unwrap();
    assert_eq!(points.len(),8);
    assert_eq!(points.iter().filter(|point|point[0]==1.0).count(),4);

    let cutter=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&difference["cases"][6]["right"]).to_string())}),limit);
    let corner=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.difference",
        json!({"mesh":left["meshOut"],"other":cutter["meshOut"]}),limit));
    assert_eq!(corner["faces"].as_array().unwrap().len(),11);
    assert_eq!(corner["vertices"].as_array().unwrap().len(),14);

    let cutter=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&difference["cases"][7]["right"]).to_string())}),limit);
    let slab=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.difference",
        json!({"mesh":left["meshOut"],"other":cutter["meshOut"]}),limit));
    assert_eq!(slab["faces"].as_array().unwrap().len(),12);
    assert_eq!(slab["vertices"].as_array().unwrap().len(),16);

    let cutter=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&difference["cases"][12]["right"]).to_string())}),limit);
    let octant=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.difference",
        json!({"mesh":left["meshOut"],"other":cutter["meshOut"]}),limit));
    assert_eq!(octant["faces"].as_array().unwrap().len(),15);
    assert_eq!(octant["vertices"].as_array().unwrap().len(),19);

    let cutter=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&difference["cases"][16]["right"]).to_string())}),limit);
    let tunnel=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.difference",
        json!({"mesh":left["meshOut"],"other":cutter["meshOut"]}),limit));
    assert_eq!(tunnel["faces"].as_array().unwrap().len(),20);
    assert_eq!(tunnel["vertices"].as_array().unwrap().len(),24);

    let cutter=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&difference["cases"][20]["right"]).to_string())}),limit);
    let pocket=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.difference",
        json!({"mesh":left["meshOut"],"other":cutter["meshOut"]}),limit));
    assert_eq!(pocket["faces"].as_array().unwrap().len(),22);
    assert_eq!(pocket["vertices"].as_array().unwrap().len(),28);

    let cutter=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&difference["cases"][23]["right"]).to_string())}),limit);
    let cavity=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.difference",
        json!({"mesh":left["meshOut"],"other":cutter["meshOut"]}),limit));
    assert_eq!(cavity["faces"].as_array().unwrap().len(),12);
    assert_eq!(cavity["vertices"].as_array().unwrap().len(),16);

    let union:Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/🧫️fixtures/🛠️modeling/➕️convex-union.json")).unwrap();
    let left=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&union["left"]).to_string())}),limit);
    let right=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&union["cases"][0]["right"]).to_string())}),limit);
    let joined=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.union",
        json!({"mesh":left["meshOut"],"other":right["meshOut"]}),limit));
    assert_eq!(joined["faces"].as_array().unwrap().len(),10);
    assert_eq!(joined["vertices"].as_array().unwrap().len(),12);

    let right=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&box_source(&union["cases"][7]["right"]).to_string())}),limit);
    let shifted=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.union",
        json!({"mesh":left["meshOut"],"other":right["meshOut"]}),limit));
    assert_eq!(shifted["faces"].as_array().unwrap().len(),20);
    assert_eq!(shifted["vertices"].as_array().unwrap().len(),24);

    let boundary:Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/🧫️fixtures/🛠️modeling/📏️boundary-extrude.json")).unwrap();
    let patch=json!({"vertices":boundary["positions"],"faces":boundary["faces"]});
    let prepared=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&patch.to_string())}),limit);
    let request=&boundary["cases"][0]["request"];
    let v=&request["offset"];
    let output=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.extrudeBoundary",
        json!({"mesh":prepared["meshOut"],"edges":text(&request["edges"].to_string()),"offset":{"$schema":"vector","x":v[0],"y":v[1],"z":v[2]}}),limit));
    assert_eq!(output["faces"].as_array().unwrap()[1..],boundary["cases"][0]["walls"].as_array().unwrap()[..]);

    let bridge:Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/🧫️fixtures/🛠️modeling/🌉️bridge.json")).unwrap();
    let rim_surface=bridge["rimUvs"].as_array().unwrap().iter().enumerate()
        .map(|(index,uv)|json!({"kind":"cornerUv","index":index,"uv":uv})).collect::<Vec<_>>();
    let rims=json!({"vertices":bridge["positions"],"faces":bridge["rims"],"surface":rim_surface});
    let prepared=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&rims.to_string())}),limit);
    let filled=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.bridge",
        json!({"mesh":prepared["meshOut"],"edges":text(&bridge["selection"].to_string())}),limit));
    assert_eq!(filled["faces"].as_array().unwrap()[2..],bridge["expected"]["walls"].as_array().unwrap()[..]);
    let samples=filled["surface"].as_array().unwrap();
    for (index,uv) in bridge["rimUvs"].as_array().unwrap().iter().enumerate(){
        assert!(samples.iter().any(|sample|sample["kind"]=="cornerUv"&&sample["index"].as_u64()==Some(index as u64)&&sample["uv"]==*uv));
    }

    let unequal=&bridge["unequal"];
    let rims=json!({"vertices":unequal["positions"],"faces":unequal["rims"]});
    let prepared=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&rims.to_string())}),limit);
    let bridged=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.bridge",
        json!({"mesh":prepared["meshOut"],"edges":text(&unequal["selection"].to_string())}),limit));
    assert_eq!(bridged["faces"].as_array().unwrap()[2..],unequal["expected"]["walls"].as_array().unwrap()[..]);

    let closing:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧢️fill-hole-surface.json")).unwrap();
    let open=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&closing["source"].to_string())}),limit);
    let sealed=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.fillHoles",json!({"mesh":open["meshOut"]}),limit));
    assert_eq!(sealed["faces"][3],closing["expected"]["cap"]);
    let samples=sealed["surface"].as_array().unwrap();
    let uv=|index|samples.iter().find(|row|row["kind"]=="cornerUv"&&row["index"].as_u64()==Some(index)).map_or([0.0;2],|row|[row["uv"][0].as_f64().unwrap(),row["uv"][1].as_f64().unwrap()]);
    assert_eq!([uv(9),uv(10),uv(11)],[[1.0,0.0],[0.0,1.0],[0.0,0.0]]);
    for index in 0..9 {assert_eq!(uv(index),[closing["source"]["surface"][index as usize+1]["uv"][0].as_f64().unwrap(),closing["source"]["surface"][index as usize+1]["uv"][1].as_f64().unwrap()]);}
    let seams=samples.iter().filter(|row|row["kind"]=="seamEdge").map(|row|row["index"].clone()).collect::<Vec<_>>();
    assert_eq!(seams,*closing["expected"]["seams"].as_array().unwrap());

    let orientation:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧭️orient-surface.json")).unwrap();
    let inconsistent=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&orientation["source"].to_string())}),limit);
    let oriented=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.orient",json!({"mesh":inconsistent["meshOut"]}),limit));
    assert_eq!(oriented["faces"],orientation["expected"]["faces"]);
    let surface=oriented["surface"].as_array().unwrap();
    let samples=|kind:&str,field:&str|surface.iter().filter(|row|row["kind"]==kind).map(|row|row[field].clone()).collect::<Vec<_>>();
    assert_eq!(samples("cornerUv","uv"),*orientation["expected"]["uv"].as_array().unwrap());
    assert_eq!(samples("seamEdge","index"),*orientation["expected"]["seams"].as_array().unwrap());
    assert_eq!(samples("smoothFace","index"),*orientation["expected"]["smooth"].as_array().unwrap());
    let direction=&oriented["attributes"]["faceNormal"];
    let sample=&direction["values"][direction["indices"][1].as_u64().unwrap() as usize];
    assert!((sample[0].as_f64().unwrap()+0.6).abs()<1e-6);
    assert_eq!(oriented["attributes"]["pointNormal"]["domain"],"corner");

    let flip:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🔃️face-flip.json")).unwrap();
    let original=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&flip["source"].to_string())}),limit);
    let flipped=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.flip",json!({"mesh":original["meshOut"],"faces":text("[0]")}),limit));
    let winding=|face:&Value|{
        let point=|index:usize,axis:usize|flipped["vertices"][face[index].as_u64().unwrap() as usize][axis].as_f64().unwrap();
        (point(1,0)-point(0,0))*(point(2,1)-point(0,1))-(point(1,1)-point(0,1))*(point(2,0)-point(0,0))
    };
    assert!(winding(&flipped["faces"][0])<0.0&&winding(&flipped["faces"][1])>0.0);
    let projection=flipped["surface"].as_array().unwrap();
    let raw=projection.iter().find(|row|row["kind"]=="vertexNormal"&&row["index"]==0).unwrap();
    assert!((raw["normal"][0].as_f64().unwrap()+0.6).abs()<1e-6);
    assert!(projection.iter().any(|row|row["kind"]=="smoothFace"&&row["index"]==0));

    let coplanar:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧷️coplanar-surface.json")).unwrap();
    let coplanar_input=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&coplanar["source"].to_string())}),limit);
    let panels=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.mergeCoplanar",json!({"mesh":coplanar_input["meshOut"]}),limit));
    assert_eq!(panels["faces"].as_array().unwrap().iter().map(|face|face.as_array().unwrap().len()).collect::<Vec<_>>(),[6,4]);
    let uv=panels["surface"].as_array().unwrap().iter().filter(|row|row["kind"]=="cornerUv").map(|row|row["uv"].clone()).collect::<Vec<_>>();
    assert_eq!(uv.len(),10);
    assert!(uv.contains(&coplanar["expected"]["uvSamples"][0])&&uv.contains(&coplanar["expected"]["uvSamples"][1]));

    let deletion:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🗑️delete-surface.json")).unwrap();
    let deletion_input=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&deletion["source"].to_string())}),limit);
    let survivors=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.deleteFaces",json!({"mesh":deletion_input["meshOut"],"faces":text("[0]")}),limit));
    assert_eq!(survivors["faces"],deletion["expected"]["faces"]);
    let projected=survivors["surface"].as_array().unwrap();
    let rows=|kind:&str,field:&str|projected.iter().filter(|row|row["kind"]==kind).map(|row|row[field].clone()).collect::<Vec<_>>();
    assert_eq!(rows("cornerUv","uv"),*deletion["expected"]["uv"].as_array().unwrap());
    assert_eq!(rows("smoothFace","index"),*deletion["expected"]["smooth"].as_array().unwrap());
    assert_eq!(rows("seamEdge","index"),*deletion["expected"]["seams"].as_array().unwrap());

    let mixed:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🪄️mixed-face-directions.json")).unwrap();
    let faces=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&mixed["source"].to_string())}),limit);
    let rotated=mesh_source(&evaluate(&mut owner,&mut caller,"brep.mesh.rotateComponents",json!({
        "mesh":faces["meshOut"],"mode":text("vertex"),"selection":text("[1,3,4,5]"),"pivot":text("point"),
        "center":{"$schema":"point","x":2,"y":1,"z":0},"axis":{"$schema":"vector","x":0,"y":0,"z":1},
        "angle":number(std::f64::consts::FRAC_PI_2)
    }),limit));
    for name in ["faceDirection","tangent"] {
        let channel=&rotated["attributes"][name];
        let indices=channel["indices"].as_array().unwrap();
        assert_ne!(indices[0],indices[1]);
        assert_eq!(channel["values"][indices[0].as_u64().unwrap() as usize],mixed["source"]["attributes"][name]["values"][0]);
    }

    let cuts:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🪡️cut-surface.json")).unwrap();
    let authored=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&cuts["source"].to_string())}),limit);
    let point=|row:&Value|json!({"$schema":"point","x":row[0],"y":row[1],"z":row[2]});
    for case in cuts["operations"].as_array().unwrap() {
        let (operator,input)=if case["id"]=="knifeCut" {
            ("brep.mesh.knifeCut",json!({"mesh":authored["meshOut"],"face":number(0.0),"start":point(&case["start"]),"end":point(&case["end"])}))
        }else {
            ("brep.mesh.loopCut",json!({"mesh":authored["meshOut"],"edges":text(&case["edges"].to_string()),"cuts":number(1.0)}))
        };
        let output=mesh_source(&evaluate(&mut owner,&mut caller,operator,input,limit));
        let midpoint=output["vertices"].as_array().unwrap().iter().position(|p|p[0].as_f64()==Some(2.0)&&p[1].as_f64()==Some(1.0)).unwrap() as u64;
        let rows=output["surface"].as_array().unwrap();
        let mut uv=Vec::new();
        for (corner,index) in output["faces"].as_array().unwrap().iter().flat_map(|face|face.as_array().unwrap()).enumerate() {
            if index.as_u64()==Some(midpoint) {
                let sample=rows.iter().find(|row|row["kind"]=="cornerUv"&&row["index"].as_u64()==Some(corner as u64)).unwrap();
                uv.push(sample["uv"].clone());
            }
        }
        assert!(uv.contains(&cuts["expected"]["leftUv"])&&uv.contains(&cuts["expected"]["rightUv"]));
        assert_eq!(rows.iter().filter(|row|row["kind"]=="seamEdge").count(),4);
    }

    let expansion:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🎈️expanded-surface.json")).unwrap();
    let patch=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&expansion["source"].to_string())}),limit);
    for case in expansion["operations"].as_array().unwrap() {
        let (operator,input)=if case["id"]=="extrude" {
            ("brep.mesh.extrude",json!({"mesh":patch["meshOut"],"faces":text("[0]"),"distance":number(case["distance"].as_f64().unwrap())}))
        }else {
            ("brep.mesh.inset",json!({"mesh":patch["meshOut"],"faces":text("[0]"),"amount":number(case["amount"].as_f64().unwrap())}))
        };
        let output=mesh_source(&evaluate(&mut owner,&mut caller,operator,input,limit));
        let surface=output["surface"].as_array().unwrap();
        let uv_pair=|row:&Value|[row[0].as_f64().unwrap(),row[1].as_f64().unwrap()];
        let uv=|index|surface.iter().find(|row|row["kind"]=="cornerUv"&&row["index"].as_u64()==Some(index)).map_or([0.0;2],|row|uv_pair(&row["uv"]));
        let cap=(0..4).map(uv).collect::<Vec<_>>();
        let cap_uv=expansion["capUv"].as_array().unwrap().iter().map(uv_pair).collect::<Vec<_>>();
        let quad_uv=expansion["quadUv"].as_array().unwrap().iter().map(uv_pair).collect::<Vec<_>>();
        assert_eq!(cap,cap_uv);
        for side in 0..4 {assert_eq!((0..4).map(|corner|uv(4+side*4+corner)).collect::<Vec<_>>(),quad_uv);}
        let smooth:Vec<_>=surface.iter().filter(|row|row["kind"]=="smoothFace").map(|row|row["index"].clone()).collect();
        assert_eq!(smooth,*case["smooth"].as_array().unwrap());
        assert!(surface.iter().any(|row|row["kind"]=="seamEdge"&&row["index"]==0));
    }

    let mirror:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🪞️mirror-surface.json")).unwrap();
    let half=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&mirror["source"].to_string())}),limit);
    let reflected=evaluate(&mut owner,&mut caller,"brep.mesh.mirror",json!({"mesh":half["meshOut"],"axis":text("x"),"tolerance":number(0.0001)}),limit);
    let reflected=mesh_source(&reflected);
    let samples=|kind:&str,key:&str|reflected["surface"].as_array().unwrap().iter().filter(|row|row["kind"]==kind).map(|row|row[key].clone()).collect::<Vec<_>>();
    assert_eq!(reflected["faces"],mirror["expected"]["faces"]);
    assert_eq!(samples("cornerUv","uv"),*mirror["expected"]["uv"].as_array().unwrap());
    assert_eq!(samples("smoothFace","index"),*mirror["expected"]["smooth"].as_array().unwrap());
    assert_eq!(samples("seamEdge","index"),*mirror["expected"]["seams"].as_array().unwrap());
    let normals=samples("vertexNormal","normal");
    assert_eq!(normals.len(),4);
    for (sample,expected) in normals.iter().zip(mirror["expected"]["normals"].as_array().unwrap()) {
        for axis in 0..3 {assert!((sample[axis].as_f64().unwrap()-expected[axis].as_f64().unwrap()).abs()<1e-6);}
    }

    let combine:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🔗️combine-assets.json")).unwrap();
    let left=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&combine["mesh"].to_string())}),limit*2);
    let right=evaluate(&mut owner,&mut caller,"brep.mesh.construct",json!({"data":text(&combine["other"].to_string())}),limit*2);
    let joined=evaluate(&mut owner,&mut caller,"brep.mesh.combine",json!({"mesh":left["meshOut"],"other":right["meshOut"]}),limit*2);
    let joined=mesh_source(&joined);
    assert_eq!(joined["faces"],combine["expected"]["faces"]);
    assert_eq!(joined["attributes"]["surfaceMaterial"]["values"],combine["expected"]["facesMaterials"]);
    for name in ["textures","materials"] {
        let output=joined[name].as_object().unwrap();
        assert_eq!(output.len(),3);
        for input in ["mesh","other"] {
            for (key,value) in combine[input][name].as_object().unwrap() {
                assert_eq!(&output[key],value);
            }
        }
    }
    assert_eq!(joined["vertices"].as_array().unwrap().len(),6);
    let ids=&joined["attributes"]["fixtureId"];
    let samples=ids["indices"].as_array().unwrap().iter().map(|index|ids["values"][index.as_u64().unwrap() as usize].as_u64().unwrap()).collect::<Vec<_>>();
    assert_eq!(samples,[10,20,30,50,40,50]);
    let surface=joined["surface"].as_array().unwrap();
    assert_eq!(surface.iter().filter(|row|row["kind"]=="smoothFace").map(|row|row["index"].as_u64().unwrap()).collect::<Vec<_>>(),[0]);
    assert_eq!(surface.iter().filter(|row|row["kind"]=="seamEdge").map(|row|row["index"].as_u64().unwrap()).collect::<Vec<_>>(),[0]);
    assert_eq!(surface.iter().filter(|row|row["kind"]=="vertexNormal").count(),6);

    session.close();
    let stock = caller.stock();
    caller.retire(stock);
}

fn sparse_glb_fixture() -> Value {
    serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧊️glb-surface/sparse.json")).unwrap()
}

fn uri_glb_fixture() -> Value {
    serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧊️glb-surface/uri.json")).unwrap()
}

fn wide_glb_source() -> String {
    use flow_extension_sdk::mesh::{decode_base64, encode_base64};
    let fixture: Value = serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧊️glb-surface/wide.json")).unwrap();
    let input = decode_base64(sparse_glb_fixture()["source"].as_str().unwrap()).unwrap();
    let original_end = 20 + u32::from_le_bytes(input[12..16].try_into().unwrap()) as usize;
    let mut document: Value = serde_json::from_slice(&input[20..original_end]).unwrap();
    let prefix = fixture["prefix"].as_str().unwrap();
    let widen = |object: &mut Value| {
        let object = object.as_object_mut().unwrap();
        let existing = std::mem::take(object);
        for index in 0..fixture["members"].as_u64().unwrap() {
            object.insert(format!("{prefix}{index}"), json!(index));
        }
        object.extend(existing);
    };
    let node = fixture["node"].as_u64().unwrap() as usize;
    let accessor = fixture["accessor"].as_u64().unwrap() as usize;
    widen(&mut document["nodes"][node]);
    widen(&mut document["accessors"][accessor]);
    if fixture["root"] == true { widen(&mut document); }
    let mut json = serde_json::to_vec(&document).unwrap();
    json.resize((json.len() + 3) & !3, b' ');
    let remaining = &input[original_end..];
    let mut glb = input[..8].to_vec();
    glb.extend(((20 + json.len() + remaining.len()) as u32).to_le_bytes());
    glb.extend((json.len() as u32).to_le_bytes());
    glb.extend(b"JSON");
    glb.extend(json);
    glb.extend(remaining);
    encode_base64(glb.as_slice())
}

#[test]
fn mesh_glb_posts_original_work_and_closes_intake_and_scene_when_cancelled() {
    for (phase, minimum_rows) in [("mesh-glb-json", 17), ("mesh-glb-accessor-vertices", 2), ("mesh-glb-sparse-indices", 4), ("mesh-glb-uri-quartets", 5), ("mesh-glb-metadata", 3)] {
        cancel_glb_after_funded_rows(phase, minimum_rows);
    }
}

fn cancel_glb_after_funded_rows(phase: &str, minimum_rows: usize) {
    use neural_engine::{Atom, Dictionary, OperatorJobStep, OperatorPlanAdmission, OperatorPlanCursor, OperatorPlanStep, Value as FlowValue};
    let source = match phase {
        "mesh-glb-sparse-indices" => sparse_glb_fixture()["source"].as_str().unwrap().to_owned(),
        "mesh-glb-uri-quartets" => uri_glb_fixture()["source"].as_str().unwrap().to_owned(),
        "mesh-glb-metadata" => wide_glb_source(),
        _ => serde_json::from_str::<Value>(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧊️glb-surface/🔣️.json")).unwrap()["source"].as_str().unwrap().to_owned(),
    };
    let grant = RetainedCloneGrant { maximum_items: 1, ..GRANT };
    let stock = RetainedCloneGrant {
        maximum_items: 200_000, maximum_copy_bytes: 64 << 20, maximum_capacity_bytes: 64 << 20,
        maximum_release_bytes: 64 << 20, maximum_depth: 1_024,
    };
    let session = Session::new();
    let registry = semio_s_plugin_flow_brep::module_registry(&session);
    let text = Dictionary::with_schema("text").insert("value", FlowValue::Atom(Atom::String(source)));
    let mut input = Some(Dictionary::new().insert("data", FlowValue::Dictionary(text)));
    let mut plan = OperatorPlanCursor::new();
    let mut job = (0..10_000).find_map(|_| match registry.dispatch_job("brep.mesh.importGlb", &mut input, &mut plan, GRANT).unwrap() {
        OperatorPlanStep::Working => None,
        OperatorPlanStep::Admitted(OperatorPlanAdmission::Job(job)) => Some(job),
        _ => panic!("GLB must admit a retained job"),
    }).unwrap();
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(0x6D6001), Generation(7), stock, || Some(1_000), 1_500, 100_000);
    let mut rows = 0;
    let mut capacity = 0;
    let mut released = 0;
    let turn_limit = if phase == "mesh-glb-metadata" { 100_000 } else { 10_000 };
    for _ in 0..turn_limit {
        caller.turn(grant, |cx| cx.with_original_work_loan(64, 1_500, |cx| {
            let before = cx.original_turn_fuel_remaining();
            let (step, receipt) = job.step(cx).unwrap();
            let OperatorJobStep::Working(progress) = step else { panic!("fixture must stay in intake until cancellation"); };
            capacity += receipt.retained_capacity_bytes;
            released += receipt.released_bytes;
            if progress.phase == phase && receipt.copied_items != 0
                && (phase != "mesh-glb-accessor-vertices" || receipt.copied_bytes == 32)
                && (phase != "mesh-glb-metadata" || receipt.copied_bytes != 0)
            {
                assert_eq!(receipt.copied_items, 1);
                assert!(cx.original_turn_fuel_remaining() < before);
                rows += 1;
            }
            Ok(())
        })).unwrap();
        if rows >= minimum_rows && capacity > released { break; }
    }
    assert!(rows >= minimum_rows && capacity > released, "{phase}");
    caller.cancel().cancel_now();
    let mut cancelled = false;
    for _ in 0..10_000 {
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| cx.with_original_work_loan(64, 1_500, |cx| {
            let (step, receipt) = job.step(cx).unwrap();
            released += receipt.released_bytes;
            match step {
                OperatorJobStep::Working(_) => {},
                OperatorJobStep::Cancelled(_) => cancelled = true,
                OperatorJobStep::Done(_) => panic!("cancelled GLB must never publish geometry"),
            }
            Ok(())
        }))).unwrap();
        if cancelled { break; }
    }
    assert!(cancelled);
    assert!(released >= capacity);
    assert!(job.terminal_is_empty());
    session.close();
    let stock = caller.stock();
    caller.retire(stock);
}

#[test]
fn mesh_decimation_posts_original_work_and_cancels_owned_discard() {
    use neural_engine::{Atom, Dictionary, OperatorJobStep, OperatorPlanAdmission, OperatorPlanCursor, OperatorPlanStep, Value as FlowValue};
    let fixture: Value = serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🪶️decimate.json")).unwrap();
    let source = fixture["input"].to_string();
    let ratio = fixture["ratio"].as_f64().unwrap();
    let stock = RetainedCloneGrant {
        maximum_items: 200_000, maximum_copy_bytes: 64 << 20, maximum_capacity_bytes: 64 << 20,
        maximum_release_bytes: 64 << 20, maximum_depth: 1_024,
    };
    let session = Session::new();
    let mut owner = ExtensionEvaluationResources::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(0x6D6002), Generation(7), stock, || Some(1_000), 1_500, 100_000);
    assert!((0..20_000).any(|_| caller.turn(GRANT, |cx| owner.admit_registry_on_original(cx)).unwrap() == Some(true)));
    let mesh = evaluate(&mut owner, &mut caller, "brep.mesh.construct", json!({"data": text(&source)}), 20_000);
    let result = evaluate(&mut owner, &mut caller, "brep.mesh.decimate", json!({"mesh": mesh["meshOut"], "ratio": number(ratio)}), 20_000);
    let analysis = evaluate(&mut owner, &mut caller, "brep.mesh.analyze", json!({"mesh": result["meshOut"]}), 20_000);
    assert_eq!(analysis["vertices"]["value"].as_f64(), Some(4.0));
    assert_eq!(analysis["faces"]["value"].as_f64(), Some(4.0));
    assert!((analysis["volume"]["value"].as_f64().unwrap() - fixture["resultVolume"].as_f64().unwrap()).abs() < 1e-6);

    let registry = semio_s_plugin_flow_brep::module_registry(&session);
    let source = Dictionary::with_schema("mesh").insert("data", FlowValue::Atom(Atom::String(source)));
    let ratio = Dictionary::with_schema("number").insert("value", FlowValue::Atom(Atom::Decimal(ratio)));
    let mut input = Some(Dictionary::new().insert("mesh", FlowValue::Dictionary(source)).insert("ratio", FlowValue::Dictionary(ratio)));
    let mut plan = OperatorPlanCursor::new();
    let mut job = (0..10_000).find_map(|_| match registry.dispatch_job("brep.mesh.decimate", &mut input, &mut plan, GRANT).unwrap() {
        OperatorPlanStep::Working => None,
        OperatorPlanStep::Admitted(OperatorPlanAdmission::Job(job)) => Some(job),
        _ => panic!("Decimate must admit a retained job"),
    }).unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, ..GRANT };
    let mut release = 0;
    for _ in 0..10_000 {
        caller.turn(grant, |cx| cx.with_original_work_loan(64, 1_500, |cx| {
            let before = cx.original_turn_fuel_remaining();
            let (step, receipt) = job.step(cx).unwrap();
            let OperatorJobStep::Working(progress) = step else { panic!("Decimate must enter discard before publishing"); };
            if progress.phase == "decimate-retire" && receipt.released_bytes > 0 {
                assert!(cx.original_turn_fuel_remaining() < before);
                release = receipt.released_bytes;
            }
            Ok(())
        })).unwrap();
        if release != 0 {break;}
    }
    assert!(release > 0);
    caller.cancel().cancel_now();
    let mut cancelled = false;
    let mut cancelled_release = 0;
    for _ in 0..10_000 {
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| cx.with_original_work_loan(64, 1_500, |cx| {
            let (step, receipt) = job.step(cx).unwrap();
            cancelled_release += receipt.released_bytes;
            match step {
                OperatorJobStep::Working(_) => {},
                OperatorJobStep::Cancelled(_) => cancelled = true,
                OperatorJobStep::Done(_) => panic!("Cancelled decimation published"),
            }
            Ok(())
        }))).unwrap();
        if cancelled {break;}
    }
    assert!(cancelled && job.terminal_is_empty());
    assert!(cancelled_release > 0);
    session.close();
    let stock = caller.stock();
    caller.retire(stock);
}

#[test]
fn mesh_obj_cancellation_retires_import_and_export_without_publication() {
    use neural_engine::{Atom, Dictionary, Value as FlowValue};
    let fixture: Value = serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧵️obj-corners.json")).unwrap();
    let mesh=Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string())));
    cancel_mesh_after("brep.mesh.exportObj",Dictionary::new().insert("mesh",FlowValue::Dictionary(mesh)),"mesh-output-obj-normal",1);
    let source=fixture["relativeObj"].as_str().unwrap().repeat(64);
    let data=Dictionary::with_schema("text").insert("value",FlowValue::Atom(Atom::String(source)));
    cancel_mesh_after("brep.mesh.importObj",Dictionary::new().insert("data",FlowValue::Dictionary(data)),"mesh-source-obj",128);
}

#[test]
fn mesh_orientation_cancellation_keeps_the_component_unpublished() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧭️orient-shells.json")).unwrap();
    let mesh=Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["closed"].to_string())));
    cancel_mesh_after("brep.mesh.orient",Dictionary::new().insert("mesh",FlowValue::Dictionary(mesh)),"orient-components",8);
}

#[test]
fn mesh_triangulation_cancels_during_surface_remap() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🎨️triangulate-surface.json")).unwrap();
    let mesh=Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string())));
    cancel_mesh_after("brep.mesh.triangulate",Dictionary::new().insert("mesh",FlowValue::Dictionary(mesh)),"triangulate-surface",8);
}

#[test]
fn mesh_subdivision_cancels_before_publishing_interpolated_surface() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🎨️triangulate-surface.json")).unwrap();
    let mesh=Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string())));
    let selection=Dictionary::with_schema("text").insert("value",FlowValue::Atom(Atom::String("[0]".into())));
    let input=Dictionary::new().insert("mesh",FlowValue::Dictionary(mesh)).insert("faces",FlowValue::Dictionary(selection));
    cancel_mesh_after("brep.mesh.subdivide",input,"subdivide-surface",16);
}

#[test]
fn mesh_weld_cancels_before_publishing_exact_source_surfaces() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧲️weld-surface.json")).unwrap();
    let mesh=Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string())));
    let tolerance=Dictionary::with_schema("number").insert("value",FlowValue::Atom(Atom::Decimal(fixture["tolerance"].as_f64().unwrap())));
    let input=Dictionary::new().insert("mesh",FlowValue::Dictionary(mesh)).insert("tolerance",FlowValue::Dictionary(tolerance));
    cancel_mesh_after("brep.mesh.weld",input,"weld-surface",17);
}

#[test]
fn mesh_decimation_cancels_before_publishing_corner_surfaces() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🪶️decimate-surface.json")).unwrap();
    let mesh=Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string())));
    let ratio=Dictionary::with_schema("number").insert("value",FlowValue::Atom(Atom::Decimal(fixture["ratio"].as_f64().unwrap())));
    let input=Dictionary::new().insert("mesh",FlowValue::Dictionary(mesh)).insert("ratio",FlowValue::Dictionary(ratio));
    cancel_mesh_after("brep.mesh.decimate",input,"decimate-surface",17);
}

#[test]
fn mesh_combine_cancels_before_publishing_either_component() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🔗️combine-assets.json")).unwrap();
    let pair=|name:&str|FlowValue::Dictionary(Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture[name].to_string()))));
    let input=Dictionary::new().insert("mesh",pair("mesh")).insert("other",pair("other"));
    cancel_mesh_after("brep.mesh.combine",input.clone(),"mesh-combine-surface",3);
    cancel_mesh_after("brep.mesh.combine",input,"mesh-combine-assets",25);
}

#[test]
fn mirror_cancels_during_retained_surface_projection() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🪞️mirror-surface.json")).unwrap();
    let mesh=FlowValue::Dictionary(Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string()))));
    let input=Dictionary::new().insert("mesh",mesh);
    cancel_mesh_after("brep.mesh.mirror",input,"mirror-surface",10);
}

#[test]
fn extrusion_cancels_during_retained_surface_projection() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🎈️expanded-surface.json")).unwrap();
    let mesh=FlowValue::Dictionary(Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string()))));
    cancel_mesh_after("brep.mesh.extrude",Dictionary::new().insert("mesh",mesh),"extrude-surface",12);
}

#[test]
fn loop_cut_cancels_while_copying_retained_surface() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🪡️cut-surface.json")).unwrap();
    let mesh=FlowValue::Dictionary(Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string()))));
    cancel_mesh_after("brep.mesh.loopCut",Dictionary::new().insert("mesh",mesh),"loop-cut-surface",12);
}

#[test]
fn deletion_cancels_before_publishing_surviving_faces() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🗑️delete-surface.json")).unwrap();
    let mesh=FlowValue::Dictionary(Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string()))));
    cancel_mesh_after("brep.mesh.deleteFaces",Dictionary::new().insert("mesh",mesh),"delete-faces-surface",12);
}

#[test]
fn coplanar_merge_cancels_during_retained_surface() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧷️coplanar-surface.json")).unwrap();
    let mesh=FlowValue::Dictionary(Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string()))));
    cancel_mesh_after("brep.mesh.mergeCoplanar",Dictionary::new().insert("mesh",mesh),"coplanar-surface",10);
}

#[test]
fn face_flip_cancels_during_retained_normal_projection() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🔃️face-flip.json")).unwrap();
    let mesh=FlowValue::Dictionary(Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string()))));
    cancel_mesh_after("brep.mesh.flip",Dictionary::new().insert("mesh",mesh),"flip-normals",2);
}

#[test]
fn orientation_cancels_during_retained_surface() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧭️orient-surface.json")).unwrap();
    let mesh=FlowValue::Dictionary(Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string()))));
    cancel_mesh_after("brep.mesh.orient",Dictionary::new().insert("mesh",mesh),"orient-surface",10);
}

#[test]
fn hole_filling_cancels_during_retained_cap_surface() {
    use neural_engine::{Atom,Dictionary,Value as FlowValue};
    let fixture:Value=serde_json::from_str(include_str!("../../🥽️mesh/🧪️tests/🧫️fixtures/🧢️fill-hole-surface.json")).unwrap();
    let mesh=FlowValue::Dictionary(Dictionary::with_schema("mesh").insert("data",FlowValue::Atom(Atom::String(fixture["source"].to_string()))));
    cancel_mesh_after("brep.mesh.fillHoles",Dictionary::new().insert("mesh",mesh),"fill-holes-surface",24);
}

fn cancel_mesh_after(operator:&str,dictionary:neural_engine::Dictionary,phase:&str,minimum_events:usize) {
    use neural_engine::{OperatorJobStep,OperatorPlanAdmission,OperatorPlanCursor,OperatorPlanStep};
    let session=Session::new();
    let registry=semio_s_plugin_flow_brep::module_registry(&session);
    let mut input=Some(dictionary);
    let mut plan=OperatorPlanCursor::new();
    let mut job=(0..10_000).find_map(|_|match registry.dispatch_job(operator,&mut input,&mut plan,GRANT).unwrap(){
        OperatorPlanStep::Working=>None,
        OperatorPlanStep::Admitted(OperatorPlanAdmission::Job(job))=>Some(job),
        _=>panic!("Mesh operation must admit a retained job"),
    }).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,..GRANT};
    let stock=RetainedCloneGrant{maximum_items:200_000,maximum_copy_bytes:64<<20,maximum_capacity_bytes:64<<20,maximum_release_bytes:64<<20,maximum_depth:1_024};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(0x6D6003),Generation(7),stock,||Some(1_000),1_500,100_000);
    let importing=operator=="brep.mesh.importObj";
    let mut events=0;
    let mut denied_growth=false;
    let mut measured_growth=false;
    for _ in 0..30_000 {
        let copy=job.next_step_copy_byte_demand().unwrap();
        let capacity=job.next_step_capacity_byte_demand(copy).unwrap();
        let release=job.next_step_release_byte_demand().unwrap();
        if importing && copy>0 && capacity>0 && !denied_growth {
            let restricted=RetainedCloneGrant {maximum_capacity_bytes:capacity-1,..grant};
            let before=job.progress().units_done;
            caller.turn(restricted,|cx|cx.with_original_work_loan(64,1_500,|cx|{
                let (step,receipt)=job.step(cx).unwrap();
                assert!(matches!(step,OperatorJobStep::Working(_)));
                assert_eq!(job.progress().units_done,before);
                assert_eq!(receipt,Default::default());
                Ok(())
            })).unwrap();
            denied_growth=true;
        }
        caller.turn(grant,|cx|cx.with_original_work_loan(64,1_500,|cx|{
            let advancing=job.progress().phase==phase;
            let before=cx.original_turn_fuel_remaining();
            let (step,receipt)=job.step(cx).unwrap();
            assert!(matches!(step,OperatorJobStep::Working(_)));
            if advancing {
                assert!(cx.original_turn_fuel_remaining()<before);
                events+=1;
                if importing && capacity>0 {
                    assert_eq!(receipt.copied_bytes,copy);
                    assert_eq!(receipt.retained_capacity_bytes,capacity);
                    assert_eq!(receipt.released_bytes,release);
                    measured_growth|=copy>0;
                }
            }
            Ok(())
        })).unwrap();
        if events>=minimum_events {break;}
    }
    assert!(events>=minimum_events);
    assert!(!importing || (denied_growth && measured_growth));
    caller.cancel().cancel_now();
    let mut cancelled=false;
    for _ in 0..10_000 {
        caller.turn(grant,|cx|cx.with_original_retirement(|cx|cx.with_original_work_loan(64,1_500,|cx|{
            match job.step(cx).unwrap().0 {
                OperatorJobStep::Working(_)=>{},
                OperatorJobStep::Cancelled(_)=>cancelled=true,
                OperatorJobStep::Done(_)=>panic!("Cancelled Mesh operation published"),
            }
            Ok(())
        }))).unwrap();
        if cancelled {break;}
    }
    assert!(cancelled&&job.terminal_is_empty());
    session.close();
    let stock=caller.stock();
    caller.retire(stock);
}
