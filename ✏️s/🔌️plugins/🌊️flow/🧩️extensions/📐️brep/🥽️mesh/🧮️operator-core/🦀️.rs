//! 🧮️ The mesh operator job, its retained state, retirement cursors and the operation implementations.

use semio_s_spatial_kernel_semio_session::*;
use flow_extension_sdk::mesh::*;
use neural_engine::channel_output;
use neural_engine::Dictionary;
use neural_engine::EvalError;
use neural_engine::Operator;
use neural_engine::Value;
use semio_framework_3d_brep_engine::GeometryHandle;
use neural_engine::Atom;
use neural_engine::OperatorJob;
use semio_framework_value::ValueError;
use semio_framework_value::RetirementDemand;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_3d::mesh::EdgeId;
use semio_framework_3d::mesh::FaceId;
use semio_framework_3d::mesh::HalfedgeMesh;
use semio_framework_3d::mesh::VertexId;
use semio_framework_3d::mesh::WeldMode;
use semio_framework_3d::mesh::MirrorAxis;
use semio_framework_3d::mesh::MeshModelingOps;
use semio_framework_3d::mesh::MeshModelingJob;
use semio_framework_3d::mesh::MeshModelingStep;
use semio_framework_3d::mesh::MeshModelingProgress;
use semio_framework_3d::mesh::MeshTessellationJob;
use semio_framework_3d::mesh::MeshTessellationStep;
use std::collections::HashMap;
use semio_framework_3d_brep_tessellation::TessellationJob;
use semio_framework_3d_brep_tessellation::TessellationStep;
use semio_framework_mesh_engine::HistoryFoldIndex;
use semio_framework_mesh_engine::io::text::PolygonSourcePreparation;
use super::{LIMIT, add_progress, affine_matrix, analyze, component_pivot, component_vertices, count, decode_mesh, drive_job, encode_mesh, invalid, mesh_error, mesh_output, modeling_slice, operator_progress, positive, read_mesh, scalar, selection, sync_grant, tessellation_slice, turn_grant, value_error, vector};


pub enum MeshRetirement {
    Source(PolygonSourcePreparation),Modeling(MeshModelingJob),Tessellation(MeshTessellationJob),Output(MeshJobOutput),Transfer(semio_framework_3d::mesh::MeshTransfer),
    Kernel(MeshKernelRetirement),Owned(Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>),Empty,
}
pub struct MeshKernelRetirement {
    pub payload:ControlledRetirement<(TessellationJob,Option<semio_framework_3d_brep_representation::contract::MeshTransfer>,String)>,
    pub session:SessionCapture,
}
impl MeshKernelRetirement {
    pub fn new(session:Session,shape:GeometryHandle,job:TessellationJob,mesh:Option<semio_framework_3d_brep_representation::contract::MeshTransfer>)->Self {Self {payload:ControlledRetirement::new((job,mesh,shape.0)).unwrap_or_else(|_|panic!("original BRep mesh owner requires typed retirement")),session:session.capture()}}
    pub fn terminal_is_empty(&self)->bool {self.payload.terminal_is_empty() && self.session.terminal_is_empty()}
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        if !self.payload.terminal_is_empty() {return Ok(RetirementDemand {copy_bytes:self.payload.next_copy_byte_demand()?,capacity_bytes:self.payload.next_capacity_byte_demand(copy)?,release_bytes:self.payload.next_release_byte_demand()?,depth:self.payload.next_depth_demand()?});}
        Ok(RetirementDemand {copy_bytes:self.session.next_close_copy_byte_demand()?,capacity_bytes:self.session.next_close_capacity_byte_demand(copy)?,release_bytes:self.session.next_close_release_byte_demand()?,depth:self.session.next_close_depth_demand()?})
    }
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if !self.payload.terminal_is_empty() {return self.payload.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        self.session.close_step(grant)
    }
}
impl MeshRetirement {
    pub fn terminal_is_empty(&self)->bool {match self {Self::Empty|Self::Owned(None)=>true,Self::Kernel(owner)=>owner.terminal_is_empty(),_=>false}}
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        use semio_framework_value::retirement::owned_retirement_birth_bytes as birth;
        let capacity=match self {Self::Source(_)=>birth::<PolygonSourcePreparation>(),Self::Modeling(job)=>job.retirement_birth_bytes(),Self::Tessellation(job)=>job.retirement_birth_bytes(),Self::Output(_)=>birth::<MeshJobOutput>(),Self::Transfer(_)=>birth::<semio_framework_3d::mesh::MeshTransfer>(),Self::Kernel(owner)=>return owner.demands(copy),Self::Owned(Some(owner))=>return semio_framework_value::factory_ticket_demands(owner,copy),Self::Owned(None)|Self::Empty=>return Ok(Default::default())};
        Ok(RetirementDemand {capacity_bytes:capacity,depth:1,..Default::default()})
    }
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        use semio_framework_value::retirement::admit_owned_retirement;
        if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0 {return Ok(RetainedCloneStep::Progress(Default::default()));}
        match std::mem::replace(self,Self::Empty) {
            Self::Source(source)=>match admit_owned_retirement(source,grant) {Ok((owner,receipt))=>{*self=Self::Owned(Some(owner));Ok(RetainedCloneStep::Progress(receipt))},Err((error,source))=>{*self=Self::Source(source);Err(error)}},
            Self::Modeling(job)=>match job.into_retirement(grant) {Ok((owner,receipt))=>{*self=Self::Owned(Some(owner));Ok(RetainedCloneStep::Progress(receipt))},Err((error,job))=>{*self=Self::Modeling(job);Err(error)}},
            Self::Tessellation(job)=>match job.into_retirement(grant) {Ok((owner,receipt))=>{*self=Self::Owned(Some(owner));Ok(RetainedCloneStep::Progress(receipt))},Err((error,job))=>{*self=Self::Tessellation(job);Err(error)}},
            Self::Output(output)=>match admit_owned_retirement(output,grant) {Ok((owner,receipt))=>{*self=Self::Owned(Some(owner));Ok(RetainedCloneStep::Progress(receipt))},Err((error,output))=>{*self=Self::Output(output);Err(error)}},
            Self::Transfer(transfer)=>match admit_owned_retirement(transfer,grant) {Ok((owner,receipt))=>{*self=Self::Owned(Some(owner));Ok(RetainedCloneStep::Progress(receipt))},Err((error,transfer))=>{*self=Self::Transfer(transfer);Err(error)}},
            Self::Owned(mut owner)=>{let result=semio_framework_value::close_factory_ticket(&mut owner,grant);*self=Self::Owned(owner);result},
            Self::Kernel(mut owner)=>{let result=owner.close_step(grant);*self=Self::Kernel(owner);result},
            Self::Empty=>Ok(RetainedCloneStep::Complete(Default::default())),
        }
    }
}

pub struct MeshOperatorJob {
    pub job: Option<MeshModelingJob>,
    pub brep_job: Option<(Session,GeometryHandle,TessellationJob)>,
    pub import: Option<MeshImportState>,
    pub preparation: Option<MeshPreparation>,
    pub retirement: Option<MeshRetirement>,
    pub pending_output:Option<Dictionary>,
    pub value_retirement:neural_engine::ValueRetirement,
    pub receipt:RetainedCloneProgress,
    pub fault:Option<EvalError>,
    pub modeling_base: usize,
    pub output: Option<MeshJobOutput>,
    pub progress: neural_engine::OperatorProgress,
    pub cancelled: bool,
}

pub struct MeshPreparation {
    pub operation:&'static str,
    pub session:Session,
    pub input:Option<Dictionary>,
    pub source:Option<PolygonSourcePreparation>,
    pub reconstruction:Option<MeshModelingJob>,
    pub next:Option<Box<MeshOperatorJob>>,
    pub input_retirement:neural_engine::ValueRetirement,
    pub retire_input:bool,
}

pub enum MeshJobOutput { Mesh(MeshOutputState), Analysis(MeshAnalysisState) }
pub struct MeshImportState {
    pub session: Session,
    pub tessellation: Option<MeshTessellationJob>,
    pub cursor: Option<semio_framework_3d_brep_engine::MeshImportCursor>,
    pub tolerance: f64,
    pub source_retirement: Option<MeshRetirement>,
    pub transfer_retirement: Option<MeshRetirement>,
    pub fault: Option<EvalError>,
}
impl MeshJobOutput {
    pub fn phase(&self)->&'static str { match self { Self::Mesh(value)=>value.phase(),Self::Analysis(value)=>value.phase() } }
    pub fn estimate(&self)->usize { match self { Self::Mesh(value)=>value.estimate(),Self::Analysis(value)=>value.estimate() } }
    pub fn advance(&mut self)->Result<Option<Dictionary>,EvalError> { match self { Self::Mesh(value)=>value.advance(),Self::Analysis(value)=>value.advance() } }
}

pub struct MeshAnalysisState {
    pub tessellation: MeshTessellationJob,
    pub triangles: Option<semio_framework_3d::mesh::MeshTransfer>,
    pub edges: HistoryFoldIndex<(u32,u32),(usize,i32)>,
    pub boundary: usize,
    pub non_manifold: usize,
    pub inconsistent: usize,
    pub phase: u8,
    pub face: usize,
    pub start: EdgeId,
    pub next: EdgeId,
    pub flipped: bool,
    pub corners: Vec<u32>,
    pub cursor: usize,
    pub min: [f32;3],
    pub max: [f32;3],
    pub area: f64,
    pub volume: f64,
    pub degenerate: usize,
}
impl MeshAnalysisState {
    pub fn new(mesh:HalfedgeMesh)->Result<Self,EvalError> {
        if mesh.vertex_count()<3 || mesh.face_count()==0 || mesh.vertex_count()>LIMIT || mesh.face_count()>LIMIT || mesh.halfedge_count()>LIMIT*6 { return Err(invalid("analysis requires geometry within mesh capacity")); }
        Ok(Self { tessellation:MeshTessellationJob::with_preview_capacity(mesh,64_000_000),triangles:None,edges:HistoryFoldIndex::new(),boundary:0,non_manifold:0,inconsistent:0,phase:0,face:0,start:EdgeId(0),next:EdgeId(0),flipped:false,corners:Vec::new(),cursor:0,min:[f32::INFINITY;3],max:[f32::NEG_INFINITY;3],area:0.0,volume:0.0,degenerate:0 })
    }
    pub fn phase(&self)->&'static str { ["mesh-analysis-corners","mesh-analysis-topology","mesh-analysis-bounds","mesh-analysis-tessellate","mesh-analysis-triangles","mesh-analysis-done"][self.phase as usize] }
    pub fn estimate(&self)->usize { self.tessellation.source().halfedge_count().saturating_mul(32).saturating_add(self.tessellation.source().vertex_count()).saturating_add(1) }
    pub fn advance(&mut self)->Result<Option<Dictionary>,EvalError> {
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
            3=>{ match tessellation_slice(&mut self.tessellation,1).map_err(mesh_error)? { MeshTessellationStep::Done(transfer)=>{self.triangles=Some(transfer);self.cursor=0;self.phase=4;},MeshTessellationStep::Working(_)=>{},MeshTessellationStep::Cancelled(_)=>return Err(invalid("analysis cancelled")) } }
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

pub struct MeshOutputState {
    pub json_only:bool,
    pub obj_only:bool,
    pub obj_uv:bool,
    pub tessellation: Option<MeshTessellationJob>,
    pub encoder: Option<MeshPackEncodingJob>,
    pub phase: u8,
    pub vertex: usize,
    pub face: usize,
    pub start: EdgeId,
    pub next: EdgeId,
    pub flipped: bool,
    pub corners: Vec<u32>,
    pub halfedges: Vec<u32>,
    pub edge_remap: HashMap<u32,u32>,
    pub serialized_corners:Vec<u32>,
    pub serialized_edges:Vec<u32>,
    pub corner_remapped:bool,
    pub edge_remapped:bool,
    pub metadata:semio_framework_mesh_engine::io::text::MeshMetadataCursor,
    pub corner_base: usize,
    pub preview_data: Option<semio_framework_mesh_engine::MeshData>,
    pub cursor: usize,
    pub data: String,
    pub bytes: Vec<u8>,
    pub preview: String,
}

impl MeshOutputState {
    pub fn new(mesh: HalfedgeMesh) -> Result<Self, EvalError> {
        if mesh.vertex_count() < 3 || mesh.face_count() == 0 { return Err(invalid("mesh operation produced empty geometry")); }
        if mesh.vertex_count() > LIMIT || mesh.face_count() > LIMIT || mesh.halfedge_count() > LIMIT * 6 { return Err(invalid("result exceeds mesh capacity (100000 vertices, 100000 faces, 600000 corners)")); }
        Ok(Self {json_only:false,obj_only:false,obj_uv:false,tessellation: Some(MeshTessellationJob::with_preview_capacity(mesh,64_000_000)), encoder: None, phase: 0, vertex: 0, face: 0, start: EdgeId(0), next: EdgeId(0), flipped: false, corners: Vec::new(), halfedges: Vec::new(), edge_remap: HashMap::new(),serialized_corners:Vec::new(),serialized_edges:Vec::new(),corner_remapped:false,edge_remapped:false,metadata:Default::default(), corner_base: 0, preview_data: None, cursor: 0, data: "{\"vertices\":[".into(), bytes: Vec::new(), preview: String::new() })
    }
    pub fn phase(&self) -> &'static str { ["mesh-output-vertices", "mesh-output-face-corners", "mesh-output-face-json", "mesh-output-tessellate", "mesh-output-edge-identities", "mesh-output-pack", "mesh-output-base64", "mesh-output-done","mesh-output-metadata","mesh-output-json-done","mesh-output-obj-uv-scan","mesh-output-obj-uv"][self.phase as usize] }
    pub fn estimate(&self) -> usize { let mesh = self.tessellation.as_ref().unwrap().source(); mesh.vertex_count().saturating_add(mesh.face_count().saturating_mul(4)).saturating_add(mesh.halfedge_count().saturating_mul(24)).saturating_add(1) }
    pub fn advance(&mut self) -> Result<Option<Dictionary>, EvalError> {
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
                match tessellation_slice(self.tessellation.as_mut().unwrap(),1).map_err(mesh_error)? {
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

    pub fn advance_metadata(&mut self)->Result<(),EvalError> {
        let mesh=self.tessellation.as_ref().unwrap().source();
        if self.metadata.step(mesh.attributes(),mesh.materials(),mesh.textures(),None,self.corner_remapped.then_some(self.serialized_corners.as_slice()),self.edge_remapped.then_some(self.serialized_edges.as_slice()),&mut self.data).map_err(invalid)? {self.data.push('}');self.phase=if self.json_only {9}else {3};}
        Ok(())
    }
}


impl MeshOperatorJob {
    pub fn preparation(operation:&'static str,session:Session,input:&Dictionary)->Result<Self,EvalError> {
        Self::source_text(operation,input)?;
        let progress=neural_engine::OperatorProgress {phase:"mesh-source-parse",units_done:0,units_total:1};
        Ok(Self {job:None,brep_job:None,import:None,preparation:Some(MeshPreparation {operation,session,input:None,source:Some(PolygonSourcePreparation::new()),reconstruction:None,next:None,input_retirement:Default::default(),retire_input:false}),retirement:None,pending_output:None,value_retirement:Default::default(),receipt:Default::default(),fault:None,modeling_base:0,output:None,progress,cancelled:false})
    }
    pub fn source_text<'a>(operation:&str,input:&'a Dictionary)->Result<&'a str,EvalError> {if operation=="construct" {return input.get("data").and_then(Value::as_dictionary).and_then(|text|text.get("value")).and_then(Value::as_atom).and_then(Atom::as_str).ok_or_else(||invalid("data must be text"));}let mesh=input.get("mesh").and_then(Value::as_dictionary).filter(|mesh|mesh.schema()==Some("mesh")).ok_or_else(||invalid("mesh requires a mesh"))?;mesh.get("data").and_then(Value::as_atom).and_then(Atom::as_str).ok_or_else(||invalid("mesh data is missing"))}
    pub fn step_preparation(&mut self,budget:usize,grant:RetainedCloneGrant)->Result<neural_engine::OperatorJobStep,EvalError> {
        if budget==0 || grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
        let child=turn_grant(grant);
        for _ in 0..budget {
            let state=self.preparation.as_mut().expect("mesh preparation");
            let mut spent=false;
            if self.cancelled || self.fault.is_some() {
                if self.retirement.is_none() {if let Some(source)=state.source.take() {self.retirement=Some(MeshRetirement::Source(source));}}
                if self.retirement.is_none() {if let Some(job)=state.reconstruction.take() {self.retirement=Some(MeshRetirement::Modeling(job));}}
                state.retire_input=true;
                if let Some(next)=&mut state.next {
                    next.cancel();
                    let step=next.close_step(child)?;
                    self.receipt=add_progress(self.receipt,step.progress());
                    if matches!(step,RetainedCloneStep::Complete(_)) {state.next=None;}
                    break;
                }
            }
            if let Some(retirement)=&mut self.retirement {
                let step=retirement.step(child)?;
                self.receipt=add_progress(self.receipt,step.progress());
                if retirement.terminal_is_empty() {self.retirement=None;}
                self.progress.phase="mesh-source-retire";
                spent=true;
            } else if state.retire_input && state.input.is_some() {
                let input=state.input.take().expect("mesh input");
                match state.input_retirement.push_dictionary(input,child) {
                    Ok(progress)=>self.receipt=add_progress(self.receipt,progress),
                    Err((error,input))=>{state.input=Some(input);return Err(error.into());}
                }
                self.progress.phase="mesh-input-retire";
                spent=true;
            } else if !state.input_retirement.terminal_is_empty() {
                let step=state.input_retirement.close_step(child)?;
                self.receipt=add_progress(self.receipt,step.progress());
                self.progress.phase="mesh-input-retire";
                spent=true;
            } else if self.cancelled || self.fault.is_some() {
                self.preparation=None;
                if !self.cancelled {return Err(self.fault.take().expect("retired source failure"));}
                self.fault=None;
                return Ok(neural_engine::OperatorJobStep::Cancelled(self.progress));
            } else if let Some(source)=&mut state.source {
                let text=Self::source_text(state.operation,state.input.as_ref().ok_or_else(||invalid("mesh preparation lost its original input"))?)?;
                match source.step(text,semio_framework_mesh_engine::io::text::PolygonSourceGrant{maximum_units:1,maximum_projection_bytes:grant.maximum_copy_bytes,retirement:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth}}).map(|step|step.source) {
                    Ok(Some(source))=>{state.reconstruction=Some(HalfedgeMesh::polygon_source_job(source).map_err(mesh_error)?);self.retirement=Some(MeshRetirement::Source(state.source.take().unwrap()));self.progress.phase="mesh-source-capture";},
                    Ok(None)=>self.progress.phase=source.phase(),
                    Err(error)=>{self.fault=Some(invalid(error));self.retirement=Some(MeshRetirement::Source(state.source.take().unwrap()));state.retire_input=true;self.progress.phase="mesh-fault-retire";},
                }
            } else if let Some(job)=&mut state.reconstruction {
                match modeling_slice(job,1) {
                    Ok(MeshModelingStep::Working(progress))=>self.progress.phase=progress.phase,
                    Ok(MeshModelingStep::Done(mesh))=>{
                        state.next=Some(Box::new(if state.operation=="construct" {Self::output(mesh)?}else {Self::owned_plan(state.operation,state.session.clone(),state.input.as_ref().ok_or_else(||invalid("mesh preparation lost its original input"))?,mesh)?}));
                        self.retirement=Some(MeshRetirement::Modeling(state.reconstruction.take().unwrap()));
                        state.retire_input=true;
                    },
                    Ok(MeshModelingStep::Cancelled(_))=>unreachable!(),
                    Err(error)=>{self.fault=Some(mesh_error(error));self.retirement=Some(MeshRetirement::Modeling(state.reconstruction.take().unwrap()));state.retire_input=true;self.progress.phase="mesh-fault-retire";},
                }
            } else {
                let mut next=*state.next.take().expect("prepared mesh continuation");
                let base=self.progress.units_done.saturating_add(1);
                next.modeling_base=next.modeling_base.saturating_add(base);
                next.progress.units_done=next.progress.units_done.saturating_add(base);
                next.progress.units_total=next.progress.units_total.saturating_add(base);
                next.receipt=self.receipt;
                *self=next;
                return Ok(neural_engine::OperatorJobStep::Working(self.progress));
            }
            if !self.cancelled {self.progress.units_done=self.progress.units_done.saturating_add(1);self.progress.units_total=self.progress.units_total.max(self.progress.units_done.saturating_add(1));}
            if spent {break;}
        }
        Ok(neural_engine::OperatorJobStep::Working(self.progress))
    }
    pub fn close_cancelled(&mut self,grant:RetainedCloneGrant)->Result<neural_engine::OperatorJobStep,EvalError> {
        if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
        let child=turn_grant(grant);
        if self.retirement.is_none() {
            if let Some(job)=self.job.take() {self.retirement=Some(MeshRetirement::Modeling(job));}
            else if let Some((session,shape,job))=self.brep_job.take() {self.retirement=Some(MeshRetirement::Kernel(MeshKernelRetirement::new(session,shape,job,None)));}
            else if let Some(output)=self.output.take() {self.retirement=Some(MeshRetirement::Output(output));}
        }
        if let Some(retirement)=&mut self.retirement {
            let step=retirement.step(child)?;
            self.receipt=add_progress(self.receipt,step.progress());
            if !retirement.terminal_is_empty() {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
            self.retirement=None;
            return Ok(neural_engine::OperatorJobStep::Working(self.progress));
        }
        if let Some(output)=self.pending_output.take() {
            match self.value_retirement.push_dictionary(output,child) {
                Ok(progress)=>{self.receipt=add_progress(self.receipt,progress);return Ok(neural_engine::OperatorJobStep::Working(self.progress));},
                Err((error,output))=>{self.pending_output=Some(output);return Err(error.into());}
            }
        }
        if !self.value_retirement.terminal_is_empty() {
            let step=self.value_retirement.close_step(child)?;
            self.receipt=add_progress(self.receipt,step.progress());
            return Ok(neural_engine::OperatorJobStep::Working(self.progress));
        }
        if self.job.is_some() || self.brep_job.is_some() || self.output.is_some() {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
        Ok(neural_engine::OperatorJobStep::Cancelled(self.progress))
    }
    pub fn close_failed(&mut self,grant:RetainedCloneGrant)->Result<neural_engine::OperatorJobStep,EvalError> {
        match self.close_cancelled(grant)? {
            neural_engine::OperatorJobStep::Cancelled(progress)=>if self.cancelled {self.fault=None;Ok(neural_engine::OperatorJobStep::Cancelled(progress))}else {Err(self.fault.take().expect("retired mesh failure"))},
            step=>Ok(step),
        }
    }
    pub fn to_brep(session:Session,mesh:HalfedgeMesh,tolerance:f64)->Result<Self,EvalError> {
        Ok(Self {job:None,brep_job:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),receipt:Default::default(),fault:None,import:Some(MeshImportState {session,tessellation:Some(MeshTessellationJob::with_preview_capacity(mesh,64_000_000)),cursor:None,tolerance,source_retirement:None,transfer_retirement:None,fault:None}),modeling_base:0,output:None,cancelled:false,progress:neural_engine::OperatorProgress {units_done:0,units_total:1,phase:"mesh-to-brep-tessellate"}})
    }
    pub fn step_import(&mut self,budget:usize,grant:RetainedCloneGrant)->Result<neural_engine::OperatorJobStep,EvalError> {
        if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(neural_engine::OperatorJobStep::Working(self.progress));}
        let child=turn_grant(grant);
        for _ in 0..budget {
            let mut spent=false;
            let import=self.import.as_mut().expect("retained import");
            if self.cancelled && import.tessellation.is_some() {import.source_retirement=Some(MeshRetirement::Tessellation(import.tessellation.take().unwrap()));}
            let retirement=if import.source_retirement.is_some() {&mut import.source_retirement}else {&mut import.transfer_retirement};
            if let Some(cursor)=retirement {
                let step=cursor.step(child)?;
                self.receipt=add_progress(self.receipt,step.progress());
                if cursor.terminal_is_empty() {*retirement=None;}
                self.progress.phase="mesh-to-brep-retire-source";
                spent=true;
            } else if self.cancelled && import.cursor.is_none() {
                self.import=None;return Ok(neural_engine::OperatorJobStep::Cancelled(self.progress));
            } else if let Some(error)=import.fault.take() {self.import=None;return Err(error);
            } else if let Some(tessellation)=&mut import.tessellation {
                match tessellation_slice(tessellation,1) {
                    Err(error)=>{import.fault=Some(mesh_error(error));import.source_retirement=Some(MeshRetirement::Tessellation(import.tessellation.take().unwrap()));}
                    Ok(MeshTessellationStep::Cancelled(_))=>{import.source_retirement=Some(MeshRetirement::Tessellation(import.tessellation.take().unwrap()));self.cancelled=true;}
                    Ok(MeshTessellationStep::Working(_))=>{},
                    Ok(MeshTessellationStep::Done(mut mesh))=>{
                        let cursor=semio_framework_3d_brep_engine::MeshImportCursor::validate_admission_counts(mesh.positions.len(),mesh.indices.len(),import.tolerance).and_then(|()|semio_framework_3d_brep_engine::MeshImportCursor::new(std::mem::take(&mut mesh.positions),std::mem::take(&mut mesh.normals),std::mem::take(&mut mesh.indices),import.tolerance)).map_err(|error|invalid(error.to_string()));
                        import.source_retirement=Some(MeshRetirement::Tessellation(import.tessellation.take().unwrap()));
                        import.transfer_retirement=Some(MeshRetirement::Transfer(mesh));
                        match cursor {Ok(cursor)=>import.cursor=Some(cursor),Err(error)=>import.fault=Some(error)}
                        self.progress.phase="mesh-to-brep-admit";
                    }
                }
            } else {
                let cursor=import.cursor.as_mut().expect("retained mesh-I/O cursor");
                let result=import.session.step_mesh_import(cursor,1)?;
                self.progress.phase=cursor.progress().2;
                if cursor.retirement_complete() {self.import=None;return Ok(neural_engine::OperatorJobStep::Cancelled(self.progress));}
                if let Some(handle)=result {
                    let value=import.session.with_kernel_read(|kernel|geometry_dict(kernel,&handle))?;
                    self.import=None;self.progress.units_done=self.progress.units_done.saturating_add(1);self.progress.units_total=self.progress.units_done;
                    return Ok(neural_engine::OperatorJobStep::Done(channel_output("geometry",value)));
                }
            }
            self.progress.units_done=self.progress.units_done.saturating_add(1);self.progress.units_total=self.progress.units_total.max(self.progress.units_done.saturating_add(1));
            if spent {break;}
        }
        Ok(neural_engine::OperatorJobStep::Working(self.progress))
    }
    pub fn from_brep(session:Session,shape:GeometryHandle,deflection:f64)->Result<Self,EvalError> {
        let job=session.with_kernel_read(|kernel|kernel.tessellate_job_sync(&shape,deflection).map_err(|error|map_kernel_error(&error)))?;
        let progress=job.progress();Ok(Self {job:None,brep_job:Some((session,shape,job)),import:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),receipt:Default::default(),fault:None,modeling_base:0,output:None,cancelled:false,progress:neural_engine::OperatorProgress {units_done:progress.units_done,units_total:progress.units_total,phase:progress.phase.tag()}})
    }
    pub fn modeling_progress(&self,progress:MeshModelingProgress)->neural_engine::OperatorProgress {let mut progress=operator_progress(progress);progress.units_done=progress.units_done.saturating_add(self.modeling_base);progress.units_total=progress.units_total.saturating_add(self.modeling_base);progress}
    pub fn analysis(mesh:HalfedgeMesh)->Result<Self,EvalError> {
        let output=MeshJobOutput::Analysis(MeshAnalysisState::new(mesh)?);
        Ok(Self { job:None,brep_job:None,import:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),receipt:Default::default(),fault:None,modeling_base:0,progress:neural_engine::OperatorProgress { units_done:0,units_total:output.estimate(),phase:output.phase() },output:Some(output),cancelled:false })
    }
    pub fn obj(mesh:HalfedgeMesh)->Result<Self,EvalError> {let mut job=Self::output(mesh)?;let Some(MeshJobOutput::Mesh(output))=&mut job.output else {unreachable!()};output.obj_only=true;output.data="# kernel_3d_mesh OBJ export\n".into();Ok(job)}
    pub fn json(mesh:HalfedgeMesh)->Result<Self,EvalError> {let mut job=Self::output(mesh)?;let Some(MeshJobOutput::Mesh(output))=&mut job.output else {unreachable!()};output.json_only=true;Ok(job)}
    pub fn output(mesh: HalfedgeMesh) -> Result<Self, EvalError> {
        let output = MeshJobOutput::Mesh(MeshOutputState::new(mesh)?);
        Ok(Self { job: None, brep_job:None,import:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),receipt:Default::default(),fault:None,modeling_base:0,progress: neural_engine::OperatorProgress { units_done: 0, units_total: output.estimate(), phase: output.phase() }, output: Some(output), cancelled: false })
    }
}

impl MeshOperatorJob {
    pub fn advance(&mut self,budget:usize,grant:RetainedCloneGrant)->Result<neural_engine::OperatorJobStep,EvalError> {
        if self.preparation.is_some() {return self.step_preparation(budget,grant);}
        if self.fault.is_some() {return self.close_failed(grant);}
        if self.cancelled && self.import.is_none() {return self.close_cancelled(grant);}
        if self.import.is_some() { return self.step_import(budget,grant); }
        if let Some(retirement)=&mut self.retirement {
            let step=retirement.step(turn_grant(grant))?;
            self.receipt=add_progress(self.receipt,step.progress());
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
            if done {let (session,shape,mut job)=self.brep_job.take().unwrap();let (mut mesh,_)=job.take_mesh().ok_or_else(||invalid("BRep tessellation completed without geometry"))?;let next=HalfedgeMesh::indexed_triangle_job(std::mem::take(&mut mesh.position),std::mem::take(&mut mesh.index),std::mem::take(&mut mesh.normal));self.retirement=Some(MeshRetirement::Kernel(MeshKernelRetirement::new(session,shape,job,Some(mesh))));let job=next.map_err(mesh_error)?;self.modeling_base=self.progress.units_done;self.progress=self.modeling_progress(job.progress());self.job=Some(job);}
            return Ok(neural_engine::OperatorJobStep::Working(self.progress));
        }
        if self.output.is_none() {
            let job = self.job.as_mut().ok_or_else(|| invalid("mesh job is retired"))?;
            let step=match modeling_slice(job,budget) {Ok(step)=>step,Err(error)=>{self.fault=Some(mesh_error(error));self.retirement=Some(MeshRetirement::Modeling(self.job.take().unwrap()));self.progress.phase="mesh-fault-retire";return Ok(neural_engine::OperatorJobStep::Working(self.progress));}};
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
    pub fn close_demand(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        use semio_framework_value::retirement::owned_retirement_birth_bytes as birth;
        let nested=|capacity:usize|->Result<RetirementDemand,ValueError> {Ok(RetirementDemand {capacity_bytes:capacity,depth:1,..Default::default()})};
        let value=|owner:&neural_engine::ValueRetirement|->Result<RetirementDemand,ValueError> {Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})};
        if let Some(retirement)=&self.retirement {return retirement.demands(copy);}
        if let Some(import)=&self.import {
            if let Some(retirement)=import.source_retirement.as_ref().or(import.transfer_retirement.as_ref()) {return retirement.demands(copy);}
            if let Some(job)=&import.tessellation {return nested(job.retirement_birth_bytes());}
        }
        if let Some(state)=&self.preparation {
            if state.retire_input && state.input.is_some() {return nested(neural_engine::ValueRetirement::domain_frame_birth_bytes());}
            if !state.input_retirement.terminal_is_empty() {return value(&state.input_retirement);}
            if state.source.is_some() {return nested(birth::<PolygonSourcePreparation>());}
            if let Some(job)=&state.reconstruction {return nested(job.retirement_birth_bytes());}
            if let Some(next)=&state.next {return next.close_demand(copy);}
            if state.input.is_some() {return nested(neural_engine::ValueRetirement::domain_frame_birth_bytes());}
        }
        if let Some(job)=&self.job {return nested(job.retirement_birth_bytes());}
        if self.brep_job.is_some() {return nested(std::mem::size_of::<MeshKernelRetirement>());}
        if self.output.is_some() {return nested(birth::<MeshJobOutput>());}
        if self.pending_output.is_some() {return nested(neural_engine::ValueRetirement::domain_frame_birth_bytes());}
        if !self.value_retirement.terminal_is_empty() {return value(&self.value_retirement);}
        Ok(Default::default())
    }
    pub fn terminal_is_empty(&self)->bool {
        self.job.is_none() && self.brep_job.is_none() && self.import.is_none() && self.preparation.is_none() && self.retirement.is_none() && self.pending_output.is_none() && self.output.is_none() && self.value_retirement.terminal_is_empty()
    }
    pub fn attach_input(&mut self,input:Dictionary,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Dictionary)> {
        if let Some(state)=self.preparation.as_mut() {
            if grant.maximum_items==0 {return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"mesh preparation input transfer requires one admitted item"),input));}
            state.input=Some(input);
            return Ok(RetainedCloneProgress {copied_items:1,..Default::default()});
        }
        self.value_retirement.push_dictionary(input,turn_grant(grant))
    }
}

impl neural_engine::OperatorJob for MeshOperatorJob {
    fn step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(neural_engine::OperatorJobStep,RetainedCloneProgress),neural_engine::OperatorWorkFailure>{
        self.receipt=Default::default();let grant=cx.retained_grant();
        if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok((neural_engine::OperatorJobStep::Working(self.progress),Default::default()))}
        if cx.is_cancelled(){self.cancel();}
        let result=self.advance(1,grant);cx.consume_fuel(1);
        let step=result.map_err(|primary|neural_engine::OperatorWorkFailure::debit_evaluation(primary,self.receipt,cx))?;
        cx.consume_retained(self.receipt).map_err(|primary|neural_engine::OperatorWorkFailure::Original(primary.into()))?;
        Ok((step,self.receipt))
    }
    fn normal_step_progress(&self)->RetainedCloneProgress {self.receipt}
    fn next_step_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(0)}
    fn next_step_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError> {Ok(0)}
    fn next_step_release_byte_demand(&self)->Result<usize,ValueError> {Ok(0)}
    fn next_step_depth_demand(&self)->Result<usize,ValueError> {Ok(1)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.close_demand(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,maximum_copy_bytes:usize)->Result<usize,ValueError> {Ok(self.close_demand(maximum_copy_bytes)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.close_demand(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError> {Ok(self.close_demand(0)?.depth.max(usize::from(!self.terminal_is_empty())))}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        self.receipt=Default::default();
        if !self.cancelled {self.cancel();}
        let result=if self.preparation.is_some() {self.step_preparation(grant.maximum_items.max(1),grant)}
            else if self.import.is_some() {self.step_import(grant.maximum_items.max(1),grant)}
            else if self.fault.is_some() {self.close_failed(grant)}
            else {self.close_cancelled(grant)};
        let progress=self.receipt;
        match result {
            Ok(_) if self.terminal_is_empty() => Ok(RetainedCloneStep::Complete(progress)),
            Ok(_) => Ok(RetainedCloneStep::Progress(progress)),
            Err(_) if self.terminal_is_empty() => Ok(RetainedCloneStep::Complete(progress)),
            Err(error) => Err(value_error(error)),
        }
    }
    fn terminal_is_empty(&self)->bool {MeshOperatorJob::terminal_is_empty(self)}
    fn progress(&self) -> neural_engine::OperatorProgress { self.progress }
    fn cancel(&mut self) {
        if self.cancelled {return;}
        if self.preparation.is_some() {self.cancelled=true;return;}
        if let Some(import)=&mut self.import {self.cancelled=true;if let Some(cursor)=&mut import.cursor {cursor.cancel();}return;}
        if self.cancelled || (self.output.is_none() && self.job.is_none() && self.brep_job.is_none() && self.retirement.is_none() && self.pending_output.is_none()) { return; }
        if let Some(job) = &mut self.job { job.cancel(); let progress=job.progress();self.progress=neural_engine::OperatorProgress {units_done:progress.units_done.saturating_add(self.modeling_base),units_total:progress.units_total.saturating_add(self.modeling_base),phase:progress.phase}; }
        if let Some((_,_,job))=&mut self.brep_job {job.cancel();}
        self.cancelled=true;

    }
}

pub enum MeshEvaluation { Mesh(HalfedgeMesh), Value(Dictionary) }

pub struct MeshOperation(&'static str, SessionCapture);
impl MeshOperation {
    pub fn plan_job(&self, input: &Dictionary) -> Result<Option<MeshOperatorJob>, EvalError> {
        if self.0=="transform" {HalfedgeMesh::validate_affine_matrix(affine_matrix(input)?).map_err(mesh_error)?;}
        if self.0=="construct" {return Ok(Some(MeshOperatorJob::preparation(self.0,self.1.clone(),input)?));}
        let primitive=match self.0 {
            "box"=>Some(HalfedgeMesh::box_primitive_job(positive(input,"width")?,positive(input,"height")?,positive(input,"depth")?)),
            "plane"=>Some(HalfedgeMesh::plane_primitive_job(positive(input,"width")?,positive(input,"depth")?)),
            "cylinder"=>Some(HalfedgeMesh::cylinder_primitive_job(positive(input,"radius")?,positive(input,"height")?,count(input,"segments",3,1024)?)),
            "cone"=>Some(HalfedgeMesh::cone_primitive_job(positive(input,"radius")?,positive(input,"height")?,count(input,"segments",3,1024)?)),
            "sphere"=>Some(HalfedgeMesh::sphere_primitive_job(positive(input,"radius")?,count(input,"subdivisions",0,5)?)),_=>None,
        };if let Some(job)=primitive {let job=job.map_err(mesh_error)?;let progress=operator_progress(job.progress());return Ok(Some(MeshOperatorJob {job:Some(job),brep_job:None,import:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),receipt:Default::default(),fault:None,modeling_base:0,output:None,progress,cancelled:false}));}
        if self.0=="fromBrep" {return Ok(Some(MeshOperatorJob::from_brep(self.1.clone(),read_geometry(input,"geometry")?,positive(input,"deflection")? as f64)?));}
        if matches!(self.0,"toBrep"|"analyze"|"exportObj"|"exportJson") {return Ok(Some(MeshOperatorJob::preparation(self.0,self.1.clone(),input)?));}
        if !matches!(self.0, "bevel" | "decimate" | "mirror" | "mergeVertices" | "mergeCoplanar" | "dissolveEdges" | "dissolveVertices" | "weld" | "subdivide" | "loopCut" | "knifeCut" | "extrude" | "inset" | "orient" | "fillHoles" | "transform" | "translate" | "rotate" | "scale" | "translateComponents" | "rotateComponents" | "scaleComponents" | "moveVertices" | "triangulate" | "snapVertices" | "moveProportional" | "deleteFaces" | "flip") {
            if matches!(self.0,"inspectVertex" | "inspectEdge" | "inspectFace" | "analyze" | "exportObj" | "exportJson" | "toBrep") { return Ok(None); }
            let MeshEvaluation::Mesh(mesh) = self.prepare(input)? else { return Err(invalid("mesh operator produced an unexpected value")); };
            return Ok(Some(MeshOperatorJob::output(mesh)?));
        }
        Ok(Some(MeshOperatorJob::preparation(self.0,self.1.clone(),input)?))
    }

    pub fn plan_capacity() -> usize { std::mem::size_of::<MeshOperatorJob>().saturating_add(neural_engine::ValueRetirement::domain_frame_birth_bytes()) }

}
impl Operator for MeshOperation {
    retire_geometry_capture!(1);
    fn step_plan(&self,input:Dictionary,grant:RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,RetainedCloneProgress),(EvalError,Dictionary)> {
        use neural_engine::OperatorPlanAdmission;
        if grant.maximum_items==0 || grant.maximum_depth==0 || grant.maximum_capacity_bytes<Self::plan_capacity() {
            return Err((EvalError::from(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"mesh plan admission requires one item and its job frame")),input));
        }
        let mut job=match self.plan_job(&input) {Ok(Some(job))=>job,Ok(None)=>return OperatorPlanAdmission::immediate(input,grant),Err(error)=>return Err((error,input))};
        match job.attach_input(input,grant) {
            Ok(progress)=>Ok((OperatorPlanAdmission::Job(Box::new(job)),RetainedCloneProgress {retained_capacity_bytes:progress.retained_capacity_bytes.saturating_add(std::mem::size_of::<MeshOperatorJob>()),..progress})),
            Err((error,input))=>{
                job.cancel();
                for _ in 0..4096 {if matches!(neural_engine::OperatorJob::close_step(&mut job,sync_grant()),Ok(RetainedCloneStep::Complete(_))|Err(_)) {break;}}
                Err((error.into(),input))
            }
        }
    }
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError> {Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,ValueError> {Ok(Self::plan_capacity())}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError> {Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,ValueError> {Ok(1)}

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        if let Some(mut job)=self.plan_job(input)? {
            job.attach_input(input.clone(),sync_grant()).map_err(|(error,_)|EvalError::from(error))?;
            return drive_job(job);
        }
        match self.prepare(input)? { MeshEvaluation::Mesh(mesh) => mesh_output(&mesh), MeshEvaluation::Value(output) => Ok(output) }
    }
}

impl MeshOperatorJob {
    pub fn owned_plan(operation:&'static str,session:Session,input:&Dictionary,mesh:HalfedgeMesh)->Result<Self,EvalError> {
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
        Ok(MeshOperatorJob { job: Some(job), brep_job:None,import:None,preparation:None,retirement:None,pending_output:None,value_retirement:Default::default(),receipt:Default::default(),fault:None,modeling_base:0,output: None, progress, cancelled: false })
    }
}

impl MeshOperation {
    pub fn prepare(&self, input: &Dictionary) -> Result<MeshEvaluation, EvalError> {
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
