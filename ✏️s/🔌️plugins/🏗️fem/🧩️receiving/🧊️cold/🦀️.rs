//! 🧊️ Cold persistent-list assembly shared by the FEM models: every partial row and page stays retained, and each owner-less action runs in one pump-issued Source turn funded by exactly its quote.
use semio_framework_value::{RetainedAllocationEffect,RetirementDemand,ValueError,retained_clone::RetainedCloneGrant,retirement::{RetireOwned,controlled::ControlledRetirement},shared_issuance::{ColdRetire,SealedShared,SharedIssuer}};
use semio_framework_value_persistent::list::persistent::{PersistentList,PersistentListBuilder};

const COLD_RETIRE_STEPS:usize=1_000_000;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn cold<R>(d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<R,ValueError>{
 semio_framework_job::OriginalRetirementPump::new().turn(1<<16,u64::MAX,||Some(0),grant(d))?.with_effect(run)?.map(|(output,_)|output).ok_or_else(||ValueError::invariant("cold FEM assembly quote was refused"))
}
use crate::reply::step as settled;
fn seal<T:RetireOwned+Sync>(value:T)->Result<SealedShared<T>,(ValueError,T)>{
 let d=match SealedShared::<T>::source_admission_demand(){Ok(d)=>d,Err(error)=>return Err((error,value))};
 let mut input=Some(value);let mut output=None;
 let error=match cold(d,|effect|SealedShared::admit_from(&mut input,SharedIssuer::owned(),&mut output,effect)){Ok(turn)=>match turn.result{Ok(_)=>match output{Some(sealed)=>return Ok(sealed),None=>ValueError::invariant("cold FEM row issuance was refused")},Err(error)=>error},Err(error)=>error};
 Err((error,input.take().expect("a refused cold issuance keeps its input")))
}
/// 📝️ Seals one cold text field.
pub fn text(value:String)->Result<SealedShared<String>,ValueError>{seal(value).map_err(|(error,_)|error)}
/// ♻️ Releases an unpublished cold text field.
pub fn release_text(value:SealedShared<String>){let d=match value.original_transfer_demand(){Ok(d)=>d,Err(_)=>unreachable!("cold string remains initialized")};match value.try_unwrap(grant(d)).result{Ok((_value,_))=>{},Err(_)=>unreachable!("cold string is exclusively owned")}}
fn release_row<T:ColdRetire+Send+Sync>(value:Option<SealedShared<T>>){if let Some(value)=value{let d=match value.original_transfer_demand(){Ok(d)=>d,Err(_)=>unreachable!("cold row remains initialized")};match value.try_unwrap(grant(d)).result{Ok((value,_))=>value.cold_release(),Err(_)=>unreachable!("cold row is exclusively owned")}}}
struct Builder<T:ColdRetire>{inner:PersistentListBuilder<T>}
impl<T:ColdRetire> Drop for Builder<T>{fn drop(&mut self){if !self.inner.cold_dismantle(){unreachable!("cold construction never publishes aliases")}}}
/// ❄️ Assembles cold persistent pages from authored rows, each built by `receive`.
pub fn list<T:ColdRetire,S>(rows:Vec<S>,receive:impl Fn(S)->Result<T,ValueError>)->Result<PersistentList<T>,ValueError>{
 let mut builder=Builder{inner:PersistentListBuilder::new(rows.len())};for source in rows {
  while !builder.inner.accepts_row(){let d=builder.inner.demand()?;cold(d,|effect|settled(effect,|effect,reply|builder.inner.advance_into(effect,reply)))??;}
  let value=receive(source)?;let mut value=match seal(value){Ok(value)=>Some(value),Err((error,value))=>{value.cold_release();return Err(error)}};
  let d=match builder.inner.adopt_demand(){Ok(d)=>d,Err(error)=>{release_row(value.take());return Err(error)}};
  match cold(d,|effect|settled(effect,|effect,reply|builder.inner.adopt_into(&mut value,effect,reply))){Ok(Ok(_)) if value.is_none()=>{},Ok(Ok(_))=>{release_row(value.take());return Err(ValueError::invariant("exact cold row adoption was refused"))},Ok(Err(error))|Err(error)=>{release_row(value.take());return Err(error)}}
 }
 let d=builder.inner.finish_demand()?;cold(d,|effect|settled(effect,|effect,reply|builder.inner.finish_into(effect,reply)))??;if !builder.inner.finishing(){return Err(ValueError::invariant("exact cold finishing was refused"))}
 while !builder.inner.output_ready(){let d=builder.inner.demand()?;cold(d,|effect|settled(effect,|effect,reply|builder.inner.advance_into(effect,reply)))??;}
 let d=builder.inner.take_demand()?;let mut output=None;cold(d,|effect|settled(effect,|effect,reply|builder.inner.take_into(&mut output,effect,reply)))??;
 output.ok_or_else(||ValueError::invariant("exact cold publication was refused"))
}
/// ❄️ Assembles and seals one cold persistent list so a snapshot field can alias it.
pub fn sealed_list<T:ColdRetire+Send+Sync,S>(rows:Vec<S>,receive:impl Fn(S)->Result<T,ValueError>)->Result<SealedShared<PersistentList<T>>,ValueError>{
 let value=list(rows,receive)?;
 seal(value).map_err(|(error,value)|{if discard(value).is_err(){unreachable!("unpublished cold pages have no aliases")}error})
}
/// ♻️ Dismantles an unpublished sealed cold list.
pub fn discard_sealed<T:ColdRetire+Send+Sync>(value:SealedShared<PersistentList<T>>)->Result<(),()>{
 let d=value.original_transfer_demand().map_err(|_|())?;
 match value.try_unwrap(grant(d)).result{Ok((list,_))=>discard(list).map_err(|_|()),Err(_)=>Err(())}
}
/// ♻️ Dismantles an unpublished cold candidate.
pub fn discard<T:ColdRetire>(mut value:PersistentList<T>)->Result<(),PersistentList<T>>{if value.cold_dismantle(){Ok(())}else{Err(value)}}
/// ♻️ Retires one decoded but unused owner in owner-less pump turns, each funded by exactly its quote.
pub fn retire<T:RetireOwned>(value:T){
 let Ok(mut owner)=ControlledRetirement::new(value) else{return};
 for _ in 0..COLD_RETIRE_STEPS{
  if owner.terminal_is_empty(){return}
  let Ok(first)=owner.next_demand(0) else{return};let body=4096.max(first.copy_bytes);
  let Ok(demand)=owner.next_demand(body) else{return};
  let funded=RetainedCloneGrant{maximum_copy_bytes:body,..grant(demand)};
  let Ok(polled)=semio_framework_job::OriginalRetirementPump::new().turn(1<<16,u64::MAX,||Some(0),funded).and_then(|mut turn|turn.with_effect(|effect|owner.step(body,effect))) else{return};
  match polled{Some((semio_framework_value::RetainedWorkPoll::Complete(turn),_)) if turn.result.is_ok()=>{},_=>return}
 }
}
