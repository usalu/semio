//! 🕸️ Complete managed halfedge state is distinct from independent source text and child identity.
use semio_framework_value::DslValue;
use semio_framework_dsl_record_derive::DslRecord;
use semio_framework_value_derive::{ToValue,FromValue};
use semio_framework_3d::mesh::{MeshAttributeDomain,MeshAttributeSemantic,MeshAttributeInterpolation};
pub use semio_framework_3d::mesh::{MeshAttributeDomain as LowpolyMeshAttributeDomain,MeshAttributeSemantic as LowpolyMeshAttributeSemantic,MeshAttributeInterpolation as LowpolyMeshAttributeInterpolation};

#[derive(Clone,Debug,PartialEq,DslRecord)]
pub struct LowpolyMeshState {
    pub vertices:Vec<LowpolyMeshVertex>,
    pub halfedges:Vec<LowpolyMeshHalfedge>,
    pub faces:Vec<LowpolyMeshFace>,
    pub uv_seams:Vec<u32>,
    pub attributes:Vec<LowpolyMeshAttribute>,
    pub materials:Vec<LowpolyMeshMaterial>,
    pub textures:Vec<LowpolyMeshTexture>,
}
#[derive(Clone,Debug,DslRecord,ToValue,FromValue)]
pub struct LowpolyMeshVertex {pub position:[f32;3],pub normal:Option<[f32;3]>,pub halfedge:Option<u32>}
#[derive(Clone,Debug,DslRecord,ToValue,FromValue)]
pub struct LowpolyMeshHalfedge {pub vertex:u32,pub twin:Option<u32>,pub next:u32,pub face:Option<u32>,pub uv:[f32;2]}
#[derive(Clone,Debug,PartialEq,DslRecord,ToValue,FromValue)]
pub struct LowpolyMeshFace {pub halfedge:u32,pub smooth:bool,pub flipped:bool}
#[derive(Clone,Debug,DslRecord,ToValue,FromValue)]
pub struct LowpolyMeshAttribute {
    pub name:String,
    pub domain:MeshAttributeDomain,
    pub semantic:MeshAttributeSemantic,
    pub interpolation:MeshAttributeInterpolation,
    pub values:Vec<DslValue>,
    pub indices:Option<Vec<u32>>,
}
#[derive(Clone,Debug,DslRecord,ToValue,FromValue)]
pub struct LowpolyMeshMaterial {pub name:String,pub value:DslValue}
#[derive(Clone,Debug,PartialEq,DslRecord,ToValue,FromValue)]
pub struct LowpolyMeshTexture {pub name:String,pub mime:String,pub bytes:Vec<u8>}

impl LowpolyMeshState {
    /// 🕸️ A present empty owner differs from an absent mesh state.
    pub fn empty()->Self{Self{vertices:Vec::new(),halfedges:Vec::new(),faces:Vec::new(),uv_seams:Vec::new(),attributes:Vec::new(),materials:Vec::new(),textures:Vec::new()}}
    /// 🥽️ Own every kernel field directly without serializing a managed mesh document.
    pub fn from_mesh(mesh:semio_framework_3d::mesh::HalfedgeMesh)->Self {
        let (vertices,halfedges,faces,seams,attributes,materials,textures)=mesh.into_owned_parts();
        let mut uv_seams:Vec<u32>=seams.into_iter().collect();uv_seams.sort_unstable();
        Self {
            vertices:vertices.into_iter().map(|vertex|LowpolyMeshVertex{position:vertex.position,normal:vertex.normal,halfedge:vertex.halfedge}).collect(),
            halfedges:halfedges.into_iter().map(|edge|LowpolyMeshHalfedge{vertex:edge.vertex,twin:edge.twin,next:edge.next,face:edge.face,uv:edge.uv}).collect(),
            faces:faces.into_iter().map(|face|LowpolyMeshFace{halfedge:face.halfedge,smooth:face.smooth,flipped:face.flipped}).collect(),
            uv_seams,
            attributes:attributes.into_iter().map(|(name,attribute)|LowpolyMeshAttribute{name,domain:attribute.domain,semantic:attribute.semantic,interpolation:attribute.interpolation,values:attribute.values,indices:attribute.indices}).collect(),
            materials:materials.into_iter().map(|(name,value)|LowpolyMeshMaterial{name,value}).collect(),
            textures:textures.into_iter().map(|(name,texture)|LowpolyMeshTexture{name,mime:texture.mime,bytes:texture.bytes}).collect(),
        }
    }
    /// 🧊️ Editable topology stays literal while unique named owners enter the real kernel.
    pub fn into_mesh(self)->Result<semio_framework_3d::mesh::HalfedgeMesh,semio_framework_value::ValueError>{
        use semio_framework_3d::mesh::{HalfedgeMesh,MeshVertex,HalfEdge,MeshFace,MeshAttribute,MeshTexture};
        use semio_framework_value::{ValueError,ValueRefusalKind};
        let guard=semio_framework_value::DecodedValue::new(self,<Self as semio_framework_dsl_record::DslField>::retire_decoded);
        let state=guard.get();
        let unique=|names:Vec<&str>|->bool{let mut seen=std::collections::BTreeSet::new();names.into_iter().all(|name|seen.insert(name))};
        if !unique(state.attributes.iter().map(|value|value.name.as_str()).collect())||!unique(state.materials.iter().map(|value|value.name.as_str()).collect())||!unique(state.textures.iter().map(|value|value.name.as_str()).collect()){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate mesh namespace name"))}
        let mut seams=std::collections::HashSet::new();for value in &state.uv_seams{if !seams.insert(*value){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate mesh seam identity"))}}
        let state=guard.take();
        let attributes=state.attributes.into_iter().map(|value|(value.name,MeshAttribute{domain:value.domain,semantic:value.semantic,interpolation:value.interpolation,values:value.values,indices:value.indices})).collect();
        let materials=state.materials.into_iter().map(|value|(value.name,value.value)).collect();
        let textures=state.textures.into_iter().map(|value|(value.name,MeshTexture{mime:value.mime,bytes:value.bytes})).collect();
        Ok(HalfedgeMesh::from_owned_parts(
            state.vertices.into_iter().map(|vertex|MeshVertex{position:vertex.position,normal:vertex.normal,halfedge:vertex.halfedge}).collect(),
            state.halfedges.into_iter().map(|edge|HalfEdge{vertex:edge.vertex,twin:edge.twin,next:edge.next,face:edge.face,uv:edge.uv}).collect(),
            state.faces.into_iter().map(|face|MeshFace{halfedge:face.halfedge,smooth:face.smooth,flipped:face.flipped}).collect(),
            seams,attributes,materials,textures,
        ))
    }
}

fn intrinsic_words_equal(left:&DslValue,right:&DslValue)->bool{
 let mut pending=vec![(left,right)];while let Some((left,right))=pending.pop(){match(left,right){
  (DslValue::Null,DslValue::Null)=>{},(DslValue::Bool(a),DslValue::Bool(b))if a==b=>{},(DslValue::Number(semio_framework_value::Number::UInt(a)),DslValue::Number(semio_framework_value::Number::UInt(b)))if a==b=>{},(DslValue::Number(semio_framework_value::Number::Int(a)),DslValue::Number(semio_framework_value::Number::Int(b)))if a==b=>{},(DslValue::Number(semio_framework_value::Number::Float(a)),DslValue::Number(semio_framework_value::Number::Float(b)))if a.to_bits()==b.to_bits()=>{},(DslValue::String(a),DslValue::String(b))if a==b=>{},(DslValue::Bytes(a),DslValue::Bytes(b))if a==b=>{},
  (DslValue::Array(a),DslValue::Array(b))if a.len()==b.len()=>{pending.extend(a.iter().zip(b));},
  (DslValue::Object(a),DslValue::Object(b))if a.len()==b.len()=>{for((an,av),(bn,bv))in a.iter().zip(b){if an!=bn{return false}pending.push((av,bv));}},_=>return false,
 }}true
}
impl PartialEq for LowpolyMeshVertex{fn eq(&self,other:&Self)->bool{self.position.map(f32::to_bits)==other.position.map(f32::to_bits)&&self.normal.map(|value|value.map(f32::to_bits))==other.normal.map(|value|value.map(f32::to_bits))&&self.halfedge==other.halfedge}}
impl PartialEq for LowpolyMeshHalfedge{fn eq(&self,other:&Self)->bool{self.vertex==other.vertex&&self.twin==other.twin&&self.next==other.next&&self.face==other.face&&self.uv.map(f32::to_bits)==other.uv.map(f32::to_bits)}}
impl PartialEq for LowpolyMeshAttribute{fn eq(&self,other:&Self)->bool{self.name==other.name&&self.domain==other.domain&&self.semantic==other.semantic&&self.interpolation==other.interpolation&&self.indices==other.indices&&self.values.len()==other.values.len()&&self.values.iter().zip(&other.values).all(|(a,b)|intrinsic_words_equal(a,b))}}
impl PartialEq for LowpolyMeshMaterial{fn eq(&self,other:&Self)->bool{self.name==other.name&&intrinsic_words_equal(&self.value,&other.value)}}

#[path="🔣️json/🦀️.rs"]
pub mod json;
use semio_framework_value::intrinsic_json;

impl semio_framework_value::ToValue for LowpolyMeshState{fn to_value(&self)->DslValue{json::state_value(self)}}
impl semio_framework_value::FromValue for LowpolyMeshState{fn from_value(value:DslValue)->Result<Self,semio_framework_value::ValueError>{json::from_value(value)?.ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"present managed mesh state requires seven fields"))}}
