//! 🥽️ Indexed mesh widgets with explicit polygon data and B-Rep preview conversion.
use super::*;
use neural_engine::{Atom, FieldSpec, Schema, ValueType};
use semio_framework_3d::mesh::{EdgeId, FaceId, HalfedgeMesh, MeshKernelError, Vec3 as MeshVector, VertexId, WeldMode, MirrorAxis, MeshModelingJob, MeshModelingStep, MeshModelingProgress};
use std::collections::{HashMap, HashSet};

const LIMIT: usize = 100_000;
fn invalid(message: impl Into<String>) -> EvalError { EvalError::InvalidInput(message.into()) }
fn mesh_error(error: MeshKernelError) -> EvalError { invalid(error.to_string()) }
fn scalar(input: &Dictionary, name: &str) -> Result<f32, EvalError> {
    let number = read_channel_number(input, name)?;
    if !number.is_finite() || number.abs() > f32::MAX as f64 { return Err(invalid(format!("{name} must be finite"))); }
    Ok(number as f32)
}
fn positive(input: &Dictionary, name: &str) -> Result<f32, EvalError> {
    let number = scalar(input, name)?;
    if number <= 0.0 { return Err(invalid(format!("{name} must be positive"))); }
    Ok(number)
}
fn count(input: &Dictionary, name: &str, min: u32, max: u32) -> Result<u32, EvalError> {
    let number = read_channel_number(input, name)?;
    if !number.is_finite() || number.fract() != 0.0 || number < min as f64 || number > max as f64 { return Err(invalid(format!("{name} must be an integer in {min}..={max}"))); }
    Ok(number as u32)
}
fn vector(input: &Dictionary, name: &str) -> Result<MeshVector, EvalError> {
    let value = read_xyz(input, name)?;
    if value.iter().any(|number| !number.is_finite() || number.abs() > f32::MAX as f64) { return Err(invalid(format!("{name} must be finite"))); }
    Ok(MeshVector(value.map(|number| number as f32)))
}
fn selection(input: &Dictionary, name: &str, bound: usize) -> Result<Vec<u32>, EvalError> {
    let text = read_text(input, name)?;
    if text.len() > 16_000_000 { return Err(invalid("selection exceeds 16 MB")); }
    let json = pack::json::parse(&text).map_err(|error| invalid(error.to_string()))?;
    let entries = json.as_array().ok_or_else(|| invalid("selection must be an array of indices"))?;
    if entries.is_empty() || entries.len() > LIMIT * 6 { return Err(invalid("select 1..600000 elements")); }
    let mut seen = HashSet::new();
    entries.iter().map(|entry| {
        let id = entry.as_u64().filter(|id| *id < bound as u64).ok_or_else(|| invalid("selection index out of range"))? as u32;
        if !seen.insert(id) && name != "edges" && name != "selection" { return Err(invalid("duplicate selection index")); }
        Ok(id)
    }).collect()
}
fn component_vertices(input: &Dictionary, mesh: &HalfedgeMesh) -> Result<Vec<VertexId>, EvalError> {
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
fn component_pivot(input: &Dictionary, mesh: &HalfedgeMesh, vertices: &[VertexId]) -> Result<MeshVector, EvalError> {
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
fn decode_mesh(text: &str) -> Result<HalfedgeMesh, EvalError> {
    if text.len() > 16_000_000 { return Err(invalid("mesh input exceeds 16 MB")); }
    let json = pack::json::parse(text).map_err(|error| invalid(error.to_string()))?;
    if json.as_object().is_none_or(|object| object.iter().any(|(key, _)| key != "vertices" && key != "faces")) { return Err(invalid("unknown mesh field")); }
    let vertices = json.get("vertices").and_then(|v| v.as_array()).ok_or_else(|| invalid("vertices must be an array"))?;
    let faces = json.get("faces").and_then(|v| v.as_array()).ok_or_else(|| invalid("faces must be an array"))?;
    if vertices.len() < 3 || vertices.len() > LIMIT || faces.is_empty() || faces.len() > LIMIT { return Err(invalid("mesh requires 3..100000 vertices and 1..100000 faces")); }
    let positions: Vec<[f32; 3]> = vertices.iter().map(|vertex| {
        let point = vertex.as_array().filter(|point| point.len() == 3).ok_or_else(|| invalid("vertex must have three coordinates"))?;
        let mut result = [0.0; 3];
        for axis in 0..3 {
            let number = point[axis].as_f64().filter(|number| number.is_finite() && number.abs() <= f32::MAX as f64).ok_or_else(|| invalid("coordinate must be finite"))?;
            result[axis] = number as f32;
        }
        Ok(result)
    }).collect::<Result<_, EvalError>>()?;
    let mut corner_count = 0;
    let polygons: Vec<Vec<u32>> = faces.iter().map(|face| {
        let indices = face.as_array().filter(|indices| indices.len() >= 3).ok_or_else(|| invalid("face requires at least three indices"))?;
        corner_count += indices.len();
        if corner_count > LIMIT * 6 { return Err(invalid("mesh exceeds 600000 polygon corners")); }
        let mut seen = HashSet::new();
        indices.iter().map(|index| {
            let id = index.as_u64().filter(|id| *id < positions.len() as u64).ok_or_else(|| invalid("face index out of range"))? as u32;
            if !seen.insert(id) { return Err(invalid("face contains a repeated vertex")); }
            Ok(id)
        }).collect()
    }).collect::<Result<_, EvalError>>()?;
    HalfedgeMesh::from_faces(&positions, &polygons).map_err(mesh_error)
}
fn encode_mesh(mesh: &HalfedgeMesh) -> Result<String, EvalError> {
    let vertices = (0..mesh.vertex_count()).map(|id| mesh.vertex_position(VertexId(id as u32)).map(|point| pack::json::array(point.0.into_iter().map(|number| pack::json::Value::from(number as f64))))).collect::<Result<Vec<_>, _>>().map_err(mesh_error)?;
    let faces = (0..mesh.face_count()).map(|id| mesh.face_vertex_ids(FaceId(id as u32)).map(|vertices| pack::json::array(vertices.into_iter().map(|vertex| pack::json::Value::from(vertex.0))))).collect::<Result<Vec<_>, _>>().map_err(mesh_error)?;
    Ok(pack::json::to_string(&pack::json::object([("vertices".into(), pack::json::array(vertices)), ("faces".into(), pack::json::array(faces))])))
}
fn indexed_triangle_mesh(positions: &[f32], indices: &[u32]) -> Result<HalfedgeMesh, EvalError> {
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
fn read_mesh(input: &Dictionary, name: &str) -> Result<HalfedgeMesh, EvalError> {
    let mesh = input.get(name).and_then(|value| value.as_dictionary()).filter(|mesh| mesh.schema() == Some("mesh")).ok_or_else(|| invalid(format!("{name} requires a mesh")))?;
    let data = mesh.get("data").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).ok_or_else(|| invalid("mesh data is missing"))?;
    decode_mesh(data)
}
fn mesh_output(mesh: &HalfedgeMesh) -> Result<Dictionary, EvalError> {
    let data = encode_mesh(mesh)?;
    let transfer = mesh.tessellate().map_err(mesh_error)?;
    let preview = semio_framework_mesh_engine::MeshData { positions: transfer.positions, normals: transfer.normals, indices: transfer.indices, face_ids: transfer.face_ids, vertex_ids: transfer.vertex_ids, edge_positions: transfer.edge_positions, edge_ids: transfer.edge_ids, uvs: transfer.uvs, edge_uvs: transfer.edge_uvs, edge_is_seam: transfer.edge_is_seam, ..Default::default() };
    if preview.positions.iter().any(|number| !number.is_finite()) { return Err(invalid("mesh operation produced non-finite coordinates")); }
    let preview = encode_base64(&encode_mesh_pack(&preview).map_err(invalid)?);
    let mesh = Dictionary::with_schema("mesh").insert("data", Value::Atom(Atom::String(data))).insert("preview", Value::Atom(Atom::String(preview)));
    Ok(channel_output("meshOut", mesh))
}
fn analyze(mesh: &HalfedgeMesh) -> Result<Dictionary, EvalError> {
    let mut edges = HashMap::<(u32, u32), (usize, i32)>::new();
    for face in 0..mesh.face_count() {
        let vertices = mesh.face_vertex_ids(FaceId(face as u32)).map_err(mesh_error)?;
        for i in 0..vertices.len() {
            let a = vertices[i].0;
            let b = vertices[(i + 1) % vertices.len()].0;
            let entry = edges.entry((a.min(b), a.max(b))).or_default();
            entry.0 += 1;
            entry.1 += if a < b { 1 } else { -1 };
        }
    }
    let boundary = edges.values().filter(|edge| edge.0 == 1).count();
    let non_manifold = edges.values().filter(|edge| edge.0 > 2).count();
    let inconsistent = edges.values().filter(|edge| edge.0 == 2 && edge.1 != 0).count();
    let triangles = mesh.tessellate().map_err(mesh_error)?;
    let mut area = 0.0f64;
    let mut volume = 0.0f64;
    let reference = mesh.vertex_position(VertexId(0)).map_err(mesh_error)?.0.map(f64::from);
    let sub = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let cross = |a: [f64; 3], b: [f64; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for id in 0..mesh.vertex_count() {
        let point = mesh.vertex_position(VertexId(id as u32)).map_err(mesh_error)?.0;
        for axis in 0..3 { min[axis] = min[axis].min(point[axis]); max[axis] = max[axis].max(point[axis]); }
    }
    let mut degenerate = 0;
    for triangle in triangles.indices.chunks_exact(3) {
        let point = |index: u32| [triangles.positions[index as usize * 3] as f64, triangles.positions[index as usize * 3 + 1] as f64, triangles.positions[index as usize * 3 + 2] as f64];
        let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
        let normal = cross(sub(b, a), sub(c, a));
        let triangle_area = normal[0].hypot(normal[1]).hypot(normal[2]) / 2.0;
        area += triangle_area;
        if triangle_area == 0.0 { degenerate += 1; }
        let relative = sub(a, reference);
        let normal = cross(sub(b, reference), sub(c, reference));
        volume += relative.iter().zip(normal).map(|(a, b)| a * b).sum::<f64>() / 6.0;
    }
    let mut report = Dictionary::new();
    for (key, value) in [("vertices", mesh.vertex_count() as f64), ("faces", mesh.face_count() as f64), ("edges", edges.len() as f64), ("triangles", triangles.indices.len() as f64 / 3.0), ("boundaryEdges", boundary as f64), ("nonManifoldEdges", non_manifold as f64), ("inconsistentEdges", inconsistent as f64), ("degenerateTriangles", degenerate as f64), ("area", area)] {
        report = report.insert(key, Value::Dictionary(number_dictionary(value)));
    }
    if boundary == 0 && non_manifold == 0 && inconsistent == 0 && degenerate == 0 { report = report.insert("volume", Value::Dictionary(number_dictionary(volume.abs()))); }
    Ok(report.insert("minimum", Value::Dictionary(point_dictionary(min.map(f64::from)))).insert("maximum", Value::Dictionary(point_dictionary(max.map(f64::from)))))
}

fn operator_progress(progress: MeshModelingProgress) -> neural_engine::OperatorProgress {
    neural_engine::OperatorProgress { units_done: progress.units_done, units_total: progress.units_total, phase: progress.phase }
}

struct MeshOperatorJob {
    job: Option<MeshModelingJob>,
    output: Option<HalfedgeMesh>,
    progress: neural_engine::OperatorProgress,
    cancelled: bool,
}

impl neural_engine::OperatorJob for MeshOperatorJob {
    fn step(&mut self, budget: usize) -> Result<neural_engine::OperatorJobStep, EvalError> {
        if self.cancelled { return Ok(neural_engine::OperatorJobStep::Cancelled(self.progress)); }
        if let Some(mesh) = &self.output { return mesh_output(mesh).map(neural_engine::OperatorJobStep::Done); }
        let job = self.job.as_mut().ok_or_else(|| invalid("mesh job is missing"))?;
        match job.step(budget).map_err(mesh_error)? {
            MeshModelingStep::Working(progress) => {
                self.progress = operator_progress(progress);
                Ok(neural_engine::OperatorJobStep::Working(self.progress))
            }
            MeshModelingStep::Cancelled(progress) => {
                self.progress = operator_progress(progress); self.cancelled = true; self.job = None;
                Ok(neural_engine::OperatorJobStep::Cancelled(self.progress))
            }
            MeshModelingStep::Done(mesh) => {
                self.progress = operator_progress(job.progress());
                self.job = None; self.output = Some(mesh);
                mesh_output(self.output.as_ref().unwrap()).map(neural_engine::OperatorJobStep::Done)
            }
        }
    }
    fn progress(&self) -> neural_engine::OperatorProgress { self.progress }
    fn cancel(&mut self) {
        if self.output.is_some() || self.cancelled { return; }
        if let Some(job) = &mut self.job { job.cancel(); self.progress = operator_progress(job.progress()); }
        self.cancelled = true; self.job = None;
    }
}

struct MeshOperation(&'static str);
impl Operator for MeshOperation {
    fn step_plan(&self, input: &Dictionary) -> Result<Option<Box<dyn neural_engine::OperatorJob>>, EvalError> {
        if !matches!(self.0, "bevel" | "decimate") { return Ok(None); }
        let mesh = read_mesh(input, "mesh")?;
        let job = if self.0 == "bevel" {
            let edges = selection(input, "edges", mesh.halfedge_count())?.into_iter().map(EdgeId).collect::<Vec<_>>();
            mesh.bevel_job(&edges, positive(input, "amount")?, count(input, "segments", 1, 64)?).map_err(mesh_error)?
        } else {
            let ratio = positive(input, "ratio")?;
            if ratio > 1.0 { return Err(invalid("decimation ratio must be at most one")); }
            mesh.decimate_job(ratio).map_err(mesh_error)?
        };
        let progress = operator_progress(job.progress());
        Ok(Some(Box::new(MeshOperatorJob { job: Some(job), output: None, progress, cancelled: false })))
    }
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let mut mesh = match self.0 {
            "construct" => decode_mesh(&read_text(input, "data")?)?,
            "box" => HalfedgeMesh::box_prim(positive(input, "width")?, positive(input, "height")?, positive(input, "depth")?).map_err(mesh_error)?,
            "plane" => HalfedgeMesh::plane_prim(positive(input, "width")?, positive(input, "depth")?).map_err(mesh_error)?,
            "sphere" => HalfedgeMesh::ico_sphere_prim(positive(input, "radius")?, count(input, "subdivisions", 0, 5)?).map_err(mesh_error)?,
            "cylinder" => HalfedgeMesh::cylinder_prim(positive(input, "radius")?, positive(input, "height")?, count(input, "segments", 3, 1024)?).map_err(mesh_error)?,
            "cone" => HalfedgeMesh::cone_prim(positive(input, "radius")?, positive(input, "height")?, count(input, "segments", 3, 1024)?).map_err(mesh_error)?,
            "fromBrep" => with_kernel_read(|kernel| {
                let transfer = kernel.tessellate(&read_geometry(input, "geometry")?, positive(input, "deflection")? as f64).map_err(|error| map_kernel_error(&error))?;
                indexed_triangle_mesh(&transfer.position, &transfer.index)
            })?,
            _ => read_mesh(input, "mesh")?,
        };
        match self.0 {
            "translate" => mesh.translate(vector(input, "offset")?).map_err(mesh_error)?,
            "rotate" => {
                let axis = vector(input, "axis")?;
                if axis.0.iter().all(|coordinate| *coordinate == 0.0) { return Err(invalid("rotation axis cannot be zero")); }
                mesh.rotate(axis, scalar(input, "angle")?).map_err(mesh_error)?;
            }
            "scale" => {
                let factors = vector(input, "factor")?;
                if factors.0.iter().any(|value| *value == 0.0) { return Err(invalid("scale factors cannot be zero")); }
                mesh.scale(factors).map_err(mesh_error)?;
                if factors.0.iter().filter(|value| **value < 0.0).count() % 2 == 1 { mesh.flip_faces(&(0..mesh.face_count()).map(|id| FaceId(id as u32)).collect::<Vec<_>>()).map_err(mesh_error)?; }
            }
            "translateComponents" | "rotateComponents" | "scaleComponents" => {
                let ids = component_vertices(input, &mesh)?;
                match self.0 {
                    "translateComponents" => mesh.move_vertices(&ids, vector(input, "offset")?).map_err(mesh_error)?,
                    "rotateComponents" => {
                        let pivot = component_pivot(input, &mesh, &ids)?;
                        mesh.rotate_vertices(&ids, vector(input, "axis")?, scalar(input, "angle")?, pivot).map_err(mesh_error)?;
                    }
                    _ => {
                        let pivot = component_pivot(input, &mesh, &ids)?;
                        mesh.scale_vertices(&ids, vector(input, "factor")?, pivot).map_err(mesh_error)?;
                    }
                }
            }
            "moveVertices" => {
                let ids = selection(input, "vertices", mesh.vertex_count())?.into_iter().map(VertexId).collect::<Vec<_>>();
                mesh.move_vertices(&ids, vector(input, "offset")?).map_err(mesh_error)?;
            }
            "bevel" | "dissolveEdges" => {
                let ids = selection(input, "edges", mesh.halfedge_count())?.into_iter().map(EdgeId).collect::<Vec<_>>();
                if self.0 == "bevel" { mesh.bevel_edges(&ids, positive(input, "amount")?, count(input, "segments", 1, 64)?).map_err(mesh_error)?; }
                else { mesh.dissolve_edges(&ids).map_err(mesh_error)?; }
            }
            "moveProportional" | "snapVertices" | "mergeVertices" | "dissolveVertices" => {
                let ids = selection(input, "selection", mesh.vertex_count())?.into_iter().map(VertexId).collect::<Vec<_>>();
                match self.0 {
                    "moveProportional" => mesh.move_vertices_proportional(&ids, vector(input, "offset")?, vector(input, "center")?, positive(input, "radius")?).map_err(mesh_error)?,
                    "snapVertices" => mesh.snap_vertices_to_grid(&ids, positive(input, "grid")?).map_err(mesh_error)?,
                    "dissolveVertices" => mesh.dissolve_vertices(&ids).map_err(mesh_error)?,
                    _ => {
                        let mode = match read_text(input, "mode")?.as_str() { "first" => WeldMode::First, "center" => WeldMode::Center, "distance" => WeldMode::ByDistance, _ => return Err(invalid("merge mode must be first, center, or distance")) };
                        let tolerance = scalar(input, "tolerance")?;
                        if tolerance < 0.0 { return Err(invalid("merge tolerance must be nonnegative")); }
                        mesh.merge_vertices(&ids, mode, tolerance).map_err(mesh_error)?;
                    }
                }
            }
            "mirror" => {
                let axis = match read_text(input, "axis")?.as_str() { "x" => MirrorAxis::X, "y" => MirrorAxis::Y, "z" => MirrorAxis::Z, _ => return Err(invalid("mirror axis must be x, y, or z")) };
                let tolerance = scalar(input, "tolerance")?;
                if tolerance < 0.0 { return Err(invalid("mirror tolerance must be nonnegative")); }
                mesh.mirror(axis, tolerance).map_err(mesh_error)?;
            }
            "decimate" => {
                let ratio = positive(input, "ratio")?;
                if ratio > 1.0 { return Err(invalid("decimation ratio must be at most one")); }
                mesh.decimate(ratio).map_err(mesh_error)?;
            }
            "mergeCoplanar" => { mesh.merge_coplanar_faces().map_err(mesh_error)?; }
            "loopCut" => {
                let ids = selection(input, "edges", mesh.halfedge_count())?.into_iter().map(EdgeId).collect::<Vec<_>>();
                mesh.loop_cut(&ids, count(input, "cuts", 1, 256)?).map_err(mesh_error)?;
            }
            "knifeCut" => mesh.knife_cut(FaceId(count(input, "face", 0, mesh.face_count().saturating_sub(1) as u32)?), vector(input, "start")?, vector(input, "end")?).map_err(mesh_error)?,
            "extrude" | "inset" | "subdivide" | "flip" | "deleteFaces" => {
                let ids = selection(input, "faces", mesh.face_count())?.into_iter().map(FaceId).collect::<Vec<_>>();
                match self.0 {
                    "extrude" => mesh.extrude_faces(&ids, scalar(input, "distance")?).map_err(mesh_error)?,
                    "inset" => mesh.inset_faces(&ids, positive(input, "amount")?).map_err(mesh_error)?,
                    "subdivide" => mesh.subdivide_faces(&ids).map_err(mesh_error)?,
                    "flip" => mesh.flip_faces(&ids).map_err(mesh_error)?,
                    _ => {
                        let positions = (0..mesh.vertex_count()).map(|id| mesh.vertex_position(VertexId(id as u32)).map(|point| point.0)).collect::<Result<Vec<_>, _>>().map_err(mesh_error)?;
                        let faces = (0..mesh.face_count()).filter(|id| !ids.contains(&FaceId(*id as u32))).map(|id| mesh.face_vertex_ids(FaceId(id as u32)).map(|vertices| vertices.into_iter().map(|id| id.0).collect())).collect::<Result<Vec<Vec<u32>>, _>>().map_err(mesh_error)?;
                        if faces.is_empty() { return Err(invalid("deletion would leave an empty mesh")); }
                        mesh = HalfedgeMesh::from_faces(&positions, &faces).map_err(mesh_error)?;
                    }
                }
            }
            "triangulate" => mesh.triangulate().map_err(mesh_error)?,
            "weld" => { mesh.weld_coincident_vertices(positive(input, "tolerance")?).map_err(mesh_error)?; }
            "orient" => { mesh.orient_faces_consistently().map_err(mesh_error)?; }
            "fillHoles" => { mesh.fill_holes().map_err(mesh_error)?; }
            "inspectVertex" => {
                let id = VertexId(count(input, "index", 0, mesh.vertex_count().saturating_sub(1) as u32)?);
                return Ok(channel_output("point", point_dictionary(mesh.vertex_position(id).map_err(mesh_error)?.0.map(f64::from))));
            }
            "inspectEdge" => {
                let id = EdgeId(count(input, "index", 0, mesh.halfedge_count().saturating_sub(1) as u32)?);
                let (a, b) = mesh.edge_endpoints(id).map_err(mesh_error)?;
                let start = mesh.vertex_position(a).map_err(mesh_error)?.0.map(f64::from);
                let end = mesh.vertex_position(b).map_err(mesh_error)?.0.map(f64::from);
                let length = (end[0] - start[0]).hypot(end[1] - start[1]).hypot(end[2] - start[2]);
                return Ok(Dictionary::new().insert("start", Value::Dictionary(point_dictionary(start))).insert("end", Value::Dictionary(point_dictionary(end))).insert("length", Value::Dictionary(number_dictionary(length))));
            }
            "inspectFace" => {
                let id = FaceId(count(input, "index", 0, mesh.face_count().saturating_sub(1) as u32)?);
                let ids = mesh.face_vertex_ids(id).map_err(mesh_error)?;
                let normal = mesh.face_normal(id).map_err(mesh_error)?.0.map(f64::from);
                if normal.iter().all(|value| *value == 0.0) { return Err(invalid("face normal is degenerate")); }
                let mut center = [0.0; 3];
                for &vertex in &ids {
                    let point = mesh.vertex_position(vertex).map_err(mesh_error)?.0;
                    for axis in 0..3 { center[axis] += point[axis] as f64 / ids.len() as f64; }
                }
                let vertices = pack::json::to_string(&pack::json::array(ids.iter().map(|id| pack::json::Value::from(id.0))));
                return Ok(Dictionary::new().insert("vertices", Value::Dictionary(text_dictionary(vertices))).insert("normal", Value::Dictionary(vector_dictionary(normal))).insert("center", Value::Dictionary(point_dictionary(center))));
            }
            "analyze" => return analyze(&mesh),
            "exportObj" => return Ok(channel_output("text", text_dictionary(mesh.to_obj().map_err(mesh_error)?))),
            "exportJson" => return Ok(channel_output("text", text_dictionary(encode_mesh(&mesh)?))),
            "toBrep" => return with_kernel(|kernel| {
                let handle = kernel.import_obj(&mesh.to_obj().map_err(mesh_error)?, positive(input, "tolerance")? as f64).map_err(|error| map_kernel_error(&error))?;
                Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
            }),
            _ => {}
        }
        if mesh.vertex_count() > LIMIT || mesh.face_count() > LIMIT { return Err(invalid("result exceeds mesh capacity")); }
        mesh_output(&mesh)
    }
}

pub(super) fn register_mesh(registry: &mut Registry) {
    registry.register_schema(Schema { id: "mesh".into(), module: "brep".into(), name: "Polygon Mesh".into(), icon: "emoji:🥽️".into(), summary: "Indexed polygon vertices and faces".into(), fields: vec![FieldSpec::new("data", ValueType::Text), FieldSpec::new("preview", ValueType::Text)] });
    let definitions: &[(&str, &str, &str, &[(&str, f64)])] = &[
        ("construct", "Construct Mesh", "Mesh Creation", &[]),
        ("box", "Mesh Box", "Mesh Creation", &[("width", 1.0), ("height", 1.0), ("depth", 1.0)]),
        ("plane", "Mesh Plane", "Mesh Creation", &[("width", 1.0), ("depth", 1.0)]),
        ("sphere", "Mesh Sphere", "Mesh Creation", &[("radius", 1.0), ("subdivisions", 2.0)]),
        ("cylinder", "Mesh Cylinder", "Mesh Creation", &[("radius", 1.0), ("height", 1.0), ("segments", 32.0)]),
        ("cone", "Mesh Cone", "Mesh Creation", &[("radius", 1.0), ("height", 1.0), ("segments", 32.0)]),
        ("fromBrep", "Mesh from B-Rep", "Mesh Conversion", &[("deflection", 0.1)]),
        ("toBrep", "Faceted B-Rep from Mesh", "Mesh Conversion", &[("tolerance", 0.001)]),
        ("translate", "Translate Mesh", "Mesh Editing", &[]),
        ("rotate", "Rotate Mesh", "Mesh Editing", &[("angle", 0.0)]),
        ("scale", "Scale Mesh", "Mesh Editing", &[]),
        ("moveVertices", "Move Mesh Vertices", "Mesh Editing", &[]),
        ("translateComponents", "Move Mesh Components", "Mesh Editing", &[]),
        ("rotateComponents", "Rotate Mesh Components", "Mesh Editing", &[("angle", 0.0)]),
        ("scaleComponents", "Scale Mesh Components", "Mesh Editing", &[]),
        ("bevel", "Bevel Mesh Edges", "Mesh Editing", &[("amount", 0.1), ("segments", 1.0)]),
        ("dissolveEdges", "Dissolve Mesh Edges", "Mesh Editing", &[]),
        ("dissolveVertices", "Dissolve Mesh Vertices", "Mesh Editing", &[]),
        ("mergeVertices", "Merge Mesh Vertices", "Mesh Editing", &[("tolerance", 0.0001)]),
        ("moveProportional", "Move Mesh Proportionally", "Mesh Editing", &[("radius", 1.0)]),
        ("snapVertices", "Snap Mesh Vertices to Grid", "Mesh Editing", &[("grid", 1.0)]),
        ("mirror", "Mirror Mesh Half", "Mesh Editing", &[("tolerance", 0.0001)]),
        ("decimate", "Simplify Mesh", "Mesh Editing", &[("ratio", 0.5)]),
        ("mergeCoplanar", "Merge Coplanar Mesh Faces", "Mesh Repair", &[]),
        ("loopCut", "Cut Mesh Loops", "Mesh Editing", &[("cuts", 1.0)]),
        ("knifeCut", "Knife Cut Mesh Face", "Mesh Editing", &[("face", 0.0)]),
        ("extrude", "Extrude Mesh Faces", "Mesh Editing", &[("distance", 1.0)]),
        ("inset", "Inset Mesh Faces", "Mesh Editing", &[("amount", 0.1)]),
        ("subdivide", "Subdivide Mesh Faces", "Mesh Editing", &[]),
        ("flip", "Flip Mesh Faces", "Mesh Editing", &[]),
        ("deleteFaces", "Delete Mesh Faces", "Mesh Editing", &[]),
        ("triangulate", "Triangulate Mesh", "Mesh Editing", &[]),
        ("weld", "Weld Mesh Vertices", "Mesh Repair", &[("tolerance", 0.0001)]),
        ("orient", "Orient Mesh Faces", "Mesh Repair", &[]),
        ("fillHoles", "Fill Mesh Holes", "Mesh Repair", &[]),
        ("inspectVertex", "Inspect Mesh Vertex", "Mesh Analysis", &[("index", 0.0)]),
        ("inspectEdge", "Inspect Mesh Edge", "Mesh Analysis", &[("index", 0.0)]),
        ("inspectFace", "Inspect Mesh Face", "Mesh Analysis", &[("index", 0.0)]),
        ("analyze", "Analyze Mesh", "Mesh Analysis", &[]),
        ("exportObj", "Mesh to OBJ", "Mesh Interchange", &[]),
        ("exportJson", "Mesh to JSON", "Mesh Interchange", &[]),
    ];
    for &(operation, name, group, parameters) in definitions {
        let id = format!("brep.mesh.{operation}");
        let mut inputs = Vec::new();
        if !matches!(operation, "construct" | "box" | "plane" | "sphere" | "cylinder" | "cone" | "fromBrep") { inputs.push(ChannelSpec::requires("mesh", &[&id]).with_value_types(&["mesh"])); }
        if operation == "construct" { inputs.push(ChannelSpec::requires("data", &[&id]).with_value_types(&["text"])); }
        if operation == "fromBrep" { inputs.push(geometry_channel("geometry", &id)); }
        if matches!(operation, "extrude" | "inset" | "subdivide" | "flip" | "deleteFaces" | "moveVertices" | "loopCut" | "bevel" | "dissolveEdges") {
            inputs.push(ChannelSpec::requires(match operation { "moveVertices" => "vertices", "loopCut" | "bevel" | "dissolveEdges" => "edges", _ => "faces" }, &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("[0]"))));
        }
        if matches!(operation, "moveProportional" | "snapVertices" | "mergeVertices" | "dissolveVertices") {
            inputs.push(ChannelSpec::requires("selection", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary(if operation == "mergeVertices" { "[0,1]" } else { "[0]" }))));
        }
        if operation == "moveProportional" { inputs.push(vector_channel("center", &id, [0.0; 3])); }
        if operation == "mergeVertices" { inputs.push(ChannelSpec::requires("mode", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("center")))); }
        if operation == "mirror" { inputs.push(ChannelSpec::requires("axis", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("x")))); }
        if operation.ends_with("Components") {
            for (key, value) in [("mode", "vertex"), ("selection", "[0]")] { inputs.push(ChannelSpec::requires(key, &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary(value)))); }
            if operation != "translateComponents" {
                inputs.push(ChannelSpec::requires("pivot", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("selection"))));
                inputs.push(ChannelSpec::requires("center", &[&id]).with_value_types(&["point", "vector"]).with_default(Value::Dictionary(point_dictionary([0.0; 3]))));
            }
        }
        if operation == "knifeCut" {
            for (key, point) in [("start", [0.0, -1.0, 0.0]), ("end", [0.0, 1.0, 0.0])] {
                inputs.push(ChannelSpec::requires(key, &[&id]).with_value_types(&["point", "vector"]).with_default(Value::Dictionary(point_dictionary(point))));
            }
        }
        match operation {
            "translate" | "moveVertices" | "translateComponents" | "moveProportional" => inputs.push(vector_channel("offset", &id, [0.0, 0.0, 1.0])),
            "rotate" | "rotateComponents" => inputs.push(vector_channel("axis", &id, [0.0, 0.0, 1.0])),
            "scale" | "scaleComponents" => inputs.push(vector_channel("factor", &id, [1.0, 1.0, 1.0])),
            _ => {}
        }
        inputs.extend(parameters.iter().map(|(key, value)| number_channel(key, &id, *value)));
        let (outputs, produced) = match operation {
            "analyze" => {
                let mut channels: Vec<_> = ["vertices", "faces", "edges", "triangles", "boundaryEdges", "nonManifoldEdges", "inconsistentEdges", "degenerateTriangles", "area", "volume"].iter().map(|&key| ChannelSpec::named(key, key, key, key).with_value_types(&["number"])).collect();
                for channel in &mut channels { if channel.name == "volume" { channel.cardinality = neural_engine::Cardinality::ZeroOrOne; } }
                channels.extend([out_point("Minimum"), out_point("Maximum")].into_iter().zip(["minimum", "maximum"]).map(|(mut channel, name)| { channel.name = name.into(); channel }));
                (channels, vec!["number", "point"])
            }
            "inspectVertex" => (vec![out_point("VertexPosition")], vec!["point"]),
            "inspectEdge" => (vec![ChannelSpec::named("S", "Start", "start", "EdgeStart").with_value_types(&["point"]), ChannelSpec::named("E", "End", "end", "EdgeEnd").with_value_types(&["point"]), out_length()], vec!["point", "number"]),
            "inspectFace" => (vec![ChannelSpec::named("V", "Verts", "vertices", "FaceVertices").with_value_types(&["text"]), out_normal("FaceNormal"), out_center().with_value_types(&["point"])], vec!["text", "vector", "point"]),
            "toBrep" => (vec![out_geometry("FacetedGeometry")], vec!["geometry"]),
            "exportObj" | "exportJson" => (vec![ChannelSpec::named("T", "Text", "text", "MeshText").with_value_types(&["text"])], vec!["text"]),
            _ => (vec![ChannelSpec::named("M", "Mesh", "meshOut", "PolygonMesh").with_value_types(&["mesh"])], vec!["mesh"]),
        };
        let summary = match operation {
            "construct" => "Create a polygon mesh from JSON vertices and zero-based face indices.",
            "box" => "Create a closed box with six quad faces and independent width, height, and depth.",
            "plane" => "Create one open quad in the horizontal plane.",
            "sphere" => "Create a closed triangular sphere; each subdivision increases surface detail.",
            "cylinder" => "Create a closed cylinder with polygon caps and quad sides.",
            "cone" => "Create a closed cone with a polygon base and triangular sides.",
            "fromBrep" => "Tessellate a B-Rep into a triangle mesh; smaller deflection produces more detail.",
            "toBrep" => "Convert mesh polygons to planar B-Rep faces without reconstructing curved surfaces.",
            "translate" => "Move every mesh vertex by the offset vector.",
            "rotate" => "Rotate the mesh around an axis through the origin; angle is in radians.",
            "scale" => "Scale each axis about the origin; negative factors mirror and preserve outward winding.",
            "moveVertices" => "Move selected zero-based vertex indices by the offset vector.",
            "translateComponents" => "Move selected vertices, edges, or faces; shared vertices move once.",
            "rotateComponents" => "Rotate selected components about their centroid or a chosen point; angle is in radians.",
            "scaleComponents" => "Scale selected components about their centroid or a chosen point, preserving polygon indices.",
            "bevel" => "Round selected edges of a closed convex mesh with 1–64 profile segments; reject widths crossing adjacent vertices.",
            "dissolveEdges" => "Remove selected connecting edges and join their neighboring polygon faces.",
            "dissolveVertices" => "Join a planar connected vertex neighborhood into one polygon without removing its surface.",
            "mergeVertices" => "Merge selected vertices at the first position, their mean, or within the distance tolerance; clean collapsed polygon loops.",
            "moveProportional" => "Move selected vertices fully and surrounding vertices with linear distance falloff from the center within the radius.",
            "snapVertices" => "Snap selected vertices to an origin-aligned grid with positive spacing.",
            "mirror" => "Reflect a one-sided mesh across an origin axis plane; weld only matching seam vertices within the tolerance.",
            "decimate" => "Approximate mesh simplification by shortest-edge collapse with winding and manifold checks; may stop before the target ratio.",
            "mergeCoplanar" => "Join adjacent coplanar faces without changing their surface.",
            "loopCut" => "Cut connected quad strips through selected preview edges; 1–256 cuts share vertices and crossing strips form grids.",
            "knifeCut" => "Split one face along the projected line through two points, sharing new boundary vertices with its neighbors.",
            "extrude" => "Extrude selected faces along their normals and connect the boundary with side faces.",
            "inset" => "Inset each selected face by a positive distance, preserving connected border faces.",
            "subdivide" => "Split selected faces into triangles while preserving their boundary edges.",
            "flip" => "Reverse the winding and normals of selected faces.",
            "deleteFaces" => "Remove selected faces, leaving open boundaries for further editing.",
            "triangulate" => "Triangulate polygon faces, including concave planar polygons.",
            "weld" => "Merge vertices within the tolerance and remove collapsed faces.",
            "orient" => "Make adjacent face winding consistent across connected components.",
            "fillHoles" => "Cap open boundary loops with polygon faces.",
            "inspectVertex" => "Read the position of a zero-based vertex without changing mesh data.",
            "inspectEdge" => "Read endpoints and length of a preview halfedge index without changing mesh data.",
            "inspectFace" => "Read polygon corner indices, unit normal, and the arithmetic mean of corner positions.",
            "analyze" => "Measure surface area, bounds, and topology; volume is available for closed, consistently oriented meshes.",
            "exportObj" => "Serialize mesh vertices and polygon faces as OBJ text.",
            "exportJson" => "Serialize editable indexed vertices and polygon faces as JSON text.",
            _ => name,
        };
        register_untyped(registry, operator_info_with_outputs(&id, name, name, "emoji:🥽️", summary, inputs, outputs, &[group]), Box::new(MeshOperation(operation)), &produced);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
