//! ⏸️ An actual caller delegates its remaining wallet once and retains the owned issuing recipient through await.
use semio_framework_job_context::{StepContext,StepBudget,OriginalCallerSourceParent,OriginalCallerSourceLoan,OriginalRetainedLoanIdentity,OriginalTurnFailure,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,InteractiveJobCloseStep,ClockStride,OriginalSourceBinding,OriginalSourceTurn,receive_original_retained_work};
use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;

struct OwnedTurnWallet{fuel:u64,deadline:u64,grant:RetainedCloneGrant,clock:fn()->Option<u64>,clock_binding:Option<u64>,epoch:u64,clock_observation:ClockStride,payload_page_granted:bool,stage:&'static str}

/// 🧳️ See `README.md`.
pub struct OriginalOwnedCallerTurn{dispatch:OriginalSourceTurn,binding:OriginalSourceBinding,parent_identity:usize,active:bool,ready:bool,source:ManuallyDrop<Option<OriginalCallerSourceParent>>,loan:ManuallyDrop<Option<OriginalCallerSourceLoan>>,wallet:Option<OwnedTurnWallet>,sequence:u64,recipient:RetainedCloneProgress,issued:bool,initial_fuel:u64,work_units_spent:u128,receiving_valid:bool,initial_grant:RetainedCloneGrant,deadline:u64,epoch:u64,clock:fn()->Option<u64>}

/// 🎟️ Parks the actual caller's remaining wallet once so the owned issuing recipient survives an await.
pub trait OriginalParkExt{
    /// 🎟️ See `README.md`.
    fn park_owned_turn(&self,output:&mut Option<OriginalOwnedCallerTurn>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
}
impl OriginalParkExt for OriginalCallerSourceParent{
    fn park_owned_turn(&self,output:&mut Option<OriginalOwnedCallerTurn>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate_borrowed_context(cx)?;let epoch=cx.original_source_epoch().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"owned async turn requires its genuinely issued source epoch"))?;let identity=self.source_identity()?;
        if let Some(owner)=output.as_ref(){if !owner.binding.matches(cx)||owner.parent_identity!=identity{return Err(ValueError::invariant("owned async admission requires its unchanged original caller").into())}if owner.ready{return Ok(true)}}
        let grant=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        if output.is_none(){self.admit_owned_turn(output,cx,epoch,OriginalOwnedCallerTurn::from_admitted_source)?;return Ok(false)}
        let owner=output.as_mut().ok_or_else(||ValueError::invariant("output absent"))?;
        if cx.fuel_remaining()<2||cx.retained_grant().maximum_items<2{return Ok(false)}
        OriginalOwnedCallerTurn::post_control(cx)?;
        let grant=cx.retained_grant();let fuel=cx.fuel_remaining();let deadline=cx.deadline_us();let clock=cx.clock_source();let clock_binding=cx.original_clock_binding();let posted=cx.retained_progress();let fields=cx.park_fields();let sequence=std::mem::take(fields.preview_sequence);let clock_observation=fields.clock.get();let payload_page_granted=*fields.payload_page_granted;let stage=*fields.stage;
        owner.wallet=Some(OwnedTurnWallet{fuel,deadline,grant,clock,clock_binding,epoch,clock_observation,payload_page_granted,stage});owner.sequence=sequence;owner.initial_fuel=fuel;owner.initial_grant=grant;owner.ready=true;
        let fields=cx.park_fields();*fields.fuel_remaining=0;*fields.retained=RetainedCloneGrant{maximum_items:posted.copied_items,maximum_copy_bytes:posted.copied_bytes,maximum_capacity_bytes:posted.retained_capacity_bytes,maximum_release_bytes:posted.released_bytes,maximum_depth:0};Ok(true)
    }
}

impl OriginalOwnedCallerTurn{
    pub(crate) fn from_admitted_source(source:OriginalCallerSourceParent,cx:&StepContext<'_>,parent_identity:usize,epoch:u64,dispatch:OriginalSourceTurn)->Self{Self{dispatch,binding:OriginalSourceBinding::borrow(cx),parent_identity,active:true,ready:false,source:ManuallyDrop::new(Some(source)),loan:ManuallyDrop::new(None),wallet:None,sequence:0,recipient:Default::default(),issued:false,initial_fuel:0,work_units_spent:0,receiving_valid:true,initial_grant:RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0},deadline:cx.deadline_us(),epoch,clock:cx.clock_source()}}
    fn post_control(cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{receive_original_retained_work(cx,semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1})?;Ok(())}
    pub fn admission_is_ready(&self)->bool{self.ready}
    pub fn admission_is_empty(&self)->bool{!self.active&&self.source.is_none()&&self.loan.is_none()}

    /// ♻️ See `README.md`.
    pub fn close_admission_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{parent.validate_borrowed_context(cx)?;if self.parent_identity!=parent.source_identity()?||!self.binding.matches(cx){return Err(ValueError::invariant("partial owned async return requires its actual source").into())}self.close_source(parent,cx)}
    /// 📏️ See `README.md`.
    pub fn original_grant(&self)->RetainedCloneGrant{self.initial_grant}
    pub fn original_fuel(&self)->u64{self.initial_fuel}
    pub fn original_deadline_us(&self)->u64{self.deadline}
    pub fn original_epoch(&self)->u64{self.epoch}
    pub fn original_clock(&self)->fn()->Option<u64>{self.clock}
    /// 🤝️ See `README.md`.
    pub async fn execute<R>(mut self,run:impl for<'turn> AsyncFnOnce(&mut StepContext<'turn>)->Result<R,OriginalTurnFailure>)->OriginalOwnedCallerReturn<R>{
        let mut receiving_refusal=None;
        let original=if self.issued||!self.ready{Err(ValueError::invariant("owned original turn cannot issue its wallet twice").into())}else{
            self.issued=true;let wallet=self.wallet.take().expect("one sealed original wallet remains before its only issue");let budget=StepBudget{fuel:wallet.fuel,deadline_us:wallet.deadline,retained:wallet.grant};
            let source=self.source.as_ref().expect("paid original source remains owned through its context");
            match source.borrow_context_for_turn(&self.dispatch,budget,wallet.clock,&mut self.sequence,&mut self.recipient){
                Err(error)=>{self.wallet=Some(wallet);Err(error.into())}
                Ok(mut cx)=>{
                    {let fields=cx.park_fields();fields.clock.set(wallet.clock_observation);*fields.payload_page_granted=wallet.payload_page_granted;*fields.stage=wallet.stage;}let bound=cx.bind_original_source_epoch(wallet.epoch).and_then(|()|wallet.clock_binding.map_or(Ok(()),|binding|cx.bind_original_clock_binding(binding)));let identity=OriginalRetainedLoanIdentity::capture(&cx);let cancel=cx.original_cancel_token()as*const _ as usize;
                    let result=match bound{Ok(())=>run(&mut cx).await,Err(error)=>Err(error.into())};
                    let matched=identity.matches(&cx)&&cx.original_source_epoch()==Some(wallet.epoch)&&cancel==cx.original_cancel_token()as*const _ as usize&&cx.fuel_remaining()<=wallet.fuel;
                    self.work_units_spent=cx.original_work_units_spent();self.receiving_valid=matched;
                    if !matched{receiving_refusal=Some(ValueError::invariant("owned async producer replaced its actual issuing context"));}
                    let fuel=if matched{cx.fuel_remaining()}else{0};let fields=cx.park_fields();self.wallet=Some(OwnedTurnWallet{fuel,clock_observation:fields.clock.get(),payload_page_granted:*fields.payload_page_granted,stage:*fields.stage,..wallet});result
                }
            }
        };
        OriginalOwnedCallerReturn{turn:self,original:ManuallyDrop::new(Some(original)),receiving_refusal:ManuallyDrop::new(receiving_refusal)}
    }
    fn validate_polling_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx){return Err(ValueError::invariant("owned async return requires its genuine original caller").into())}if let Some(source)=self.source.as_ref(){source.validate_borrowed_context(cx).map_err(Into::into)}else if let Some(loan)=self.loan.as_ref(){loan.validate_context(cx)}else{Err(ValueError::invariant("owned original reply source returned before its output transfer").into())}}
    fn terminal_is_empty(&self)->bool{!self.active&&self.source.is_none()&&self.loan.is_none()}
    fn close_source(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        if self.source.is_some(){if cx.retirement_should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}let source=self.source.take().ok_or_else(||ValueError::invariant("source absent"))?;match source.into_source_loan(){Ok(loan)=>*self.loan=Some(loan),Err(source)=>{*self.source=Some(source);return Err(ValueError::invariant("owned async source cannot return while another source frontier remains").into())}}Self::post_control(cx)?;return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
        if let Some(loan)=self.loan.as_mut(){if !loan.terminal_is_empty(){let step=loan.close_step(parent,cx)?;return Ok(match step{InteractiveJobCloseStep::Complete{progress}|InteractiveJobCloseStep::Pending{progress}=>InteractiveJobCloseStep::Pending{progress},InteractiveJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked})}if cx.retirement_should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}self.loan.take();Self::post_control(cx)?;return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
        if !self.active{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}if cx.retirement_should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}self.active=false;self.ready=false;Self::post_control(cx)?;Ok(InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})
    }
}
impl Drop for OriginalOwnedCallerTurn{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned original turn reached Drop before its paid source loan return")}}

/// 📬️ See `README.md`.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
pub struct OriginalOwnedCallerOutcome<R>{pub original:Result<R,OriginalTurnFailure>,pub receiving_refusal:Option<ValueError>}
/// 🪢️ See `README.md`.
pub struct OriginalOwnedCallerReturn<R>{turn:OriginalOwnedCallerTurn,original:ManuallyDrop<Option<Result<R,OriginalTurnFailure>>>,receiving_refusal:ManuallyDrop<Option<ValueError>>}
impl<R> OriginalOwnedCallerReturn<R>{
    pub fn validate_original_receiver(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.turn.validate_polling_context(cx)}
    pub fn original(&self)->Option<&Result<R,OriginalTurnFailure>>{self.original.as_ref()}
    pub fn receiving_refusal(&self)->Option<&ValueError>{self.receiving_refusal.as_ref()}
    pub fn receipt(&self)->RetainedCloneProgress{self.turn.recipient}
    pub fn work_units_spent(&self)->u128{self.turn.work_units_spent}
    pub fn fuel_remaining(&self)->Option<u64>{self.turn.receiving_valid.then(||self.turn.wallet.as_ref().map_or(0,|wallet|wallet.fuel))}
    pub fn original_epoch(&self)->u64{self.turn.epoch}
    pub fn original_deadline_us(&self)->u64{self.turn.deadline}
    pub fn original_grant(&self)->RetainedCloneGrant{self.turn.initial_grant}
    /// 📤️ See `README.md`.
    pub fn take_original_on_caller(&mut self,cx:&mut StepContext<'_>)->Result<Option<OriginalOwnedCallerOutcome<R>>,OriginalTurnFailure>{
        if self.original.is_none(){return Err(ValueError::invariant("owned async reply was already transferred").into())}
        self.turn.validate_polling_context(cx)?;let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
        let now=cx.latest_us();let fields=cx.park_fields();*fields.preview_sequence=(*fields.preview_sequence).max(self.turn.sequence);if let Some(wallet)=self.turn.wallet.as_ref(){if now.is_none_or(|now|wallet.clock_observation.latest_us().is_some_and(|returned|returned>now)){fields.clock.set(wallet.clock_observation);}}
        Ok(Some(OriginalOwnedCallerOutcome{original:self.original.take().unwrap(),receiving_refusal:self.receiving_refusal.take()}))
    }
    /// ♻️ See `README.md`.
    pub fn close_source(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if self.original.is_some(){return Err(ValueError::invariant("owned original source stays retained until its output transfer").into())}self.turn.close_source(parent,cx)}
    pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.receiving_refusal.is_none()&&self.turn.terminal_is_empty()}
}
impl<R> Drop for OriginalOwnedCallerReturn<R>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned async reply reached Drop before original transfer and paid source return")}}
