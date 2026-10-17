//! 🧰️ Mesh widget argument readers, mesh text codec, job drivers and grants shared by the operators.

use semio_s_spatial_kernel_semio_session::*;
use flow_extension_sdk::mesh::*;
use neural_engine::Dictionary;
use neural_engine::EvalError;
use neural_engine::Value;
use neural_engine::OperatorJob;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_3d::mesh::EdgeId;
use semio_framework_3d::mesh::FaceId;
use semio_framework_3d::mesh::HalfedgeMesh;
use semio_framework_3d::mesh::MeshKernelError;
use semio_framework_3d::mesh::Vec3 as MeshVector;
use semio_framework_3d::mesh::VertexId;
use semio_framework_3d::mesh::MeshModelingJob;
use semio_framework_3d::mesh::MeshModelingStep;
use semio_framework_3d::mesh::MeshModelingProgress;
use semio_framework_3d::mesh::MeshTessellationJob;
use semio_framework_3d::mesh::MeshTessellationStep;
use std::collections::HashMap;
use std::collections::HashSet;
use semio_framework_mesh_engine::PolygonMeshSource;
use semio_framework_mesh_engine::io::text::parse_polygon_mesh_source;
use super::{MeshOperatorJob};

pub const LIMIT: usize = 100_000;
pub fn invalid(message: impl Into<String>) -> EvalError { EvalError::InvalidInput(message.into()) }
/// 🎟️ Advances a kernel modeling job by up to `budget` units under the same funding the kernel's own synchronous drivers grant it.
pub fn modeling_slice(job: &mut MeshModelingJob, budget: usize) -> Result<MeshModelingStep, MeshKernelError> {
    let grant = RetainedCloneGrant { maximum_items: budget.max(1), maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };
    job.step(budget, grant, &mut RetainedCloneProgress::default())
}

/// 🎟️ Advances a kernel tessellation by up to `events` producer events, each under the exact grant its own quote asks for.
pub fn tessellation_slice(job: &mut MeshTessellationJob, events: usize) -> Result<MeshTessellationStep, MeshKernelError> {
    let mut last = MeshTessellationStep::Working(job.progress());
    for _ in 0..events.max(1) {
        let copy = job.next_normal_copy_byte_demand().map_err(MeshKernelError::Retained)?.max(128);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: job.next_normal_capacity_byte_demand(copy).map_err(MeshKernelError::Retained)?, maximum_release_bytes: job.next_normal_release_byte_demand().map_err(MeshKernelError::Retained)?, maximum_depth: job.next_normal_depth_demand().map_err(MeshKernelError::Retained)? };
        last = job.step(grant).map_err(MeshKernelError::Retained)?.0;
        if !matches!(last, MeshTessellationStep::Working(_)) {
            break;
        }
    }
    Ok(last)
}
pub fn mesh_error(error: MeshKernelError) -> EvalError {match error{MeshKernelError::Retained(error)=>EvalError::Retained(error),error=>invalid(error.to_string())}}
pub fn affine_matrix(input:&Dictionary)->Result<[f64;16],EvalError> {
    let list=input.get("matrix").and_then(Value::as_dictionary).ok_or_else(||invalid("matrix must be a list of 16 numbers"))?;
    if list.schema()!=Some("list") || list.len()!=17 {return Err(invalid("matrix must be a list of exactly 16 numbers"));}
    let mut matrix=[0.0;16];for (index,value) in matrix.iter_mut().enumerate() {*value=read_channel_number(list,&index.to_string())?;}Ok(matrix)
}
pub fn scalar(input: &Dictionary, name: &str) -> Result<f32, EvalError> {
    let number = read_channel_number(input, name)?;
    if !number.is_finite() || number.abs() > f32::MAX as f64 { return Err(invalid(format!("{name} must be finite"))); }
    Ok(number as f32)
}
pub fn positive(input: &Dictionary, name: &str) -> Result<f32, EvalError> {
    let number = scalar(input, name)?;
    if number <= 0.0 { return Err(invalid(format!("{name} must be positive"))); }
    Ok(number)
}
pub fn count(input: &Dictionary, name: &str, min: u32, max: u32) -> Result<u32, EvalError> {
    let number = read_channel_number(input, name)?;
    if !number.is_finite() || number.fract() != 0.0 || number < min as f64 || number > max as f64 { return Err(invalid(format!("{name} must be an integer in {min}..={max}"))); }
    Ok(number as u32)
}
pub fn vector(input: &Dictionary, name: &str) -> Result<MeshVector, EvalError> {
    let value = read_xyz(input, name)?;
    if value.iter().any(|number| !number.is_finite() || number.abs() > f32::MAX as f64) { return Err(invalid(format!("{name} must be finite"))); }
    Ok(MeshVector(value.map(|number| number as f32)))
}
pub fn selection(input: &Dictionary, name: &str, bound: usize) -> Result<Vec<u32>, EvalError> {
    let text = read_text(input, name)?;
    if text.len() > 16_000_000 { return Err(invalid("selection exceeds 16 MB")); }
    let json = semio_framework_pack_json::parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| invalid(error.to_string()))?;
    let entries = json.as_array().ok_or_else(|| invalid("selection must be an array of indices"))?;
    if entries.is_empty() || entries.len() > LIMIT * 6 { return Err(invalid("select 1..600000 elements")); }
    let mut seen = HashSet::new();
    entries.iter().map(|entry| {
        let id = entry.as_u64().filter(|id| *id < bound as u64).ok_or_else(|| invalid("selection index out of range"))? as u32;
        if !seen.insert(id) && name != "edges" && name != "selection" { return Err(invalid("duplicate selection index")); }
        Ok(id)
    }).collect()
}
pub fn component_vertices(input: &Dictionary, mesh: &HalfedgeMesh) -> Result<Vec<VertexId>, EvalError> {
    let mode = read_text(input, "mode")?;
    let bound = match mode.as_str() {
        "vertex" => mesh.vertex_count(), "edge" => mesh.halfedge_count(), "face" => mesh.face_count(),
        _ => return Err(invalid("component mode must be vertex, edge, or face")),
    };
    let mut vertices = Vec::new();
    for id in selection(input, "selection", bound)? {
        match mode.as_str() {
            "vertex" => vertices.push(VertexId(id)),
            "face" => vertices.extend(mesh.face_vertex_ids(FaceId(id)).map_err(mesh_error)?),
            _ => { let (a, b) = mesh.edge_endpoints(EdgeId(id)).map_err(mesh_error)?; vertices.extend([a, b]); }
        }
    }
    vertices.sort_unstable_by_key(|vertex| vertex.0);
    vertices.dedup();
    Ok(vertices)
}
pub fn component_pivot(input: &Dictionary, mesh: &HalfedgeMesh, vertices: &[VertexId]) -> Result<MeshVector, EvalError> {
    match read_text(input, "pivot")?.as_str() {
        "point" => vector(input, "center"),
        "selection" => {
            let mut sum = [0.0f64; 3];
            for &vertex in vertices {
                let point = mesh.vertex_position(vertex).map_err(mesh_error)?;
                for axis in 0..3 { sum[axis] += point.0[axis] as f64; }
            }
            Ok(MeshVector(sum.map(|value| (value / vertices.len() as f64) as f32)))
        }
        _ => Err(invalid("pivot must be selection or point")),
    }
}
pub fn decode_mesh(text:&str)->Result<HalfedgeMesh,EvalError> {parse_polygon_mesh_source(text).map_err(invalid)?.mesh()}
pub trait PolygonMeshSourceGeometry {
    fn from_mesh(mesh:&HalfedgeMesh)->Result<PolygonMeshSource,EvalError>;
    fn mesh(self)->Result<HalfedgeMesh,EvalError>;
}
impl PolygonMeshSourceGeometry for PolygonMeshSource {
    fn from_mesh(mesh:&HalfedgeMesh) -> Result<Self,EvalError> {
        let vertices = (0..mesh.vertex_count()).map(|id| mesh.vertex_position(VertexId(id as u32)).map(|point| point.0)).collect::<Result<Vec<_>,_>>().map_err(mesh_error)?;
        let faces = (0..mesh.face_count()).map(|id| mesh.face_vertex_ids(FaceId(id as u32)).map(|vertices| vertices.into_iter().map(|vertex| vertex.0).collect())).collect::<Result<Vec<_>,_>>().map_err(mesh_error)?;
        let mut corner_ids=Vec::new();let mut edge_ids=Vec::new();
        for face in 0..mesh.face_count() {
            let (start,flipped)=mesh.face_boundary(FaceId(face as u32)).map_err(mesh_error)?;
            let mut next=start;let mut halfedges=Vec::new();
            loop {halfedges.push(next.0);next=mesh.boundary_corner(next).map_err(mesh_error)?.1;if next==start {break;}}
            for cursor in 0..halfedges.len() {
                let index=if flipped {halfedges.len()-1-cursor}else{cursor};
                corner_ids.push(halfedges[index]);edge_ids.push(halfedges[if flipped {(index+halfedges.len()-1)%halfedges.len()}else{index}]);
            }
        }
        let mut attributes=mesh.attributes().clone();
        for slot in 0..attributes.slot_count() {
            let Some((_,attribute))=attributes.slot_entry_mut(slot) else {continue};
            let ids=match attribute.domain {semio_framework_mesh_engine::MeshAttributeDomain::Corner=>Some(&corner_ids),semio_framework_mesh_engine::MeshAttributeDomain::Edge=>Some(&edge_ids),_=>None};
            if let Some(ids)=ids {
                if attribute.indices.is_some() || ids.iter().enumerate().any(|(index,id)|index!=*id as usize) {
                    attribute.indices=Some(ids.iter().map(|id|attribute.indices.as_ref().map_or(*id,|indices|indices[*id as usize])).collect());
                }
            }
        }
        Ok(Self { vertices,faces,attributes,materials:mesh.materials().clone(),textures:mesh.textures().clone() })
    }
    fn mesh(self)->Result<HalfedgeMesh,EvalError> {let mut job=HalfedgeMesh::polygon_source_job(self).map_err(mesh_error)?;loop {match modeling_slice(&mut job,4096).map_err(mesh_error)? {MeshModelingStep::Done(mesh)=>return Ok(mesh),MeshModelingStep::Working(_)=>{},_=>return Err(invalid("mesh construction cancelled"))}}}
}
pub fn encode_mesh(mesh: &HalfedgeMesh) -> Result<String, EvalError> { Ok(semio_framework_mesh_engine::io::text::encode_polygon_mesh_source(&PolygonMeshSource::from_mesh(mesh)?)) }
pub fn indexed_triangle_mesh(positions: &[f32], indices: &[u32]) -> Result<HalfedgeMesh, EvalError> {
    if positions.len() % 3 != 0 || indices.len() % 3 != 0 { return Err(invalid("invalid triangulation buffers")); }
    let mut unique = HashMap::new();
    let mut vertices = Vec::new();
    let mut remap = Vec::new();
    for point in positions.chunks_exact(3) {
        let key = point.iter().map(|number| if *number == 0.0 { 0 } else { number.to_bits() }).collect::<Vec<_>>();
        let id = *unique.entry(key).or_insert_with(|| { vertices.push([point[0], point[1], point[2]]); vertices.len() as u32 - 1 });
        remap.push(id);
    }
    let faces = indices.chunks_exact(3).map(|triangle| triangle.iter().map(|id| remap.get(*id as usize).copied().ok_or_else(|| invalid("triangulation index out of range"))).collect()).collect::<Result<Vec<Vec<u32>>, EvalError>>()?;
    HalfedgeMesh::from_faces(&vertices, &faces).map_err(mesh_error)
}
pub fn read_mesh(input: &Dictionary, name: &str) -> Result<HalfedgeMesh, EvalError> {
    let mesh = input.get(name).and_then(|value| value.as_dictionary()).filter(|mesh| mesh.schema() == Some("mesh")).ok_or_else(|| invalid(format!("{name} requires a mesh")))?;
    let data = mesh.get("data").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).ok_or_else(|| invalid("mesh data is missing"))?;
    decode_mesh(data)
}
pub fn drive_job(mut job:MeshOperatorJob)->Result<Dictionary,EvalError> {
    use neural_engine::{OperatorJob,OperatorJobStep};
    let output=loop {
        match job.step(4096,sync_grant())?.0 {
            OperatorJobStep::Done(output)=>break output,
            OperatorJobStep::Cancelled(_)=>return Err(invalid("mesh operation cancelled")),
            OperatorJobStep::Working(_)=>{},
        }
    };
    for _ in 0..1_000_000 {if matches!(job.close_step(sync_grant())?,RetainedCloneStep::Complete(_)) {return Ok(output);}}
    Err(invalid("mesh operation did not reach its terminal retirement"))
}
pub fn mesh_output(mesh: &HalfedgeMesh) -> Result<Dictionary, EvalError> {drive_job(MeshOperatorJob::output(mesh.clone())?)}
pub fn analyze(mesh: &HalfedgeMesh) -> Result<Dictionary, EvalError> {drive_job(MeshOperatorJob::analysis(mesh.clone())?)}

pub fn operator_progress(progress: MeshModelingProgress) -> neural_engine::OperatorProgress {
    neural_engine::OperatorProgress { units_done: progress.units_done, units_total: progress.units_total, phase: progress.phase }
}

pub fn add_progress(left:RetainedCloneProgress,right:RetainedCloneProgress)->RetainedCloneProgress {
    RetainedCloneProgress {copied_items:left.copied_items.saturating_add(right.copied_items),copied_bytes:left.copied_bytes.saturating_add(right.copied_bytes),retained_capacity_bytes:left.retained_capacity_bytes.saturating_add(right.retained_capacity_bytes),released_bytes:left.released_bytes.saturating_add(right.released_bytes)}
}
pub fn turn_grant(grant:RetainedCloneGrant)->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,..grant}}
pub fn sync_grant()->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:256}}
pub fn value_error(error:EvalError)->ValueError {ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())}
