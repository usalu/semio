use super::*;
use flow_extension_sdk::evaluate_json;
use neural_engine::{Atom, Value};
use semio_framework_3d::brep::engine::Brep;
use std::sync::{Mutex, OnceLock};

async fn point(x: f64, y: f64, z: f64) -> Dictionary {
    Dictionary::with_schema("point").insert("x", Value::Atom(Atom::Decimal(x))).insert("y", Value::Atom(Atom::Decimal(y))).insert("z", Value::Atom(Atom::Decimal(z)))
}

async fn vector(x: f64, y: f64, z: f64) -> Dictionary {
    Dictionary::with_schema("vector").insert("x", Value::Atom(Atom::Decimal(x))).insert("y", Value::Atom(Atom::Decimal(y))).insert("z", Value::Atom(Atom::Decimal(z)))
}

/// 🔒️ Serialises the tests that share the process-wide brep kernel. Recovers from a poisoned
/// lock so that one failing test reports its own assertion instead of cascading `PoisonError`s
/// through every sibling.
pub(super) async fn test_serial() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let lock = LOCK.get_or_init(|| Mutex::new(()));
    lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// 🔗️ Brep handles are content-addressed: `Brep::mint` hashes the geometry with blake3 and uses
/// the hex digest verbatim, so a handle carries no kind prefix — `kind` is the separate field.
async fn is_geometry_handle(geometry: &Dictionary) -> bool {
    let Some(handle) = geometry.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) else {
        return false;
    };
    handle.len() == 64 && handle.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

/// 🧊️ One channel payload, cloned straight into a cold boundary: the clone outlives the OUT
/// dictionary it came from, so it becomes the FINAL owner of its own pairs and must be retired.
async fn channel_payload(out: &Dictionary, channel: &str) -> neural_engine::ColdOwner<Dictionary> {
    neural_engine::ColdOwner::new(out.get(channel).and_then(|v| v.as_dictionary()).cloned().expect("channel payload"))
}

async fn reset_test_kernel() {
    geometry_session().retain_geometry_handles(&[]);
}

#[semio_framework_async_macros::async_test]
async fn box_emits_geometry_handle() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let input = Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(3.0))).insert("height", Value::Dictionary(number_dictionary(4.0)));
    let out = reg.dispatch_cold("brep.prim3d.box", input).unwrap();
    let solid = channel_payload(&out, "solid").await;
    assert_eq!(solid.schema(), Some("geometry"));
    assert!(is_geometry_handle(&solid).await);
    assert_eq!(solid.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("solid"));
}

#[semio_framework_async_macros::async_test]
async fn line_curve_emits_curve_handle() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let out = reg.dispatch_cold("brep.curve.line", Dictionary::new().insert("start", Value::Dictionary(point(0.0, 0.0, 0.0).await)).insert("end", Value::Dictionary(point(1.0, 0.0, 0.0).await))).unwrap();
    let curve = channel_payload(&out, "curve").await;
    assert_eq!(curve.schema(), Some("geometry"));
    assert!(is_geometry_handle(&curve).await);
    assert_eq!(curve.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("curve"));
}

#[semio_framework_async_macros::async_test]
async fn dwg_export_import_round_trips_a_box() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let solid = channel_payload(
        &reg.dispatch_cold("brep.prim3d.box", Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(3.0))).insert("height", Value::Dictionary(number_dictionary(4.0)))).unwrap(),
        "solid",
    )
    .await;
    let dwg = channel_payload(&reg.dispatch_cold("brep.io.exportDwg", Dictionary::new().insert("geometry", Value::Dictionary(solid.into_inner())).insert("deflection", Value::Dictionary(number_dictionary(0.1)))).unwrap(), "dwg").await;
    let data = dwg.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("dwg base64").to_string();
    assert!(!data.is_empty());

    let imported = channel_payload(&reg.dispatch_cold("brep.io.importDwg", Dictionary::new().insert("data", Value::Dictionary(text_dictionary(data))).insert("tolerance", Value::Dictionary(number_dictionary(0.1)))).unwrap(), "geometry").await;
    assert_eq!(imported.schema(), Some("geometry"));
    assert!(imported.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).is_some());
}

async fn box_handle(reg: &Registry) -> String {
    let solid = channel_payload(
        &reg.dispatch_cold("brep.prim3d.box", Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(3.0))).insert("height", Value::Dictionary(number_dictionary(4.0)))).unwrap(),
        "solid",
    )
    .await;
    solid.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).expect("box handle").to_string()
}

#[semio_framework_async_macros::async_test]
async fn step_export_import_round_trips_a_box() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let handle = box_handle(&reg).await;
    let exported = semio_framework_pack_json::parse(&geometry_session().export_solid_json(&[handle], "step", 0.1), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(exported.get("error").is_none(), "{exported:?}");
    assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(false));
    let data = exported.get("data").and_then(|value| value.as_str()).expect("step text").to_string();
    assert!(!data.is_empty());
    let imported = semio_framework_pack_json::parse(&geometry_session().import_solid_json("step", &data, 0.1), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(imported.get("error").is_none(), "{imported:?}");
    assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn obj_export_import_round_trips_a_box() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let handle = box_handle(&reg).await;
    let exported = semio_framework_pack_json::parse(&geometry_session().export_solid_json(&[handle], "obj", 0.1), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(exported.get("error").is_none(), "{exported:?}");
    let data = exported.get("data").and_then(|value| value.as_str()).expect("obj text").to_string();
    assert!(data.contains('v'));
    let imported = semio_framework_pack_json::parse(&geometry_session().import_solid_json("obj", &data, 0.1), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(imported.get("error").is_none(), "{imported:?}");
    assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn stl_export_import_round_trips_a_box() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let handle = box_handle(&reg).await;
    let exported = semio_framework_pack_json::parse(&geometry_session().export_solid_json(&[handle], "stl", 0.1), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(exported.get("error").is_none(), "{exported:?}");
    assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(true));
    let data = exported.get("data").and_then(|value| value.as_str()).expect("stl base64").to_string();
    assert!(!data.is_empty());
    let imported = semio_framework_pack_json::parse(&geometry_session().import_solid_json("stl", &data, 0.1), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(imported.get("error").is_none(), "{imported:?}");
    assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn glb_export_import_round_trips_a_box_through_the_mesh_bridge() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let handle = box_handle(&reg).await;
    let exported = semio_framework_pack_json::parse(&geometry_session().export_solid_json(&[handle], "glb", 0.1), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(exported.get("error").is_none(), "{exported:?}");
    assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(true));
    let data = exported.get("data").and_then(|value| value.as_str()).expect("glb base64").to_string();
    assert!(!data.is_empty());
    let imported = semio_framework_pack_json::parse(&geometry_session().import_solid_json("glb", &data, 0.1), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(imported.get("error").is_none(), "{imported:?}");
    assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn export_solid_json_rejects_unsupported_format() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let handle = box_handle(&reg).await;
    let exported = semio_framework_pack_json::parse(&geometry_session().export_solid_json(&[handle], "fbx", 0.1), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(exported.get("error").is_some());
}

#[semio_framework_async_macros::async_test]
async fn extrude_and_area() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let wire = channel_payload(&reg.dispatch_cold("brep.curve.rectangle", Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0)))).unwrap(), "wire").await;
    let face = channel_payload(&reg.dispatch_cold("brep.surf.planarFaceWire", Dictionary::new().insert("wire", Value::Dictionary(wire.into_inner()))).unwrap(), "face").await;
    let solid = channel_payload(&reg.dispatch_cold("brep.sweep.extrude", Dictionary::new().insert("face", Value::Dictionary(face.into_inner())).insert("vector", Value::Dictionary(vector(0.0, 0.0, 3.0).await))).unwrap(), "solid").await;
    assert_eq!(solid.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("solid"));
    let area = channel_payload(&reg.dispatch_cold("brep.measure.area", Dictionary::new().insert("geometry", Value::Dictionary(solid.into_inner()))).unwrap(), "area").await;
    assert_eq!(area.schema(), Some("number"));
    let value = area.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
    assert!(value > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn extrude_curve_wire_uses_vector_magnitude() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let wire = channel_payload(&reg.dispatch_cold("brep.curve.rectangle", Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0)))).unwrap(), "wire").await;
    let solid = channel_payload(&reg.dispatch_cold("brep.solid.extrude", Dictionary::new().insert("wire", Value::Dictionary(wire.into_inner())).insert("vector", Value::Dictionary(vector(0.0, 0.0, 4.0).await))).unwrap(), "solid").await;
    assert_eq!(solid.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("solid"));
    let volume = channel_payload(&reg.dispatch_cold("brep.measure.volume", Dictionary::new().insert("geometry", Value::Dictionary(solid.into_inner()))).unwrap(), "volume").await;
    let value = volume.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
    assert!((value - 16.0).abs() < 1e-3);
}

#[semio_framework_async_macros::async_test]
async fn fillet_translate_chain() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let box_out = channel_payload(
        &reg.dispatch_cold("brep.prim3d.box", Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0)))).unwrap(),
        "solid",
    )
    .await;
    let fillet_out = channel_payload(&reg.dispatch_cold("brep.solid.fillet", Dictionary::new().insert("geometry", Value::Dictionary(box_out.into_inner())).insert("radius", Value::Dictionary(number_dictionary(0.1)))).unwrap(), "solid").await;
    let moved = channel_payload(&reg.dispatch_cold("brep.xform.translate", Dictionary::new().insert("geometry", Value::Dictionary(fillet_out.into_inner())).insert("offset", Value::Dictionary(vector(1.0, 0.0, 0.0).await))).unwrap(), "geometryOut").await;
    assert_eq!(moved.schema(), Some("geometry"));
}

#[semio_framework_async_macros::async_test]
async fn manifest_lists_brep_operators() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let json = build_manifest_json("brep", "Brep", env!("CARGO_PKG_VERSION"), &neural_engine::ColdOwner::new(module_registry(geometry_session())), vec!["onStartup".into()], vec![], vec![], vec![]);
    assert!(json.contains("brep.prim3d.box"));
    assert!(json.contains("brep.curve.line"));
    assert!(json.contains("brep.solid.extrude"));
    assert!(json.contains("brep.sweep.extrude"));
    assert!(json.contains("brep.measure.area"));
    assert!(json.contains("\"operators\""));
    assert!(json.contains("brep.xform.translate"));
    assert!(json.contains("brep.geometry"));
    assert!(json.contains("brep.brep"));
    assert!(json.contains("\"Schemas\""));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_json_box() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let reg = neural_engine::ColdOwner::new(module_registry(geometry_session()));
    let json_number = |value: f64| semio_framework_pack_json::object([("$schema".to_string(), semio_framework_pack_json::Value::from("number")), ("value".to_string(), semio_framework_pack_json::Value::from(value))]);
    let input_json = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("width".to_string(), json_number(1.0)), ("depth".to_string(), json_number(1.0)), ("height".to_string(), json_number(1.0))]));
    let out_json = evaluate_json(&reg, "brep.prim3d.box", &input_json);
    let out = semio_framework_pack_json::parse(&out_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(out.get("solid").and_then(|value| value.get("$schema")).and_then(semio_framework_pack_json::Value::as_str), Some("geometry"));
}

#[semio_framework_async_macros::async_test]
async fn retain_geometry_handles_sweeps_orphaned_shapes() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let box_out = channel_payload(
        &reg.dispatch_cold("brep.prim3d.box", Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0)))).unwrap(),
        "solid",
    )
    .await;
    let orphan = channel_payload(
        &reg.dispatch_cold("brep.prim3d.box", Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0)))).unwrap(),
        "solid",
    )
    .await;
    let live_handle = box_out.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap().to_string();
    let orphan_handle = orphan.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap().to_string();
    geometry_session().retain_geometry_handles(std::slice::from_ref(&live_handle));
    let live_mesh = geometry_session().tessellate_geometry(&live_handle, 0.1).expect("live tessellation");
    assert!(!live_mesh.positions.is_empty());
    assert!(geometry_session().tessellate_geometry(&orphan_handle, 0.1).is_err());
}

#[semio_framework_async_macros::async_test]
async fn tessellate_geometry_is_memoized() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let box_out = channel_payload(
        &reg.dispatch_cold("brep.prim3d.box", Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0)))).unwrap(),
        "solid",
    )
    .await;
    let handle = box_out.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap();
    let first = geometry_session().tessellate_geometry(handle, 0.1).expect("mesh");
    let second = geometry_session().tessellate_geometry(handle, 0.1).expect("mesh");
    assert_eq!(first.positions, second.positions);
    assert!(!first.positions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn brep_component_deconstructs_solid_topology() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let solid = channel_payload(
        &reg.dispatch_cold("brep.prim3d.box", Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0)))).unwrap(),
        "solid",
    )
    .await;
    let deconstructed = reg.dispatch_cold("brep.brep", Dictionary::new().insert("brep", Value::Dictionary(solid.into_inner())).insert("edgeLabels",Value::Dictionary(text_dictionary("[]"))).insert("faceLabels",Value::Dictionary(text_dictionary("[]"))).insert("sourceHandle",Value::Dictionary(text_dictionary("")))).unwrap();
    let vertices = deconstructed.get("vertex").and_then(Value::as_dictionary).expect("vertex list");
    let edges = deconstructed.get("edge").and_then(Value::as_dictionary).expect("edge list");
    let faces = deconstructed.get("face").and_then(Value::as_dictionary).expect("face list");
    let shells = deconstructed.get("shell").and_then(Value::as_dictionary).expect("shell list");
    assert_eq!(list_indices(vertices).len(), 8);
    assert_eq!(list_indices(edges).len(), 12);
    assert_eq!(list_indices(faces).len(), 6);
    assert_eq!(list_indices(shells).len(), 1);
    assert_eq!(reg.schema("shell").unwrap().name, "Shell");
}

#[semio_framework_async_macros::async_test]
async fn schema_component_deconstructs_geometry() {
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let geometry = neural_engine::ColdOwner::new(Dictionary::with_schema("geometry").insert("handle", Value::Atom(Atom::String("solid-1".into()))).insert("kind", Value::Atom(Atom::String("solid".into()))));
    let out = reg.dispatch_cold("brep.geometry", Dictionary::new().insert("geometry", Value::Dictionary(geometry.clone()))).unwrap();
    assert_eq!(out.get("handleOut").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("solid-1"));
    assert_eq!(out.get("kindOut").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("solid"));
}

/// 🧩️ The live topology widget exposes every reachable component through its existing owner.
#[semio_framework_async_macros::async_test]
async fn topology_widget_exposes_every_shape_kind_and_exact_labels() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧫️fixtures/🎯️component-picking/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let session = Session::new();
    let mut registry = Registry::new();
    register(&mut registry, &session);
    let registry = neural_engine::ColdOwner::new(registry);
    let width = fixture.get("wire").unwrap().get("width").unwrap().as_f64().unwrap();
    let height = fixture.get("wire").unwrap().get("height").unwrap().as_f64().unwrap();
    let shapes = session.with_kernel(|kernel| {
        let wire = kernel.rectangle_wire(width, height).map_err(|error| map_kernel_error(&error))?;
        let face = kernel.face_from_wire(&wire).map_err(|error| map_kernel_error(&error))?;
        let solid = kernel.box_prim(width, height, 1.0).map_err(|error| map_kernel_error(&error))?;
        let components = kernel.deconstruct(&solid).map_err(|error| map_kernel_error(&error))?;
        let second = kernel.box_prim(width, height, 1.0).map_err(|error| map_kernel_error(&error))?;
        let compound = kernel.compound(&[solid.clone(), second]).map_err(|error| map_kernel_error(&error))?;
        let shared = kernel.compound(&[solid.clone(), solid.clone()]).map_err(|error| map_kernel_error(&error))?;
        let vertex = kernel.vertex([0.0, 0.0, 0.0]).map_err(|error| map_kernel_error(&error))?;
        let curve = kernel.line_curve([0.0, 0.0, 0.0], [width, 0.0, 0.0]).map_err(|error| map_kernel_error(&error))?;
        let surface = kernel.plane_surface([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).map_err(|error| map_kernel_error(&error))?;
        let circle = kernel.circle_curve([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], fixture.get("measurements").unwrap().get("circleRadius").unwrap().as_f64().unwrap()).map_err(|error| map_kernel_error(&error))?;
        Ok(std::collections::BTreeMap::from([("wire", wire), ("face", face), ("solid", solid), ("edge", components.edges[0].clone()), ("shell", components.shells[0].clone()), ("compound", compound), ("shared-compound", shared), ("vertex", vertex), ("curve", curve), ("surface", surface), ("circle", circle)]))
    }).unwrap();
    for row in fixture.get("deconstruction").unwrap().as_array().unwrap() {
        let kind = row.get("kind").unwrap().as_str().unwrap();
        let handle = &shapes[kind];
        let geometry = session.with_kernel_read(|kernel| geometry_dict(kernel, handle)).unwrap();
        let output = registry.dispatch_cold("brep.brep", Dictionary::new().insert("brep", Value::Dictionary(geometry)).insert("edgeLabels",Value::Dictionary(text_dictionary("[]"))).insert("faceLabels",Value::Dictionary(text_dictionary("[]"))).insert("sourceHandle",Value::Dictionary(text_dictionary("")))).unwrap();
        for (index, channel) in ["vertex", "edge", "face", "shell"].iter().enumerate() {
            let components = output.get(channel).and_then(Value::as_dictionary).unwrap_or_else(|| panic!("{kind}.{channel}"));
            let indices = list_indices(components);
            assert_eq!(indices.len(), row.get("counts").unwrap().as_array().unwrap()[index].as_u64().unwrap() as usize, "{kind}.{channel}");
            for index in indices {
                let component = components.get(&index.to_string()).and_then(Value::as_dictionary).unwrap();
                let source = Dictionary::new().insert("geometry", Value::Dictionary(component.clone()));
                let expected = session.with_kernel_read(|kernel| Ok(kernel.label(&read_geometry(&source, "geometry")?).unwrap().to_string())).unwrap();
                let label = registry.dispatch_cold("brep.topology.label", source).unwrap();
                let label = label.get("label").and_then(Value::as_dictionary).unwrap();
                assert_eq!(label.schema(), Some("text"));
                assert_eq!(label.get("value").and_then(Value::as_atom).and_then(Atom::as_str), Some(expected.as_str()));
            }
        }
    }
    let mut measurements = 0;
    for metric in ["length", "area", "volume"] {
        for row in fixture.get("measurements").unwrap().get(metric).unwrap().as_array().unwrap() {
            let kind = row.get("kind").unwrap().as_str().unwrap();
            let geometry = session.with_kernel_read(|kernel| geometry_dict(kernel, &shapes[kind])).unwrap();
            let output = registry.dispatch_cold(&format!("brep.measure.{metric}"), Dictionary::new().insert("geometry", Value::Dictionary(geometry))).unwrap();
            let value = number_value(output.get(metric).and_then(Value::as_dictionary).unwrap());
            assert!((value - row.get("value").unwrap().as_f64().unwrap()).abs() < 1e-6, "{metric} {kind}: {value}");
            measurements += 1;
        }
    }
    eprintln!("[DEBUG] BRep topology widget shapeKinds=10 labelEncoding=decimal-text shellPort=true measurements={measurements}");
    drop(registry);
    session.close();
    assert!(session.terminal_is_empty());
}

#[cfg(feature = "component-guest")]
#[semio_framework_async_macros::async_test]
async fn extension_bundle_extends_flow_and_evaluates_box() {
    use semio_framework_plugin::{extension_activate, extension_invoke, extension_manifest, install_extension_bundle};

    let _serial = test_serial().await;
    reset_test_kernel().await;
    let bundle = super::extension_guest::bundle();
    assert!(install_extension_bundle(&mut Some(bundle)).await.unwrap());
    extension_activate().await.unwrap();
    assert_eq!(extension_manifest().await.extension_id, "flow-extension-brep");
    let json_number = |value: f64| semio_framework_pack_json::object([("$schema".to_string(), semio_framework_pack_json::Value::from("number")), ("value".to_string(), semio_framework_pack_json::Value::from(value))]);
    let input_json = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("width".to_string(), json_number(1.0)), ("depth".to_string(), json_number(1.0)), ("height".to_string(), json_number(1.0))]));
    let req =
        semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("operatorId".to_string(), semio_framework_pack_json::Value::from("brep.prim3d.box")), ("inputJson".to_string(), semio_framework_pack_json::Value::from(input_json)), ("nodeHash".to_string(), semio_framework_pack_json::Value::from(1_i64))]));
    // ⏱️ `evaluate` answers the BUDGET envelope, not a bare out dictionary — a primitive finishes
    // inside its first round trip, so this one is `done` with its output inside
    // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`).
    let envelope = semio_framework_pack_json::parse_bytes(&extension_invoke("evaluate", req.as_bytes()).await.unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(envelope.get("done").and_then(semio_framework_pack_json::Value::as_bool), Some(true));
    let out = semio_framework_pack_json::parse(envelope.get("outputJson").and_then(semio_framework_pack_json::Value::as_str).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(out.get("solid").and_then(|value| value.get("$schema")).and_then(semio_framework_pack_json::Value::as_str), Some("geometry"));
    semio_framework_plugin::plugin_runtime::extension_dispose_cold().unwrap();
}

fn number_value(dict: &Dictionary) -> f64 {
    dict.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).expect("number channel value")
}

async fn box_of(reg: &Registry, size: f64) -> neural_engine::ColdOwner<Dictionary> {
    channel_payload(
        &reg.dispatch_cold("brep.prim3d.box", Dictionary::new().insert("width", Value::Dictionary(number_dictionary(size))).insert("depth", Value::Dictionary(number_dictionary(size))).insert("height", Value::Dictionary(number_dictionary(size))))
            .unwrap(),
        "solid",
    )
    .await
}

/// 🏄️ Surfaces family: every `(u, v)` on a plane must stay in-plane regardless of the
/// surface's own (arbitrary) in-plane basis — a basis-independent invariant to check against.
#[semio_framework_async_macros::async_test]
async fn surface_family_plane_point_stays_in_plane_and_normal_matches() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let surface = channel_payload(&reg.dispatch_cold("brep.surf.plane", Dictionary::new().insert("origin", Value::Dictionary(point(0.0, 0.0, 0.0).await)).insert("normal", Value::Dictionary(vector(0.0, 0.0, 1.0).await))).unwrap(), "surface").await;
    let evaluated = channel_payload(
        &reg.dispatch_cold("brep.eval.surfPoint", Dictionary::new().insert("surface", Value::Dictionary(surface.clone())).insert("u", Value::Dictionary(number_dictionary(1.0))).insert("v", Value::Dictionary(number_dictionary(-2.0)))).unwrap(),
        "point",
    )
    .await;
    let z = evaluated.get("z").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
    assert!(z.abs() < 1e-9, "a point on the z=0 plane must have z=0 regardless of (u, v), got z={z}");
    let normal = channel_payload(
        &reg.dispatch_cold("brep.eval.surfNormal", Dictionary::new().insert("surface", Value::Dictionary(surface.into_inner())).insert("u", Value::Dictionary(number_dictionary(0.0))).insert("v", Value::Dictionary(number_dictionary(0.0)))).unwrap(),
        "normal",
    )
    .await;
    let normal_z = normal.get("z").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
    assert!(normal_z.abs() > 0.999, "plane normal must be parallel to the world Z axis, got z={normal_z}");
}

/// 🔗️ Booleans family: overlapping unit-ish boxes, checked by plausible volume bounds only —
/// `fuse`/`cut`/`intersect` are `MeshDerivedBRep` today (not exact), so exact numerics would
/// be over-claiming precision the kernel does not yet provide.
#[semio_framework_async_macros::async_test]
async fn boolean_family_fuse_cut_intersect_report_plausible_volumes() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let a = box_of(&reg, 2.0).await;
    let b_raw = box_of(&reg, 2.0).await;
    let b = channel_payload(&reg.dispatch_cold("brep.xform.translate", Dictionary::new().insert("geometry", Value::Dictionary(b_raw.into_inner())).insert("offset", Value::Dictionary(vector(1.0, 0.0, 0.0).await))).unwrap(), "geometryOut").await;

    let fused = channel_payload(&reg.dispatch_cold("brep.bool.fuse", Dictionary::new().insert("a", Value::Dictionary(a.clone())).insert("b", Value::Dictionary(b.clone()))).unwrap(), "solid").await;
    let fused_volume = number_value(&*channel_payload(&reg.dispatch_cold("brep.measure.volume", Dictionary::new().insert("geometry", Value::Dictionary(fused.into_inner()))).unwrap(), "volume").await);
    assert!((8.0..16.0).contains(&fused_volume), "fused volume {fused_volume} should exceed either box's own 8.0 but stay below the disjoint sum 16.0");

    let cut = channel_payload(&reg.dispatch_cold("brep.bool.cut", Dictionary::new().insert("a", Value::Dictionary(a.clone())).insert("b", Value::Dictionary(b.clone()))).unwrap(), "solid").await;
    let cut_volume = number_value(&*channel_payload(&reg.dispatch_cold("brep.measure.volume", Dictionary::new().insert("geometry", Value::Dictionary(cut.into_inner()))).unwrap(), "volume").await);
    assert!((0.0..8.0).contains(&cut_volume), "cut volume {cut_volume} should be less than the untouched box's 8.0");

    let intersected = channel_payload(&reg.dispatch_cold("brep.bool.intersect", Dictionary::new().insert("a", Value::Dictionary(a.into_inner())).insert("b", Value::Dictionary(b.into_inner()))).unwrap(), "solid").await;
    let intersect_volume = number_value(&*channel_payload(&reg.dispatch_cold("brep.measure.volume", Dictionary::new().insert("geometry", Value::Dictionary(intersected.into_inner()))).unwrap(), "volume").await);
    assert!((0.0..8.0).contains(&intersect_volume), "intersection volume {intersect_volume} should be less than either box's own 8.0");
}

/// 🔁️ Transforms family, `rotate_about` specifically — distinguishes it from `rotate` (world
/// origin only): rotating 180° about an explicit off-origin point must move the geometry far
/// from where a world-origin rotation would leave it (audit §6.2's bounding-box-center bug).
#[semio_framework_async_macros::async_test]
async fn rotate_about_rotates_around_the_given_origin_not_the_world_origin() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let solid = box_of(&reg, 1.0).await;
    let rotated = channel_payload(
        &reg.dispatch_cold(
            "brep.xform.rotateAbout",
            Dictionary::new()
                .insert("geometry", Value::Dictionary(solid.into_inner()))
                .insert("origin", Value::Dictionary(point(5.0, 0.0, 0.0).await))
                .insert("axis", Value::Dictionary(vector(0.0, 0.0, 1.0).await))
                .insert("angle", Value::Dictionary(number_dictionary(std::f64::consts::PI))),
        )
        .unwrap(),
        "geometryOut",
    )
    .await;
    let center = channel_payload(&reg.dispatch_cold("brep.measure.centerOfMass", Dictionary::new().insert("geometry", Value::Dictionary(rotated.into_inner()))).unwrap(), "center").await;
    let x = center.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
    assert!((x - 9.5).abs() < 1e-6, "180° about origin (5,0,0) should move the unit box's center from x=0.5 to x=9.5, got x={x}");
}

/// 🎯️ Evaluation family, closest-parameter/UV specifically — both are exact closed forms for
/// a line and a plane, so the achieved distance is checked exactly, not just bounded.
#[semio_framework_async_macros::async_test]
async fn evaluation_family_closest_parameter_and_closest_uv_report_certified_distance() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let curve = channel_payload(&reg.dispatch_cold("brep.curve.line", Dictionary::new().insert("start", Value::Dictionary(point(0.0, 0.0, 0.0).await)).insert("end", Value::Dictionary(point(10.0, 0.0, 0.0).await))).unwrap(), "curve").await;
    let out = reg.dispatch_cold("brep.eval.curveClosestParameter", Dictionary::new().insert("curve", Value::Dictionary(curve.into_inner())).insert("point", Value::Dictionary(point(4.0, 3.0, 0.0).await))).unwrap();
    let distance = number_value(&*channel_payload(&out, "distance").await);
    assert!((distance - 3.0).abs() < 1e-9, "closest distance from (4,3,0) to the segment along the x axis should be 3, got {distance}");
    let closest = channel_payload(&out, "pointOut").await;
    assert!((closest.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap() - 4.0).abs() < 1e-9);

    let surface = channel_payload(&reg.dispatch_cold("brep.surf.plane", Dictionary::new().insert("origin", Value::Dictionary(point(0.0, 0.0, 0.0).await)).insert("normal", Value::Dictionary(vector(0.0, 0.0, 1.0).await))).unwrap(), "surface").await;
    let uv_out = reg.dispatch_cold("brep.eval.surfaceClosestUv", Dictionary::new().insert("surface", Value::Dictionary(surface.into_inner())).insert("point", Value::Dictionary(point(1.0, 1.0, 5.0).await))).unwrap();
    let uv_distance = number_value(&*channel_payload(&uv_out, "distance").await);
    assert!((uv_distance - 5.0).abs() < 1e-6, "closest distance from (1,1,5) to the z=0 plane should be 5, got {uv_distance}");
}

/// 🐚️ Topology family — the new wave-1 handle capabilities: shells, compound/explode, and
/// the persistent label round trip.
#[semio_framework_async_macros::async_test]
async fn topology_family_shells_compound_explode_and_label() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg, geometry_session());
    let reg = neural_engine::ColdOwner::new(reg);
    let box_a = box_of(&reg, 1.0).await;
    let box_b = box_of(&reg, 2.0).await;
    let handle_a = box_a.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("box a handle").to_string();
    let handle_b = box_b.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("box b handle").to_string();

    let shells_out = reg.dispatch_cold("brep.topology.shells", Dictionary::new().insert("solid", Value::Dictionary(box_a.clone()))).unwrap();
    let shells = shells_out.get("shells").and_then(Value::as_dictionary).expect("shells list");
    assert_eq!(list_indices(shells).len(), 1, "a simple box has exactly one outer shell");

    let solids_in = topology_list("geometry", vec![GeometryHandle(handle_a), GeometryHandle(handle_b)]);
    let compound_out = reg.dispatch_cold("brep.topology.compound", Dictionary::new().insert("solids", Value::Dictionary(solids_in))).unwrap();
    let compound = channel_payload(&compound_out, "compound").await;
    assert_eq!(compound.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("compound"));

    let exploded_out = reg.dispatch_cold("brep.topology.explode", Dictionary::new().insert("compound", Value::Dictionary(compound.into_inner()))).unwrap();
    let solids_out = exploded_out.get("solids").and_then(Value::as_dictionary).expect("solids list");
    assert_eq!(list_indices(solids_out).len(), 2, "exploding must recover both original solids");

    let source = neural_engine::ColdOwner::new(Dictionary::new().insert("geometry", Value::Dictionary(box_a.into_inner())));
    let expected_label = geometry_session().with_kernel_read(|kernel| Ok(kernel.label(&read_geometry(&source, "geometry")?).unwrap())).unwrap();
    let label_out = reg.dispatch_cold("brep.topology.label", source.into_inner()).unwrap();
    let label = channel_payload(&label_out, "label").await;
    assert_eq!(label.schema(), Some("text"));
    assert_eq!(label.get("value").and_then(Value::as_atom).and_then(Atom::as_str), Some(expected_label.to_string().as_str()));
    eprintln!("[DEBUG] BRep persistent label text={expected_label}");
}

/// 🎯️ `q`'s tag round-trips through the live registry — the contract's `operation_quality`
/// stays the single source of truth: nothing here hardcodes an `OpQuality` a second time.
#[semio_framework_async_macros::async_test]
async fn operation_quality_tags_match_the_kernel_contract() {
    let reg = neural_engine::ColdOwner::new(module_registry(geometry_session()));
    for (id, method) in NODE_KERNEL_METHOD.iter().copied() {
        let info = reg.operator_info(id).unwrap_or_else(|| panic!("node {id:?} is registered in NODE_KERNEL_METHOD but not in the live Registry"));
        let expected = format!("[quality:{:?}]", operation_quality(method));
        assert!(info.summary.contains(expected.as_str()), "node {id:?}'s summary {:?} does not carry {expected:?} for its wrapped method {method:?}", info.summary);
    }
}

/// 📇️ Every `BrepKernel` trait method is either wrapped by exactly one node, or explicitly
/// listed as unexposed — nothing falls through both lists, and nothing in either list names a
/// method the trait does not actually have.
#[semio_framework_async_macros::async_test]
async fn every_kernel_operation_is_either_a_node_or_explicitly_unexposed() {
    let mut seen = std::collections::HashSet::new();
    for (id, method) in NODE_KERNEL_METHOD {
        assert!(seen.insert(*method), "BrepKernel method {method:?} (node {id:?}) is wrapped by more than one flow node");
    }
    for (method, _reason) in INTENTIONALLY_UNEXPOSED {
        assert!(seen.insert(*method), "{method:?} is listed in both NODE_KERNEL_METHOD and INTENTIONALLY_UNEXPOSED");
    }
    let known: std::collections::HashSet<&str> = BREP_KERNEL_OPERATIONS.iter().chain(semio_s_artifact_stdio_step::geometry::STEP_GEOMETRY_OPERATIONS).copied().collect();
    for method in &seen {
        assert!(known.contains(method), "{method:?} in NODE_KERNEL_METHOD/INTENTIONALLY_UNEXPOSED is not a real BrepKernel method");
    }
    for operation in BREP_KERNEL_OPERATIONS {
        assert!(seen.contains(operation), "BrepKernel method {operation:?} has neither a flow node nor an INTENTIONALLY_UNEXPOSED entry");
    }
    assert_eq!(seen.len(), BREP_KERNEL_OPERATIONS.len()+semio_s_artifact_stdio_step::geometry::STEP_GEOMETRY_OPERATIONS.len());
}

/// ⏱️ The BUDGET law of the `evaluate` capability — its own module because the law is about how a
/// long set operation YIELDS, not about what any one operator computes. Mounted INSIDE this module
/// so it shares the process-wide kernel serialisation and reset helpers above
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[path = "../🔬️evaluate-budget/🦀️.rs"]
mod evaluate_budget;

#[semio_framework_async_macros::async_test]
async fn every_brep_port_has_explicit_types_and_distinct_identifiers() {
    let registry = neural_engine::ColdOwner::new(module_registry(geometry_session()));
    for info in registry.operator_infos() {
        for (direction, channels) in [("inputs", &info.inputs), ("outputs", &info.outputs)] {
            let mut codes = std::collections::HashSet::new();
            let mut abbreviations = std::collections::HashSet::new();
            let mut names = std::collections::HashSet::new();
            let mut full_names = std::collections::HashSet::new();
            for channel in channels {
                assert!(!channel.value_types.is_empty(), "{} {direction} {} has no value types", info.id, channel.name);
                assert!(codes.insert(&channel.code), "{} {direction} repeats code {}", info.id, channel.code);
                assert!(abbreviations.insert(&channel.abbreviation), "{} {direction} repeats abbreviation {}", info.id, channel.abbreviation);
                assert!(names.insert(&channel.name), "{} {direction} repeats name {}", info.id, channel.name);
                assert!(full_names.insert(&channel.full_name), "{} {direction} repeats full name {}", info.id, channel.full_name);
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn packaged_widget_descriptor_matches_live_registration() {
    let descriptor = semio_framework_pack_json::parse(include_str!("../../🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let manifest_json = extension_manifest_json().await;
    let manifest = semio_framework_pack_json::parse(&manifest_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let expected: serde_json::Value = serde_json::from_str(&manifest_json).unwrap();
    for topic in descriptor.get("manifest").unwrap().get("topicContributions").unwrap().as_array().unwrap() {
        let packaged_json = topic.get("payload").unwrap().get("manifestJson").unwrap().as_str().unwrap();
        let packaged = semio_framework_pack_json::parse(packaged_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&packaged)).unwrap();
        if actual != expected {
            if let Ok(root) = std::env::var("SEMIO_TEST_ARTIFACT_DIR") {
                std::fs::create_dir_all(&root).unwrap();
                std::fs::write(std::path::Path::new(&root).join("brep-manifest-actual.json"), serde_json::to_string(&actual).unwrap()).unwrap();
                std::fs::write(std::path::Path::new(&root).join("brep-manifest-expected.json"), serde_json::to_string(&expected).unwrap()).unwrap();
            }
        }
        assert_eq!(actual, expected);
    }
    eprintln!("[DEBUG] BRep packaged/live widget manifest topics={} operators={}", descriptor.get("manifest").unwrap().get("topicContributions").unwrap().as_array().unwrap().len(), manifest.get("contributes").unwrap().get("operators").unwrap().as_array().unwrap().len());
}

#[semio_framework_async_macros::async_test]
async fn portable_channel_identity_fixtures_match_live_widgets() {
    let registry = neural_engine::ColdOwner::new(module_registry(geometry_session()));
    let fixtures = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🪪️channels/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for fixture in fixtures.get("cases").unwrap().as_array().unwrap() {
        let info = registry.operator_info(fixture.get("operator").unwrap().as_str().unwrap()).unwrap();
        let channels = if fixture.get("direction").unwrap().as_str() == Some("inputs") { &info.inputs } else { &info.outputs };
        for (name, fields) in fixture.get("expected").unwrap().as_object().unwrap() {
            let channel = channels.iter().find(|channel| channel.name == *name).unwrap();
            if let Some(code) = fields.get("code") { assert_eq!(channel.code, code.as_str().unwrap()); }
            if let Some(abbreviation) = fields.get("abbreviation") { assert_eq!(channel.abbreviation, abbreviation.as_str().unwrap()); }
            let types: Vec<_> = fields.get("valueTypes").unwrap().as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect();
            assert_eq!(channel.value_types, types);
            if let Some(types) = fields.get("itemTypes") { assert_eq!(channel.item_types, types.as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect::<Vec<_>>()); }
            if let Some(cardinality) = fields.get("cardinality") { assert_eq!(semio_framework_pack_json::to_json_string(&channel.cardinality), semio_framework_pack_json::to_json_string(cardinality)); }
            if let Some(default) = fields.get("default") { assert_eq!(semio_framework_pack_json::to_json_string(channel.default.as_ref().unwrap()), semio_framework_pack_json::to_json_string(default)); }
        }
    }
}

fn geometry_session() -> &'static Session {
    static SESSION: std::sync::OnceLock<Session> = std::sync::OnceLock::new();
    SESSION.get_or_init(Session::new)
}

/// 🎯️ Exact component labels resolve only within the current source topology.
#[semio_framework_async_macros::async_test]
async fn brep_deconstruct_resolves_selected_labels_without_ordinal_identity() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️component-label-selection/🔣️.json")).unwrap();
    let row = &fixture["box"];
    let point = parry3d::math::Point::new;
    let edge_length = parry3d::shape::Segment::new(point(0.0,0.0,0.0),point(row["width"].as_f64().unwrap() as f32,0.0,0.0)).length();
    let face_area = parry3d::shape::Triangle::new(point(0.0,0.0,0.0),point(2.0,0.0,0.0),point(2.0,3.0,0.0)).area() * 2.0;
    assert_eq!(edge_length as f64,row["edgeLength"].as_f64().unwrap());
    assert_eq!(face_area as f64,row["faceArea"].as_f64().unwrap());
    let registry = neural_engine::ColdOwner::new(module_registry(geometry_session()));
    let source = channel_payload(&registry.dispatch_cold("brep.prim3d.box",Dictionary::new().insert("width",Value::Dictionary(number_dictionary(2.0))).insert("depth",Value::Dictionary(number_dictionary(3.0))).insert("height",Value::Dictionary(number_dictionary(4.0)))).unwrap(),"solid").await;
    let input = neural_engine::ColdOwner::new(Dictionary::new().insert("brep",Value::Dictionary(source.into_inner())));
    let (edge,face) = geometry_session().with_kernel(|kernel| {
        let topology = kernel.deconstruct(&read_geometry(&input,"brep")?).map_err(|error|map_kernel_error(&error))?;
        let edge = topology.edges.iter().find(|handle|kernel.length(handle).is_ok_and(|length|(length-edge_length as f64).abs()<1e-8)).unwrap();
        let face = topology.faces.iter().find(|handle|kernel.area(handle).is_ok_and(|area|(area-face_area as f64).abs()<1e-8)).unwrap();
        Ok((kernel.label(edge).unwrap().to_string(),kernel.label(face).unwrap().to_string()))
    }).unwrap();
    let selected = registry.dispatch_cold("brep.brep",(*input).clone().insert("edgeLabels",Value::Dictionary(text_dictionary(serde_json::json!([edge,edge]).to_string()))).insert("faceLabels",Value::Dictionary(text_dictionary(serde_json::json!([face]).to_string()))).insert("sourceHandle",Value::Dictionary(text_dictionary("")))).unwrap();
    for (channel,label,expected,operation) in [("selectedEdges",edge.as_str(),edge_length as f64,"length"),("selectedFaces",face.as_str(),face_area as f64,"area")] {
        let list = selected.get(channel).and_then(Value::as_dictionary).expect("selected current topology list");
        assert_eq!(list_indices(list),vec![0]);
        let geometry = list.get("0").and_then(Value::as_dictionary).unwrap();
        geometry_session().with_kernel_read(|kernel| {
            let handle = read_geometry(&neural_engine::ColdOwner::new(Dictionary::new().insert("geometry",Value::Dictionary(geometry.clone()))),"geometry")?;
            assert_eq!(kernel.label(&handle).unwrap().to_string(),label);
            let value = if operation=="length" {kernel.length(&handle)} else {kernel.area(&handle)}.unwrap();
            assert!((value-expected).abs()<1e-8);
            Ok(())
        }).unwrap();
    }
    for text in fixture["invalidSelectors"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).chain(std::iter::once(fixture["missingLabel"].as_str().unwrap())) {
        let text = if text==fixture["missingLabel"].as_str().unwrap() {serde_json::json!([text]).to_string()} else {text.into()};
        assert!(registry.dispatch_cold("brep.brep",(*input).clone().insert("edgeLabels",Value::Dictionary(text_dictionary(text))).insert("faceLabels",Value::Dictionary(text_dictionary("[]"))).insert("sourceHandle",Value::Dictionary(text_dictionary("")))).is_err());
    }
    println!("[DEBUG] BRep scoped exact-label selection edgeLength={edge_length} faceArea={face_area} independentParry=true");
}

pub(crate) fn compact_evaluation_request(request:&str)->String {
    let mut value=semio_framework_pack_json::parse(request,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let object=value.as_object_mut().unwrap();
    object.insert("inputJson",semio_framework_pack_json::Value::from(""));object.insert("dependencyJson",semio_framework_pack_json::Value::from(""));object.insert("resume",semio_framework_pack_json::Value::from(true));semio_framework_pack_json::to_string(&value)
}

#[semio_framework_async_macros::async_test]
async fn brep_deconstruct_scopes_source_handles_across_collection_reorder() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let registry = neural_engine::ColdOwner::new(module_registry(geometry_session()));
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️component-label-selection/🔣️.json")).unwrap();
    let shape = |width| registry.dispatch_cold("brep.prim3d.box",Dictionary::new().insert("width",Value::Dictionary(number_dictionary(width))).insert("depth",Value::Dictionary(number_dictionary(3.0))).insert("height",Value::Dictionary(number_dictionary(4.0)))).unwrap();
    let selected = channel_payload(&shape(2.0),"solid").await;
    let other = channel_payload(&shape(5.0),"solid").await;
    let handle = selected.get("handle").and_then(Value::as_atom).and_then(Atom::as_str).unwrap();
    for case in fixtures["sourceScopes"]["cases"].as_array().unwrap() {
        let mut list = neural_engine::ColdDictionaryBuilder::from_dictionary(Dictionary::with_schema("list"));
        for (index, name) in case["order"].as_array().unwrap().iter().enumerate() { list.insert(index.to_string(),Value::Dictionary(if name == "selected" {(*selected).clone()} else {(*other).clone()})); }
        let request = Dictionary::new().insert("brep",Value::Dictionary(list.finish())).insert("edgeLabels",Value::Dictionary(text_dictionary("[]"))).insert("faceLabels",Value::Dictionary(text_dictionary("[]"))).insert("sourceHandle",Value::Dictionary(text_dictionary(handle)));
        let actual = registry.dispatch_cold("brep.brep",request);
        if case["error"] == true { assert!(actual.is_err(),"{}",case["name"]); continue; }
        let actual = actual.unwrap();
        let geometry = actual.get(&neural_engine::produced_channel_id("brep")).and_then(Value::as_dictionary).expect("scoped current geometry");
        assert_eq!(geometry.get("handle").and_then(Value::as_atom).and_then(Atom::as_str),Some(handle));
        let volume = geometry_session().with_kernel_read(|kernel| {
            let request = neural_engine::ColdOwner::new(Dictionary::new().insert("geometry",Value::Dictionary(geometry.clone())));
            let shape = read_geometry(&request,"geometry")?;
            kernel.volume(&shape).map_err(|error|map_kernel_error(&error))
        }).unwrap();
        assert!((volume - f64::from(parry3d::shape::Shape::mass_properties(&parry3d::shape::Cuboid::new(parry3d::math::Vector::new(1.0,1.5,2.0)),1.0).mass())).abs()<1e-8);
        assert_eq!(actual.get("sourceIndex").and_then(Value::as_dictionary).and_then(|value|value.get("value")).and_then(Value::as_atom).and_then(Atom::as_f64),case["index"].as_f64());
    }
    let oracle=parry3d::shape::Shape::mass_properties(&parry3d::shape::Cuboid::new(parry3d::math::Vector::new(1.0,1.5,2.0)),1.0).mass();
    assert_eq!(oracle,24.0);
    println!("[DEBUG] BRep source scope current=reordered exactHandle=true absent=refused ambiguous=refused parryVolume={oracle}");
}
