//! 🔣️ Public managed mesh JSON uses raw words and the canonical tagged intrinsic value facet.
use super::*;
use semio_framework_value::{DslValue as Value,FromValue,ToValue,ValueError,ValueRefusalKind};
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
struct MeshBuild(LowpolyMeshState);
impl Drop for MeshBuild{fn drop(&mut self){for attribute in&mut self.0.attributes{for value in attribute.values.drain(..){value.retire_decoded();}}for material in self.0.materials.drain(..){material.value.retire_decoded();}}}
struct Owned(Value);
impl Drop for Owned{fn drop(&mut self){std::mem::replace(&mut self.0,Value::Null).retire_decoded();}}
fn fields(value:&Value,names:&[&str])->Result<(),ValueError>{let Value::Object(entries)=value else{return Err(invalid("managed mesh JSON requires an object"))};if entries.len()!=names.len()||names.iter().any(|name|entries.iter().filter(|(key,_)|key==name).count()!=1){return Err(invalid("managed mesh JSON field set differs"))}Ok(())}
fn list(value:&Value)->Result<&[Value],ValueError>{value.as_array().ok_or_else(||invalid("managed mesh JSON requires an ordered collection"))}
fn text(value:&Value)->Result<&str,ValueError>{value.as_str().ok_or_else(||invalid("managed mesh JSON requires text"))}
fn word(value:&Value)->Result<f32,ValueError>{let source=text(value)?;if source.len()!=8||!source.bytes().all(|value|value.is_ascii_digit()||(b'a'..=b'f').contains(&value)){return Err(invalid("managed mesh JSON binary32 word differs"))}let bits=u32::from_str_radix(source,16).map_err(|_|invalid("managed mesh JSON binary32 width"))?;Ok(f32::from_bits(bits))}
fn vector<const N:usize>(value:&Value)->Result<[f32;N],ValueError>{let values=list(value)?;if values.len()!=N{return Err(invalid("managed mesh JSON vector width differs"))}let mut output=[0.0;N];for(index,value)in values.iter().enumerate(){output[index]=word(value)?;}Ok(output)}
fn optional_index(value:&Value)->Result<Option<u32>,ValueError>{if matches!(value,Value::Null){Ok(None)}else{index(value).map(Some)}}
fn index(value:&Value)->Result<u32,ValueError>{value.as_u64().and_then(|value|u32::try_from(value).ok()).ok_or_else(||invalid("managed mesh JSON UInt32 range"))}
fn flag(value:&Value)->Result<bool,ValueError>{value.as_bool().ok_or_else(||invalid("managed mesh JSON boolean differs"))}
fn octets(value:&Value)->Result<Vec<u8>,ValueError>{list(value)?.iter().map(|value|value.as_u64().and_then(|value|u8::try_from(value).ok()).ok_or_else(||invalid("managed mesh JSON octet range"))).collect()}
fn object(entries:impl IntoIterator<Item=(&'static str,Value)>)->Value{Value::object(entries.into_iter().map(|(name,value)|(name.into(),value)))}
fn words<const N:usize>(values:&[f32;N])->Value{Value::Array(values.iter().map(|value|Value::String(format!("{:08x}",value.to_bits()))).collect())}

/// 📥️ Decode exactly the seven managed fields while retaining source text independently.
pub fn from_value(value:Value)->Result<Option<LowpolyMeshState>,ValueError>{
 let owned=Owned(value);if matches!(owned.0,Value::Null){return Ok(None)}let value=&owned.0;fields(value,&["vertices","halfedges","faces","uvSeams","attributes","materials","textures"])?;
 let mut build=MeshBuild(LowpolyMeshState{vertices:Vec::new(),halfedges:Vec::new(),faces:Vec::new(),uv_seams:Vec::new(),attributes:Vec::new(),materials:Vec::new(),textures:Vec::new()});let state=&mut build.0;let mut seams=std::collections::HashSet::new();let mut attributes=std::collections::HashSet::new();let mut materials=std::collections::HashSet::new();let mut textures=std::collections::HashSet::new();
 for value in list(&value["vertices"])?{fields(value,&["position","normal","halfedge"])?;state.vertices.push(LowpolyMeshVertex{position:vector(&value["position"])?,normal:if matches!(value["normal"],Value::Null){None}else{Some(vector(&value["normal"])?)},halfedge:optional_index(&value["halfedge"])?});}
 for value in list(&value["halfedges"])?{fields(value,&["vertex","twin","next","face","uv"])?;state.halfedges.push(LowpolyMeshHalfedge{vertex:index(&value["vertex"])?,twin:optional_index(&value["twin"])?,next:index(&value["next"])?,face:optional_index(&value["face"])?,uv:vector(&value["uv"])?});}
 for value in list(&value["faces"])?{fields(value,&["halfedge","smooth","flipped"])?;state.faces.push(LowpolyMeshFace{halfedge:index(&value["halfedge"])?,smooth:flag(&value["smooth"])?,flipped:flag(&value["flipped"])?});}
 for value in list(&value["uvSeams"])?{let seam=index(value)?;if !seams.insert(seam){return Err(invalid("duplicate managed mesh seam"))}state.uv_seams.push(seam);}
 for value in list(&value["attributes"])?{fields(value,&["name","domain","semantic","interpolation","values","indices"])?;let name=text(&value["name"])?;if !attributes.insert(name){return Err(invalid("duplicate managed mesh attribute"))}state.attributes.push(LowpolyMeshAttribute{name:name.into(),domain:match text(&value["domain"])?{"vertex"=>MeshAttributeDomain::Vertex,"corner"=>MeshAttributeDomain::Corner,"face"=>MeshAttributeDomain::Face,"edge"=>MeshAttributeDomain::Edge,_=>return Err(invalid("managed mesh attribute domain differs"))},semantic:match text(&value["semantic"])?{"normal"=>MeshAttributeSemantic::Normal,"uv"=>MeshAttributeSemantic::Uv,"color"=>MeshAttributeSemantic::Color,"material"=>MeshAttributeSemantic::Material,"custom"=>MeshAttributeSemantic::Custom,_=>return Err(invalid("managed mesh attribute semantic differs"))},interpolation:match text(&value["interpolation"])?{"linear"=>MeshAttributeInterpolation::Linear,"nearest"=>MeshAttributeInterpolation::Nearest,"constant"=>MeshAttributeInterpolation::Constant,_=>return Err(invalid("managed mesh attribute interpolation differs"))},values:list(&value["values"])?.iter().map(intrinsic_json::decode).collect::<Result<_,_>>()?,indices:if matches!(value["indices"],Value::Null){None}else{Some(list(&value["indices"])?.iter().map(index).collect::<Result<_,_>>()?)}});}
 for value in list(&value["materials"])?{fields(value,&["name","value"])?;let name=text(&value["name"])?;if !materials.insert(name){return Err(invalid("duplicate managed mesh material"))}state.materials.push(LowpolyMeshMaterial{name:name.into(),value:intrinsic_json::decode(&value["value"])?});}
 for value in list(&value["textures"])?{fields(value,&["name","mime","bytes"])?;let name=text(&value["name"])?;if !textures.insert(name){return Err(invalid("duplicate managed mesh texture"))}state.textures.push(LowpolyMeshTexture{name:name.into(),mime:text(&value["mime"])?.into(),bytes:octets(&value["bytes"])?});}
 Ok(Some(std::mem::replace(&mut build.0,LowpolyMeshState{vertices:Vec::new(),halfedges:Vec::new(),faces:Vec::new(),uv_seams:Vec::new(),attributes:Vec::new(),materials:Vec::new(),textures:Vec::new()})))
}
/// 📤️ Publish complete mesh words, typed metadata and octets through the declared JSON domain.
pub fn to_value(state:&Option<LowpolyMeshState>)->Value{
 let Some(state)=state else{return Value::Null};
 object([
  ("vertices",Value::Array(state.vertices.iter().map(|value|object([("position",words(&value.position)),("normal",value.normal.as_ref().map_or(Value::Null,words)),("halfedge",value.halfedge.to_value())])).collect())),
  ("halfedges",Value::Array(state.halfedges.iter().map(|value|object([("vertex",value.vertex.to_value()),("twin",value.twin.to_value()),("next",value.next.to_value()),("face",value.face.to_value()),("uv",words(&value.uv))])).collect())),
  ("faces",Value::Array(state.faces.iter().map(|value|object([("halfedge",value.halfedge.to_value()),("smooth",value.smooth.to_value()),("flipped",value.flipped.to_value())])).collect())),
  ("uvSeams",state.uv_seams.to_value()),
  ("attributes",Value::Array(state.attributes.iter().map(|value|object([("name",value.name.to_value()),("domain",value.domain.to_value()),("semantic",value.semantic.to_value()),("interpolation",value.interpolation.to_value()),("values",Value::Array(value.values.iter().map(intrinsic_json::encode).collect())),("indices",value.indices.to_value())])).collect())),
  ("materials",Value::Array(state.materials.iter().map(|value|object([("name",value.name.to_value()),("value",intrinsic_json::encode(&value.value))])).collect())),
  ("textures",Value::Array(state.textures.iter().map(|value|object([("name",value.name.to_value()),("mime",value.mime.to_value()),("bytes",value.bytes.to_value())])).collect())),
 ])
}
