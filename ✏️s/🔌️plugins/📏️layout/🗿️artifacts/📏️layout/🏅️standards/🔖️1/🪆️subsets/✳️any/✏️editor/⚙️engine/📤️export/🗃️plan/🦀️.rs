//! 🗃️ Export plan rows retain original paged backing and owned append failures through paid teardown.
use crate::output::OUTPUT_BODY_BYTES;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value_persistent::list::PagedList;
use semio_framework_2d_text_font::work::RetainedWorkLedger;
#[derive(semio_framework_value::RetireOwned)]
struct OriginalRows<T:RetireOwned>{rows:PagedList<T,{usize::MAX}>,fault:Option<T>}
pub struct PlanList<T:RetireOwned>{source:Option<OriginalRows<T>>,retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>}
pub type PlanOffsets=PlanList<u32>;
fn refusal(code:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,code)}
impl<T:RetireOwned> PlanList<T>{
 pub fn new()->Self{Self{source:Some(OriginalRows{rows:PagedList::new(),fault:None}),retirement:None}}
 pub fn len(&self)->usize{self.source.as_ref().map_or(0,|source|source.rows.len())}
 pub fn is_empty(&self)->bool{self.len()==0}
 pub fn capacity(&self)->usize{self.source.as_ref().map_or(0,|source|source.rows.capacity())}
 pub fn get(&self,index:usize)->Option<&T>{self.source.as_ref()?.rows.get(index)}
 pub fn get_mut(&mut self,index:usize)->Option<&mut T>{self.source.as_mut()?.rows.get_mut(index)}
 pub fn last(&self)->Option<&T>{self.len().checked_sub(1).and_then(|index|self.get(index))}
 pub fn last_mut(&mut self)->Option<&mut T>{self.len().checked_sub(1).and_then(|index|self.get_mut(index))}
 pub fn iter(&self)->impl Iterator<Item=&T>{self.source.iter().flat_map(|source|source.rows.iter())}
 pub fn next_append_demand(&self)->Result<RetirementDemand,ValueError>{let source=self.source.as_ref().ok_or_else(||refusal("layout plan source closed"))?;if source.fault.is_some(){return Err(refusal("layout plan owned append fault"));}RetainedWorkLedger::slot_demand(&source.rows).map_err(refusal)}
 pub fn reserve_one_in(&mut self,work:&mut RetainedWorkLedger,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<bool,ValueError>{let source=self.source.as_mut().ok_or_else(||refusal("layout plan source closed"))?;if source.fault.is_some(){return Err(refusal("layout plan owned append fault"));}work.slot(&mut source.rows,effect)}
 pub fn push_in(&mut self,value:T,work:&mut RetainedWorkLedger)->Result<(),ValueError>{let source=self.source.as_mut().expect("plan producer requires original reservation");match work.push(&mut source.rows,value){Ok(())=>Ok(()),Err((code,value))=>{source.fault=Some(value);Err(refusal(code).with_retained_progress(work.receipt()))}}}
 pub fn next_close_demand(&self)->Result<RetirementDemand,ValueError>{owner_close_demand(&self.source,&self.retirement)}
 pub fn close_one(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
    close_owner_slot(&mut self.source,&mut self.retirement,effect)
 }
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.retirement.is_none()}
}
impl PlanOffsets{
 pub fn push_reserved_in(&mut self,value:u32,work:&mut RetainedWorkLedger)->Result<(),ValueError>{
    let source=self.source.as_mut().ok_or_else(||refusal("layout plan source closed"))?;
    if !source.rows.has_reserved_slot()||!work.copy_bytes(std::mem::size_of::<u32>()){
        source.fault=Some(value);
        return Err(refusal("layout plan offset lacks its admitted reservation"))
    }
    source.rows.push_reserved(value).map_err(|_|refusal("layout plan reserved append failed"))
 }
}
pub fn owner_close_demand<T:RetireOwned>(source:&Option<T>,retirement:&Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>)->Result<RetirementDemand,ValueError>{
    if let Some(owner)=retirement.as_ref(){return semio_framework_value::factory_ticket_demands(owner,OUTPUT_BODY_BYTES)}
    source.as_ref().map_or(Ok(Default::default()),semio_framework_value::retirement::controlled::controlled_retirement_birth_demand)
}
pub fn close_owner_slot<T:RetireOwned>(source:&mut Option<T>,retirement:&mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
    use semio_framework_value::{RetainedWorkOutcome,RetainedWorkPoll};
    if retirement.is_some(){return close_owner_box(retirement,effect)}
    if source.is_none(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
    let turn=semio_framework_value::admit_owned_retirement(source,effect);
    let result=turn.result.map(|value|{
        let progress=if let Some((owner,progress))=value{*retirement=Some(owner);progress}else{Default::default()};
        RetainedCloneStep::Progress(progress)
    }).map_err(Into::into);
    RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:turn.work})
}
pub fn close_owner_box(retirement:&mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
    if let Some(owner)=retirement.as_ref(){
        match semio_framework_value::factory_ticket_demands(owner,OUTPUT_BODY_BYTES){
            Err(error)=>return semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Err(error.into()),work:0}),
            Ok(demand) if !semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),demand)=>return semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}),
            Ok(_)=>{},
        }
    }
    semio_framework_value::close_factory_ticket(retirement,OUTPUT_BODY_BYTES,effect)
}
impl<T:RetireOwned> std::ops::Index<usize> for PlanList<T>{type Output=T;fn index(&self,index:usize)->&T{self.get(index).expect("plan item address")}}
impl<T:RetireOwned> std::ops::IndexMut<usize> for PlanList<T>{fn index_mut(&mut self,index:usize)->&mut T{self.get_mut(index).expect("plan item address")}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
