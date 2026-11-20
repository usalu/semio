//! 📦️ Sparse collection staging never clones an existing row or document snapshot.
use super::{keyed_plan::{KeyedPlan,PlanAdvance},borrowed_rows::{BorrowedRows,BorrowedDelta,BorrowedKey},list_move::{TypedListMove,OwnedInsertion}};
use semio_os_kernel::{ApplyCapability,RetainedFieldPreparation,RetainedFlight as Flight,retained_native_child,retained_routed};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,retained_clone::{RetainedClone,RetainedCloneRef,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{size_of,ManuallyDrop};
/// 📤 A real sparse delta transfers its already-admitted insertion vector once.
pub trait OwnedDelta<R>:BorrowedDelta<R>+RetainedClone{type Insertion:OwnedInsertion<R>;fn insertion_rows(&mut self)->&mut Vec<Self::Insertion>;}
/// 🧳 The original sparse payload, source lease, rows and numeric plans retain explicit custody.
pub struct RetainedCollection<R:RetireOwned+BorrowedKey,D:OwnedDelta<R>>{plan:KeyedPlan,field:RetainedFieldPreparation<D>,staged:ManuallyDrop<Option<D>>,authored:ManuallyDrop<Option<Vec<D::Insertion>>>,rows:TypedListMove<R,D::Insertion>,retirement:RetainedCloneClose,flight:Flight,phase:u8,closing:bool}
impl<R:RetireOwned+BorrowedKey,D:OwnedDelta<R>> Default for RetainedCollection<R,D>{fn default()->Self{Self::new()}}
impl<R:RetireOwned+BorrowedKey,D:OwnedDelta<R>> RetainedCollection<R,D>{
 pub fn new()->Self{Self{plan:KeyedPlan::new(),field:Default::default(),staged:ManuallyDrop::new(None),authored:ManuallyDrop::new(None),rows:TypedListMove::new(),retirement:Default::default(),flight:Default::default(),phase:0,closing:false}}
 fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
 fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:128,depth:1,..Default::default()}}
 fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=128;d.depth+=1;d}
 fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-128,maximum_depth:g.maximum_depth-1,..g}}
 fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=128;p.copied_items=p.copied_items.max(1);p}
 fn refused(error:ValueError)->ValueError{let p=Self::receipt(error.retained_progress());error.with_retained_progress(p)}
 pub fn preflight_demand(&self,source:RetainedCloneRef<'_,D>,post:&[R],body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{
    if self.closing||self.phase>5{return Err(Self::fault("sparse collection preflight is settled"))}
    let inner=body.saturating_sub(128);
    match self.phase{
        0=>self.plan.demand(&BorrowedRows{held:post,delta:source.get()}).map(Self::nested),
        1=>self.field.demand(source,inner,maximum.saturating_sub(1)).map(Self::nested),
        2=>self.field.exchange_demand().map(Self::nested),
        4=>self.rows.reserve_demand(post.len(),source.get().inserted_len(),self.plan.origins().expect("admitted numeric list").len()).map(Self::nested),
        5=>Ok(Self::fixed()),
        _=>Err(Self::fault("sparse collection preflight phase is invalid")),
    }
 }
 pub fn preflight_work_demand(&self,source:RetainedCloneRef<'_,D>,post:&[R],body:usize,maximum:u64)->Result<u64,ValueError>{
    self.preflight_demand(source,post,body,maximum)?;
    let child=match self.phase{1=>self.field.advance_work_demand(source,body.saturating_sub(128),maximum.saturating_sub(1))?,2|4=>1,_=>0};
    child.checked_add(1).ok_or_else(||Self::fault("sparse collection work exceeds its bound"))
 }
 pub fn preflight(&mut self,source:RetainedCloneRef<'_,D>,post:&[R],body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<PlanAdvance,RetirementPollExit>{
    let base=effect.performed_work();
    let mut spent=0;
    let result=(|| -> Result<PlanAdvance,RetirementPollExit>{
        let mut result=PlanAdvance{progress:Default::default(),done:false,error:None};
        let grant=effect.grant();
        if !recover{
            let demand=self.preflight_demand(source,post,body,maximum)?;
            if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand)
                ||self.preflight_work_demand(source,post,body,maximum)?>maximum.min(effect.remaining_work_units()?){return Ok(result)}
            if !effect.performed(1)?{return Err(Self::fault("sparse collection preflight exceeded original work").into())}
            spent=1;
        }
        let inner=body.saturating_sub(128);
        let limit=maximum.saturating_sub(1);
        match self.phase{
            0=>{
                let source=BorrowedRows{held:post,delta:source.get()};
                let demand=self.plan.demand(&source)?;
                if demand.capacity_bytes!=0{
                    if effect.actual().is_some(){return Err(Self::fault("sparse collection allocation marker is occupied").into())}
                    *effect.actual()=Some(0);
                }
                let step=self.plan.advance(&source,Self::child(grant));
                if demand.capacity_bytes!=0{
                    let progress=step.as_ref().map_or_else(ValueError::retained_progress,|step|step.progress);
                    *effect.actual()=Some(progress.retained_capacity_bytes);
                }
                let step=step.map_err(Self::refused)?;
                result.progress=Self::receipt(step.progress);
                result.error=step.error;
                if step.done{self.phase=1}
            },
            1=>{
                let step=retained_routed!(self.flight,effect,recover,1,inner,limit,None,spent,
                    |r| self.field.advance_into(source,inner,limit,&mut effect.nested(Self::child(grant)),r),
                    self.field.recover_advance_into(source,inner,limit,&mut effect.nested(Self::child(grant)),r))?;
                result.progress=Self::receipt(step.progress());
                if self.field.value().is_some(){self.phase=2}
            },
            2=>{
                let step=retained_native_child(effect,&mut spent,|e,r|self.field.take_into(&mut self.staged,&mut e.nested(Self::child(grant)),r))??;
                result.progress=Self::receipt(step.progress());
                if self.staged.is_some(){self.phase=4}
            },
            4=>{
                let output=self.plan.origins().expect("admitted output coordinates").len();
                let step=self.rows.reserve_effect(post.len(),source.get().inserted_len(),output,&mut effect.nested(Self::child(grant))).map_err(Self::refused)?;
                result.progress=Self::receipt(step.progress());
                if matches!(step,RetainedCloneStep::Complete(_)){self.phase=5;result.done=true}
            },
            5=>{result.progress=Self::receipt(Default::default());result.done=true},
            _=>return Err(Self::fault("sparse collection preflight phase is invalid").into()),
        }
        Ok(result)
    })();
    *work+=spent.max(effect.performed_work().saturating_sub(base));
    result
 }
 pub fn membership_commit_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.phase<5{return Err(Self::fault("sparse collection commit lacks read-only admission"))}if self.phase==5{Ok(RetirementDemand{copy_bytes:size_of::<Option<Vec<D::Insertion>>>()*2+size_of::<Vec<D::Insertion>>()*2+128,depth:1,..Default::default()})}else{self.rows.commit_demand().map(Self::nested)}}
 pub fn commit_membership(&mut self,post:&mut Vec<R>,effect:&mut RetainedAllocationEffect<'_>,capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>{let d=self.membership_commit_demand()?;let g=effect.grant();if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d)||effect.remaining_work_units()?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(Self::fault("sparse collection commit exceeded original work"))}if self.phase==5{*self.authored=Some(std::mem::take(self.staged.as_mut().expect("admitted authored sparse delta").insertion_rows()));self.phase=6;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}if self.phase==6{let step=self.rows.capture(post,&mut self.authored,Self::child(g),capability).map_err(Self::refused)?;self.phase=7;return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())))}let step=self.rows.advance(post,self.plan.origins().expect("admitted sparse origins"),Self::child(g),capability).map_err(Self::refused)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=8;Ok(RetainedCloneStep::Complete(Self::receipt(step.progress())))}else{Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())))}}
 pub fn origins(&self)->Option<&[Option<super::keyed_plan::Origin>]>{self.plan.origins()}
 pub fn modification_targets(&self)->Option<&[usize]>{self.plan.modification_targets()}
 pub fn staged(&self)->Option<&D>{self.staged.as_ref()}
 pub fn staged_mut(&mut self,_capability:ApplyCapability)->Result<&mut D,ValueError>{if self.phase<5||self.phase>8||self.closing{return Err(Self::fault("sparse patch access requires membership commit"))}Ok(self.staged.as_mut().expect("original authored sparse owner"))}
 pub fn begin_close(&mut self){self.closing=true;self.rows.begin_close();self.plan.begin_close()}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.field.terminal_is_empty(){return if self.field.close_started(){self.field.close_demand(body)}else{Ok(self.field.begin_close_demand())}}if !self.rows.terminal_is_empty(){return self.rows.close_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<D>(false,&None,body)}if self.staged.is_some(){return self.retirement.next_owner_demands::<D>(true,&None,body)}if self.authored.is_some(){return self.retirement.next_owner_demands::<Vec<D::Insertion>>(true,&None,body)}if !self.plan.terminal_is_empty(){return Ok(self.plan.close_demand())}Ok(Default::default())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
    if !self.field.terminal_is_empty(){return if self.field.close_started(){self.field.close_work_demand(body)}else{Ok(self.field.begin_close_work_demand())}}
    if !self.rows.terminal_is_empty(){return self.rows.close_work_demand(body)}
    if !self.retirement.is_empty(){return self.retirement.next_owner_work::<D>(false,&None,body)}
    Ok(u64::from(self.staged.is_some()||self.authored.is_some()||!self.plan.terminal_is_empty()))
 }
 pub fn requires_original_birth(&self)->bool{
    if !self.flight.idle(){return false}
    if !self.field.terminal_is_empty(){return self.field.close_started()&&self.field.requires_original_birth()}
    if !self.rows.terminal_is_empty(){return self.rows.requires_original_birth()}
    self.retirement.requires_original_birth()
 }
 pub fn requires_original_control(&self)->bool{
    if let Some(loaned)=self.flight.loaned(){return loaned}
    if !self.field.terminal_is_empty(){return self.field.close_started()&&self.field.requires_original_control()}
    if !self.rows.terminal_is_empty(){return self.rows.requires_original_control()}
    self.retirement.requires_original_control()
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    if !self.field.terminal_is_empty(){return self.field.close_original_birth_step(original,grant)}
    if !self.rows.terminal_is_empty(){return self.rows.close_original_birth_step(original,grant)}
    self.retirement.step_original_birth(original,grant)
 }
 pub fn close_step(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
    if !self.closing{return Err(Self::fault("sparse collection original close not begun").into())}
    let grant=effect.grant();
    if !recover&&(!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,self.close_demand(body)?)||self.close_work_demand(body)?>effect.remaining_work_units()?){
        return Ok(RetainedCloneStep::Progress(Default::default()))
    }
    if self.field.terminal_is_empty()&&!self.rows.terminal_is_empty(){
        return self.rows.close_step(original,body,effect,recover,work).map(|step|RetainedCloneStep::Progress(step.progress()))
    }
    let base=effect.performed_work();
    let mut spent=0;
    let result=(|| -> Result<RetainedCloneStep,RetirementPollExit>{
        if !recover{
            if !self.field.terminal_is_empty()&&!self.field.close_started(){
                return Ok(RetainedCloneStep::Progress(self.field.begin_close_paid(effect).result?))
            }
            if self.field.terminal_is_empty()&&self.retirement.is_empty(){
                if let Some(step)=self.retirement.begin_granted(&mut self.staged,effect).result?{return Ok(step)}
                if let Some(step)=self.retirement.begin_granted(&mut self.authored,effect).result?{return Ok(step)}
                if !self.plan.terminal_is_empty(){
                    if !effect.performed(1)?{return Err(Self::fault("sparse collection plan close exceeded original work").into())}
                    return Ok(RetainedCloneStep::Progress(self.plan.close_step(grant)?))
                }
                return Ok(RetainedCloneStep::Complete(Default::default()))
            }
        }
        let missing=||Self::fault("sparse collection original control is missing");
        let route=if recover{self.flight.route().ok_or_else(missing)?}else if !self.field.terminal_is_empty(){2}else{3};
        let loaned=self.requires_original_control();
        let loan=if loaned{Some(original.as_deref_mut().ok_or_else(missing)?)}else{None};
        let step=retained_routed!(self.flight,effect,recover,route,body,0,loan,spent,
            |r| match (route,loaned){
                (2,false)=>self.field.close_into(body,effect,r),
                (2,true)=>self.field.close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
                (3,false)=>self.retirement.step_into(body,effect,r),
                (3,true)=>self.retirement.step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
                _=>Err(missing()),
            },
            match (route,loaned){
                (2,false)=>self.field.recover_close_into(body,effect,r),
                (2,true)=>self.field.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
                (3,false)=>self.retirement.recover_step_into(body,effect,r),
                (3,true)=>self.retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
                _=>Err(missing()),
            })?;
        Ok(RetainedCloneStep::Progress(step.progress()))
    })();
    *work+=spent.max(effect.performed_work().saturating_sub(base));
    result
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.field.terminal_is_empty()&&self.rows.terminal_is_empty()&&self.staged.is_none()&&self.authored.is_none()&&self.retirement.is_empty()&&self.plan.terminal_is_empty()&&self.flight.idle()}
}
impl<R:RetireOwned+BorrowedKey,D:OwnedDelta<R>> Drop for RetainedCollection<R,D>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original sparse collection ownership needs funded close")}}
