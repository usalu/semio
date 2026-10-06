//! 📏️ Borrowed fields forecast both native grammars without constructing record or payload owners.
use crate::LowpolyMeshState;
use store::sqlite_snapshot::artifact::NativeEncodingBound;
use semio_framework_value::{DslValue as Value,ValueError};

/// 🕸️ Forecast each complete managed field while paying only real borrowed traversal storage.
pub fn mesh(mesh:&LowpolyMeshState,bound:&mut NativeEncodingBound<'_,'_>)->Result<(),ValueError>{
 bound.add(4096)?;
 for _ in&mesh.vertices{bound.add(256)?;}
 for _ in&mesh.halfedges{bound.add(256)?;}
 for _ in&mesh.faces{bound.add(128)?;}
 for _ in&mesh.uv_seams{bound.add(32)?;}
 let mut pending=bound.allocate_frontier(0)?;
 for attribute in&mesh.attributes{bound.add(512)?;bound.repeated(attribute.name.len(),12)?;if let Some(indices)=&attribute.indices{for _ in indices{bound.add(32)?;}}for value in&attribute.values{bound.push_frontier(&mut pending,value)?;}}
 for material in&mesh.materials{bound.add(128)?;bound.repeated(material.name.len(),12)?;bound.push_frontier(&mut pending,&material.value)?;}
 for texture in&mesh.textures{bound.add(128)?;bound.repeated(texture.name.len(),12)?;bound.repeated(texture.mime.len(),12)?;bound.repeated(texture.bytes.len(),8)?;}
 while let Some(value)=pending.pop(){bound.add(1024)?;match value{Value::String(value)=>bound.repeated(value.len(),12)?,Value::Bytes(value)=>bound.repeated(value.len(),8)?,Value::Array(values)=>{for value in values{bound.push_frontier(&mut pending,value)?;}},Value::Object(values)=>{for(name,value)in values{bound.add(128)?;bound.repeated(name.len(),12)?;bound.push_frontier(&mut pending,value)?;}},_=>{}}}
 Ok(())
}
