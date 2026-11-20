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
    session.close();
    let stock = caller.stock();
    caller.retire(stock);
}
