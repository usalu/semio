        let _serial = test_serial();
        reset_test_kernel();
        let reg = module_registry();
        let input = Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0)));
        let out_json = evaluate_json(&reg, "brep.prim3d.box", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(channel_payload(&out, "solid").schema(), Some("geometry"));
    }

    #[test]
    fn retain_geometry_handles_sweeps_orphaned_shapes() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let box_out = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0))))
                .unwrap(),
            "solid",
        );
        let orphan = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0))))
                .unwrap(),
            "solid",
        );
        let live_handle = box_out.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap().to_string();
        let orphan_handle = orphan.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap().to_string();
        retain_geometry_handles(std::slice::from_ref(&live_handle));
        let live_mesh = tessellate_geometry(&live_handle, 0.1).expect("live tessellation");
        assert!(!live_mesh.positions.is_empty());
        assert!(tessellate_geometry(&orphan_handle, 0.1).is_err());
    }

    #[test]
    fn tessellate_geometry_is_memoized() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let box_out = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0))))
                .unwrap(),
            "solid",
        );
        let handle = box_out.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap();
        let first = tessellate_geometry(handle, 0.1).expect("mesh");
        let second = tessellate_geometry(handle, 0.1).expect("mesh");
        assert_eq!(first.positions, second.positions);
        assert!(!first.positions.is_empty());
    }

    #[test]
    fn brep_component_deconstructs_solid_topology() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let solid = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0))))
                .unwrap(),
            "solid",
        );
        let deconstructed = reg.dispatch("brep.brep", &Dictionary::new().insert("brep", Value::Dictionary(solid))).unwrap();
        let vertices = deconstructed.get("vertex").and_then(Value::as_dictionary).expect("vertex list");
        let edges = deconstructed.get("edge").and_then(Value::as_dictionary).expect("edge list");
        let faces = deconstructed.get("face").and_then(Value::as_dictionary).expect("face list");
        assert_eq!(list_indices(vertices).len(), 8);
        assert_eq!(list_indices(edges).len(), 12);
        assert_eq!(list_indices(faces).len(), 6);
    }

    #[test]
    fn schema_component_deconstructs_geometry() {
        let mut reg = Registry::new();
        register(&mut reg);
        let geometry = Dictionary::with_schema("geometry").insert("handle", Value::Atom(Atom::String("solid-1".into()))).insert("kind", Value::Atom(Atom::String("solid".into())));
        let out = reg.dispatch("brep.geometry", &Dictionary::new().insert("geometry", Value::Dictionary(geometry.clone()))).unwrap();
        assert_eq!(out.get("handle").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("solid-1"));
        assert_eq!(out.get("kind").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("solid"));
    }
}
// #endregion 🔖️Tests

// #region 🔖️Tessellation
/// 🧹️ Retains only geometry handles referenced by the current evaluation outputs.
pub fn retain_geometry_handles(live: &[String]) {
    let live_set: HashSet<String> = live.iter().cloned().collect();
    if let Ok(mut guard) = kernel().write() {
        block_on(guard.retain(&live_set));
    }
    evict_mesh_cache_for_handles(live);
}

/// 🧊️ Tessellates a geometry handle owned by the in-process brep kernel into preview `MeshData`.
pub fn tessellate_geometry(handle: &str, tolerance: f64) -> Result<semio_framework_core::MeshData, String> {
    let key = (handle.to_string(), tolerance.to_bits());
    if let Ok(cache) = mesh_cache().lock() {
        if let Some(cached) = cache.get(&key) {
            return Ok(cached.clone());
        }
    }
    let guard = kernel().read().map_err(|_| "brep kernel lock poisoned".to_string())?;
    let mesh = {
        let geometry = GeometryHandle(handle.to_string());
        block_on(guard.tessellate(&geometry, tolerance)).map_err(|error| error.to_string())?
    };
    let data = semio_s_3d::brep::kernel::mesh_data_from_mesh_transfer(&mesh);
    if let Ok(mut cache) = mesh_cache().lock() {
        cache.insert(key, data.clone());
    }
    Ok(data)
}

fn tessellate_geometry_json_for_wasm(handle: &str, tolerance: f64) -> String {
    match tessellate_geometry(handle, tolerance) {
        Ok(mesh) => serde_json::to_string(&mesh).unwrap_or_else(|_| serde_json::json!({ "error": "mesh encode failed" }).to_string()),
        Err(error) => serde_json::json!({ "error": error }).to_string(),
    }
}

/// 🗑️ Disposes a geometry handle owned by the in-process brep kernel.
pub fn dispose_geometry(handle: &str) {
    evict_mesh_cache_for_handle(handle);
    if let Ok(mut kernel) = kernel().write() {
        block_on(kernel.dispose(&GeometryHandle(handle.to_string())));
    }
}
// #endregion 🔖️Tessellation

// #region ⚠️ Errors
/// 🧯️ Internal error type for the brep module's media import/export bridging helpers (`export_solid_json`/`import_solid_json` still surface it flattened to JSON `{"error"}` strings, matching prior behaviour byte-for-byte).
#[derive(Debug, thiserror::Error)]
enum BrepModuleError {
    #[error("brep kernel lock poisoned")]
    LockPoisoned,
    #[error(transparent)]
    Kernel(#[from] semio_s_3d::brep::engine::BrepError),
    #[error(transparent)]
    Codec(#[from] neural_engine::EvalError),
    #[error("{0}")]
    Mesh(String),
    #[error("unsupported solid export format: {0}")]
    UnsupportedExportFormat(String),
    #[error("unsupported solid import format: {0}")]
    UnsupportedImportFormat(String),
}
// #endregion ⚠️ Errors

// #region 🔖️MediaExport
/// 📤️ Exports geometry handles owned by the in-process brep kernel to a solid/mesh interchange format. STEP/OBJ/STL go through the kernel's native codecs; GLB bridges through tessellation (`tessellate` → merged `MeshData` → `GlbExporter`) since the kernel has no native GLB writer wired here. Binary formats are base64-encoded. Returns `{"data","binary","format"}` or `{"error"}` JSON.
pub fn export_solid_json(handles: &[String], format: &str, deflection: f64) -> String {
    let shapes: Vec<GeometryHandle> = handles.iter().cloned().map(GeometryHandle).collect();
    let outcome: Result<(String, bool), BrepModuleError> = kernel().read().map_err(|_| BrepModuleError::LockPoisoned).and_then(|guard| {
        let guard = &**guard;
        match format {
            "step" => block_on(guard.export_step(&shapes)).map(|text| (text, false)).map_err(BrepModuleError::from),
            "obj" => block_on(guard.export_obj(&shapes, deflection)).map(|text| (text, false)).map_err(BrepModuleError::from),
            "stl" => block_on(guard.export_stl(&shapes, deflection)).map(|data| (encode_base64(&data), true)).map_err(BrepModuleError::from),
            "glb" => export_glb_via_tessellation(guard, &shapes, deflection).map(|data| (encode_base64(&data), true)),
            other => Err(BrepModuleError::UnsupportedExportFormat(other.to_string())),
        }
    });
    match outcome {
        Ok((data, binary)) => serde_json::json!({ "data": data, "binary": binary, "format": format }).to_string(),
        Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
    }
}

/// 🧊️ Bridges GLB export through mesh tessellation: tessellates every shape, merges the resulting triangle soup into one `MeshData`, and encodes it with the shared `GlbExporter` mesh codec (the same codec every other app uses for GLB).
fn export_glb_via_tessellation(kernel: &dyn BrepKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepModuleError> {
    use semio_framework_core::MeshExporter;
    let mut merged = semio_framework_core::MeshData::default();
    for shape in shapes {
        let transfer = block_on(kernel.tessellate(shape, deflection))?;
        let mesh = semio_s_3d::brep::kernel::mesh_data_from_mesh_transfer(&transfer);
        let offset = (merged.positions.len() / 3) as u32;
        merged.positions.extend(mesh.positions);
        merged.normals.extend(mesh.normals);
        merged.indices.extend(mesh.indices.into_iter().map(|index| index + offset));
    }
    semio_framework_core::GlbExporter.export(&merged).map_err(BrepModuleError::Mesh)
}

/// 📥️ Imports STEP/OBJ/STL solid data (or GLB mesh data, bridged through the kernel's OBJ ingestion since it has no raw-mesh entry point) into the in-process kernel. STEP/OBJ expect UTF-8 text in `data`; STL/GLB expect base64-encoded bytes. Returns `{"handles":[...]}` or `{"error"}` JSON.
pub fn import_solid_json(format: &str, data: &str, tolerance: f64) -> String {
    let outcome: Result<Vec<String>, BrepModuleError> = kernel().write().map_err(|_| BrepModuleError::LockPoisoned).and_then(|mut guard| {
        let guard = &mut **guard;
        match format {
            "step" => block_on(guard.import_step(data)).map(|handles| handles.into_iter().map(|handle| handle.0).collect()).map_err(BrepModuleError::from),
            "obj" => block_on(guard.import_obj(data, tolerance)).map(|handle| vec![handle.0]).map_err(BrepModuleError::from),
            "stl" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| block_on(guard.import_stl(&bytes, tolerance)).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)),
            "glb" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| import_glb_via_tessellation(guard, &bytes, tolerance)),
            other => Err(BrepModuleError::UnsupportedImportFormat(other.to_string())),
        }
    });
    match outcome {
        Ok(handles) => serde_json::json!({ "handles": handles }).to_string(),
        Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
    }
}

/// 🧊️ Bridges GLB import through the mesh codec: decodes GLB bytes to `MeshData` via `GlbImporter`, re-encodes it as OBJ text, and ingests that through the kernel's own OBJ importer.
fn import_glb_via_tessellation(kernel: &mut dyn BrepKernel, bytes: &[u8], tolerance: f64) -> Result<Vec<String>, BrepModuleError> {
    use semio_framework_core::MeshImporter;
    let mesh = semio_framework_core::GlbImporter.import(bytes).map_err(BrepModuleError::Mesh)?;
    let obj_text = semio_framework_core::mesh_to_obj(&mesh, "glb-import");
    block_on(kernel.import_obj(&obj_text, tolerance)).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)
}
// #endregion 🔖️MediaExport

// #region 🔖️WasmExt
#[cfg(all(target_arch = "wasm32", feature = "standalone-wasm"))]
mod wasm_ext {
    use super::module_registry;
    use flow_extension_sdk::{build_manifest_json, command_json, evaluate_json};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn manifest() -> String {
        build_manifest_json("brep", "Brep", "0.3.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![])
    }

    #[wasm_bindgen]
    pub fn evaluate(kind_id: &str, input_json: &str) -> String {
        evaluate_json(&module_registry(), kind_id, input_json)
    }

    #[wasm_bindgen]
    pub fn command(command_id: &str, args_json: &str) -> String {
        command_json(command_id, args_json)
    }

    #[wasm_bindgen]
    pub fn tessellate(handle: &str, tolerance: f64) -> String {
        super::tessellate_geometry_json_for_wasm(handle, tolerance)
    }

    #[wasm_bindgen]
    pub fn dispose(handle: &str) {
        super::dispose_geometry(handle);
    }

    #[wasm_bindgen]
    pub fn activate() {}

    #[wasm_bindgen]
    pub fn deactivate() {}
}
// #endregion 🔖️WasmExt
