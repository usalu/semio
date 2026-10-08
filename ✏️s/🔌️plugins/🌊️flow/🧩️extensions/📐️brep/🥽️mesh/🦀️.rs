//! 🥽️ Indexed mesh widgets with explicit polygon data and B-Rep preview conversion.
use super::*;
use neural_engine::{Atom, FieldSpec, Schema,OperatorJob};
use semio_framework_value::ValueType;
use semio_framework_3d::mesh::{EdgeId, FaceId, HalfedgeMesh, MeshKernelError, Vec3 as MeshVector, VertexId, WeldMode, MirrorAxis, MeshModelingJob, MeshModelingStep, MeshModelingProgress, MeshTessellationJob, MeshTessellationStep};
use std::collections::{BTreeMap, HashMap, HashSet};
use semio_framework_3d::brep::queries::tessellation::{TessellationJob,TessellationStep};
use semio_framework_mesh_engine::{MeshAttribute, MeshTexture, PolygonMeshSource};
use semio_framework_mesh_engine::io::text::{PolygonSourcePreparation, parse_polygon_mesh_source};

const LIMIT: usize = 100_000;
fn invalid(message: impl Into<String>) -> EvalError { EvalError::InvalidInput(message.into()) }
fn mesh_error(error: MeshKernelError) -> EvalError { invalid(error.to_string()) }
fn affine_matrix(input:&Dictionary)->Result<[f64;16],EvalError> {
    let list=input.get("matrix").and_then(Value::as_dictionary).ok_or_else(||invalid("matrix must be a list of 16 numbers"))?;
    if list.schema()!=Some("list") || list.len()!=17 {return Err(invalid("matrix must be a list of exactly 16 numbers"));}
    let mut matrix=[0.0;16];for (index,value) in matrix.iter_mut().enumerate() {*value=read_channel_number(list,&index.to_string())?;}Ok(matrix)
}
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
fn decode_mesh(text:&str)->Result<HalfedgeMesh,EvalError> {parse_polygon_mesh_source(text).map_err(invalid)?.mesh()}
trait PolygonMeshSourceGeometry {
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
        for attribute in attributes.values_mut() {
            let ids=match attribute.domain {semio_framework_mesh_engine::MeshAttributeDomain::Corner=>Some(&corner_ids),semio_framework_mesh_engine::MeshAttributeDomain::Edge=>Some(&edge_ids),_=>None};
            if let Some(ids)=ids {
                if attribute.indices.is_some() || ids.iter().enumerate().any(|(index,id)|index!=*id as usize) {
                    attribute.indices=Some(ids.iter().map(|id|attribute.indices.as_ref().map_or(*id,|indices|indices[*id as usize])).collect());
                }
            }
        }
        Ok(Self { vertices,faces,attributes,materials:mesh.materials().clone(),textures:mesh.textures().clone() })
    }
    fn mesh(self)->Result<HalfedgeMesh,EvalError> {let mut job=HalfedgeMesh::polygon_source_job(self).map_err(mesh_error)?;loop {match job.step(4096).map_err(mesh_error)? {MeshModelingStep::Done(mesh)=>return Ok(mesh),MeshModelingStep::Working(_)=>{},_=>return Err(invalid("mesh construction cancelled"))}}}
}
fn encode_mesh(mesh: &HalfedgeMesh) -> Result<String, EvalError> { Ok(semio_framework_mesh_engine::io::text::encode_polygon_mesh_source(&PolygonMeshSource::from_mesh(mesh)?)) }
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
    use neural_engine::OperatorJob;
    let mut job = MeshOperatorJob::output(mesh.clone())?;
    loop { if let neural_engine::OperatorJobStep::Done(output) = job.step(4096)? { return Ok(output); } }
}
fn analyze(mesh: &HalfedgeMesh) -> Result<Dictionary, EvalError> {
    use neural_engine::{OperatorJob,OperatorJobStep};
    let mut job=MeshOperatorJob::analysis(mesh.clone())?;
    loop { if let OperatorJobStep::Done(value)=job.step(4096)? { return Ok(value); } }
}

fn operator_progress(progress: MeshModelingProgress) -> neural_engine::OperatorProgress {
    neural_engine::OperatorProgress { units_done: progress.units_done, units_total: progress.units_total, phase: progress.phase }
}

fn mesh_retirement_grant(items:usize,bytes:usize)->semio_framework_value::retained_clone::RetainedCloneGrant {semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:items,maximum_copy_bytes:bytes,maximum_capacity_bytes:bytes,maximum_release_bytes:bytes,maximum_depth:256}}

enum MeshRetirement {
    Source(PolygonSourcePreparation),Modeling(MeshModelingJob),Tessellation(MeshTessellationJob),Output(MeshJobOutput),Transfer(semio_framework_3d::mesh::MeshTransfer),
    Kernel(semio_framework_3d::brep::engine::retirement::PayloadRetirement),Owned(Box<dyn semio_framework_value::ErasedSnapshotRetirement>),Empty,
}
impl MeshRetirement {
    fn terminal_is_empty(&self)->bool {match self {Self::Empty=>true,Self::Owned(owner)=>owner.terminal_is_empty(),Self::Kernel(owner)=>owner.terminal_is_empty(),_=>false}}
    fn next_close_byte_demand(&self)->usize {
        use semio_framework_value::retirement::owned_retirement_birth_bytes as birth;
        match self {Self::Source(_)=>birth::<PolygonSourcePreparation>(),Self::Modeling(job)=>job.retirement_birth_bytes(),Self::Tessellation(job)=>job.retirement_birth_bytes(),Self::Output(_)=>birth::<MeshJobOutput>(),Self::Transfer(_)=>birth::<semio_framework_3d::mesh::MeshTransfer>(),Self::Kernel(owner)=>owner.next_close_byte_demand(),Self::Owned(owner)=>{let copy=owner.next_copy_byte_demand().expect("mesh copy demand");let release=owner.next_release_byte_demand().expect("mesh release demand");copy.max(owner.next_capacity_byte_demand(copy.max(release)).expect("mesh capacity demand")).max(release)},Self::Empty=>0}
    }
    fn step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(),EvalError> {
        use semio_framework_value::retirement::admit_owned_retirement;
        if grant.maximum_items==0 {return Ok(());}
        match std::mem::replace(self,Self::Empty) {
            Self::Source(source)=>match admit_owned_retirement(source,grant) {Ok((owner,_))=>*self=Self::Owned(owner),Err((_,source))=>*self=Self::Source(source)},
            Self::Modeling(job)=>match job.into_retirement(grant) {Ok((owner,_))=>*self=Self::Owned(owner),Err((_,job))=>*self=Self::Modeling(job)},
            Self::Tessellation(job)=>match job.into_retirement(grant) {Ok((owner,_))=>*self=Self::Owned(owner),Err((_,job))=>*self=Self::Tessellation(job)},
            Self::Output(output)=>match admit_owned_retirement(output,grant) {Ok((owner,_))=>*self=Self::Owned(owner),Err((_,output))=>*self=Self::Output(output)},
            Self::Transfer(transfer)=>match admit_owned_retirement(transfer,grant) {Ok((owner,_))=>*self=Self::Owned(owner),Err((_,transfer))=>*self=Self::Transfer(transfer)},
            Self::Owned(mut owner)=>{let result=owner.close_step(grant);*self=Self::Owned(owner);result.map_err(|error|invalid(error.to_string()))?;},
            Self::Kernel(mut owner)=>{owner.close_step(grant.maximum_items,grant.maximum_release_bytes.min(grant.maximum_capacity_bytes));*self=Self::Kernel(owner);},
            Self::Empty=>{},
        }
        Ok(())
    }
}

struct MeshOperatorJob {
    job: Option<MeshModelingJob>,
    brep_job: Option<(Session,GeometryHandle,TessellationJob)>,
    import: Option<MeshImportState>,
    preparation: Option<MeshPreparation>,
    retirement: Option<MeshRetirement>,
    pending_output:Option<Dictionary>,
    value_retirement:neural_engine::ValueRetirement,
    fault:Option<EvalError>,
    modeling_base: usize,
    output: Option<MeshJobOutput>,
    progress: neural_engine::OperatorProgress,
    cancelled: bool,
}

struct MeshPreparation {
    operation:&'static str,
    session:Session,
    input:Option<Dictionary>,
    source:Option<PolygonSourcePreparation>,
    reconstruction:Option<MeshModelingJob>,
    next:Option<Box<MeshOperatorJob>>,
    input_retirement:neural_engine::ValueRetirement,
}

enum MeshJobOutput { Mesh(MeshOutputState), Analysis(MeshAnalysisState) }
struct MeshImportState {
    session: Session,
    tessellation: Option<MeshTessellationJob>,
    cursor: Option<semio_framework_3d::brep::engine::MeshImportCursor>,
    tolerance: f64,
    source_retirement: Option<MeshRetirement>,
    transfer_retirement: Option<MeshRetirement>,
    fault: Option<EvalError>,
}
impl MeshJobOutput {
    fn phase(&self)->&'static str { match self { Self::Mesh(value)=>value.phase(),Self::Analysis(value)=>value.phase() } }
    fn estimate(&self)->usize { match self { Self::Mesh(value)=>value.estimate(),Self::Analysis(value)=>value.estimate() } }
    fn advance(&mut self)->Result<Option<Dictionary>,EvalError> { match self { Self::Mesh(value)=>value.advance(),Self::Analysis(value)=>value.advance() } }
}

struct MeshAnalysisState {
    tessellation: MeshTessellationJob,
    triangles: Option<semio_framework_3d::mesh::MeshTransfer>,
    edges: HashMap<(u32,u32),(usize,i32)>,
    boundary: usize,
    non_manifold: usize,
    inconsistent: usize,
    phase: u8,
    face: usize,
    start: EdgeId,
    next: EdgeId,
    flipped: bool,
    corners: Vec<u32>,
    cursor: usize,
    min: [f32;3],
    max: [f32;3],
    area: f64,
    volume: f64,
    degenerate: usize,
}
impl MeshAnalysisState {
    fn new(mesh:HalfedgeMesh)->Result<Self,EvalError> {
        if mesh.vertex_count()<3 || mesh.face_count()==0 || mesh.vertex_count()>LIMIT || mesh.face_count()>LIMIT || mesh.halfedge_count()>LIMIT*6 { return Err(invalid("analysis requires geometry within mesh capacity")); }
        Ok(Self { tessellation:MeshTessellationJob::with_preview_capacity(mesh,64_000_000),triangles:None,edges:HashMap::new(),boundary:0,non_manifold:0,inconsistent:0,phase:0,face:0,start:EdgeId(0),next:EdgeId(0),flipped:false,corners:Vec::new(),cursor:0,min:[f32::INFINITY;3],max:[f32::NEG_INFINITY;3],area:0.0,volume:0.0,degenerate:0 })
    }
    fn phase(&self)->&'static str { ["mesh-analysis-corners","mesh-analysis-topology","mesh-analysis-bounds","mesh-analysis-tessellate","mesh-analysis-triangles","mesh-analysis-done"][self.phase as usize] }
    fn estimate(&self)->usize { self.tessellation.source().halfedge_count().saturating_mul(32).saturating_add(self.tessellation.source().vertex_count()).saturating_add(1) }
    fn advance(&mut self)->Result<Option<Dictionary>,EvalError> {
        let mesh=self.tessellation.source();
        match self.phase {
            0=>{
                if self.face==mesh.face_count() { self.phase=2;self.cursor=0; }
                else {
                    if self.corners.is_empty() { (self.start,self.flipped)=mesh.face_boundary(FaceId(self.face as u32)).map_err(mesh_error)?;self.next=self.start; }
                    let (vertex,next)=mesh.boundary_corner(self.next).map_err(mesh_error)?;self.corners.push(vertex.0);self.next=next;
                    if next==self.start { self.phase=1;self.cursor=0; }
                }
            }
            1=>{
                if self.cursor==self.corners.len() { self.corners.clear();self.face+=1;self.phase=0; }
                else {
                    let n=self.corners.len();let i=if self.flipped { n-1-self.cursor } else { self.cursor };let j=if self.flipped { (i+n-1)%n } else { (i+1)%n };
                    let (a,b)=(self.corners[i],self.corners[j]);let edge=self.edges.entry((a.min(b),a.max(b))).or_default();
                    self.boundary-=usize::from(edge.0==1);self.non_manifold-=usize::from(edge.0>2);self.inconsistent-=usize::from(edge.0==2 && edge.1!=0);
                    edge.0+=1;edge.1+=if a<b { 1 } else { -1 };
                    self.boundary+=usize::from(edge.0==1);self.non_manifold+=usize::from(edge.0>2);self.inconsistent+=usize::from(edge.0==2 && edge.1!=0);self.cursor+=1;
                }
            }
            2=>{
                if self.cursor==mesh.vertex_count() { self.phase=3; }
                else { let point=mesh.vertex_position(VertexId(self.cursor as u32)).map_err(mesh_error)?.0;for axis in 0..3 { self.min[axis]=self.min[axis].min(point[axis]);self.max[axis]=self.max[axis].max(point[axis]); }self.cursor+=1; }
            }
            3=>{ match self.tessellation.step(1).map_err(mesh_error)? { MeshTessellationStep::Done(transfer)=>{self.triangles=Some(transfer);self.cursor=0;self.phase=4;},MeshTessellationStep::Working(_)=>{},MeshTessellationStep::Cancelled(_)=>return Err(invalid("analysis cancelled")) } }
            4=>{
                let triangles=self.triangles.as_ref().unwrap();
                if self.cursor==triangles.indices.len() { self.phase=5; }
                else {
                    let point=|index:u32|[0,1,2].map(|axis|triangles.positions[index as usize*3+axis] as f64);
                    let [a,b,c]=[0,1,2].map(|i|point(triangles.indices[self.cursor+i]));let reference=mesh.vertex_position(VertexId(0)).map_err(mesh_error)?.0.map(f64::from);
                    let sub=|a:[f64;3],b:[f64;3]|[0,1,2].map(|i|a[i]-b[i]);let cross=|a:[f64;3],b:[f64;3]|[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];
                    let normal=cross(sub(b,a),sub(c,a));let area=normal[0].hypot(normal[1]).hypot(normal[2])/2.0;self.area+=area;self.degenerate+=usize::from(area==0.0);
                    self.volume+=sub(a,reference).into_iter().zip(cross(sub(b,reference),sub(c,reference))).map(|(a,b)|a*b).sum::<f64>()/6.0;self.cursor+=3;
                }
            }
            5=>{
                let mut report=Dictionary::new();
                for (key,value) in [("vertices",mesh.vertex_count() as f64),("faces",mesh.face_count() as f64),("edges",self.edges.len() as f64),("triangles",self.triangles.as_ref().unwrap().indices.len() as f64/3.0),("boundaryEdges",self.boundary as f64),("nonManifoldEdges",self.non_manifold as f64),("inconsistentEdges",self.inconsistent as f64),("degenerateTriangles",self.degenerate as f64),("area",self.area)] { report=report.insert(key,Value::Dictionary(number_dictionary(value))); }
                if self.boundary==0 && self.non_manifold==0 && self.inconsistent==0 && self.degenerate==0 { report=report.insert("volume",Value::Dictionary(number_dictionary(self.volume.abs()))); }
                return Ok(Some(report.insert("minimum",Value::Dictionary(point_dictionary(self.min.map(f64::from)))).insert("maximum",Value::Dictionary(point_dictionary(self.max.map(f64::from))))));
            }
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct MeshOutputState {
    json_only:bool,
    obj_only:bool,
    obj_uv:bool,
    tessellation: Option<MeshTessellationJob>,
    encoder: Option<MeshPackEncodingJob>,
    phase: u8,
    vertex: usize,
    face: usize,
    start: EdgeId,
    next: EdgeId,
    flipped: bool,
    corners: Vec<u32>,
    halfedges: Vec<u32>,
    edge_remap: HashMap<u32,u32>,
    serialized_corners:Vec<u32>,
    serialized_edges:Vec<u32>,
    corner_remapped:bool,
    edge_remapped:bool,
    metadata:semio_framework_mesh_engine::io::text::MeshMetadataCursor,
    corner_base: usize,
    preview_data: Option<semio_framework_mesh_engine::MeshData>,
    cursor: usize,
    data: String,
    bytes: Vec<u8>,
    preview: String,
}

impl MeshOutputState {
    fn new(mesh: HalfedgeMesh) -> Result<Self, EvalError> {
        if mesh.vertex_count() < 3 || mesh.face_count() == 0 { return Err(invalid("mesh operation produced empty geometry")); }
        if mesh.vertex_count() > LIMIT || mesh.face_count() > LIMIT || mesh.halfedge_count() > LIMIT * 6 { return Err(invalid("result exceeds mesh capacity (100000 vertices, 100000 faces, 600000 corners)")); }
        Ok(Self {json_only:false,obj_only:false,obj_uv:false,tessellation: Some(MeshTessellationJob::with_preview_capacity(mesh,64_000_000)), encoder: None, phase: 0, vertex: 0, face: 0, start: EdgeId(0), next: EdgeId(0), flipped: false, corners: Vec::new(), halfedges: Vec::new(), edge_remap: HashMap::new(),serialized_corners:Vec::new(),serialized_edges:Vec::new(),corner_remapped:false,edge_remapped:false,metadata:Default::default(), corner_base: 0, preview_data: None, cursor: 0, data: "{\"vertices\":[".into(), bytes: Vec::new(), preview: String::new() })
    }
    fn phase(&self) -> &'static str { ["mesh-output-vertices", "mesh-output-face-corners", "mesh-output-face-json", "mesh-output-tessellate", "mesh-output-edge-identities", "mesh-output-pack", "mesh-output-base64", "mesh-output-done","mesh-output-metadata","mesh-output-json-done","mesh-output-obj-uv-scan","mesh-output-obj-uv"][self.phase as usize] }
    fn estimate(&self) -> usize { let mesh = self.tessellation.as_ref().unwrap().source(); mesh.vertex_count().saturating_add(mesh.face_count().saturating_mul(4)).saturating_add(mesh.halfedge_count().saturating_mul(24)).saturating_add(1) }
    fn advance(&mut self) -> Result<Option<Dictionary>, EvalError> {
        match self.phase {
            0 => {
                let mesh = self.tessellation.as_ref().unwrap().source();
                if self.vertex < mesh.vertex_count() {
                    let point = mesh.vertex_position(VertexId(self.vertex as u32)).map_err(mesh_error)?.0;
                    if point.iter().any(|number| !number.is_finite()) { return Err(invalid("mesh operation produced non-finite coordinates")); }
                    if self.obj_only {self.data.push_str(&format!("v {} {} {}\n",point[0],point[1],point[2]));self.vertex+=1;if self.data.len()>16_000_000 {return Err(invalid("OBJ output exceeds 16 MB"));}return Ok(None);}if self.vertex > 0 { self.data.push(','); }
                    self.data.push_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::array(point.into_iter().map(|number| semio_framework_pack_json::Value::from(number as f64)))));
                    self.vertex += 1;
                } else {if self.obj_only {self.cursor=0;self.phase=10;}else {self.data.push_str("],\"faces\":[");self.phase=1;} }
            }
            1 => {
                let mesh = self.tessellation.as_ref().unwrap().source();
                if self.face == mesh.face_count() {if self.obj_only {self.phase=9;}else {self.data.push(']');self.phase=8;}}
                else {
                    if self.corners.is_empty() { (self.start,self.flipped) = mesh.face_boundary(FaceId(self.face as u32)).map_err(mesh_error)?; self.next = self.start; }
                    let (vertex,next) = mesh.boundary_corner(self.next).map_err(mesh_error)?;
                    self.corners.push(vertex.0); self.halfedges.push(self.next.0); self.next = next;
                    if self.next==self.start {if self.obj_only {self.data.push('f');}else {if self.face>0 {self.data.push(',');}self.data.push('[');}self.phase=2;self.cursor=0;}
                }
            }
            2 => {
                if self.cursor < self.corners.len() {
                    if !self.obj_only && self.cursor>0 {self.data.push(',');}
                    let index = if self.flipped { self.corners.len()-1-self.cursor } else { self.cursor };
                    let edge = self.halfedges[if self.flipped { (index+self.corners.len()-1)%self.corners.len() } else { index }];
                    if self.obj_only {self.data.push_str(&if self.obj_uv {format!(" {}/{}",self.corners[index]+1,self.halfedges[index]+1)}else {format!(" {}",self.corners[index]+1)});self.cursor+=1;if self.data.len()>16_000_000 {return Err(invalid("OBJ output exceeds 16 MB"));}return Ok(None);}
                    self.edge_remap.insert(edge,(self.corner_base+self.cursor) as u32);
                    self.corner_remapped|=self.serialized_corners.len()!=self.halfedges[index] as usize;self.edge_remapped|=self.serialized_edges.len()!=edge as usize;
                    self.serialized_corners.push(self.halfedges[index]);self.serialized_edges.push(edge);
                    self.data.push_str(&self.corners[index].to_string()); self.cursor += 1;
                } else {self.data.push(if self.obj_only {'\n'}else {']'});self.corner_base += self.corners.len(); self.corners.clear(); self.halfedges.clear(); self.face += 1; self.phase = 1; }
            }
            3 => {
                match self.tessellation.as_mut().unwrap().step(1).map_err(mesh_error)? {
                    MeshTessellationStep::Working(_) => {},
                    MeshTessellationStep::Cancelled(_) => return Err(invalid("mesh output cancelled")),
                    MeshTessellationStep::Done(transfer) => {
                        let preview = semio_framework_mesh_engine::MeshData { colors:transfer.colors,attributes:transfer.attributes,materials:transfer.materials,textures:transfer.textures,positions: transfer.positions, normals: transfer.normals, indices: transfer.indices, face_ids: transfer.face_ids, vertex_ids: transfer.vertex_ids, edge_positions: transfer.edge_positions, edge_ids: transfer.edge_ids, uvs: transfer.uvs, edge_uvs: transfer.edge_uvs, edge_is_seam: transfer.edge_is_seam, ..Default::default() };
                        self.preview_data = Some(preview); self.cursor = 0; self.phase = 4;
                    }
                }
            }
            4 => {
                let preview = self.preview_data.as_mut().unwrap();
                if self.cursor < preview.edge_ids.len() { let id = &mut preview.edge_ids[self.cursor]; *id = *self.edge_remap.get(id).ok_or_else(||invalid("preview edge identity is absent from serialized mesh"))?; self.cursor += 1; }
                else { self.encoder = Some(MeshPackEncodingJob::new(self.preview_data.take().unwrap(),64_000_000).map_err(invalid)?); self.phase = 5; }
            }
            5 => {
                if let Some(bytes) = self.encoder.as_mut().unwrap().step(1).map_err(invalid)? { self.bytes = bytes; self.encoder = None; self.phase = 6; self.cursor = 0; }
            }
            6 => {
                let end = (self.cursor+3072).min(self.bytes.len());
                self.preview.push_str(&encode_base64(&self.bytes[self.cursor..end])); self.cursor = end;
                if end == self.bytes.len() { self.phase = 7; }
            }
            7 => {
                let mesh = Dictionary::with_schema("mesh").insert("data", Value::Atom(Atom::String(std::mem::take(&mut self.data)))).insert("preview", Value::Atom(Atom::String(std::mem::take(&mut self.preview))));
                return Ok(Some(channel_output("meshOut",mesh)));
            }
            8 => { self.advance_metadata()?; }
            9=>return Ok(Some(channel_output("text",text_dictionary(std::mem::take(&mut self.data))))),
            10=>{let mesh=self.tessellation.as_ref().unwrap().source();if self.cursor==mesh.halfedge_count() {self.cursor=0;self.phase=if self.obj_uv {11}else {1};}else {self.obj_uv|=mesh.corner_uv(EdgeId(self.cursor as u32)).map_err(mesh_error)?!=[0.0,0.0];self.cursor+=1;}},
            11=>{let mesh=self.tessellation.as_ref().unwrap().source();if self.cursor==mesh.halfedge_count() {self.cursor=0;self.phase=1;}else {let uv=mesh.corner_uv(EdgeId(self.cursor as u32)).map_err(mesh_error)?;self.data.push_str(&format!("vt {} {}\n",uv[0],uv[1]));self.cursor+=1;}},
            _ => unreachable!(),
        }
        if self.data.len() > 16_000_000 { return Err(invalid("mesh output exceeds 16 MB")); }
        Ok(None)
    }

    fn advance_metadata(&mut self)->Result<(),EvalError> {
        let mesh=self.tessellation.as_ref().unwrap().source();
        if self.metadata.step(mesh.attributes(),mesh.materials(),mesh.textures(),None,self.corner_remapped.then_some(self.serialized_corners.as_slice()),self.edge_remapped.then_some(self.serialized_edges.as_slice()),&mut self.data).map_err(invalid)? {self.data.push('}');self.phase=if self.json_only {9}else {3};}
        Ok(())
    }
}


impl MeshOperatorJob {
    fn preparation(operation:&'static str,session:Session,input:&Dictionary)->Result<Self,EvalError> {
        Self::source_text(operation,input)?;
        let progress=neural_engine::OperatorProgress {phase:"mesh-source-parse",units_done:0,units_total:1};
        Ok(Self {job:None,brep_job:None,import:None,preparation:Some(MeshPreparation {operation,session,input:Some(input.clone()),source:Some(PolygonSourcePreparation::new()),reconstruction:None,next:None,input_retirement:Default::default()}),retirement:None,pending_output:None,value_retirement:Default::default(),fault:None,modeling_base:0,output:None,progress,cancelled:false})
    }
    fn source_text<'a>(operation:&str,input:&'a Dictionary)->Result<&'a str,EvalError> {if operation=="construct" {return input.get("data").and_then(Value::as_dictionary).and_then(|text|text.get("value")).and_then(Value::as_atom).and_then(Atom::as_str).ok_or_else(||invalid("data must be text"));}let mesh=input.get("mesh").and_then(Value::as_dictionary).filter(|mesh|mesh.schema()==Some("mesh")).ok_or_else(||invalid("mesh requires a mesh"))?;mesh.get("data").and_then(Value::as_atom).and_then(Atom::as_str).ok_or_else(||invalid("mesh data is missing"))}
    fn step_preparation(&mut self,budget:usize,bytes:usize)->Result<neural_engine::OperatorJobStep,EvalError> {
        if budget==0 || bytes==0 {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
        for _ in 0..budget {
            let state=self.preparation.as_mut().expect("mesh preparation");
            if self.cancelled || self.fault.is_some() {
                if self.retirement.is_none() {if let Some(source)=state.source.take() {self.retirement=Some(MeshRetirement::Source(source));}}
                if self.retirement.is_none() {if let Some(job)=state.reconstruction.take() {self.retirement=Some(MeshRetirement::Modeling(job));}}
                if let Some(input)=state.input.take() {state.input_retirement.push_dictionary(input);}
                if let Some(next)=&mut state.next {next.cancel();match next.close_step(1,bytes)? {neural_engine::OperatorJobStep::Working(_)=>continue,_=>state.next=None}}
            }
            if let Some(retirement)=&mut self.retirement {
                retirement.step(mesh_retirement_grant(1,bytes))?;
                if retirement.terminal_is_empty() {self.retirement=None;}
                self.progress.phase="mesh-source-retire";
            } else if !state.input_retirement.terminal_is_empty() {
                state.input_retirement.close_step(1,bytes);
                self.progress.phase="mesh-input-retire";
            } else if self.cancelled || self.fault.is_some() {
                self.preparation=None;
                if !self.cancelled {return Err(self.fault.take().expect("retired source failure"));}
                self.fault=None;
                return Ok(neural_engine::OperatorJobStep::Cancelled(self.progress));
            } else if let Some(source)=&mut state.source {
                let text=Self::source_text(state.operation,state.input.as_ref().unwrap())?;
                match source.step(text,semio_framework_mesh_engine::io::text::PolygonSourceGrant{maximum_units:1,maximum_projection_bytes:bytes,retirement:semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:4096,maximum_release_bytes:1024*1024,maximum_depth:256}}).map(|step|step.source) {
                    Ok(Some(source))=>{state.reconstruction=Some(HalfedgeMesh::polygon_source_job(source).map_err(mesh_error)?);self.retirement=Some(MeshRetirement::Source(state.source.take().unwrap()));self.progress.phase="mesh-source-capture";},
                    Ok(None)=>self.progress.phase=source.phase(),
                    Err(error)=>{self.fault=Some(invalid(error));self.retirement=Some(MeshRetirement::Source(state.source.take().unwrap()));state.input_retirement.push_dictionary(state.input.take().unwrap());self.progress.phase="mesh-fault-retire";},
                }
            } else if let Some(job)=&mut state.reconstruction {
                match job.step(1) {
                    Ok(MeshModelingStep::Working(progress))=>self.progress.phase=progress.phase,
                    Ok(MeshModelingStep::Done(mesh))=>{
                        state.next=Some(Box::new(if state.operation=="construct" {Self::output(mesh)?}else {Self::owned_plan(state.operation,state.session.clone(),state.input.as_ref().unwrap(),mesh)?}));
                        self.retirement=Some(MeshRetirement::Modeling(state.reconstruction.take().unwrap()));
                        state.input_retirement.push_dictionary(state.input.take().unwrap());
                    },
                    Ok(MeshModelingStep::Cancelled(_))=>unreachable!(),
                    Err(error)=>{self.fault=Some(mesh_error(error));self.retirement=Some(MeshRetirement::Modeling(state.reconstruction.take().unwrap()));state.input_retirement.push_dictionary(state.input.take().unwrap());self.progress.phase="mesh-fault-retire";},
                }
            } else {
                let mut next=*state.next.take().expect("prepared mesh continuation");
                let base=self.progress.units_done.saturating_add(1);
                next.modeling_base=next.modeling_base.saturating_add(base);
                next.progress.units_done=next.progress.units_done.saturating_add(base);
                next.progress.units_total=next.progress.units_total.saturating_add(base);
                *self=next;
                return Ok(neural_engine::OperatorJobStep::Working(self.progress));
            }
            if !self.cancelled {self.progress.units_done=self.progress.units_done.saturating_add(1);self.progress.units_total=self.progress.units_total.max(self.progress.units_done.saturating_add(1));}
        }
        Ok(neural_engine::OperatorJobStep::Working(self.progress))
    }
    fn close_cancelled(&mut self,items:usize,bytes:usize)->Result<neural_engine::OperatorJobStep,EvalError> {
        if items==0 || bytes==0 {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
        if self.retirement.is_none() {
            if let Some(job)=self.job.take() {self.retirement=Some(MeshRetirement::Modeling(job));}
            else if let Some((_,_,job))=self.brep_job.take() {let mut payloads=semio_framework_3d::brep::engine::retirement::PayloadRetirement::default();job.detach_retirement(&mut payloads);self.retirement=Some(MeshRetirement::Kernel(payloads));}
            else if let Some(output)=self.output.take() {self.retirement=Some(MeshRetirement::Output(output));}
        }
        if let Some(retirement)=&mut self.retirement {retirement.step(mesh_retirement_grant(items,bytes))?;if !retirement.terminal_is_empty() {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}self.retirement=None;if self.job.is_some() || self.brep_job.is_some() || self.output.is_some() || !self.value_retirement.terminal_is_empty() {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}}
        if !self.value_retirement.terminal_is_empty() {self.value_retirement.close_step(items,bytes);return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
        if self.output.is_some() {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
        Ok(neural_engine::OperatorJobStep::Cancelled(self.progress))
    }
    fn close_failed(&mut self,items:usize,bytes:usize)->Result<neural_engine::OperatorJobStep,EvalError> {
        match self.close_cancelled(items,bytes)? {
            neural_engine::OperatorJobStep::Cancelled(progress)=>if self.cancelled {self.fault=None;Ok(neural_engine::OperatorJobStep::Cancelled(progress))}else {Err(self.fault.take().expect("retired mesh failure"))},
            step=>Ok(step),
        }
    }
    fn to_brep(session:Session,mesh:HalfedgeMesh,tolerance:f64)->Result<Self,EvalError> {
        Ok(Self {job:None,brep_job:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),fault:None,import:Some(MeshImportState {session,tessellation:Some(MeshTessellationJob::with_preview_capacity(mesh,64_000_000)),cursor:None,tolerance,source_retirement:None,transfer_retirement:None,fault:None}),modeling_base:0,output:None,cancelled:false,progress:neural_engine::OperatorProgress {units_done:0,units_total:1,phase:"mesh-to-brep-tessellate"}})
    }
    fn step_import(&mut self,budget:usize)->Result<neural_engine::OperatorJobStep,EvalError> {
        self.step_import_with_bytes(budget,4096)
    }
    fn step_import_with_bytes(&mut self,budget:usize,bytes:usize)->Result<neural_engine::OperatorJobStep,EvalError> {
        if bytes==0 {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
        for _ in 0..budget {
            let import=self.import.as_mut().expect("retained import");
            if self.cancelled && import.tessellation.is_some() {import.source_retirement=Some(MeshRetirement::Tessellation(import.tessellation.take().unwrap()));}
            let retirement=if import.source_retirement.is_some() {&mut import.source_retirement}else {&mut import.transfer_retirement};
            if let Some(cursor)=retirement {
                cursor.step(mesh_retirement_grant(1,bytes))?;
                if cursor.terminal_is_empty() {*retirement=None;}
                self.progress.phase="mesh-to-brep-retire-source";
            } else if self.cancelled && import.cursor.is_none() {
                self.import=None;return Ok(neural_engine::OperatorJobStep::Cancelled(self.progress));
            } else if let Some(error)=import.fault.take() {self.import=None;return Err(error);
            } else if let Some(tessellation)=&mut import.tessellation {
                match tessellation.step(1) {
                    Err(error)=>{import.fault=Some(mesh_error(error));import.source_retirement=Some(MeshRetirement::Tessellation(import.tessellation.take().unwrap()));}
                    Ok(MeshTessellationStep::Cancelled(_))=>{import.source_retirement=Some(MeshRetirement::Tessellation(import.tessellation.take().unwrap()));self.cancelled=true;}
                    Ok(MeshTessellationStep::Working(_))=>{},
                    Ok(MeshTessellationStep::Done(mut mesh))=>{
                        let cursor=semio_framework_3d::brep::engine::MeshImportCursor::validate_admission_counts(mesh.positions.len(),mesh.indices.len(),import.tolerance).and_then(|()|semio_framework_3d::brep::engine::MeshImportCursor::new(std::mem::take(&mut mesh.positions),std::mem::take(&mut mesh.normals),std::mem::take(&mut mesh.indices),import.tolerance)).map_err(|error|invalid(error.to_string()));
                        import.source_retirement=Some(MeshRetirement::Tessellation(import.tessellation.take().unwrap()));
                        import.transfer_retirement=Some(MeshRetirement::Transfer(mesh));
                        match cursor {Ok(cursor)=>import.cursor=Some(cursor),Err(error)=>import.fault=Some(error)}
                        self.progress.phase="mesh-to-brep-admit";
                    }
                }
            } else {
                let cursor=import.cursor.as_mut().expect("retained mesh-I/O cursor");
                let result=import.session.close_mesh_import(cursor,1,bytes)?;
                self.progress.phase=cursor.progress().2;
                if cursor.retirement_complete() {self.import=None;return Ok(neural_engine::OperatorJobStep::Cancelled(self.progress));}
                if let Some(handle)=result {
                    let value=import.session.with_kernel_read(|kernel|geometry_dict(kernel,&handle))?;
                    self.import=None;self.progress.units_done=self.progress.units_done.saturating_add(1);self.progress.units_total=self.progress.units_done;
                    return Ok(neural_engine::OperatorJobStep::Done(channel_output("geometry",value)));
                }
            }
            self.progress.units_done=self.progress.units_done.saturating_add(1);self.progress.units_total=self.progress.units_total.max(self.progress.units_done.saturating_add(1));
        }
        Ok(neural_engine::OperatorJobStep::Working(self.progress))
    }
    fn from_brep(session:Session,shape:GeometryHandle,deflection:f64)->Result<Self,EvalError> {
        let job=session.with_kernel_read(|kernel|kernel.tessellate_job_sync(&shape,deflection).map_err(|error|map_kernel_error(&error)))?;
        let progress=job.progress();Ok(Self {job:None,brep_job:Some((session,shape,job)),import:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),fault:None,modeling_base:0,output:None,cancelled:false,progress:neural_engine::OperatorProgress {units_done:progress.units_done,units_total:progress.units_total,phase:progress.phase.tag()}})
    }
    fn modeling_progress(&self,progress:MeshModelingProgress)->neural_engine::OperatorProgress {let mut progress=operator_progress(progress);progress.units_done=progress.units_done.saturating_add(self.modeling_base);progress.units_total=progress.units_total.saturating_add(self.modeling_base);progress}
    fn analysis(mesh:HalfedgeMesh)->Result<Self,EvalError> {
        let output=MeshJobOutput::Analysis(MeshAnalysisState::new(mesh)?);
        Ok(Self { job:None,brep_job:None,import:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),fault:None,modeling_base:0,progress:neural_engine::OperatorProgress { units_done:0,units_total:output.estimate(),phase:output.phase() },output:Some(output),cancelled:false })
    }
    fn obj(mesh:HalfedgeMesh)->Result<Self,EvalError> {let mut job=Self::output(mesh)?;let Some(MeshJobOutput::Mesh(output))=&mut job.output else {unreachable!()};output.obj_only=true;output.data="# kernel_3d_mesh OBJ export\n".into();Ok(job)}
    fn json(mesh:HalfedgeMesh)->Result<Self,EvalError> {let mut job=Self::output(mesh)?;let Some(MeshJobOutput::Mesh(output))=&mut job.output else {unreachable!()};output.json_only=true;Ok(job)}
    fn output(mesh: HalfedgeMesh) -> Result<Self, EvalError> {
        let output = MeshJobOutput::Mesh(MeshOutputState::new(mesh)?);
        Ok(Self { job: None, brep_job:None,import:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),fault:None,modeling_base:0,progress: neural_engine::OperatorProgress { units_done: 0, units_total: output.estimate(), phase: output.phase() }, output: Some(output), cancelled: false })
    }
}

impl neural_engine::OperatorJob for MeshOperatorJob {
    fn step(&mut self, budget: usize) -> Result<neural_engine::OperatorJobStep, EvalError> {
        if self.preparation.is_some() {return self.step_preparation(budget,4096);}
        if self.fault.is_some() {return self.close_failed(budget,4096);}
        if self.cancelled && self.import.is_none() {return self.close_cancelled(budget,4096);}
        if self.import.is_some() { return self.step_import(budget); }
        if let Some(retirement)=&mut self.retirement {
            retirement.step(mesh_retirement_grant(budget,4096))?;
            if retirement.terminal_is_empty() {self.retirement=None;}
            self.progress.units_done=self.progress.units_done.saturating_add(budget);self.progress.units_total=self.progress.units_total.max(self.progress.units_done.saturating_add(1));self.modeling_base=self.modeling_base.saturating_add(budget);
            return Ok(neural_engine::OperatorJobStep::Working(self.progress));
        }
        if let Some(output)=self.pending_output.take() {self.progress.units_total=self.progress.units_done;return Ok(neural_engine::OperatorJobStep::Done(output));}
        if let Some((session,shape,job))=&mut self.brep_job {
            let step=session.with_kernel_read(|kernel|kernel.step_tessellation_job_sync(shape,job,budget).map_err(|error|invalid(error.to_string())))?;
            let (done,cancelled,progress)=match step {TessellationStep::Working(progress)=>(false,false,progress),TessellationStep::Done(progress)=>(true,false,progress),TessellationStep::Cancelled(progress)=>(false,true,progress)};
            self.progress=neural_engine::OperatorProgress {units_done:progress.units_done,units_total:progress.units_total,phase:progress.phase.tag()};
            if cancelled {self.cancel();return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
            if done {let (_,_,mut job)=self.brep_job.take().unwrap();let (mut mesh,_)=job.take_mesh().ok_or_else(||invalid("BRep tessellation completed without geometry"))?;let mut payloads=semio_framework_3d::brep::engine::retirement::PayloadRetirement::default();job.detach_retirement(&mut payloads);let next=HalfedgeMesh::indexed_triangle_job(std::mem::take(&mut mesh.position),std::mem::take(&mut mesh.index),std::mem::take(&mut mesh.normal));payloads.mesh_transfer(mesh);self.retirement=Some(MeshRetirement::Kernel(payloads));let job=next.map_err(mesh_error)?;self.modeling_base=self.progress.units_done;self.progress=self.modeling_progress(job.progress());self.job=Some(job);}
            return Ok(neural_engine::OperatorJobStep::Working(self.progress));
        }
        if self.output.is_none() {
            let job = self.job.as_mut().ok_or_else(|| invalid("mesh job is retired"))?;
            let step=match job.step(budget) {Ok(step)=>step,Err(error)=>{self.fault=Some(mesh_error(error));self.retirement=Some(MeshRetirement::Modeling(self.job.take().unwrap()));self.progress.phase="mesh-fault-retire";return Ok(neural_engine::OperatorJobStep::Working(self.progress));}};
            match step {
            MeshModelingStep::Working(progress) => {
                self.progress = self.modeling_progress(progress);
                return Ok(neural_engine::OperatorJobStep::Working(self.progress));
            }
            MeshModelingStep::Cancelled(progress) => {
                self.progress = self.modeling_progress(progress); self.cancelled = true; self.job = None;
                return Ok(neural_engine::OperatorJobStep::Cancelled(self.progress));
            }
            MeshModelingStep::Done(mesh) => {
                let progress=job.progress();self.progress=neural_engine::OperatorProgress {units_done:progress.units_done.saturating_add(self.modeling_base),units_total:progress.units_total.saturating_add(self.modeling_base),phase:progress.phase};
                self.retirement=Some(MeshRetirement::Modeling(self.job.take().unwrap()));self.output = Some(MeshJobOutput::Mesh(MeshOutputState::new(mesh)?));
                self.progress.units_total = self.progress.units_done.saturating_add(self.output.as_ref().unwrap().estimate());
                self.progress.phase = self.output.as_ref().unwrap().phase();
                return Ok(neural_engine::OperatorJobStep::Working(self.progress));
            }
            }
        }
        for _ in 0..budget {
            let output = self.output.as_mut().unwrap();
            let result=match output.advance() {Ok(result)=>result,Err(error)=>{self.fault=Some(error);self.retirement=Some(MeshRetirement::Output(self.output.take().unwrap()));self.progress.phase="mesh-fault-retire";return Ok(neural_engine::OperatorJobStep::Working(self.progress));}};
            self.progress.units_done = self.progress.units_done.saturating_add(1);
            self.progress.units_total = if result.is_some() { self.progress.units_done } else { self.progress.units_total.max(self.progress.units_done.saturating_add(1)) };
            self.progress.phase = output.phase();
            if let Some(result)=result {self.pending_output=Some(result);self.retirement=Some(MeshRetirement::Output(self.output.take().unwrap()));self.progress.phase="mesh-output-retire";return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
        }
        Ok(neural_engine::OperatorJobStep::Working(self.progress))
    }
    fn next_close_byte_demand(&self)->usize {
        if let Some(retirement)=&self.retirement {return retirement.next_close_byte_demand();}
        if let Some(import)=&self.import {if let Some(retirement)=import.source_retirement.as_ref().or(import.transfer_retirement.as_ref()) {return retirement.next_close_byte_demand();}if let Some(job)=&import.tessellation {return job.retirement_birth_bytes();}}
        if let Some(state)=&self.preparation {if state.source.is_some() {return semio_framework_value::retirement::owned_retirement_birth_bytes::<PolygonSourcePreparation>();}if let Some(job)=&state.reconstruction {return job.retirement_birth_bytes();}if let Some(next)=&state.next {return next.next_close_byte_demand();}}
        if let Some(job)=&self.job {return job.retirement_birth_bytes();}
        if self.brep_job.is_some() {return 1;}
        if self.output.is_some() {return semio_framework_value::retirement::owned_retirement_birth_bytes::<MeshJobOutput>();}
        self.value_retirement.next_close_byte_demand().expect("mesh value release demand")
    }
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<neural_engine::OperatorJobStep,EvalError> {
        if self.preparation.is_some() {return self.step_preparation(maximum_items,maximum_bytes);}
        if self.import.is_some() {return self.step_import_with_bytes(maximum_items,maximum_bytes);}
        if self.fault.is_some() {return self.close_failed(maximum_items,maximum_bytes);}
        if self.cancelled {return self.close_cancelled(maximum_items,maximum_bytes);}
        if maximum_bytes==0 {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
        self.step(maximum_items)
    }
    fn progress(&self) -> neural_engine::OperatorProgress { self.progress }
    fn cancel(&mut self) {
        if self.cancelled {return;}
        if self.preparation.is_some() {self.cancelled=true;return;}
        if let Some(import)=&mut self.import {self.cancelled=true;if let Some(cursor)=&mut import.cursor {cursor.cancel();}return;}
        if self.cancelled || (self.output.is_none() && self.job.is_none() && self.brep_job.is_none() && self.retirement.is_none() && self.pending_output.is_none()) { return; }
        if let Some(job) = &mut self.job { job.cancel(); let progress=job.progress();self.progress=neural_engine::OperatorProgress {units_done:progress.units_done.saturating_add(self.modeling_base),units_total:progress.units_total.saturating_add(self.modeling_base),phase:progress.phase}; }
        if let Some((_,_,job))=&mut self.brep_job {job.cancel();}
        self.cancelled=true;
        if let Some(output)=self.pending_output.take() {self.value_retirement.push_dictionary(output);}

    }
}

enum MeshEvaluation { Mesh(HalfedgeMesh), Value(Dictionary) }

struct MeshOperation(&'static str, SessionCapture);
impl Operator for MeshOperation {
    retire_geometry_capture!(1);
    fn step_plan(&self, input: &Dictionary) -> Result<Option<Box<dyn neural_engine::OperatorJob>>, EvalError> {
        if self.0=="transform" {HalfedgeMesh::validate_affine_matrix(affine_matrix(input)?).map_err(mesh_error)?;}
        if self.0=="construct" {return Ok(Some(Box::new(MeshOperatorJob::preparation(self.0,self.1.clone(),input)?)));}
        let primitive=match self.0 {
            "box"=>Some(HalfedgeMesh::box_primitive_job(positive(input,"width")?,positive(input,"height")?,positive(input,"depth")?)),
            "plane"=>Some(HalfedgeMesh::plane_primitive_job(positive(input,"width")?,positive(input,"depth")?)),
            "cylinder"=>Some(HalfedgeMesh::cylinder_primitive_job(positive(input,"radius")?,positive(input,"height")?,count(input,"segments",3,1024)?)),
            "cone"=>Some(HalfedgeMesh::cone_primitive_job(positive(input,"radius")?,positive(input,"height")?,count(input,"segments",3,1024)?)),
            "sphere"=>Some(HalfedgeMesh::sphere_primitive_job(positive(input,"radius")?,count(input,"subdivisions",0,5)?)),_=>None,
        };if let Some(job)=primitive {let job=job.map_err(mesh_error)?;let progress=operator_progress(job.progress());return Ok(Some(Box::new(MeshOperatorJob {job:Some(job),brep_job:None,import:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),fault:None,modeling_base:0,output:None,progress,cancelled:false})));}
        if self.0=="fromBrep" {return Ok(Some(Box::new(MeshOperatorJob::from_brep(self.1.clone(),read_geometry(input,"geometry")?,positive(input,"deflection")? as f64)?)));}
        if matches!(self.0,"toBrep"|"analyze"|"exportObj"|"exportJson") {return Ok(Some(Box::new(MeshOperatorJob::preparation(self.0,self.1.clone(),input)?)));}
        if !matches!(self.0, "bevel" | "decimate" | "mirror" | "mergeVertices" | "mergeCoplanar" | "dissolveEdges" | "dissolveVertices" | "weld" | "subdivide" | "loopCut" | "knifeCut" | "extrude" | "inset" | "orient" | "fillHoles" | "transform" | "translate" | "rotate" | "scale" | "translateComponents" | "rotateComponents" | "scaleComponents" | "moveVertices" | "triangulate" | "snapVertices" | "moveProportional" | "deleteFaces" | "flip") {
            if matches!(self.0,"inspectVertex" | "inspectEdge" | "inspectFace" | "analyze" | "exportObj" | "exportJson" | "toBrep") { return Ok(None); }
            let MeshEvaluation::Mesh(mesh) = self.prepare(input)? else { return Err(invalid("mesh operator produced an unexpected value")); };
            return Ok(Some(Box::new(MeshOperatorJob::output(mesh)?)));
        }
        Ok(Some(Box::new(MeshOperatorJob::preparation(self.0,self.1.clone(),input)?)))
    }

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        if let Some(mut job)=self.step_plan(input)? {loop {match job.step(4096)? {neural_engine::OperatorJobStep::Done(value)=>return Ok(value),neural_engine::OperatorJobStep::Cancelled(_)=>return Err(invalid("mesh operation cancelled")),neural_engine::OperatorJobStep::Working(_)=>{}}}}
        match self.prepare(input)? { MeshEvaluation::Mesh(mesh) => mesh_output(&mesh), MeshEvaluation::Value(output) => Ok(output) }
    }
}

impl MeshOperatorJob {
    fn owned_plan(operation:&'static str,session:Session,input:&Dictionary,mesh:HalfedgeMesh)->Result<Self,EvalError> {
        match operation {
            "toBrep"=>return Self::to_brep(session,mesh,positive(input,"tolerance")? as f64),
            "analyze"=>return Self::analysis(mesh),
            "exportObj"=>return Self::obj(mesh),
            "exportJson"=>return Self::json(mesh),
            _=>{},
        }
        let source_vertices=mesh.vertex_count();let source_corners=mesh.halfedge_count();let source_faces=mesh.face_count();
        let job = if matches!(operation,"snapVertices"|"moveProportional") {let ids=selection(input,"selection",source_vertices)?.into_iter().map(VertexId).collect();if operation=="snapVertices" {mesh.snap_vertices_job_owned(ids,positive(input,"grid")?).map_err(mesh_error)?}else {mesh.move_proportional_job_owned(ids,vector(input,"offset")?,vector(input,"center")?,positive(input,"radius")?).map_err(mesh_error)?}}else if matches!(operation,"translateComponents"|"rotateComponents"|"scaleComponents"|"moveVertices") {
            let (vertices,edges,faces)=if operation=="moveVertices" {(selection(input,"vertices",source_vertices)?.into_iter().map(VertexId).collect(),Vec::new(),Vec::new())}else {match read_text(input,"mode")?.as_str() {
                "vertex"=>(selection(input,"selection",source_vertices)?.into_iter().map(VertexId).collect(),Vec::new(),Vec::new()),
                "edge"=>(Vec::new(),selection(input,"selection",source_corners)?.into_iter().map(EdgeId).collect(),Vec::new()),
                "face"=>(Vec::new(),Vec::new(),selection(input,"selection",source_faces)?.into_iter().map(FaceId).collect()),
                _=>return Err(invalid("component mode must be vertex, edge, or face")),
            }};
            if matches!(operation,"translateComponents"|"moveVertices") {mesh.move_components_job_owned(vertices,edges,faces,vector(input,"offset")?).map_err(mesh_error)?}else {
                let pivot=match read_text(input,"pivot")?.as_str() {"point"=>Some(vector(input,"center")?),"selection"=>None,_=>return Err(invalid("pivot must be selection or point"))};
                if operation=="rotateComponents" {mesh.rotate_components_job_owned(vertices,edges,faces,vector(input,"axis")?,scalar(input,"angle")?,pivot).map_err(mesh_error)?}else {mesh.scale_components_job_owned(vertices,edges,faces,vector(input,"factor")?,pivot).map_err(mesh_error)?}
            }
        }else if operation=="mergeCoplanar" {mesh.merge_coplanar_faces_job_owned().map_err(mesh_error)?}
        else if operation=="dissolveEdges" {let ids=selection(input,"edges",source_corners)?.into_iter().map(EdgeId).collect::<Vec<_>>();mesh.dissolve_edges_job_owned(&ids).map_err(mesh_error)?}
        else if operation=="dissolveVertices" {let ids=selection(input,"vertices",source_vertices)?.into_iter().map(VertexId).collect::<Vec<_>>();mesh.dissolve_vertices_job_owned(&ids).map_err(mesh_error)?}
        else if operation=="weld" {mesh.weld_coincident_vertices_job_owned(positive(input,"tolerance")?).map_err(mesh_error)?}
        else if operation=="transform" {mesh.affine_transform_job_owned(affine_matrix(input)?).map_err(mesh_error)?}
        else if operation=="translate" {mesh.translate_job_owned(vector(input,"offset")?).map_err(mesh_error)?}
        else if operation=="rotate" {mesh.rotate_job_owned(vector(input,"axis")?,scalar(input,"angle")?).map_err(mesh_error)?}
        else if operation=="scale" {let factor=vector(input,"factor")?;mesh.scale_job_owned(factor,factor.0.iter().filter(|x|**x<0.0).count()%2==1).map_err(mesh_error)?}
        else if operation=="loopCut" {let ids=selection(input,"edges",source_corners)?.into_iter().map(EdgeId).collect::<Vec<_>>();mesh.loop_cut_job_owned(&ids,count(input,"cuts",1,256)?).map_err(mesh_error)?}
        else if operation=="knifeCut" {mesh.knife_cut_job_owned(FaceId(count(input,"face",0,source_faces.saturating_sub(1) as u32)?),vector(input,"start")?,vector(input,"end")?).map_err(mesh_error)?}
        else if matches!(operation,"extrude"|"inset") {let ids=selection(input,"faces",source_faces)?.into_iter().map(FaceId).collect::<Vec<_>>();if operation=="extrude" {mesh.extrude_faces_job_owned(&ids,scalar(input,"distance")?).map_err(mesh_error)?}else {mesh.inset_faces_job_owned(&ids,positive(input,"amount")?).map_err(mesh_error)?}}
        else if operation=="orient" {mesh.orient_faces_job_owned().map_err(mesh_error)?}
        else if operation=="fillHoles" {mesh.fill_holes_job_owned().map_err(mesh_error)?}
        else if operation=="flip" {let ids=selection(input,"faces",source_faces)?.into_iter().map(FaceId).collect::<Vec<_>>();mesh.flip_faces_job_owned(&ids).map_err(mesh_error)?}
        else if operation=="deleteFaces" {let ids=selection(input,"faces",source_faces)?.into_iter().map(FaceId).collect::<Vec<_>>();mesh.delete_faces_job_owned(&ids).map_err(mesh_error)?}
        else if operation=="triangulate" {mesh.triangulate_job_owned().map_err(mesh_error)?}
        else if operation == "bevel" {
            let edges = selection(input, "edges", source_corners)?.into_iter().map(EdgeId).collect::<Vec<_>>();
            mesh.bevel_job_owned(&edges, positive(input, "amount")?, count(input, "segments", 1, 64)?).map_err(mesh_error)?
        } else if operation == "decimate" {
            let ratio = positive(input, "ratio")?;
            if ratio > 1.0 { return Err(invalid("decimation ratio must be at most one")); }
            mesh.decimate_job_owned(ratio).map_err(mesh_error)?
        } else if operation == "mirror" {
            let axis = match read_text(input,"axis")?.as_str() { "x" => MirrorAxis::X, "y" => MirrorAxis::Y, "z" => MirrorAxis::Z, _ => return Err(invalid("mirror axis must be x, y, or z")) };
            let tolerance = scalar(input,"tolerance")?;
            if tolerance < 0.0 { return Err(invalid("mirror tolerance must be nonnegative")); }
            mesh.mirror_job_owned(axis,tolerance).map_err(mesh_error)?
        } else if operation == "subdivide" {
            let ids = selection(input,"faces",source_faces)?.into_iter().map(FaceId).collect::<Vec<_>>();
            mesh.subdivide_faces_job_owned(&ids).map_err(mesh_error)?
        } else {
            let ids = selection(input,"selection",source_vertices)?.into_iter().map(VertexId).collect::<Vec<_>>();
            let mode = match read_text(input,"mode")?.as_str() { "first"=>WeldMode::First,"center"=>WeldMode::Center,"distance"=>WeldMode::ByDistance,_=>return Err(invalid("merge mode must be first, center, or distance")) };
            let tolerance = scalar(input,"tolerance")?;
            if tolerance < 0.0 { return Err(invalid("merge tolerance must be nonnegative")); }
            mesh.merge_vertices_job_owned(&ids,mode,tolerance).map_err(mesh_error)?
        };
        let progress = operator_progress(job.progress());
        Ok(MeshOperatorJob { job: Some(job), brep_job:None,import:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),fault:None,modeling_base:0,output: None, progress, cancelled: false })
    }
}

impl MeshOperation {
    fn prepare(&self, input: &Dictionary) -> Result<MeshEvaluation, EvalError> {
        let mut mesh = match self.0 {
            "construct" => decode_mesh(&read_text(input, "data")?)?,
            "box" => HalfedgeMesh::box_prim(positive(input, "width")?, positive(input, "height")?, positive(input, "depth")?).map_err(mesh_error)?,
            "plane" => HalfedgeMesh::plane_prim(positive(input, "width")?, positive(input, "depth")?).map_err(mesh_error)?,
            "sphere" => HalfedgeMesh::ico_sphere_prim(positive(input, "radius")?, count(input, "subdivisions", 0, 5)?).map_err(mesh_error)?,
            "cylinder" => HalfedgeMesh::cylinder_prim(positive(input, "radius")?, positive(input, "height")?, count(input, "segments", 3, 1024)?).map_err(mesh_error)?,
            "cone" => HalfedgeMesh::cone_prim(positive(input, "radius")?, positive(input, "height")?, count(input, "segments", 3, 1024)?).map_err(mesh_error)?,
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
                    _ => mesh.delete_faces(&ids).map_err(mesh_error)?,
                }
            }
            "triangulate" => mesh.triangulate().map_err(mesh_error)?,
            "weld" => { mesh.weld_coincident_vertices(positive(input, "tolerance")?).map_err(mesh_error)?; }
            "orient" => { mesh.orient_faces_consistently().map_err(mesh_error)?; }
            "fillHoles" => { mesh.fill_holes().map_err(mesh_error)?; }
            "inspectVertex" => {
                let id = VertexId(count(input, "index", 0, mesh.vertex_count().saturating_sub(1) as u32)?);
                return Ok(MeshEvaluation::Value(channel_output("point", point_dictionary(mesh.vertex_position(id).map_err(mesh_error)?.0.map(f64::from)))));
            }
            "inspectEdge" => {
                let id = EdgeId(count(input, "index", 0, mesh.halfedge_count().saturating_sub(1) as u32)?);
                let (a, b) = mesh.edge_endpoints(id).map_err(mesh_error)?;
                let start = mesh.vertex_position(a).map_err(mesh_error)?.0.map(f64::from);
                let end = mesh.vertex_position(b).map_err(mesh_error)?.0.map(f64::from);
                let length = (end[0] - start[0]).hypot(end[1] - start[1]).hypot(end[2] - start[2]);
                return Ok(MeshEvaluation::Value(Dictionary::new().insert("start", Value::Dictionary(point_dictionary(start))).insert("end", Value::Dictionary(point_dictionary(end))).insert("length", Value::Dictionary(number_dictionary(length)))));
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
                let vertices = semio_framework_pack_json::to_string(&semio_framework_pack_json::array(ids.iter().map(|id| semio_framework_pack_json::Value::from(id.0))));
                return Ok(MeshEvaluation::Value(Dictionary::new().insert("vertices", Value::Dictionary(text_dictionary(vertices))).insert("normal", Value::Dictionary(vector_dictionary(normal))).insert("center", Value::Dictionary(point_dictionary(center)))));
            }
            "analyze" => return analyze(&mesh).map(MeshEvaluation::Value),
            "exportObj" => return Ok(MeshEvaluation::Value(channel_output("text", text_dictionary(mesh.to_obj().map_err(mesh_error)?)))),
            "exportJson" => return Ok(MeshEvaluation::Value(channel_output("text", text_dictionary(encode_mesh(&mesh)?)))),
            _ => {}
        }
        if mesh.vertex_count() > LIMIT || mesh.face_count() > LIMIT { return Err(invalid("result exceeds mesh capacity")); }
        Ok(MeshEvaluation::Mesh(mesh))
    }
}

/// 🥽️ Declares the representation fidelity of each owned polygon operator.
pub(super) fn operation_quality(operation: &str) -> &'static str {
    match operation { "fromBrep" => "TessellatedMesh", "toBrep" => "MeshDerivedBRep", _ => "PolygonMesh" }
}

pub(super) fn register_mesh(registry: &mut Registry, session: &Session) {
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
        ("transform", "Transform Mesh", "Mesh Editing", &[]),
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
        if operation == "construct" { inputs.push(ChannelSpec::text_default("data", r#"{"vertices":[[0,0,0],[1,0,0],[0,1,0]],"faces":[[0,1,2]]}"#, &[&id])); }
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
            "transform"=>{let identity=[1.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,1.0];let list=identity.into_iter().enumerate().fold(Dictionary::with_schema("list"),|list,(index,value)|list.insert(index.to_string(),Value::Dictionary(number_dictionary(value))));let mut channel=ChannelSpec::named("MA","mat","matrix","Matrix").with_operators(vec![id.clone()]).with_value_types(&["list"]).with_item_types(&["number"]).with_default(Value::Dictionary(list));channel.cardinality=neural_engine::Cardinality::Exactly(16);inputs.push(channel);},
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
            "transform" => "Apply a column-major affine matrix with inverse-transpose normals and reflection winding repair.",
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
        register_untyped(registry, operator_info_with_outputs(&id, name, name, "emoji:🥽️", &format!("{summary} [quality:{}]", operation_quality(operation)), inputs, outputs, &[group]), Box::new(MeshOperation(operation, session.capture())), &produced);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

impl semio_framework_value::retirement::RetireOwned for MeshJobOutput {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {match self {Self::Mesh(value)=>semio_framework_value::retirement::deferred(value),Self::Analysis(value)=>semio_framework_value::retirement::deferred(value)}}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(match self {Self::Mesh(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::Analysis(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value)})}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for MeshOutputState {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.tessellation),semio_framework_value::retirement::deferred(self.encoder),semio_framework_value::retirement::deferred(self.corners),semio_framework_value::retirement::deferred(self.halfedges),semio_framework_value::retirement::deferred(self.edge_remap),semio_framework_value::retirement::deferred(self.serialized_corners),semio_framework_value::retirement::deferred(self.serialized_edges),semio_framework_value::retirement::deferred(self.metadata),semio_framework_value::retirement::deferred(self.preview_data),semio_framework_value::retirement::deferred(self.data),semio_framework_value::retirement::deferred(self.bytes),semio_framework_value::retirement::deferred(self.preview)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.tessellation),semio_framework_value::retirement::deferred_birth_bytes_for(&self.encoder),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.halfedges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edge_remap),semio_framework_value::retirement::deferred_birth_bytes_for(&self.serialized_corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.serialized_edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.metadata),semio_framework_value::retirement::deferred_birth_bytes_for(&self.preview_data),semio_framework_value::retirement::deferred_birth_bytes_for(&self.data),semio_framework_value::retirement::deferred_birth_bytes_for(&self.bytes),semio_framework_value::retirement::deferred_birth_bytes_for(&self.preview)])}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for MeshAnalysisState {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.tessellation),semio_framework_value::retirement::deferred(self.triangles),semio_framework_value::retirement::deferred(self.edges),semio_framework_value::retirement::deferred(self.corners)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.tessellation),semio_framework_value::retirement::deferred_birth_bytes_for(&self.triangles),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corners)])}
    fn controlled_retirement_supported()->bool {true}
}
