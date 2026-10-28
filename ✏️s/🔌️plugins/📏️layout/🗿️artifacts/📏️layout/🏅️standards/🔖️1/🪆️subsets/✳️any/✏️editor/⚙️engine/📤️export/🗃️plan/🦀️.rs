//! 🗃️ Export plan rows retain original paged backing and owned append failures through paid teardown.
use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value_persistent::list::PagedList;
use semio_framework_2d_text_font::work::FontWorkLedger;
#[derive(semio_framework_value::RetireOwned)]
struct OriginalRows<T:RetireOwned>{rows:PagedList<T,{usize::MAX}>,fault:Option<T>}
pub struct PlanList<T:RetireOwned>{source:Option<OriginalRows<T>>,retirement:Option<Box<dyn store::ErasedSnapshotRetirement>>}
pub(super) type PlanItems=PlanList<PdfItem>;
pub(super) type PlanPages=PlanList<PdfPagePlan>;
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
 pub fn next_append_demand(&self)->Result<RetirementDemand,ValueError>{let source=self.source.as_ref().ok_or_else(||refusal("layout plan source closed"))?;if source.fault.is_some(){return Err(refusal("layout plan owned append fault"));}FontWorkLedger::slot_demand(&source.rows).map_err(refusal)}
 pub fn reserve_one_in(&mut self,work:&mut FontWorkLedger)->Result<bool,ValueError>{let source=self.source.as_mut().ok_or_else(||refusal("layout plan source closed"))?;if source.fault.is_some(){return Err(refusal("layout plan owned append fault"));}work.slot(&mut source.rows).map_err(refusal)}
 pub fn push_in(&mut self,value:T,work:&mut FontWorkLedger)->Result<(),ValueError>{let source=self.source.as_mut().expect("plan producer requires original reservation");match work.push(&mut source.rows,value){Ok(())=>Ok(()),Err((code,value))=>{source.fault=Some(value);Err(refusal(code).with_retained_progress(work.receipt()))}}}
 pub fn next_close_demand(&self)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,0);}if self.source.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.source);}Ok(Default::default())}
 pub fn close_one(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant);}if self.source.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.source,&mut self.retirement,grant);}Ok(RetainedCloneStep::Complete(Default::default()))}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.retirement.is_none()}
}
impl<T:RetireOwned> std::ops::Index<usize> for PlanList<T>{type Output=T;fn index(&self,index:usize)->&T{self.get(index).expect("plan item address")}}
impl<T:RetireOwned> std::ops::IndexMut<usize> for PlanList<T>{fn index_mut(&mut self,index:usize)->&mut T{self.get_mut(index).expect("plan item address")}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
