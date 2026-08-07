//! 🔷️ Flow brep module: native geometry operators.

use base64::Engine;
use semio_s_3d::brep::kernel::Brep;
use semio_s_3d::brep::engine::{block_on, BrepKernel, GeometryHandle, GeometryKind, ParamDomain, PointClassification, Vec3};
use neural_engine::{channel_output, Atom, Cardinality, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock, RwLock};

static KERNEL: OnceLock<RwLock<Box<dyn BrepKernel + Send + Sync>>> = OnceLock::new();
static MESH_CACHE: OnceLock<Mutex<HashMap<(String, u64), semio_framework_core::MeshData>>> = OnceLock::new();

fn kernel() -> &'static RwLock<Box<dyn BrepKernel + Send + Sync>> {
    KERNEL.get_or_init(|| RwLock::new(Box::new(Brep::new())))
}

fn mesh_cache() -> &'static Mutex<HashMap<(String, u64), semio_framework_core::MeshData>> {
    MESH_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn evict_mesh_cache_for_handles(handles: &[String]) {
    if handles.is_empty() {
        if let Ok(mut cache) = mesh_cache().lock() {
            cache.clear();
        }
        return;
    }
    let live: HashSet<&str> = handles.iter().map(String::as_str).collect();
    if let Ok(mut cache) = mesh_cache().lock() {
        cache.retain(|(handle, _), _| live.contains(handle.as_str()));
    }
}

fn evict_mesh_cache_for_handle(handle: &str) {
    if let Ok(mut cache) = mesh_cache().lock() {
        cache.retain(|(cached_handle, _), _| cached_handle != handle);
    }
}

// #region 🔖️Helpers
fn with_kernel<T>(f: impl FnOnce(&mut dyn BrepKernel) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut guard = kernel().write().map_err(|_| EvalError::InvalidInput("brep kernel lock poisoned".into()))?;
    f(&mut **guard)
}

/// 🔓️ Read-only kernel access — lets concurrent queries (tessellate, volume, closest-point, …)
/// proceed in parallel with each other while still serializing against mutating operations.
fn with_kernel_read<T>(f: impl FnOnce(&dyn BrepKernel) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let guard = kernel().read().map_err(|_| EvalError::InvalidInput("brep kernel lock poisoned".into()))?;
    f(&**guard)
}

fn kind_label(kind: GeometryKind) -> &'static str {
    match kind {
        GeometryKind::Vertex => "vertex",
        GeometryKind::Edge => "edge",
        GeometryKind::Wire => "wire",
        GeometryKind::Face => "face",
        GeometryKind::Shell => "shell",
        GeometryKind::Solid => "solid",
        GeometryKind::Compound => "compound",
        GeometryKind::Curve => "curve",
        GeometryKind::Surface => "surface",
    }
}

fn geometry_dict(kernel: &dyn BrepKernel, handle: &GeometryHandle) -> Result<Dictionary, EvalError> {
    let kind = block_on(kernel.kind(handle)).map_err(map_kernel_error)?;
    Ok(Dictionary::with_schema("geometry").insert("handle", Value::Atom(Atom::String(handle.as_str().to_string()))).insert("kind", Value::Atom(Atom::String(kind_label(kind).into()))))
}

fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn point_dictionary(point: Vec3) -> Dictionary {
    Dictionary::with_schema("point").insert("x", Value::Atom(Atom::Decimal(point[0]))).insert("y", Value::Atom(Atom::Decimal(point[1]))).insert("z", Value::Atom(Atom::Decimal(point[2])))
}

fn vector_channel(id: &str, operator_id: &str, default: Vec3) -> ChannelSpec {
    ChannelSpec::requires(id, &["math.vector", operator_id]).with_default(Value::Dictionary(vector_dictionary(default)))
}

fn vector_dictionary(vector: Vec3) -> Dictionary {
    Dictionary::with_schema("vector").insert("x", Value::Atom(Atom::Decimal(vector[0]))).insert("y", Value::Atom(Atom::Decimal(vector[1]))).insert("z", Value::Atom(Atom::Decimal(vector[2])))
}

fn text_dictionary(value: impl Into<String>) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value.into())))
}

fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_geometry(input: &Dictionary, key: &str) -> Result<GeometryHandle, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    let handle = dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).ok_or_else(|| EvalError::MissingInput(format!("{key}.handle")))?;
    Ok(GeometryHandle(handle.to_string()))
}

fn read_optional_geometry(input: &Dictionary, key: &str) -> Option<GeometryHandle> {
    input.get(key).and_then(|value| value.as_dictionary()).and_then(|dict| dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(|handle| GeometryHandle(handle.to_string())))
}

fn read_xyz_dict(dict: &Dictionary) -> Result<Vec3, EvalError> {
    Ok([
        dict.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0),
        dict.get("y").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0),
        dict.get("z").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0),
    ])
}

fn read_xyz(input: &Dictionary, key: &str) -> Result<Vec3, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    read_xyz_dict(dict)
}

fn read_list(input: &Dictionary, key: &str) -> Result<Dictionary, EvalError> {
    input.get(key).and_then(|value| value.as_dictionary()).filter(|dict| dict.schema() == Some("list")).cloned().ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn list_indices(list: &Dictionary) -> Vec<usize> {
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    indices
}

fn read_point_list(input: &Dictionary, key: &str) -> Result<Vec<Vec3>, EvalError> {
    let list = read_list(input, key)?;
    list_indices(&list)
        .into_iter()
        .map(|index| {
            let dict = list.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}] must be a point")))?;
            read_xyz_dict(dict)
        })
        .collect()
}

fn read_geometry_list(input: &Dictionary, key: &str) -> Result<Vec<GeometryHandle>, EvalError> {
    let list = read_list(input, key)?;
    list_indices(&list)
        .into_iter()
        .map(|index| {
            let dict = list.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}] must be geometry")))?;
            dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(|handle| GeometryHandle(handle.to_string())).ok_or_else(|| EvalError::MissingInput(format!("{key}[{index}].handle")))
        })
        .collect()
}

fn read_nested_point_lists(input: &Dictionary, key: &str) -> Result<Vec<Vec<Vec3>>, EvalError> {
    let list = read_list(input, key)?;
    list_indices(&list)
        .into_iter()
        .map(|index| {
            let sub = list.get(&index.to_string()).and_then(|value| value.as_dictionary()).filter(|dict| dict.schema() == Some("list")).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}] must be a point list")))?;
            list_indices(sub)
                .into_iter()
                .map(|sub_index| {
                    let dict = sub.get(&sub_index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}][{sub_index}] must be a point")))?;
                    read_xyz_dict(dict)
                })
                .collect()
        })
        .collect()
}

fn points_to_grid(points: &[Vec3], rows: usize) -> Result<Vec<Vec<Vec3>>, EvalError> {
    if rows == 0 {
        return Err(EvalError::InvalidInput("rows must be positive".into()));
    }
    if !points.len().is_multiple_of(rows) {
        return Err(EvalError::InvalidInput("points length must divide evenly by rows".into()));
    }
    let cols = points.len() / rows;
    Ok((0..rows).map(|row| (0..cols).map(|col| points[row * cols + col]).collect()).collect())
}

fn wire_from_points(kernel: &mut dyn BrepKernel, points: &[Vec3]) -> Result<GeometryHandle, EvalError> {
    if points.len() >= 2 {
        block_on(kernel.polyline_wire(points)).map_err(map_kernel_error)
    } else if let Some(point) = points.first() {
        block_on(kernel.vertex(*point)).map_err(map_kernel_error)
    } else {
        Err(EvalError::InvalidInput("no intersection".into()))
    }
}

fn domain_span(domain: ParamDomain) -> f64 {
    domain.max - domain.min
}

fn classify_number(classification: PointClassification) -> f64 {
    match classification {
        PointClassification::Inside => 0.0,
        PointClassification::Outside => 1.0,
        PointClassification::OnBoundary => 2.0,
    }
}

fn decode_base64(text: &str) -> Result<Vec<u8>, EvalError> {
    base64::engine::general_purpose::STANDARD.decode(text.trim()).map_err(|error| EvalError::InvalidInput(format!("invalid base64: {error}")))
}

fn encode_base64(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}

fn map_kernel_error(error: semio_s_3d::brep::engine::BrepError) -> EvalError {
    EvalError::InvalidInput(error.to_string())
}

fn number_channel(id: &str, operator_id: &str, default: f64) -> ChannelSpec {
    ChannelSpec::number_default(id, default, &[operator_id])
}

fn geometry_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id])
}

fn list_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::list(id, &[operator_id])
}

fn point_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id])
}

fn out_solid(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Sld", "solid", full_name)
}

fn out_wire(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("W", "Wre", "wire", full_name)
}

fn out_curve(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("C", "Crv", "curve", full_name)
}

fn out_face(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("F", "Fce", "face", full_name)
}
