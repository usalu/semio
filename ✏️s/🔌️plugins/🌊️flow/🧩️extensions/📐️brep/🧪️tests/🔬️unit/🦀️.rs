use super::*;
use flow_extension_sdk::evaluate_json;
use neural_engine::{Atom, Value};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::Brep;
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
async fn test_serial() -> std::sync::MutexGuard<'static, ()> {
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

async fn channel_payload(out: &Dictionary, channel: &str) -> Dictionary {
    out.get(channel).and_then(|v| v.as_dictionary()).cloned().expect("channel payload")
}

async fn reset_test_kernel() {
    if let Ok(mut guard) = kernel().write() {
        *guard = Box::new(Brep::new());
    }
    if let Ok(mut cache) = mesh_cache().lock() {
        cache.clear();
    }
}

#[semio_framework_async_macros::async_test]
async fn box_emits_geometry_handle() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let input = Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(3.0))).insert("height", Value::Dictionary(number_dictionary(4.0)));
    let out = reg.dispatch("brep.prim3d.box", &input).unwrap();
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
    register(&mut reg).await;
    let out = reg.dispatch("brep.curve.line", &Dictionary::new().insert("start", Value::Dictionary(point(0.0, 0.0, 0.0).await)).insert("end", Value::Dictionary(point(1.0, 0.0, 0.0).await))).unwrap();
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
    register(&mut reg).await;
    let solid = channel_payload(
        &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(3.0))).insert("height", Value::Dictionary(number_dictionary(4.0)))).unwrap(),
        "solid",
    )
    .await;
    let dwg = channel_payload(&reg.dispatch("brep.io.exportDwg", &Dictionary::new().insert("geometry", Value::Dictionary(solid)).insert("deflection", Value::Dictionary(number_dictionary(0.1)))).unwrap(), "dwg").await;
    let data = dwg.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("dwg base64").to_string();
    assert!(!data.is_empty());

    let imported = channel_payload(&reg.dispatch("brep.io.importDwg", &Dictionary::new().insert("data", Value::Dictionary(text_dictionary(data))).insert("tolerance", Value::Dictionary(number_dictionary(0.1)))).unwrap(), "geometry").await;
    assert_eq!(imported.schema(), Some("geometry"));
    assert!(imported.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).is_some());
}

async fn box_handle(reg: &mut Registry) -> String {
    let solid = channel_payload(
        &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(3.0))).insert("height", Value::Dictionary(number_dictionary(4.0)))).unwrap(),
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
    register(&mut reg).await;
    let handle = box_handle(&mut reg).await;
    let exported = pack::json::parse(&export_solid_json(&[handle], "step", 0.1)).unwrap();
    assert!(exported.get("error").is_none(), "{exported:?}");
    assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(false));
    let data = exported.get("data").and_then(|value| value.as_str()).expect("step text").to_string();
    assert!(!data.is_empty());
    let imported = pack::json::parse(&import_solid_json("step", &data, 0.1)).unwrap();
    assert!(imported.get("error").is_none(), "{imported:?}");
    assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn obj_export_import_round_trips_a_box() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let handle = box_handle(&mut reg).await;
    let exported = pack::json::parse(&export_solid_json(&[handle], "obj", 0.1)).unwrap();
    assert!(exported.get("error").is_none(), "{exported:?}");
    let data = exported.get("data").and_then(|value| value.as_str()).expect("obj text").to_string();
    assert!(data.contains('v'));
    let imported = pack::json::parse(&import_solid_json("obj", &data, 0.1)).unwrap();
    assert!(imported.get("error").is_none(), "{imported:?}");
    assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn stl_export_import_round_trips_a_box() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let handle = box_handle(&mut reg).await;
    let exported = pack::json::parse(&export_solid_json(&[handle], "stl", 0.1)).unwrap();
    assert!(exported.get("error").is_none(), "{exported:?}");
    assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(true));
    let data = exported.get("data").and_then(|value| value.as_str()).expect("stl base64").to_string();
    assert!(!data.is_empty());
    let imported = pack::json::parse(&import_solid_json("stl", &data, 0.1)).unwrap();
    assert!(imported.get("error").is_none(), "{imported:?}");
    assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn glb_export_import_round_trips_a_box_through_the_mesh_bridge() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let handle = box_handle(&mut reg).await;
    let exported = pack::json::parse(&export_solid_json(&[handle], "glb", 0.1)).unwrap();
    assert!(exported.get("error").is_none(), "{exported:?}");
    assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(true));
    let data = exported.get("data").and_then(|value| value.as_str()).expect("glb base64").to_string();
    assert!(!data.is_empty());
    let imported = pack::json::parse(&import_solid_json("glb", &data, 0.1)).unwrap();
    assert!(imported.get("error").is_none(), "{imported:?}");
    assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn export_solid_json_rejects_unsupported_format() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let handle = box_handle(&mut reg).await;
    let exported = pack::json::parse(&export_solid_json(&[handle], "fbx", 0.1)).unwrap();
    assert!(exported.get("error").is_some());
}

#[semio_framework_async_macros::async_test]
async fn extrude_and_area() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let wire = channel_payload(&reg.dispatch("brep.curve.rectangle", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0)))).unwrap(), "wire").await;
    let face = channel_payload(&reg.dispatch("brep.surf.planarFaceWire", &Dictionary::new().insert("wire", Value::Dictionary(wire))).unwrap(), "face").await;
    let solid = channel_payload(&reg.dispatch("brep.sweep.extrude", &Dictionary::new().insert("face", Value::Dictionary(face)).insert("vector", Value::Dictionary(vector(0.0, 0.0, 3.0).await))).unwrap(), "solid").await;
    assert_eq!(solid.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("solid"));
    let area = channel_payload(&reg.dispatch("brep.measure.area", &Dictionary::new().insert("geometry", Value::Dictionary(solid))).unwrap(), "area").await;
    assert_eq!(area.schema(), Some("number"));
    let value = area.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
    assert!(value > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn extrude_curve_wire_uses_vector_magnitude() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let wire = channel_payload(&reg.dispatch("brep.curve.rectangle", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0)))).unwrap(), "wire").await;
    let solid = channel_payload(&reg.dispatch("brep.solid.extrude", &Dictionary::new().insert("wire", Value::Dictionary(wire)).insert("vector", Value::Dictionary(vector(0.0, 0.0, 4.0).await))).unwrap(), "solid").await;
    assert_eq!(solid.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("solid"));
    let volume = channel_payload(&reg.dispatch("brep.measure.volume", &Dictionary::new().insert("geometry", Value::Dictionary(solid))).unwrap(), "volume").await;
    let value = volume.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
    assert!((value - 16.0).abs() < 1e-3);
}

#[semio_framework_async_macros::async_test]
async fn fillet_translate_chain() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let box_out = channel_payload(
        &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0)))).unwrap(),
        "solid",
    )
    .await;
    let fillet_out = channel_payload(&reg.dispatch("brep.solid.fillet", &Dictionary::new().insert("geometry", Value::Dictionary(box_out)).insert("radius", Value::Dictionary(number_dictionary(0.1)))).unwrap(), "solid").await;
    let moved = channel_payload(&reg.dispatch("brep.xform.translate", &Dictionary::new().insert("geometry", Value::Dictionary(fillet_out)).insert("offset", Value::Dictionary(vector(1.0, 0.0, 0.0).await))).unwrap(), "geometry").await;
    assert_eq!(moved.schema(), Some("geometry"));
}

#[semio_framework_async_macros::async_test]
async fn manifest_lists_brep_operators() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let json = build_manifest_json("brep", "Brep", "0.3.0", &neural_engine::ColdOwner::new(module_registry().await), vec!["onStartup".into()], vec![], vec![], vec![]);
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
    let reg = module_registry().await;
    let json_number = |value: f64| pack::json::object([("$schema".to_string(), pack::json::Value::from("number")), ("value".to_string(), pack::json::Value::from(value))]);
    let input_json = pack::json::to_string(&pack::json::object([("width".to_string(), json_number(1.0)), ("depth".to_string(), json_number(1.0)), ("height".to_string(), json_number(1.0))]));
    let out_json = evaluate_json(&reg, "brep.prim3d.box", &input_json);
    let out = pack::json::parse(&out_json).unwrap();
    assert_eq!(out.get("solid").and_then(|value| value.get("$schema")).and_then(pack::json::Value::as_str), Some("geometry"));
}

#[semio_framework_async_macros::async_test]
async fn retain_geometry_handles_sweeps_orphaned_shapes() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let box_out = channel_payload(
        &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0)))).unwrap(),
        "solid",
    )
    .await;
    let orphan = channel_payload(
        &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0)))).unwrap(),
        "solid",
    )
    .await;
    let live_handle = box_out.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap().to_string();
    let orphan_handle = orphan.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap().to_string();
    retain_geometry_handles(std::slice::from_ref(&live_handle));
    let live_mesh = tessellate_geometry(&live_handle, 0.1).expect("live tessellation");
    assert!(!live_mesh.positions.is_empty());
    assert!(tessellate_geometry(&orphan_handle, 0.1).is_err());
}

#[semio_framework_async_macros::async_test]
async fn tessellate_geometry_is_memoized() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let box_out = channel_payload(
        &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0)))).unwrap(),
        "solid",
    )
    .await;
    let handle = box_out.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap();
    let first = tessellate_geometry(handle, 0.1).expect("mesh");
    let second = tessellate_geometry(handle, 0.1).expect("mesh");
    assert_eq!(first.positions, second.positions);
    assert!(!first.positions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn brep_component_deconstructs_solid_topology() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let solid = channel_payload(
        &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0)))).unwrap(),
        "solid",
    )
    .await;
    let deconstructed = reg.dispatch("brep.brep", &Dictionary::new().insert("brep", Value::Dictionary(solid))).unwrap();
    let vertices = deconstructed.get("vertex").and_then(Value::as_dictionary).expect("vertex list");
    let edges = deconstructed.get("edge").and_then(Value::as_dictionary).expect("edge list");
    let faces = deconstructed.get("face").and_then(Value::as_dictionary).expect("face list");
    assert_eq!(list_indices(vertices).len(), 8);
    assert_eq!(list_indices(edges).len(), 12);
    assert_eq!(list_indices(faces).len(), 6);
}

#[semio_framework_async_macros::async_test]
async fn schema_component_deconstructs_geometry() {
    let mut reg = Registry::new();
    register(&mut reg).await;
    let geometry = Dictionary::with_schema("geometry").insert("handle", Value::Atom(Atom::String("solid-1".into()))).insert("kind", Value::Atom(Atom::String("solid".into())));
    let out = reg.dispatch("brep.geometry", &Dictionary::new().insert("geometry", Value::Dictionary(geometry.clone()))).unwrap();
    assert_eq!(out.get("handle").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("solid-1"));
    assert_eq!(out.get("kind").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("solid"));
}

#[semio_framework_async_macros::async_test]
async fn extension_bundle_extends_flow_and_evaluates_box() {
    use semio_framework_plugin::{extension_activate, extension_invoke, extension_manifest, install_extension_bundle, ExtensionBundle};

    let _serial = test_serial().await;
    reset_test_kernel().await;
    let manifest_json = extension_manifest_json().await;
    let flow_topic = flow_extension_sdk::flow_extension_topic_contribution("flow-play", "brep", "Brep", "brep", &manifest_json);
    let procedural3d_topic = flow_extension_sdk::flow_extension_topic_contribution("procedural3d-play", "brep", "Brep", "brep", &manifest_json);
    let evaluation_registry = neural_engine::ColdOwner::new(module_registry().await);
    let bundle = ExtensionBundle::new("flow-extension-brep", "Brep", "0.3.0")
        .extends("flow")
        .contributes_topic(flow_topic.topic, flow_topic.payload)
        .contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload)
        .handler("evaluate", move |req| Ok(flow_extension_sdk::evaluate_invoke_json(&evaluation_registry, req).unwrap()));
    install_extension_bundle(bundle).await;
    extension_activate().await.unwrap();
    assert_eq!(extension_manifest().await.extension_id, "flow-extension-brep");
    let json_number = |value: f64| pack::json::object([("$schema".to_string(), pack::json::Value::from("number")), ("value".to_string(), pack::json::Value::from(value))]);
    let input_json = pack::json::to_string(&pack::json::object([("width".to_string(), json_number(1.0)), ("depth".to_string(), json_number(1.0)), ("height".to_string(), json_number(1.0))]));
    let req =
        pack::json::to_string(&pack::json::object([("operatorId".to_string(), pack::json::Value::from("brep.prim3d.box")), ("inputJson".to_string(), pack::json::Value::from(input_json)), ("nodeHash".to_string(), pack::json::Value::from(1_i64))]));
    // ⏱️ `evaluate` answers the BUDGET envelope, not a bare out dictionary — a primitive finishes
    // inside its first round trip, so this one is `done` with its output inside
    // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`).
    let envelope = pack::json::parse_bytes(&extension_invoke("evaluate", req.as_bytes()).await.unwrap()).unwrap();
    assert_eq!(envelope.get("done").and_then(pack::json::Value::as_bool), Some(true));
    let out = pack::json::parse(envelope.get("outputJson").and_then(pack::json::Value::as_str).unwrap()).unwrap();
    assert_eq!(out.get("solid").and_then(|value| value.get("$schema")).and_then(pack::json::Value::as_str), Some("geometry"));
}

fn number_value(dict: &Dictionary) -> f64 {
    dict.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).expect("number channel value")
}

async fn box_of(reg: &mut Registry, size: f64) -> Dictionary {
    channel_payload(
        &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(size))).insert("depth", Value::Dictionary(number_dictionary(size))).insert("height", Value::Dictionary(number_dictionary(size))))
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
    register(&mut reg).await;
    let surface = channel_payload(&reg.dispatch("brep.surf.plane", &Dictionary::new().insert("origin", Value::Dictionary(point(0.0, 0.0, 0.0).await)).insert("normal", Value::Dictionary(vector(0.0, 0.0, 1.0).await))).unwrap(), "surface").await;
    let evaluated = channel_payload(
        &reg.dispatch("brep.eval.surfPoint", &Dictionary::new().insert("surface", Value::Dictionary(surface.clone())).insert("u", Value::Dictionary(number_dictionary(1.0))).insert("v", Value::Dictionary(number_dictionary(-2.0)))).unwrap(),
        "point",
    )
    .await;
    let z = evaluated.get("z").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
    assert!(z.abs() < 1e-9, "a point on the z=0 plane must have z=0 regardless of (u, v), got z={z}");
    let normal = channel_payload(
        &reg.dispatch("brep.eval.surfNormal", &Dictionary::new().insert("surface", Value::Dictionary(surface)).insert("u", Value::Dictionary(number_dictionary(0.0))).insert("v", Value::Dictionary(number_dictionary(0.0)))).unwrap(),
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
    register(&mut reg).await;
    let a = box_of(&mut reg, 2.0).await;
    let b_raw = box_of(&mut reg, 2.0).await;
    let b = channel_payload(&reg.dispatch("brep.xform.translate", &Dictionary::new().insert("geometry", Value::Dictionary(b_raw)).insert("offset", Value::Dictionary(vector(1.0, 0.0, 0.0).await))).unwrap(), "geometry").await;

    let fused = channel_payload(&reg.dispatch("brep.bool.fuse", &Dictionary::new().insert("a", Value::Dictionary(a.clone())).insert("b", Value::Dictionary(b.clone()))).unwrap(), "solid").await;
    let fused_volume = number_value(&channel_payload(&reg.dispatch("brep.measure.volume", &Dictionary::new().insert("geometry", Value::Dictionary(fused))).unwrap(), "volume").await);
    assert!((8.0..16.0).contains(&fused_volume), "fused volume {fused_volume} should exceed either box's own 8.0 but stay below the disjoint sum 16.0");

    let cut = channel_payload(&reg.dispatch("brep.bool.cut", &Dictionary::new().insert("a", Value::Dictionary(a.clone())).insert("b", Value::Dictionary(b.clone()))).unwrap(), "solid").await;
    let cut_volume = number_value(&channel_payload(&reg.dispatch("brep.measure.volume", &Dictionary::new().insert("geometry", Value::Dictionary(cut))).unwrap(), "volume").await);
    assert!((0.0..8.0).contains(&cut_volume), "cut volume {cut_volume} should be less than the untouched box's 8.0");

    let intersected = channel_payload(&reg.dispatch("brep.bool.intersect", &Dictionary::new().insert("a", Value::Dictionary(a)).insert("b", Value::Dictionary(b))).unwrap(), "solid").await;
    let intersect_volume = number_value(&channel_payload(&reg.dispatch("brep.measure.volume", &Dictionary::new().insert("geometry", Value::Dictionary(intersected))).unwrap(), "volume").await);
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
    register(&mut reg).await;
    let solid = box_of(&mut reg, 1.0).await;
    let rotated = channel_payload(
        &reg.dispatch(
            "brep.xform.rotateAbout",
            &Dictionary::new()
                .insert("geometry", Value::Dictionary(solid))
                .insert("origin", Value::Dictionary(point(5.0, 0.0, 0.0).await))
                .insert("axis", Value::Dictionary(vector(0.0, 0.0, 1.0).await))
                .insert("angle", Value::Dictionary(number_dictionary(std::f64::consts::PI))),
        )
        .unwrap(),
        "geometry",
    )
    .await;
    let center = channel_payload(&reg.dispatch("brep.measure.centerOfMass", &Dictionary::new().insert("geometry", Value::Dictionary(rotated))).unwrap(), "center").await;
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
    register(&mut reg).await;
    let curve = channel_payload(&reg.dispatch("brep.curve.line", &Dictionary::new().insert("start", Value::Dictionary(point(0.0, 0.0, 0.0).await)).insert("end", Value::Dictionary(point(10.0, 0.0, 0.0).await))).unwrap(), "curve").await;
    let out = reg.dispatch("brep.eval.curveClosestParameter", &Dictionary::new().insert("curve", Value::Dictionary(curve)).insert("point", Value::Dictionary(point(4.0, 3.0, 0.0).await))).unwrap();
    let distance = number_value(&channel_payload(&out, "distance").await);
    assert!((distance - 3.0).abs() < 1e-9, "closest distance from (4,3,0) to the segment along the x axis should be 3, got {distance}");
    let closest = channel_payload(&out, "point").await;
    assert!((closest.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap() - 4.0).abs() < 1e-9);

    let surface = channel_payload(&reg.dispatch("brep.surf.plane", &Dictionary::new().insert("origin", Value::Dictionary(point(0.0, 0.0, 0.0).await)).insert("normal", Value::Dictionary(vector(0.0, 0.0, 1.0).await))).unwrap(), "surface").await;
    let uv_out = reg.dispatch("brep.eval.surfaceClosestUv", &Dictionary::new().insert("surface", Value::Dictionary(surface)).insert("point", Value::Dictionary(point(1.0, 1.0, 5.0).await))).unwrap();
    let uv_distance = number_value(&channel_payload(&uv_out, "distance").await);
    assert!((uv_distance - 5.0).abs() < 1e-6, "closest distance from (1,1,5) to the z=0 plane should be 5, got {uv_distance}");
}

/// 🐚️ Topology family — the new wave-1 handle capabilities: shells, compound/explode, and
/// the persistent label round trip.
#[semio_framework_async_macros::async_test]
async fn topology_family_shells_compound_explode_and_label() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let mut reg = Registry::new();
    register(&mut reg).await;
    let box_a = box_of(&mut reg, 1.0).await;
    let box_b = box_of(&mut reg, 2.0).await;
    let handle_a = box_a.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("box a handle").to_string();
    let handle_b = box_b.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("box b handle").to_string();

    let shells_out = reg.dispatch("brep.topology.shells", &Dictionary::new().insert("solid", Value::Dictionary(box_a.clone()))).unwrap();
    let shells = shells_out.get("shells").and_then(Value::as_dictionary).expect("shells list");
    assert_eq!(list_indices(shells).len(), 1, "a simple box has exactly one outer shell");

    let solids_in = topology_list("geometry", vec![GeometryHandle(handle_a), GeometryHandle(handle_b)]);
    let compound_out = reg.dispatch("brep.topology.compound", &Dictionary::new().insert("solids", Value::Dictionary(solids_in))).unwrap();
    let compound = channel_payload(&compound_out, "compound").await;
    assert_eq!(compound.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("compound"));

    let exploded_out = reg.dispatch("brep.topology.explode", &Dictionary::new().insert("compound", Value::Dictionary(compound))).unwrap();
    let solids_out = exploded_out.get("solids").and_then(Value::as_dictionary).expect("solids list");
    assert_eq!(list_indices(solids_out).len(), 2, "exploding must recover both original solids");

    let label_out = reg.dispatch("brep.topology.label", &Dictionary::new().insert("geometry", Value::Dictionary(box_a))).unwrap();
    let label = number_value(&channel_payload(&label_out, "label").await);
    assert!(label >= 0.0);
}

/// 🎯️ `q`'s tag round-trips through the live registry — the contract's `operation_quality`
/// stays the single source of truth: nothing here hardcodes an `OpQuality` a second time.
#[semio_framework_async_macros::async_test]
async fn operation_quality_tags_match_the_kernel_contract() {
    let reg = module_registry().await;
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
    let known: std::collections::HashSet<&str> = BREP_KERNEL_OPERATIONS.iter().copied().collect();
    for method in &seen {
        assert!(known.contains(method), "{method:?} in NODE_KERNEL_METHOD/INTENTIONALLY_UNEXPOSED is not a real BrepKernel method");
    }
    for operation in BREP_KERNEL_OPERATIONS {
        assert!(seen.contains(operation), "BrepKernel method {operation:?} has neither a flow node nor an INTENTIONALLY_UNEXPOSED entry");
    }
    assert_eq!(seen.len(), BREP_KERNEL_OPERATIONS.len());
}

/// ⏱️ The BUDGET law of the `evaluate` capability — its own module because the law is about how a
/// long set operation YIELDS, not about what any one operator computes. Mounted INSIDE this module
/// so it shares the process-wide kernel serialisation and reset helpers above
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[path = "../🔬️evaluate-budget/🦀️.rs"]
mod evaluate_budget;
