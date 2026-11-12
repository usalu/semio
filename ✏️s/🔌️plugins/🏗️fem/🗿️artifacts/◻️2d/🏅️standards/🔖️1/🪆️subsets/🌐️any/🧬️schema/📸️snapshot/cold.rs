//! ❄️ Cold FEM2d rows build their sealed text fields and publish through the shared cold persistent-list assembly.
use semio_framework_value::{shared_issuance::SealedShared,ValueError};
use semio_framework_value_persistent::list::persistent::PersistentList;
use semio_s_fem_receiving::cold::{discard_sealed,sealed_list,release_text,text};
use semio_s_fem_node::planar::{FemNode,FemNodeValue};use semio_s_fem_material::planar::{FemMaterial,FemMaterialValue};

fn node(value:FemNodeValue)->Result<FemNode,ValueError>{let id=text(value.id)?;Ok(FemNode{id,x:value.x,y:value.y})}
fn material(value:FemMaterialValue)->Result<FemMaterial,ValueError>{let id=text(value.id)?;let name=match text(value.name){Ok(name)=>name,Err(error)=>{release_text(id);return Err(error)}};Ok(FemMaterial{id,name,e:value.e,nu:value.nu,rho:value.rho})}
/// 📍️ Constructs cold node pages.
pub fn nodes2d(rows:Vec<FemNodeValue>)->Result<SealedShared<PersistentList<FemNode>>,ValueError>{sealed_list(rows,node)}
/// 🧱️ Constructs cold material pages.
pub fn materials2d(rows:Vec<FemMaterialValue>)->Result<SealedShared<PersistentList<FemMaterial>>,ValueError>{sealed_list(rows,material)}
/// ♻️ Dismantles an unpublished node candidate.
pub fn discard_nodes2d(value:SealedShared<PersistentList<FemNode>>)->Result<(),()>{discard_sealed(value)}
