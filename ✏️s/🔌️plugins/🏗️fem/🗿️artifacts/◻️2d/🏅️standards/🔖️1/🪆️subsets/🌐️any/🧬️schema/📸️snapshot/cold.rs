//! ❄️ Cold FEM assembly retains each partial row and dismantles exclusive pages without allocation.
use semio_framework_value::{RetainedAllocationEffect,RetirementDemand,ValueError,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneGrant,RetainedCloneStep},retirement::RetireOwned,shared_issuance::{SealedShared,SharedIssuer,ColdRetire}};
use semio_framework_value_persistent::list::persistent::{PersistentList,PersistentListBuilder};
use semio_s_fem_node::{planar,volumetric};use semio_s_fem_material::{planar as pm,volumetric as vm};

fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
/// 🧊️ Runs one owner-less cold construction action inside a pump turn funded by exactly its quote.
fn cold<R>(d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<R,ValueError>{
 semio_framework_job::OriginalRetirementPump::new().turn(1<<16,u64::MAX,||Some(0),grant(d))?.with_effect(run)?.map(|(output,_)|output).ok_or_else(||ValueError::invariant("cold FEM assembly quote was refused"))
}
/// 📬️ Runs one reply-shaped step inside its own action; a reply left pending is refused typed.
fn settled(effect:&mut RetainedAllocationEffect<'_>,call:impl for<'r,'e,'a> FnOnce(&'e mut RetainedAllocationEffect<'a>,OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>)->Result<RetainedCloneStep,ValueError>{
 let mut slot=None;
 let done=call(&mut *effect,OriginalEffectReply::borrow(&mut slot)?)?;
 drop(done);
 slot.take().ok_or_else(||ValueError::invariant("cold FEM assembly step stayed pending"))?
}
fn seal<T:RetireOwned+Sync>(value:T)->Result<SealedShared<T>,(ValueError,T)>{
 let d=match SealedShared::<T>::source_admission_demand(){Ok(d)=>d,Err(error)=>return Err((error,value))};
 let mut input=Some(value);let mut output=None;
 let turn=cold(d,|effect|SealedShared::admit_from(&mut input,SharedIssuer::owned(),&mut output,effect));
 let error=match turn{Ok(turn)=>match turn.result{Ok(_)=>match output{Some(sealed)=>return Ok(sealed),None=>ValueError::invariant("cold FEM row issuance was refused")},Err(error)=>error},Err(error)=>error};
 Err((error,input.take().expect("a refused cold issuance keeps its input")))
}
const COLD_RETIRE_STEPS:usize=1_000_000;
/// ♻️ Retires one decoded but unused owner in owner-less pump turns, each funded by exactly its quote.
pub(super) fn retire<T:RetireOwned>(value:T){
 let Ok(mut owner)=semio_framework_value::retirement::controlled::ControlledRetirement::new(value) else{return};
 for _ in 0..COLD_RETIRE_STEPS{
  if owner.terminal_is_empty(){return}
  let Ok(first)=owner.next_demand(0) else{return};let body=4096.max(first.copy_bytes);
  let Ok(demand)=owner.next_demand(body) else{return};
  let funded=RetainedCloneGrant{maximum_copy_bytes:body,..grant(demand)};
  let Ok(poll)=semio_framework_job::OriginalRetirementPump::new().turn(1<<16,u64::MAX,||Some(0),funded).and_then(|mut turn|turn.with_effect(|effect|owner.step(body,effect))) else{return};
  match poll{Some((semio_framework_value::RetainedWorkPoll::Complete(turn),_)) if turn.result.is_ok()=>{},_=>return}
 }
}
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
  while !builder.inner.accepts_row(){let d=builder.inner.demand()?;cold(d,|effect|settled(effect,|effect,reply|builder.inner.advance_into(effect,reply)))??;}
  let value=T::receive(source)?;let mut value=match seal(value){Ok(value)=>Some(value),Err((error,value))=>{value.cold_release();return Err(error)}};
  let d=match builder.inner.adopt_demand(){Ok(d)=>d,Err(error)=>{release_row(value.take());return Err(error)}};
  match cold(d,|effect|settled(effect,|effect,reply|builder.inner.adopt_into(&mut value,effect,reply))){Ok(Ok(_)) if value.is_none()=>{},Ok(Ok(_))=>{release_row(value.take());return Err(ValueError::invariant("exact cold row adoption was refused"))},Ok(Err(error))|Err(error)=>{release_row(value.take());return Err(error)}}
 }
 let d=builder.inner.finish_demand()?;cold(d,|effect|settled(effect,|effect,reply|builder.inner.finish_into(effect,reply)))??;if !builder.inner.finishing(){return Err(ValueError::invariant("exact cold finishing was refused"))}
 while !builder.inner.output_ready(){let d=builder.inner.demand()?;cold(d,|effect|settled(effect,|effect,reply|builder.inner.advance_into(effect,reply)))??;}
 let d=builder.inner.take_demand()?;let mut output=None;cold(d,|effect|settled(effect,|effect,reply|builder.inner.take_into(&mut output,effect,reply)))??;
 output.ok_or_else(||ValueError::invariant("exact cold publication was refused"))
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
