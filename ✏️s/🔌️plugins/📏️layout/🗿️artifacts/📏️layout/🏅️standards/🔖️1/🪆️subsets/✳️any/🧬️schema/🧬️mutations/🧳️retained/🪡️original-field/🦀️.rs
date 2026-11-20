//! 🪡️ Original command fields retain their exact callback route across transfer and cancellation.
use semio_os_kernel::{RetainedFieldPreparation,RetainedFlight as Flight,retained_native_child,retained_routed};
use semio_framework_value::{
    OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll,
    RetirementDemand,RetirementPollExit,ValueError,
    retained_clone::{RetainedClone,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep},
};

pub struct OriginalField<T:RetainedClone>{
    owner:RetainedFieldPreparation<T>,
    flight:Flight,
    closing:bool,
}

impl<T:RetainedClone> Default for OriginalField<T>{
    fn default()->Self{Self{owner:Default::default(),flight:Default::default(),closing:false}}
}

impl<T:RetainedClone> OriginalField<T>{
    pub fn embedded_birth_work()->Result<u64,ValueError>{
        RetainedFieldPreparation::<T>::embedded_birth_work()?.checked_add(2).ok_or_else(invalid)
    }
    pub fn demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{
        self.owner.demand(source,body,maximum)
    }
    pub fn work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<u64,ValueError>{
        self.owner.advance_work_demand(source,body,maximum)
    }
    pub fn advance(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
        if self.closing{return Err(invalid().into())}
        let mut work=0;
        let step=retained_routed!(self.flight,effect,recover,1,body,maximum,None,work,
            |r|self.owner.advance_into(source,body,maximum,effect,r),
            self.owner.recover_advance_into(source,body,maximum,effect,r))?;
        let _=work;
        Ok(step)
    }
    pub fn value(&self)->Option<&T>{self.owner.value()}
    pub fn exchange_demand(&self)->Result<RetirementDemand,ValueError>{self.owner.exchange_demand()}
    pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<(Option<T>,RetainedCloneProgress),ValueError>{
        let mut output=None;
        let mut work=0;
        let step=retained_native_child(effect,&mut work,|effect,reply|self.owner.take_into(&mut output,effect,reply))??;
        Ok((output,step.progress()))
    }
    pub fn begin_close(&mut self){self.closing=true}
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.owner.close_started(){self.owner.close_demand(body)}else{Ok(self.owner.begin_close_demand())}
    }
    pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if self.owner.close_started(){self.owner.close_work_demand(body)}else{Ok(self.owner.begin_close_work_demand())}
    }
    pub fn requires_original_birth(&self)->bool{self.flight.idle()&&self.owner.close_started()&&self.owner.requires_original_birth()}
    pub fn requires_original_control(&self)->bool{
        self.flight.loaned().unwrap_or_else(||self.owner.close_started()&&self.owner.requires_original_control())
    }
    pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
        self.owner.close_original_birth_step(original,grant)
    }
    pub fn close_step(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
        if !self.closing{return Err(invalid().into())}
        if !recover&&!self.owner.close_started(){
            return Ok(RetainedCloneStep::Progress(self.owner.begin_close_paid(effect).result?))
        }
        let loaned=self.requires_original_control();
        let loan=if loaned{Some(original.as_deref_mut().ok_or_else(invalid)?)}else{None};
        let mut work=0;
        let step=retained_routed!(self.flight,effect,recover,2,body,0,loan,work,
            |r|if loaned{self.owner.close_original_into(original.as_deref_mut().ok_or_else(invalid)?,body,effect,r)}else{self.owner.close_into(body,effect,r)},
            if loaned{self.owner.recover_close_original_into(original.as_deref_mut().ok_or_else(invalid)?,body,effect,r)}else{self.owner.recover_close_into(body,effect,r)})?;
        let _=work;
        Ok(RetainedCloneStep::Progress(step.progress()))
    }
    pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()&&self.flight.idle()}
}

fn invalid()->ValueError{ValueError::invariant("Layout command changed its original field custody")}

pub fn settle_poll(poll:OriginalRetirementPoll)->Result<RetainedCloneStep,RetirementPollExit>{
    match poll{
        RetainedWorkPoll::Pending{work}=>Err(RetirementPollExit::Pending(work)),
        RetainedWorkPoll::Complete(turn)=>turn.result.map_err(Into::into),
    }
}
pub fn close_binding(binding:&mut Option<RetainedCloneBinding>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
    let poll=if RetainedCloneBinding::requires_original_control(binding){
        RetainedCloneBinding::close_original_step(binding,original.ok_or_else(invalid)?,body,effect)
    }else{RetainedCloneBinding::close_one(binding,body,effect)};
    settle_poll(poll)
}
