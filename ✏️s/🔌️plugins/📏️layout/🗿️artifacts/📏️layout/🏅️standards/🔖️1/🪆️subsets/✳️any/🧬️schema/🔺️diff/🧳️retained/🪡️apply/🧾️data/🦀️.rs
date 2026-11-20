//! 🧾️ Optional dictionaries preserve presence independently of their sparse entry owners.
use crate::{LayoutDiff,LayoutDataEntriesDelta,DataFieldsPresence};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_value_dictionary::{QuestionDictionary,QuestionDictionaryEntry};
use semio_os_kernel::{ApplyCapability,RetainedFlight as Flight,retained_routed,retained_settle};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use super::super::{collection::RetainedCollection,keyed_plan::PlanAdvance};
use std::mem::{size_of,ManuallyDrop};
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<QuestionDictionary>>()*4+128,depth:1,..Default::default()}}
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout retained dictionary phase is invalid")}
fn source(d:RetainedCloneRef<'_,LayoutDiff>)->RetainedCloneRef<'_,LayoutDataEntriesDelta>{d.project(11,|d|d.data_fields.as_ref().unwrap()).project(1,|d|d.entries.as_ref().unwrap())}
pub struct DataWork{entries:RetainedCollection<QuestionDictionaryEntry,LayoutDataEntriesDelta>,old:ManuallyDrop<Option<QuestionDictionary>>,retirement:RetainedCloneClose,flight:Flight,phase:u8,closing:bool}
impl Default for DataWork{fn default()->Self{Self{entries:RetainedCollection::new(),old:ManuallyDrop::new(None),retirement:Default::default(),flight:Default::default(),phase:0,closing:false}}}
impl DataWork{
 fn active(d:&LayoutDiff)->bool{d.data_fields.as_ref().is_some_and(|d|d.presence!=Some(DataFieldsPresence::Deleted)&&d.entries.is_some())}
 fn baseline<'a>(d:&LayoutDiff,post:&'a LayoutSnapshot)->&'a [QuestionDictionaryEntry]{if d.data_fields.as_ref().is_some_and(|d|d.presence==Some(DataFieldsPresence::Replaced)){&[]}else{post.data_fields.as_ref().map_or(&[],|d|d.entries.as_slice())}}
 fn missing(d:&LayoutDiff,post:&LayoutSnapshot)->bool{
    post.data_fields.is_none()&&d.data_fields.as_ref().is_some_and(|delta|{
        delta.presence.is_none()&&delta.entries.as_ref().is_some_and(|e|!e.removed.is_empty()||!e.inserted.is_empty()||!e.moved.is_empty())
    })
 }
 pub fn preflight_demand(&self,d:RetainedCloneRef<'_,LayoutDiff>,post:&LayoutSnapshot,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{
    if Self::active(d.get())&&!Self::missing(d.get(),post){
        self.entries.preflight_demand(source(d),Self::baseline(d.get(),post),body,maximum)
    }else{Ok(fixed())}
 }
 pub fn preflight_work_demand(&self,d:RetainedCloneRef<'_,LayoutDiff>,post:&LayoutSnapshot,body:usize,maximum:u64)->Result<u64,ValueError>{
    if Self::active(d.get())&&!Self::missing(d.get(),post){
        self.entries.preflight_work_demand(source(d),Self::baseline(d.get(),post),body,maximum)
    }else{Ok(1)}
 }
 pub fn preflight(&mut self,d:RetainedCloneRef<'_,LayoutDiff>,post:&LayoutSnapshot,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<PlanAdvance,RetirementPollExit>{
    let missing=Self::missing(d.get(),post);
    if Self::active(d.get())&&!missing{
        return self.entries.preflight(source(d),Self::baseline(d.get(),post),body,maximum,effect,recover,work)
    }
    if recover{return Err(fault().into())}
    let demand=fixed();
    if !semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),demand)
        ||maximum.min(effect.remaining_work_units()?)==0{
        return Ok(PlanAdvance{progress:Default::default(),done:false,error:None})
    }
    if !effect.performed(1)?{return Err(fault().into())}
    *work+=1;
    Ok(PlanAdvance{
        progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},
        done:!missing,error:missing.then_some("layout.data.missing-target"),
    })
 }
 pub fn commit_demand(&self,d:&LayoutDiff)->Result<RetirementDemand,ValueError>{
    if self.phase==0||!Self::active(d){Ok(fixed())}else{self.entries.membership_commit_demand()}
 }
 pub fn commit_work_demand(&self,d:&LayoutDiff)->Result<u64,ValueError>{self.commit_demand(d)?;Ok(1)}
 pub fn commit(&mut self,d:&LayoutDiff,post:&mut LayoutSnapshot,effect:&mut RetainedAllocationEffect<'_>,capability:ApplyCapability)->OriginalRetirementOutcome{
    let base=effect.performed_work();
    let result=(||{
        let demand=self.commit_demand(d)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?==0{
            return Ok(RetainedCloneStep::Progress(Default::default()))
        }
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        if self.phase==0{
            if !effect.performed(1)?{return Err(fault())}
            if let Some(delta)=&d.data_fields{
                match delta.presence{
                    Some(DataFieldsPresence::Deleted)=>*self.old=post.data_fields.take(),
                    Some(DataFieldsPresence::Replaced)=>{
                        *self.old=post.data_fields.take();
                        post.data_fields=Some(QuestionDictionary{entries:Vec::new()})
                    },
                    Some(DataFieldsPresence::Created) if post.data_fields.is_none()=>post.data_fields=Some(QuestionDictionary{entries:Vec::new()}),
                    _=>{},
                }
            }
            self.phase=1;
            return Ok(RetainedCloneStep::Progress(progress))
        }
        if Self::active(d){
            if let Some(data)=post.data_fields.as_mut(){return self.entries.commit_membership(&mut data.entries,effect,capability)}
        }
        if !effect.performed(1)?{return Err(fault())}
        Ok(RetainedCloneStep::Complete(progress))
    })().map_err(OriginalRetirementFailure::from);
    let work=effect.performed_work().saturating_sub(base);
    retained_settle(effect,base,result,work)
 }
 pub fn begin_close(&mut self){self.closing=true;self.entries.begin_close()}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.entries.terminal_is_empty(){return self.entries.close_demand(body)}self.retirement.next_owner_demands::<QuestionDictionary>(self.old.is_some(),&None,body)}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
    if !self.entries.terminal_is_empty(){return self.entries.close_work_demand(body)}
    self.retirement.next_owner_work::<QuestionDictionary>(self.old.is_some(),&None,body)
 }
 pub fn requires_original_birth(&self)->bool{
    if !self.entries.terminal_is_empty(){return self.entries.requires_original_birth()}
    self.flight.idle()&&self.retirement.requires_original_birth()
 }
 pub fn requires_original_control(&self)->bool{
    if !self.entries.terminal_is_empty(){return self.entries.requires_original_control()}
    self.flight.loaned().unwrap_or_else(||self.retirement.requires_original_control())
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    if !self.entries.terminal_is_empty(){return self.entries.close_original_birth_step(original,grant)}
    self.retirement.step_original_birth(original,grant)
 }
 pub fn close_step(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
    if !self.closing{return Err(fault().into())}
    if !self.entries.terminal_is_empty(){
        return self.entries.close_step(original,body,effect,recover,work).map(|step|RetainedCloneStep::Progress(step.progress()))
    }
    let base=effect.performed_work();
    let mut spent=0;
    let result=(|| -> Result<RetainedCloneStep,RetirementPollExit>{
        if !recover{
            if !semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),self.close_demand(body)?)
                ||self.close_work_demand(body)?>effect.remaining_work_units()?{
                return Ok(RetainedCloneStep::Progress(Default::default()))
            }
            if self.retirement.is_empty(){
                return Ok(self.retirement.begin_granted(&mut self.old,effect).result?.unwrap_or(RetainedCloneStep::Complete(Default::default())))
            }
        }
        let loaned=self.requires_original_control();
        let loan=if loaned{Some(original.as_deref_mut().ok_or_else(fault)?)}else{None};
        let step=retained_routed!(self.flight,effect,recover,1,body,0,loan,spent,
            |r| if loaned{self.retirement.step_original_into(original.as_deref_mut().ok_or_else(fault)?,body,effect,r)}else{self.retirement.step_into(body,effect,r)},
            if loaned{self.retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(fault)?,body,effect,r)}else{self.retirement.recover_step_into(body,effect,r)})?;
        Ok(RetainedCloneStep::Progress(step.progress()))
    })();
    *work+=spent.max(effect.performed_work().saturating_sub(base));
    result
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.entries.terminal_is_empty()&&self.old.is_none()&&self.retirement.is_empty()&&self.flight.idle()}
}
impl Drop for DataWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"dictionary original owners require funded close")}}
