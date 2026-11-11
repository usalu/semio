//! ❄️ Cold FEM3d rows build their sealed text fields and publish through the shared cold persistent-list assembly.
use semio_framework_value::ValueError;
use semio_framework_value_persistent::list::persistent::PersistentList;
use semio_s_fem_receiving::cold::{discard,list,release_text,text};
use semio_s_fem_node::volumetric::{FemNode,FemNodeValue};use semio_s_fem_material::volumetric::{FemMaterial,FemMaterialValue};

fn node(value:FemNodeValue)->Result<FemNode,ValueError>{let id=text(value.id)?;Ok(FemNode{id,x:value.x,y:value.y,z:value.z})}
fn material(value:FemMaterialValue)->Result<FemMaterial,ValueError>{let id=text(value.id)?;let name=match text(value.name){Ok(name)=>name,Err(error)=>{release_text(id);return Err(error)}};Ok(FemMaterial{id,name,e:value.e,nu:value.nu,rho:value.rho,g:value.g})}
/// 📍️ Constructs cold node pages.
pub fn nodes(rows:Vec<FemNodeValue>)->Result<PersistentList<FemNode>,ValueError>{list(rows,node)}
/// 🧱️ Constructs cold material pages.
pub fn materials(rows:Vec<FemMaterialValue>)->Result<PersistentList<FemMaterial>,ValueError>{list(rows,material)}
/// ♻️ Dismantles an unpublished node candidate.
pub fn discard_nodes(value:PersistentList<FemNode>)->Result<(),PersistentList<FemNode>>{discard(value)}
