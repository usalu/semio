//! ❄️ Cold FEM assembly retains each partial row and dismantles exclusive pages without allocation.
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::RetainedCloneGrant,retirement::RetireOwned,shared_issuance::{SealedShared,SharedIssuer,ColdRetire}};
use semio_framework_value_persistent::list::persistent::{PersistentList,PersistentListBuilder};
use semio_s_fem_node::{planar,volumetric};use semio_s_fem_material::{planar as pm,volumetric as vm};

fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn seal<T:RetireOwned+Sync>(value:T)->Result<SealedShared<T>,(ValueError,T)>{let d=RetirementDemand{capacity_bytes:SealedShared::<T>::birth_bytes(),depth:1,..Default::default()};SealedShared::admit(value,SharedIssuer::owned(),grant(d)).result.map(|(value,_)|value)}
fn text(value:String)->Result<SealedShared<String>,ValueError>{seal(value).map_err(|(error,_)|error)}
fn release_text(value:SealedShared<String>){let d=match value.original_transfer_demand(){Ok(d)=>d,Err(_)=>unreachable!("cold string remains initialized")};match value.try_unwrap(grant(d)).result{Ok((_value,_))=>{},Err(_)=>unreachable!("cold string is exclusively owned")}}
trait ColdRow:ColdRetire{type Source;fn receive(value:Self::Source)->Result<Self,ValueError> where Self:Sized;}
macro_rules! node{($row:path,$source:path,{$($field:ident),+})=>{
 impl ColdRow for $row{type Source=$source;fn receive(value:Self::Source)->Result<Self,ValueError>{let id=text(value.id)?;Ok(Self{id,$($field:value.$field),+})}}
}}
macro_rules! material{($row:path,$source:path,{$($field:ident),+})=>{
 impl ColdRow for $row{type Source=$source;fn receive(value:Self::Source)->Result<Self,ValueError>{let id=text(value.id)?;let name=match text(value.name){Ok(name)=>name,Err(error)=>{release_text(id);return Err(error)}};Ok(Self{id,name,$($field:value.$field),+})}}
}}
node!(planar::FemNode,planar::FemNodeValue,{x,y});node!(volumetric::FemNode,volumetric::FemNodeValue,{x,y,z});
material!(pm::FemMaterial,pm::FemMaterialValue,{e,nu,rho});material!(vm::FemMaterial,vm::FemMaterialValue,{e,nu,rho,g});
struct Builder<T:ColdRow>{inner:PersistentListBuilder<T>}
impl<T:ColdRow> Drop for Builder<T>{fn drop(&mut self){if !self.inner.cold_dismantle(){unreachable!("cold construction never publishes aliases")}}}
fn list<T:ColdRow>(rows:Vec<T::Source>)->Result<PersistentList<T>,ValueError>{
 let mut builder=Builder{inner:PersistentListBuilder::new(rows.len())};for source in rows {
  while !builder.inner.accepts_row(){let d=builder.inner.demand()?;builder.inner.advance(grant(d)).result?;}
  let value=T::receive(source)?;let mut value=match seal(value){Ok(value)=>Some(value),Err((error,value))=>{value.cold_release();return Err(error)}};
  let d=match builder.inner.adopt_demand(){Ok(d)=>d,Err(error)=>{release_row(value.take());return Err(error)}};
  match builder.inner.adopt(&mut value,grant(d)).result{Ok(Some(_))=>{},Ok(None)=>{release_row(value.take());return Err(ValueError::invariant("exact cold row adoption was refused"))},Err(error)=>{release_row(value.take());return Err(error)}}
 }
 let d=builder.inner.finish_demand()?;if builder.inner.finish(grant(d)).result?.is_none(){return Err(ValueError::invariant("exact cold finishing was refused"))}
 while !builder.inner.output_ready(){let d=builder.inner.demand()?;builder.inner.advance(grant(d)).result?;}
 let d=builder.inner.take_demand()?;let (output,_)=builder.inner.take(grant(d)).result?.ok_or_else(||ValueError::invariant("exact cold publication was refused"))?;
 Ok(output)
}
fn release_row<T:ColdRow>(value:Option<SealedShared<T>>){if let Some(value)=value{let d=match value.original_transfer_demand(){Ok(d)=>d,Err(_)=>unreachable!("cold row remains initialized")};match value.try_unwrap(grant(d)).result{Ok((value,_))=>value.cold_release(),Err(_)=>unreachable!("cold row is exclusively owned")}}}
fn discard<T:ColdRow>(mut value:PersistentList<T>)->Result<(),PersistentList<T>>{if value.cold_dismantle(){Ok(())}else{Err(value)}}
/// 📍️ Constructs cold planar node pages.
pub fn nodes2d(rows:Vec<planar::FemNodeValue>)->Result<PersistentList<planar::FemNode>,ValueError>{list(rows)}
/// 📍️ Constructs cold volumetric node pages.
pub fn nodes3d(rows:Vec<volumetric::FemNodeValue>)->Result<PersistentList<volumetric::FemNode>,ValueError>{list(rows)}
/// 🧱️ Constructs cold planar material pages.
pub fn materials2d(rows:Vec<pm::FemMaterialValue>)->Result<PersistentList<pm::FemMaterial>,ValueError>{list(rows)}
/// 🧱️ Constructs cold volumetric material pages.
pub fn materials3d(rows:Vec<vm::FemMaterialValue>)->Result<PersistentList<vm::FemMaterial>,ValueError>{list(rows)}
/// ♻️ Dismantles an unpublished planar node candidate.
pub fn discard_nodes2d(value:PersistentList<planar::FemNode>)->Result<(),PersistentList<planar::FemNode>>{discard(value)}
/// ♻️ Dismantles an unpublished volumetric node candidate.
pub fn discard_nodes3d(value:PersistentList<volumetric::FemNode>)->Result<(),PersistentList<volumetric::FemNode>>{discard(value)}
