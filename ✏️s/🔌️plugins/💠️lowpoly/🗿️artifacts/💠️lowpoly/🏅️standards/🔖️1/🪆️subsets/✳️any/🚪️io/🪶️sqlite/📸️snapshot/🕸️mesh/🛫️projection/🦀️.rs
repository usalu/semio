//! 🕸️ One authored mesh cell visitor supplies owned SQL and borrowed semantic admission.
use crate::{LowpolyMeshState,LowpolyMeshAttributeDomain as Domain,LowpolyMeshAttributeSemantic as Semantic,LowpolyMeshAttributeInterpolation as Interpolation};
use store::sqlite_snapshot::artifact::{RowWriter,Cell,FloatColumn};
use semio_framework_value::{DslValue,Number,ValueError,ValueRefusalKind};
const VERTEX:&[FloatColumn]=&[FloatColumn::Binary32(3),FloatColumn::Binary32(4),FloatColumn::Binary32(5),FloatColumn::Binary32(6),FloatColumn::Binary32(7),FloatColumn::Binary32(8)];
const EDGE:&[FloatColumn]=&[FloatColumn::Binary32(7),FloatColumn::Binary32(8)];
fn ordinal(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Lowpoly mesh ordinal exceeds integer64"))}
fn index(value:Option<u32>)->Cell<'static>{value.map_or(Cell::Null,|value|Cell::Integer(i64::from(value)))}
fn domain(value:Domain)->&'static str{match value{Domain::Vertex=>"vertex",Domain::Corner=>"corner",Domain::Face=>"face",Domain::Edge=>"edge"}}
fn semantic(value:Semantic)->&'static str{match value{Semantic::Normal=>"normal",Semantic::Uv=>"uv",Semantic::Color=>"color",Semantic::Material=>"material",Semantic::Custom=>"custom"}}
fn interpolation(value:Interpolation)->&'static str{match value{Interpolation::Linear=>"linear",Interpolation::Nearest=>"nearest",Interpolation::Constant=>"constant"}}
enum Attach<'a>{Sample(i64,i64),Material(i64,i64,&'a str),Array(i64,i64),Object(i64,i64,&'a str)}
fn attach(writer:&mut RowWriter<'_,'_>,relation:Attach<'_>,value:i64)->Result<(),ValueError>{match relation{Attach::Sample(parent,ordinal)=>{writer.insert("lowpoly_mesh_attribute_sample",&[Cell::Integer(parent),Cell::Integer(ordinal),Cell::Integer(value)])?;},Attach::Material(parent,ordinal,name)=>{writer.insert("lowpoly_mesh_material",&[Cell::Integer(parent),Cell::Integer(ordinal),Cell::Text(name),Cell::Integer(value)])?;},Attach::Array(parent,ordinal)=>{writer.insert("lowpoly_mesh_array_member",&[Cell::Integer(parent),Cell::Integer(ordinal),Cell::Integer(value)])?;},Attach::Object(parent,ordinal,name)=>{writer.insert("lowpoly_mesh_object_member",&[Cell::Integer(parent),Cell::Integer(ordinal),Cell::Text(name),Cell::Integer(value)])?;}}Ok(())}

fn unique<'a>(names:impl ExactSizeIterator<Item=&'a str>,writer:&mut RowWriter<'_,'_>)->Result<(),ValueError>{let mut ordered=writer.allocate_frontier(names.len())?;for name in names{writer.checkpoint()?;ordered.push(name);}let phase=writer.phase();writer.sort_frontier(&mut ordered,|a,b,control|store::sqlite_snapshot::transfer::compare_text(a,b,phase,control))?;for pair in ordered.windows(2){if writer.compare_text(pair[0],pair[1])?.is_eq(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"managed mesh named namespace repeats a key"))}}Ok(())}
/// 📤️ Preserve full topology, rich values and texture octets with paid traversal backing.
pub fn project(mesh:&LowpolyMeshState,owner:i64,writer:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 unique(mesh.attributes.iter().map(|value|value.name.as_str()),writer)?;unique(mesh.materials.iter().map(|value|value.name.as_str()),writer)?;unique(mesh.textures.iter().map(|value|value.name.as_str()),writer)?;
 let mut seams=writer.allocate_frontier(mesh.uv_seams.len())?;for seam in&mesh.uv_seams{writer.checkpoint()?;seams.push(*seam);}writer.sort_frontier(&mut seams,|a,b,_|Ok(a.cmp(b)))?;for pair in seams.windows(2){writer.checkpoint()?;if pair[0]==pair[1]{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"managed mesh seam namespace repeats an index"))}}
 writer.insert_key("lowpoly_mesh_state",owner,&[])?;
 for(at,vertex)in mesh.vertices.iter().enumerate(){let normal=vertex.normal.map(|values|values.map(Cell::Float32)).unwrap_or([Cell::Null;3]);writer.insert_float("lowpoly_mesh_vertex",&[Cell::Integer(owner),Cell::Integer(ordinal(at)?),Cell::Float32(vertex.position[0]),Cell::Float32(vertex.position[1]),Cell::Float32(vertex.position[2]),normal[0],normal[1],normal[2],index(vertex.halfedge)],VERTEX)?;}
 for(at,edge)in mesh.halfedges.iter().enumerate(){writer.insert_float("lowpoly_mesh_halfedge",&[Cell::Integer(owner),Cell::Integer(ordinal(at)?),Cell::Integer(i64::from(edge.vertex)),index(edge.twin),Cell::Integer(i64::from(edge.next)),index(edge.face),Cell::Float32(edge.uv[0]),Cell::Float32(edge.uv[1])],EDGE)?;}
 for(at,face)in mesh.faces.iter().enumerate(){writer.insert("lowpoly_mesh_face",&[Cell::Integer(owner),Cell::Integer(ordinal(at)?),Cell::Integer(i64::from(face.halfedge)),Cell::Integer(i64::from(face.smooth)),Cell::Integer(i64::from(face.flipped))])?;}
 for(at,seam)in mesh.uv_seams.iter().enumerate(){writer.insert("lowpoly_mesh_seam",&[Cell::Integer(owner),Cell::Integer(ordinal(at)?),Cell::Integer(i64::from(*seam))])?;}
 let mut pending=writer.allocate_frontier(0)?;
 for(at,attribute)in mesh.attributes.iter().enumerate(){let id=writer.insert("lowpoly_mesh_attribute",&[Cell::Integer(owner),Cell::Integer(ordinal(at)?),Cell::Text(&attribute.name),Cell::Text(domain(attribute.domain)),Cell::Text(semantic(attribute.semantic)),Cell::Text(interpolation(attribute.interpolation)),Cell::Integer(i64::from(attribute.indices.is_some()))])?;if let Some(indices)=&attribute.indices{for(at,value)in indices.iter().enumerate(){writer.insert("lowpoly_mesh_attribute_index",&[Cell::Integer(id),Cell::Integer(ordinal(at)?),Cell::Integer(i64::from(*value))])?;}}for(at,value)in attribute.values.iter().enumerate().rev(){writer.push_frontier(&mut pending,(value,Attach::Sample(id,ordinal(at)?)))?;}}
 for(at,material)in mesh.materials.iter().enumerate().rev(){writer.push_frontier(&mut pending,(&material.value,Attach::Material(owner,ordinal(at)?,material.name.as_str())))?;}
 for(at,texture)in mesh.textures.iter().enumerate(){let id=writer.insert("lowpoly_mesh_texture",&[Cell::Integer(owner),Cell::Integer(ordinal(at)?),Cell::Text(&texture.name),Cell::Text(&texture.mime)])?;for(at,byte)in texture.bytes.iter().enumerate(){writer.insert("lowpoly_mesh_texture_octet",&[Cell::Integer(id),Cell::Integer(ordinal(at)?),Cell::Integer(i64::from(*byte))])?;}}
 while let Some((value,relation))=pending.pop(){writer.checkpoint()?;let kind=match value{DslValue::Null=>"null",DslValue::Bool(_)=>"boolean",DslValue::Number(Number::UInt(_))=>"unsigned",DslValue::Number(Number::Int(_))=>"signed",DslValue::Number(Number::Float(_))=>"float",DslValue::String(_)=>"text",DslValue::Bytes(_)=>"bytes",DslValue::Array(_)=>"array",DslValue::Object(_)=>"object"};let id=writer.insert("lowpoly_mesh_value",&[Cell::Text(kind)])?;attach(writer,relation,id)?;
  match value{
   DslValue::Null=>{},DslValue::Bool(value)=>writer.insert_key("lowpoly_mesh_boolean",id,&[Cell::Integer(i64::from(*value))])?,
   DslValue::Number(Number::UInt(value))=>{let mut digits=[0u8;20];let mut at=20;let mut remaining=*value;loop{at-=1;digits[at]=b'0'+(remaining%10)as u8;remaining/=10;if remaining==0{break;}}let text=std::str::from_utf8(&digits[at..]).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Lowpoly mesh decimal alphabet"))?;writer.insert_key("lowpoly_mesh_unsigned",id,&[Cell::Text(text)])?;},
   DslValue::Number(Number::Int(value))=>writer.insert_key("lowpoly_mesh_signed",id,&[Cell::Integer(*value)])?,
   DslValue::Number(Number::Float(value))=>writer.insert_key_float("lowpoly_mesh_float",id,&[Cell::Real(*value)],&[FloatColumn::Binary64(1)])?,
   DslValue::String(value)=>writer.insert_key("lowpoly_mesh_text",id,&[Cell::Text(value)])?,
   DslValue::Bytes(values)=>{writer.insert_key("lowpoly_mesh_bytes",id,&[])?;for(at,byte)in values.iter().enumerate(){writer.insert("lowpoly_mesh_bytes_octet",&[Cell::Integer(id),Cell::Integer(ordinal(at)?),Cell::Integer(i64::from(*byte))])?;}},
   DslValue::Array(values)=>{for(at,value)in values.iter().enumerate().rev(){writer.push_frontier(&mut pending,(value,Attach::Array(id,ordinal(at)?)))?;}},
   DslValue::Object(values)=>{for(at,(name,value))in values.iter().enumerate().rev(){writer.push_frontier(&mut pending,(value,Attach::Object(id,ordinal(at)?,name.as_str())))?;}},
  }
 }
 Ok(())
}
