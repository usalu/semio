//! 🧱️ Root setters prepare only their authored fields and retain original displaced ownership.
use crate::{GridPatch, LayoutDiff};
use semio_s_artifact_layout_layout_model::{LayoutDrawingMember,snapshot::LayoutSnapshot};
use semio_os_kernel::{ApplyCapability,ArtifactLink,RetainedFieldPreparation,RetainedFlight as Flight,retained_native_child,retained_routed,retained_settle};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use super::super::setters::OwnedPatch;
use std::mem::{ManuallyDrop,size_of};

struct Field<T:RetainedClone>{
    prepared:RetainedFieldPreparation<T>,
    displaced:ManuallyDrop<Option<T>>,
    retirement:RetainedCloneClose,
    flight:Flight,
}
impl<T:RetainedClone> Default for Field<T>{
    fn default()->Self{Self{prepared:Default::default(),displaced:ManuallyDrop::new(None),retirement:Default::default(),flight:Default::default()}}
}
impl<T:RetainedClone> Field<T>{
    fn preflight(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
        let mut spent=0;
        let result=(|| -> Result<RetainedCloneStep,RetirementPollExit>{
            let step=retained_routed!(self.flight,effect,recover,1,body,maximum,None,spent,
                |r| self.prepared.advance_into(source,body,maximum,effect,r),
                self.prepared.recover_advance_into(source,body,maximum,effect,r))?;
            Ok(if self.prepared.value().is_some(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
        })();
        *work+=spent;
        result
    }
    fn install_demand(&self)->Result<RetirementDemand,ValueError>{
        if self.displaced.is_some(){return Err(invalid())}
        let mut demand=self.prepared.exchange_demand()?;
        demand.copy_bytes=demand.copy_bytes.checked_add(2*size_of::<T>()).ok_or_else(invalid)?;
        Ok(demand)
    }
    fn install(&mut self,target:&mut T,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        let demand=self.install_demand()?;
        if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<2{
            return Ok(RetainedCloneStep::Progress(Default::default()))
        }
        if !effect.performed(1)?{return Err(invalid())}
        let mut work=0;
        let step=retained_native_child(effect,&mut work,|e,r|self.prepared.take_into(&mut self.displaced,e,r))??;
        let value=self.displaced.as_mut().ok_or_else(invalid)?;
        std::mem::swap(target,value);
        let mut progress=step.progress();
        progress.copied_bytes=demand.copy_bytes;
        Ok(RetainedCloneStep::Complete(progress))
    }
    fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.prepared.terminal_is_empty(){
            return if self.prepared.close_started(){self.prepared.close_demand(body)}else{Ok(self.prepared.begin_close_demand())}
        }
        self.retirement.next_owner_demands::<T>(self.displaced.is_some(),&None,body)
    }
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if !self.prepared.terminal_is_empty(){
            return if self.prepared.close_started(){self.prepared.close_work_demand(body)}else{Ok(self.prepared.begin_close_work_demand())}
        }
        self.retirement.next_owner_work::<T>(self.displaced.is_some(),&None,body)
    }
    fn requires_original(&self,birth:bool)->bool{
        if let Some(loaned)=self.flight.loaned(){return !birth&&loaned}
        if !self.prepared.terminal_is_empty(){
            self.prepared.close_started()&&if birth{self.prepared.requires_original_birth()}else{self.prepared.requires_original_control()}
        }else if birth{self.retirement.requires_original_birth()}else{self.retirement.requires_original_control()}
    }
    fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
        if !self.prepared.terminal_is_empty(){self.prepared.close_original_birth_step(original,grant)}else{self.retirement.step_original_birth(original,grant)}
    }
    fn close_step(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
        let base=effect.performed_work();
        let mut spent=0;
        let result=(|| -> Result<RetainedCloneStep,RetirementPollExit>{
            if !recover{
                if !retained_clone_grant_admits(effect.grant(),self.close_demand(body)?)||self.close_work_demand(body)?>effect.remaining_work_units()?{
                    return Ok(RetainedCloneStep::Progress(Default::default()))
                }
                if !self.prepared.terminal_is_empty()&&!self.prepared.close_started(){
                    return Ok(RetainedCloneStep::Progress(self.prepared.begin_close_paid(effect).result?))
                }
                if self.prepared.terminal_is_empty()&&self.retirement.is_empty(){
                    let started=self.retirement.begin_granted(&mut self.displaced,effect);
                    return Ok(started.result?.unwrap_or(RetainedCloneStep::Complete(Default::default())))
                }
            }
            let route=if recover{self.flight.route().ok_or_else(invalid)?}else if !self.prepared.terminal_is_empty(){2}else{3};
            let loaned=if recover{self.flight.loaned().unwrap_or(false)}else{self.requires_original(false)};
            let loan=if loaned{Some(original.as_deref_mut().ok_or_else(invalid)?)}else{None};
            let step=retained_routed!(self.flight,effect,recover,route,body,0,loan,spent,
                |r| match (route,loaned){
                    (2,false)=>self.prepared.close_into(body,effect,r),
                    (2,true)=>self.prepared.close_original_into(original.as_deref_mut().ok_or_else(invalid)?,body,effect,r),
                    (3,false)=>self.retirement.step_into(body,effect,r),
                    (3,true)=>self.retirement.step_original_into(original.as_deref_mut().ok_or_else(invalid)?,body,effect,r),
                    _=>Err(invalid()),
                },
                match (route,loaned){
                    (2,false)=>self.prepared.recover_close_into(body,effect,r),
                    (2,true)=>self.prepared.recover_close_original_into(original.as_deref_mut().ok_or_else(invalid)?,body,effect,r),
                    (3,false)=>self.retirement.recover_step_into(body,effect,r),
                    (3,true)=>self.retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(invalid)?,body,effect,r),
                    _=>Err(invalid()),
                })?;
            Ok(RetainedCloneStep::Progress(step.progress()))
        })();
        *work+=spent.max(effect.performed_work().saturating_sub(base));
        result
    }
    fn terminal_is_empty(&self)->bool{
        self.prepared.terminal_is_empty()&&self.displaced.is_none()&&self.retirement.is_empty()&&self.flight.idle()
    }
}

#[derive(Default)]
pub struct RootScalars{
    schema:Field<String>,
    name:Field<String>,
    print:Field<Option<String>>,
    background:Field<Option<LayoutDrawingMember>>,
    reference:Field<Option<ArtifactLink>>,
    grid:Field<GridPatch>,
    closing:bool,
}

fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:128,depth:1,..Default::default()}}
fn paid()->RetainedCloneStep{RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()})}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout root scalar field is invalid")}
fn schema(d:RetainedCloneRef<'_,LayoutDiff>)->RetainedCloneRef<'_,String>{d.project(0,|d|d.schema.as_ref().unwrap())}
fn name(d:RetainedCloneRef<'_,LayoutDiff>)->RetainedCloneRef<'_,String>{d.project(1,|d|d.name.as_ref().unwrap())}
fn grid(d:RetainedCloneRef<'_,LayoutDiff>)->RetainedCloneRef<'_,GridPatch>{d.project(2,|d|d.grid.as_ref().unwrap())}
fn print(d:RetainedCloneRef<'_,LayoutDiff>)->RetainedCloneRef<'_,Option<String>>{d.project(10,|d|&d.print_target.as_ref().unwrap().target)}
fn background(d:RetainedCloneRef<'_,LayoutDiff>)->RetainedCloneRef<'_,Option<LayoutDrawingMember>>{d.project(12,|d|d.background_drawing.as_ref().unwrap())}
fn reference(d:RetainedCloneRef<'_,LayoutDiff>)->RetainedCloneRef<'_,Option<ArtifactLink>>{d.project(14,|d|d.referenced_model.as_ref().unwrap())}
macro_rules! dispatch {
    ($self:ident,$index:expr,$field:ident,$case:expr) => {
        match $index {
            0=>{let $field=&mut $self.schema;$case},
            1=>{let $field=&mut $self.name;$case},
            2=>{let $field=&mut $self.print;$case},
            3=>{let $field=&mut $self.background;$case},
            4=>{let $field=&mut $self.reference;$case},
            5=>{let $field=&mut $self.grid;$case},
            _=>Err(invalid().into()),
        }
    }
}
macro_rules! selected {
    ($self:ident,$field:ident,$case:expr,$empty:expr) => {{
        if !$self.schema.terminal_is_empty(){let $field=&$self.schema;$case}
        else if !$self.name.terminal_is_empty(){let $field=&$self.name;$case}
        else if !$self.print.terminal_is_empty(){let $field=&$self.print;$case}
        else if !$self.background.terminal_is_empty(){let $field=&$self.background;$case}
        else if !$self.reference.terminal_is_empty(){let $field=&$self.reference;$case}
        else if !$self.grid.terminal_is_empty(){let $field=&$self.grid;$case}
        else{$empty}
    }}
}
impl RootScalars{
    pub fn supplied(d:&LayoutDiff,index:usize)->bool{
        match index{0=>d.schema.is_some(),1=>d.name.is_some(),2=>d.print_target.is_some(),3=>d.background_drawing.is_some(),4=>d.referenced_model.is_some(),5..=7=>d.grid.is_some(),_=>false}
    }
    pub fn ready(&self,index:usize)->bool{
        match index{0=>self.schema.prepared.value().is_some(),1=>self.name.prepared.value().is_some(),2=>self.print.prepared.value().is_some(),3=>self.background.prepared.value().is_some(),4=>self.reference.prepared.value().is_some(),5=>self.grid.displaced.is_some(),_=>false}
    }
    pub fn preflight_demand(&self,d:RetainedCloneRef<'_,LayoutDiff>,index:usize,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{
        if !Self::supplied(d.get(),index)||self.ready(index){return Ok(fixed())}
        match index{
            0=>self.schema.prepared.demand(schema(d),body,maximum),
            1=>self.name.prepared.demand(name(d),body,maximum),
            2=>self.print.prepared.demand(print(d),body,maximum),
            3=>self.background.prepared.demand(background(d),body,maximum),
            4=>self.reference.prepared.demand(reference(d),body,maximum),
            5 if self.grid.prepared.value().is_some()=>self.grid.prepared.exchange_demand(),
            5=>self.grid.prepared.demand(grid(d),body,maximum),
            _=>Err(invalid()),
        }
    }
    pub fn preflight_work_demand(&self,d:RetainedCloneRef<'_,LayoutDiff>,index:usize,body:usize,maximum:u64)->Result<u64,ValueError>{
        if !Self::supplied(d.get(),index)||self.ready(index){return Ok(1)}
        match index{
            0=>self.schema.prepared.advance_work_demand(schema(d),body,maximum),
            1=>self.name.prepared.advance_work_demand(name(d),body,maximum),
            2=>self.print.prepared.advance_work_demand(print(d),body,maximum),
            3=>self.background.prepared.advance_work_demand(background(d),body,maximum),
            4=>self.reference.prepared.advance_work_demand(reference(d),body,maximum),
            5 if self.grid.prepared.value().is_some()=>Ok(1),
            5=>self.grid.prepared.advance_work_demand(grid(d),body,maximum),
            _=>Err(invalid()),
        }
    }
    pub fn preflight(&mut self,d:RetainedCloneRef<'_,LayoutDiff>,index:usize,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
        if !recover{
            if !retained_clone_grant_admits(effect.grant(),self.preflight_demand(d,index,body,maximum)?)
                ||self.preflight_work_demand(d,index,body,maximum)?>maximum.min(effect.remaining_work_units()?){
                return Ok(RetainedCloneStep::Progress(Default::default()))
            }
            if !Self::supplied(d.get(),index)||self.ready(index){
                if !effect.performed(1)?{return Err(invalid().into())}
                *work+=1;
                return Ok(paid())
            }
        }
        match index{
            0=>self.schema.preflight(schema(d),body,maximum,effect,recover,work),
            1=>self.name.preflight(name(d),body,maximum,effect,recover,work),
            2=>self.print.preflight(print(d),body,maximum,effect,recover,work),
            3=>self.background.preflight(background(d),body,maximum,effect,recover,work),
            4=>self.reference.preflight(reference(d),body,maximum,effect,recover,work),
            5 if self.grid.prepared.value().is_some()&&!recover=>{
                Ok(retained_native_child(effect,work,|e,r|self.grid.prepared.take_into(&mut self.grid.displaced,e,r))??)
            },
            5=>self.grid.preflight(grid(d),body,maximum,effect,recover,work).map(|step|RetainedCloneStep::Progress(step.progress())),
            _=>Err(invalid().into()),
        }
    }
    pub fn commit_demand(&self,d:&LayoutDiff,index:usize)->Result<RetirementDemand,ValueError>{
        if !Self::supplied(d,index){return Ok(fixed())}
        match index{
            0=>self.schema.install_demand(),1=>self.name.install_demand(),2=>self.print.install_demand(),
            3=>self.background.install_demand(),4=>self.reference.install_demand(),
            5..=7=>self.grid.displaced.as_ref().ok_or_else(invalid)?.field_demand(index-5),
            _=>Err(invalid()),
        }
    }
    pub fn commit_work_demand(&self,d:&LayoutDiff,index:usize)->Result<u64,ValueError>{
        self.commit_demand(d,index)?;
        Ok(if Self::supplied(d,index)&&index<5{2}else{1})
    }
    pub fn commit(&mut self,d:&LayoutDiff,post:&mut LayoutSnapshot,index:usize,effect:&mut RetainedAllocationEffect<'_>,capability:ApplyCapability)->OriginalRetirementOutcome{
        let base=effect.performed_work();
        let result=(||{
            let grant=effect.grant();
            if !retained_clone_grant_admits(grant,self.commit_demand(d,index)?)
                ||effect.remaining_work_units()?<self.commit_work_demand(d,index)?{
                return Ok(RetainedCloneStep::Progress(Default::default()))
            }
            if !Self::supplied(d,index){
                if !effect.performed(1)?{return Err(invalid())}
                return Ok(paid())
            }
            match index{
                0=>self.schema.install(&mut post.schema,effect),
                1=>self.name.install(&mut post.name,effect),
                2=>self.print.install(&mut post.print_target,effect),
                3=>self.background.install(&mut post.background_drawing,effect),
                4=>self.reference.install(&mut post.referenced_model,effect),
                5..=7=>{
                    if !effect.performed(1)?{return Err(invalid())}
                    self.grid.displaced.as_mut().ok_or_else(invalid)?.exchange_field(&mut post.grid,index-5,grant,capability)
                },
                _=>Err(invalid()),
            }
        })().map_err(OriginalRetirementFailure::from);
        let work=effect.performed_work().saturating_sub(base);
        retained_settle(effect,base,result,work)
    }
    pub fn begin_close(&mut self){self.closing=true}
    fn closing_index(&self)->Option<usize>{
        [self.schema.terminal_is_empty(),self.name.terminal_is_empty(),self.print.terminal_is_empty(),self.background.terminal_is_empty(),self.reference.terminal_is_empty(),self.grid.terminal_is_empty()].into_iter().position(|empty|!empty)
    }
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{selected!(self,f,f.close_demand(body),Ok(Default::default()))}
    pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{selected!(self,f,f.close_work_demand(body),Ok(0))}
    pub fn requires_original_birth(&self)->bool{selected!(self,f,f.requires_original(true),false)}
    pub fn requires_original_control(&self)->bool{selected!(self,f,f.requires_original(false),false)}
    pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
        match self.closing_index(){
            Some(0)=>self.schema.close_original_birth_step(original,grant),
            Some(1)=>self.name.close_original_birth_step(original,grant),
            Some(2)=>self.print.close_original_birth_step(original,grant),
            Some(3)=>self.background.close_original_birth_step(original,grant),
            Some(4)=>self.reference.close_original_birth_step(original,grant),
            Some(5)=>self.grid.close_original_birth_step(original,grant),
            _=>semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Err(invalid().into()),work:0}),
        }
    }
    pub fn close_step(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
        if !self.closing{return Err(invalid().into())}
        let Some(index)=self.closing_index()else{return Ok(RetainedCloneStep::Complete(Default::default()))};
        dispatch!(self,index,field,field.close_step(original,body,effect,recover,work))
    }
    pub fn terminal_is_empty(&self)->bool{self.closing&&self.closing_index().is_none()}
}
impl Drop for RootScalars{
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Layout root scalar originals need funded close")}
}
