//! 🏭️ Genuine Layout semantic preparation publishes typed sparse owners and exact inverse sources.
use crate::LayoutMutation;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_os_kernel as protocol;
use protocol::{RetainedMutationPreparation,RetainedMutationDecision,RetainedMutationCloseContext,RetainedFieldPreparation,RetainedDiagnosticPreparation,RetainedDiagnosticSpec,MutationMessage,MutationMessageLedgerRetirement};
use protocol::{RetainedDone as Done,RetainedFlight as Flight,RetainedReply as Reply,retained_conclude as conclude};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_factory::vector::FactoryVectorRetirement;
use super::frame::decision::FrameDecision;
use semio_s_artifact_layout_layout_mutations_decisions::page_deletion::{DeletePagePlan,DeletePageRefusal};
use super::parent_content::{ParentContentPlan, ParentContentOutput, ParentContentInverse};
use super::{simple::SimpleDecision,output::SimpleDiffOutput,inverse::SimpleInverse,story::{plan::{StorySplicePlan,StorySpliceFailure},output::StoryOutput,inverse::StoryInverse}};
use std::mem::{size_of,ManuallyDrop};
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout original semantic preparation stage is invalid")}
fn delete_page(m:RetainedCloneRef<'_,LayoutMutation>)->RetainedCloneRef<'_,crate::delete_page::DeletePage>{
 m.project(0,|m|match m{LayoutMutation::DeletePage(payload)=>payload,_=>unreachable!("selected DeletePage")})
}
fn error_level(code:&str)->semio_framework_diagnostic::Severity{match code{"mutation.target-missing"|"mutation.target-mismatch"|"mutation.target-referenced"=>semio_framework_diagnostic::Severity::Error,_=>semio_framework_diagnostic::Severity::Fatal}}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:256,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=256;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=256;p.copied_items=p.copied_items.max(1);p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
fn polled_close(
 flight:&mut Flight,route:u8,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,
 effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128,
 action:impl FnOnce(Option<&mut OriginalRetirementCloseLoan<'_>>,&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll,
)->Result<RetainedCloneStep,RetirementPollExit>{
 if recover{if flight.resume(effect,body,0,original.as_deref_mut())?!=route{return Err(fault().into())}}else{flight.start(effect,route,body,0,original.as_deref_mut())?}
 match action(original,effect){
  RetainedWorkPoll::Complete(turn)=>{*work+=turn.work;flight.settle(&mut Some(()));turn.result.map_err(Into::into)},
  RetainedWorkPoll::Pending{work:performed}=>{*work+=performed;Err(RetirementPollExit::Pending(0))},
 }
}
/// 🧳 Partial command fields, typed diff, inverse vectors and diagnostics retain their original owners.
pub struct RetainedLayoutMutation{flight:Flight,batch:super::frame::batch_prepare::BatchPreparation,batch_selected:bool,document_decision:super::document::decision::DocumentDecision,override_decision:super::overrides::decision::OverrideDecision,story_decision:StorySplicePlan,story_output:StoryOutput,story_inverse:StoryInverse,parent_content:ParentContentPlan,parent_output:ParentContentOutput,parent_inverse:ParentContentInverse<LayoutMutation>,data_decision:super::data::decision::DataFieldsDecision,frame_decision:FrameDecision,page_deletion:DeletePagePlan,decision:SimpleDecision,payload:RetainedFieldPreparation<LayoutMutation>,output:SimpleDiffOutput,inverse:SimpleInverse,payload_rows:ManuallyDrop<Option<Vec<LayoutMutation>>>,inverse_rows:ManuallyDrop<Option<Vec<LayoutMutation>>>,mutation_retirement:Option<FactoryVectorRetirement<LayoutMutation>>,diff:ManuallyDrop<Option<LayoutDiff>>,retirement:RetainedCloneClose,diagnostic:RetainedDiagnosticPreparation<0>,messages:ManuallyDrop<Option<Vec<MutationMessage>>>,ledger:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,phase:u8,inverse_phase:u8,error:Option<&'static str>,unchanged:bool,decision_ready:bool,inverse_ready:bool,closing:bool}
impl protocol::RetainedMutation<LayoutSnapshot> for LayoutMutation{type Preparation=RetainedLayoutMutation;}
impl RetainedMutationPreparation<LayoutSnapshot,LayoutMutation> for RetainedLayoutMutation{
 fn new()->Self{Self{flight:Default::default(),batch:Default::default(),batch_selected:false,document_decision:Default::default(),override_decision:Default::default(),story_decision:Default::default(),story_output:Default::default(),story_inverse:Default::default(),parent_content:Default::default(),parent_output:Default::default(),parent_inverse:Default::default(),data_decision:Default::default(),frame_decision:Default::default(),page_deletion:Default::default(),decision:Default::default(),payload:Default::default(),output:Default::default(),inverse:Default::default(),payload_rows:ManuallyDrop::new(None),inverse_rows:ManuallyDrop::new(None),mutation_retirement:None,diff:ManuallyDrop::new(None),retirement:Default::default(),diagnostic:Default::default(),messages:ManuallyDrop::new(None),ledger:ManuallyDrop::new(None),phase:0,inverse_phase:0,error:None,unchanged:false,decision_ready:false,inverse_ready:false,closing:false}}
 fn birth_work()->Result<u64,ValueError>{
  let child=[
   RetainedFieldPreparation::<LayoutMutation>::embedded_birth_work()?,
   super::frame::batch_prepare::BatchPreparation::embedded_birth_work()?,
   SimpleDiffOutput::embedded_birth_work()?,
   SimpleInverse::embedded_birth_work()?,
   StoryOutput::birth_work()?,
   StoryInverse::birth_work()?,
   ParentContentOutput::birth_work()?,
   ParentContentInverse::<LayoutMutation>::birth_work()?,
  ];
  child.into_iter().try_fold(1u64,|work,next|work.checked_add(next).ok_or_else(fault))
 }
 fn decision_demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{if super::frame::batch_prepare::selected(m.get()){return self.batch.decision_demand(m,b,body.saturating_sub(256),max.saturating_sub(1)).map(nested)}if self.closing||self.decision_ready{return Err(fault())}Ok(match self.phase{
  0=>if matches!(m.get(),LayoutMutation::DeletePage(_)){nested(self.page_deletion.demand(delete_page(m),b)?)}else if matches!(m.get(),LayoutMutation::ReplaceDocument(_)){nested(self.document_decision.demand(m,b,body.saturating_sub(256))?)}else if matches!(m.get(),LayoutMutation::EditStory(_)){nested(self.story_decision.demand(m,b)?)}else if matches!(m.get(),LayoutMutation::SetParentPageContent(_)){nested(self.parent_content.demand(m,b)?)}else if matches!(m.get(),LayoutMutation::SetPageOverrides(_)){nested(self.override_decision.demand(m,b,body.saturating_sub(256))?)}else if matches!(m.get(),LayoutMutation::ChangeDataFields(_)){nested(self.data_decision.demand(m,b,body.saturating_sub(256))?)}else if matches!(m.get(),LayoutMutation::CreateFrame(_)|LayoutMutation::DeleteFrame(_)){nested(self.frame_decision.demand(m,b,body.saturating_sub(256))?)}else{nested(self.decision.demand())},1=>if self.payload.value().is_some(){fixed()}else{nested(self.payload.demand(m,body.saturating_sub(256),max.saturating_sub(1))?)},2=>RetirementDemand{capacity_bytes:size_of::<LayoutMutation>(),..nested(RetirementDemand{copy_bytes:size_of::<Vec<LayoutMutation>>()*2,depth:1,..Default::default()})},3=>{let mut d=nested(self.payload.exchange_demand()?);d.copy_bytes+=size_of::<LayoutMutation>()*2;d},4=>nested(SimpleDiffOutput::setup_demand(super::simple::SimpleKind::of(m.get()).ok_or_else(fault)?,m.get())),5=>if matches!(m.get(),LayoutMutation::EditStory(_)){nested(self.story_output.demand(&self.story_decision,m,b,body.saturating_sub(256),max.saturating_sub(1))?)}else if matches!(m.get(),LayoutMutation::SetParentPageContent(_)){nested(self.parent_output.demand(&self.parent_content,m,b,body.saturating_sub(256),max.saturating_sub(1))?)}else{nested(self.output.build_demand(m,b,body.saturating_sub(256),max.saturating_sub(1))?)},6=>{let mut d=nested(if matches!(m.get(),LayoutMutation::EditStory(_)){self.story_output.take_demand()}else if matches!(m.get(),LayoutMutation::SetParentPageContent(_)){self.parent_output.take_demand()}else{self.output.take_demand()});d.copy_bytes+=size_of::<LayoutDiff>()*2;d},7=>fixed(),8=>RetirementDemand{copy_bytes:size_of::<LayoutDiff>()*2+256,depth:1,..Default::default()},100=>nested(RetainedDiagnosticPreparation::<0>::start_demand()),101=>nested(self.diagnostic.demand([],body.saturating_sub(256),max.saturating_sub(1))?),102=>RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>()*3+256,depth:2,..Default::default()},_=>return Err(fault())})}
 fn decision_work_demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<u64,ValueError>{
  self.decision_demand(m,b,body,max)?;
  let inner=body.saturating_sub(256);let ceiling=max.saturating_sub(1);
  let work=if super::frame::batch_prepare::selected(m.get()){self.batch.decision_work_demand(m,b,inner,ceiling)?}else{match self.phase{
   0=>match m.get(){
    LayoutMutation::DeletePage(_)=>self.page_deletion.work_demand(delete_page(m),b)?,
    LayoutMutation::ReplaceDocument(_)=>self.document_decision.work_demand(m,b,inner)?,
    LayoutMutation::SetPageOverrides(_)=>self.override_decision.work_demand(m,b)?,
    LayoutMutation::EditStory(_)=>self.story_decision.work_demand(m,b)?,
     LayoutMutation::SetParentPageContent(_)=>self.parent_content.work_demand(m,b)?,
    LayoutMutation::CreateFrame(_)|LayoutMutation::DeleteFrame(_)=>self.frame_decision.work_demand(m,b)?,
    _=>0,
   },
   1 if self.payload.value().is_none()=>self.payload.advance_work_demand(m,inner,ceiling)?,
   3|4=>1,
   5 if matches!(m.get(),LayoutMutation::EditStory(_))=>self.story_output.work_demand(&self.story_decision,m,b,inner,ceiling)?,
   5 if matches!(m.get(),LayoutMutation::SetParentPageContent(_))=>self.parent_output.work_demand(&self.parent_content,m,b,inner,ceiling)?,
   5=>self.output.build_work_demand(m,b,inner,ceiling)?,
   101=>self.diagnostic.advance_work_demand([],inner,ceiling)?,
   102=>self.diagnostic.take_work_demand()?,
   _=>0,
  }};
  let work=work.checked_add(1).ok_or_else(fault)?;
  Ok(if work<=max{work}else{0})
 }
 fn decide_reply<'r>(&mut self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let start=effect.performed_work();let g=effect.grant();let mut work=0;
  let inner=body.saturating_sub(256);let ceiling=max.saturating_sub(1);
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   let demand=if recover{Default::default()}else{
    let demand=self.decision_demand(m,b,body,max)?;
    if !protocol::retained_mutation_demand_fits(demand,g)||self.decision_work_demand(m,b,body,max)?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
    work+=1;
    if !effect.performed(1)?{return Err(fault().into())}
    demand
   };
   if super::frame::batch_prepare::selected(m.get()){
    self.batch_selected=true;
    let before=effect.performed_work();
    let step=self.batch.decide(m,b,inner,ceiling,&mut effect.nested(child(g)),recover);
    work+=effect.performed_work().saturating_sub(before);
    return match step?{RetainedCloneStep::Progress(p)=>Ok(RetainedCloneStep::Progress(receipt(p))),RetainedCloneStep::Complete(p)=>Ok(RetainedCloneStep::Complete(receipt(p)))}
   }
   let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
   match self.phase{
    0=>{
     let before=effect.performed_work();
     let step=match m.get(){
      LayoutMutation::DeletePage(_)=>self.page_deletion.advance(delete_page(m),b,&mut effect.nested(child(g))).map(|step|{
       let result=self.page_deletion.result();
       if let Some(Ok(at))=result{self.decision.at=[at.page,at.spread];}
       super::simple::SimpleAdvance{progress:step.progress(),done:result.is_some(),unchanged:false,error:result.and_then(Result::err).map(DeletePageRefusal::code)}
      }),
      LayoutMutation::ReplaceDocument(_)=>self.document_decision.advance(m,b,inner,&mut effect.nested(child(g))),
      LayoutMutation::SetParentPageContent(_)=>self.parent_content.advance(m,b,&mut effect.nested(child(g))).map(|step|{
        let done=self.parent_content.ready();
        let error=(done&&self.parent_content.parent().is_none()).then_some("mutation.target-missing");
        super::simple::SimpleAdvance{progress:step.progress(),done,unchanged:self.parent_content.unchanged(),error}
      }),
      LayoutMutation::EditStory(_)=>self.story_decision.advance(m,b,&mut effect.nested(child(g))).map(|step|{
       self.decision.at=[self.story_decision.story,0];
       let error=self.story_decision.failure.map(|failure|match failure{StorySpliceFailure::Missing=>"mutation.target-missing",StorySpliceFailure::Range|StorySpliceFailure::Overflow=>"mutation.invariant"});
       super::simple::SimpleAdvance{progress:step.progress(),done:self.story_decision.ready(),unchanged:self.story_decision.unchanged,error}
      }),
      LayoutMutation::SetPageOverrides(_)=>{let step=self.override_decision.advance(m,b,&mut effect.nested(child(g)));self.decision.at=self.override_decision.at;step},
      LayoutMutation::ChangeDataFields(_)=>self.data_decision.advance(m,b,&mut effect.nested(child(g))),
      LayoutMutation::CreateFrame(_)|LayoutMutation::DeleteFrame(_)=>{let step=self.frame_decision.advance(m,b,&mut effect.nested(child(g)));self.decision.at=self.frame_decision.at;step},
      _=>self.decision.advance(m.get(),b.get(),child(g)),
     };
     work+=effect.performed_work().saturating_sub(before);
     let step=step.map_err(refused)?;
     p=receipt(step.progress);
     if step.done{self.error=step.error;self.unchanged=step.unchanged;self.phase=if self.error.is_some(){100}else if self.unchanged{8}else if matches!(m.get(),LayoutMutation::EditStory(_)|LayoutMutation::SetParentPageContent(_)){5}else{1}}
    }
    1=>if self.payload.value().is_some(){self.phase=2}else{
     let step=protocol::retained_routed!(self.flight,effect,recover,1,inner,ceiling,None,work,|r|
      self.payload.advance_into(m,inner,ceiling,&mut effect.nested(child(g)),r),
      self.payload.recover_advance_into(m,inner,ceiling,&mut effect.nested(child(g)),r)
     )?;
     p=receipt(step.progress());
    },
    2=>{
     if effect.actual().is_some(){return Err(fault().into())}
     self.phase=254;*self.payload_rows=Some(Vec::new());
     let rows=self.payload_rows.as_mut().unwrap();
     *effect.actual()=Some(0);
     let reserved=rows.try_reserve_exact(1);
     p.retained_capacity_bytes=rows.capacity()*size_of::<LayoutMutation>();
     *effect.actual()=Some(p.retained_capacity_bytes);
     if reserved.is_err()||p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(p).into())}
     self.phase=3
    },
    3=>{
     let mut value=None;
     let step=protocol::retained_native_child(&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256-size_of::<LayoutMutation>()*2,maximum_depth:g.maximum_depth-1,..g}),&mut work,|effect,r|self.payload.take_into(&mut value,effect,r))??;
     p=receipt(step.progress());
     if let Some(value)=value{p.copied_bytes+=size_of::<LayoutMutation>()*2;self.payload_rows.as_mut().unwrap().push(value);self.phase=4}
    },
    4=>{
     let before=effect.performed_work();
     let step=self.output.setup(super::simple::SimpleKind::of(m.get()).ok_or_else(fault)?,m.get(),&mut effect.nested(child(g)));
     work+=effect.performed_work().saturating_sub(before);
     let step=step.map_err(refused)?;
     p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.phase=5}
    },
    5=>{
     let before=effect.performed_work();
     let step=if matches!(m.get(),LayoutMutation::EditStory(_)){self.story_output.advance(&self.story_decision,m,b,inner,ceiling,&mut effect.nested(child(g)),recover)}else if matches!(m.get(),LayoutMutation::SetParentPageContent(_)){self.parent_output.advance(&self.parent_content,m,b,inner,ceiling,&mut effect.nested(child(g)),recover)}else{self.output.build(self.payload_rows.as_mut().ok_or_else(fault)?,m,b,self.decision.at,self.page_deletion.result().and_then(Result::ok).and_then(|at|at.member),inner,ceiling,&mut effect.nested(child(g)),recover)};
     work+=effect.performed_work().saturating_sub(before);
     let step=step?;
     p=receipt(step.progress());
     if matches!(step,RetainedCloneStep::Complete(_)){self.phase=6}
    },
    6=>{let grant=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256-size_of::<LayoutDiff>()*2,maximum_depth:g.maximum_depth-1,..g};let(value,paid)=if matches!(m.get(),LayoutMutation::EditStory(_)){self.story_output.take(grant)}else if matches!(m.get(),LayoutMutation::SetParentPageContent(_)){self.parent_output.take(grant)}else{self.output.take(grant)}.map_err(refused)?;p=receipt(paid);p.copied_bytes+=size_of::<LayoutDiff>()*2;*self.diff=value;self.phase=7},
    7=>{self.decision_ready=true;return Ok(RetainedCloneStep::Complete(p))},
    8=>{*self.diff=Some(LayoutDiff::default());self.phase=100},
    100=>{
     let(level,code,text)=if matches!(m.get(),LayoutMutation::ReplaceDocument(_)){(self.error.map(error_level).unwrap_or(semio_framework_diagnostic::Severity::Warning),self.error.unwrap_or("mutation.no-op"),self.document_decision.diagnostic_message())}else if let Some(code)=self.error{(error_level(code),code,"The Layout command requires an admissible retained semantic source.")}else{(semio_framework_diagnostic::Severity::Warning,"mutation.no-op","The authored Layout fields already have these values.")};
     p=receipt(self.diagnostic.start(RetainedDiagnosticSpec{level,code,message:text},child(g)).map_err(refused)?);self.phase=101
    },
    101=>{
     let step=protocol::retained_routed!(self.flight,effect,recover,2,inner,ceiling,None,work,|r|
      self.diagnostic.advance_into([],inner,ceiling,&mut effect.nested(child(g)),r),
      self.diagnostic.recover_advance_into([],inner,ceiling,&mut effect.nested(child(g)),r)
     )?;
     p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.phase=102}
    },
    102=>{
     let turn=self.diagnostic.take(child(g));work+=turn.work;
     let(rows,paid)=turn.result?.ok_or_else(fault)?;
     p=receipt(paid);*self.messages=Some(rows);self.decision_ready=true;return Ok(RetainedCloneStep::Complete(p))
    },
    _=>return Err(fault().into()),
   }
   Ok(RetainedCloneStep::Progress(p))
  })();
  let work=work.max(effect.performed_work().saturating_sub(start));
  conclude(effect,start,result,work,|step|step.progress(),reply)
 }
 fn take_decision(&mut self)->RetainedMutationDecision<LayoutDiff>{if self.batch_selected{return self.batch.take_decision()}assert!(self.decision_ready&&!self.closing);let messages=self.messages.take().unwrap_or_default();if self.error.is_some(){RetainedMutationDecision::Refused(messages)}else{RetainedMutationDecision::Admitted(self.diff.take().expect("original typed Layout diff"),messages)}}
 fn inverse_demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,_:RetainedCloneRef<'_,LayoutDiff>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{if super::frame::batch_prepare::selected(m.get()){return self.batch.inverse_demand(m,b,body.saturating_sub(256),max.saturating_sub(1)).map(nested)}if self.closing||!self.decision_ready||self.error.is_some()||self.inverse_ready{return Err(fault())}if self.unchanged{return Ok(RetirementDemand{copy_bytes:size_of::<Vec<LayoutMutation>>()*2+256,depth:1,..Default::default()})}match self.inverse_phase{0 if matches!(m.get(),LayoutMutation::EditStory(_))=>self.story_inverse.demand(&self.story_decision,m,b,body.saturating_sub(256),max.saturating_sub(1)).map(nested),0 if matches!(m.get(),LayoutMutation::SetParentPageContent(_))=>self.parent_inverse.demand(&self.parent_content,m,b,body.saturating_sub(256),max.saturating_sub(1)).map(nested),0=>self.inverse.demand(super::simple::SimpleKind::of(m.get()).ok_or_else(fault)?,self.decision.at,self.page_deletion.result().and_then(Result::ok).and_then(|at|at.member),m,b,body.saturating_sub(256),max.saturating_sub(1)).map(nested),1=>{let mut d=nested(if matches!(m.get(),LayoutMutation::EditStory(_)){self.story_inverse.take_demand()}else if matches!(m.get(),LayoutMutation::SetParentPageContent(_)){self.parent_inverse.take_demand()}else{self.inverse.take_demand()});d.copy_bytes+=size_of::<Vec<LayoutMutation>>()*2;Ok(d)},_=>Err(fault())}}
 fn inverse_work_demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,d:RetainedCloneRef<'_,LayoutDiff>,body:usize,max:u64)->Result<u64,ValueError>{
  self.inverse_demand(m,b,d,body,max)?;
  let inner=body.saturating_sub(256);let ceiling=max.saturating_sub(1);
  let work=if super::frame::batch_prepare::selected(m.get()){self.batch.inverse_work_demand(m,b,inner,ceiling)?}else if !self.unchanged&&self.inverse_phase==0{
   if matches!(m.get(),LayoutMutation::EditStory(_)){self.story_inverse.work_demand(&self.story_decision,m,b,inner,ceiling)?}else if matches!(m.get(),LayoutMutation::SetParentPageContent(_)){self.parent_inverse.work_demand(&self.parent_content,m,b,inner,ceiling)?}else{self.inverse.work_demand(super::simple::SimpleKind::of(m.get()).ok_or_else(fault)?,self.decision.at,self.page_deletion.result().and_then(Result::ok).and_then(|at|at.member),m,b,inner,ceiling)?}
  }else{0};
  let work=work.checked_add(1).ok_or_else(fault)?;
  Ok(if work<=max{work}else{0})
 }
 fn inverse_reply<'r>(&mut self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,d:RetainedCloneRef<'_,LayoutDiff>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let start=effect.performed_work();let g=effect.grant();let mut work=0;
  let inner=body.saturating_sub(256);let ceiling=max.saturating_sub(1);
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   let demand=if recover{Default::default()}else{
    let demand=self.inverse_demand(m,b,d,body,max)?;
    if !protocol::retained_mutation_demand_fits(demand,g)||self.inverse_work_demand(m,b,d,body,max)?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
    work+=1;
    if !effect.performed(1)?{return Err(fault().into())}
    demand
   };
   if super::frame::batch_prepare::selected(m.get()){
    let before=effect.performed_work();
    let step=self.batch.inverse(m,b,inner,ceiling,&mut effect.nested(child(g)),recover);
    work+=effect.performed_work().saturating_sub(before);
    return match step?{RetainedCloneStep::Progress(p)=>Ok(RetainedCloneStep::Progress(receipt(p))),RetainedCloneStep::Complete(p)=>Ok(RetainedCloneStep::Complete(receipt(p)))}
   }
   if self.unchanged{*self.inverse_rows=Some(Vec::new());self.inverse_ready=true;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
   if self.inverse_phase==0{
    let before=effect.performed_work();
    let step=if matches!(m.get(),LayoutMutation::EditStory(_)){self.story_inverse.advance(&self.story_decision,m,b,inner,ceiling,&mut effect.nested(child(g)),recover)}else if matches!(m.get(),LayoutMutation::SetParentPageContent(_)){self.parent_inverse.advance(&self.parent_content,m,b,inner,ceiling,&mut effect.nested(child(g)),recover)}else{self.inverse.advance(super::simple::SimpleKind::of(m.get()).ok_or_else(fault)?,self.decision.at,self.page_deletion.result().and_then(Result::ok).and_then(|at|at.member),m,b,inner,ceiling,&mut effect.nested(child(g)),recover)};
    work+=effect.performed_work().saturating_sub(before);
    let step=step?;
    if matches!(step,RetainedCloneStep::Complete(_)){self.inverse_phase=1}
    return Ok(RetainedCloneStep::Progress(receipt(step.progress())))
   }
   let grant=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256-size_of::<Vec<LayoutMutation>>()*2,maximum_depth:g.maximum_depth-1,..g};
   let(rows,paid)=if matches!(m.get(),LayoutMutation::EditStory(_)){self.story_inverse.take(grant)}else if matches!(m.get(),LayoutMutation::SetParentPageContent(_)){self.parent_inverse.take(grant)}else{self.inverse.take(grant)}.map_err(refused)?;
   let mut p=receipt(paid);p.copied_bytes+=size_of::<Vec<LayoutMutation>>()*2;*self.inverse_rows=rows;self.inverse_ready=true;
   Ok(RetainedCloneStep::Complete(p))
  })();
  let work=work.max(effect.performed_work().saturating_sub(start));
  conclude(effect,start,result,work,|step|step.progress(),reply)
 }
 fn take_inverse(&mut self)->Vec<LayoutMutation>{if self.batch_selected{return self.batch.take_inverse()}assert!(self.inverse_ready&&!self.closing);self.inverse_rows.take().unwrap_or_default()}
 fn begin_close(&mut self){self.closing=true;self.batch.begin_close();self.document_decision.begin_close();self.override_decision.begin_close();self.story_decision.begin_close();self.page_deletion.begin_close();self.story_output.begin_close();self.story_inverse.begin_close();self.parent_content.begin_close();self.parent_output.begin_close();self.parent_inverse.begin_close();self.data_decision.begin_close();self.frame_decision.begin_close();self.decision.begin_close();self.output.begin_close();self.inverse.begin_close();self.diagnostic.begin_close()}
 fn close_started(&self)->bool{self.closing}
 fn begin_close_demand(&self)->RetirementDemand{if self.closing{Default::default()}else{fixed()}}
 fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
 fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{
  let before=effect.performed_work();let demand=self.begin_close_demand();
  let result=(||{
   if self.closing||effect.remaining_work_units()?==0||!protocol::retained_mutation_demand_fits(demand,effect.grant()){return Ok(Default::default())}
   if !effect.performed(1)?{return Err(fault())}
   self.begin_close();
   Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
  })();
  RetainedWorkOutcome{result,work:effect.performed_work().saturating_sub(before)}
 }
 fn close_demand(&self,context:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<RetirementDemand,ValueError>{if !self.batch.terminal_is_empty(){return self.batch.close_demand(context,body)}if !self.story_inverse.terminal_is_empty(){return self.story_inverse.close_demand(body)}if !self.story_output.terminal_is_empty(){return self.story_output.close_demand(body)}
   if !self.parent_inverse.terminal_is_empty(){return self.parent_inverse.close_demand(body)}
   if !self.parent_output.terminal_is_empty(){return self.parent_output.close_demand(body)}if !self.inverse.terminal_is_empty(){return self.inverse.close_demand(context,body)}if !self.payload.terminal_is_empty(){return if self.payload.close_started(){self.payload.close_demand(body)}else{Ok(self.payload.begin_close_demand())}}if !self.output.terminal_is_empty(){return self.output.close_demand(body)}if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_demand(body)}if !self.document_decision.terminal_is_empty(){return self.document_decision.close_demand(body)}if !self.override_decision.terminal_is_empty(){return self.override_decision.close_demand(body)}if !self.story_decision.terminal_is_empty(){return self.story_decision.close_demand(body)}
   if !self.parent_content.terminal_is_empty(){return self.parent_content.close_demand(body)}if !self.page_deletion.terminal_is_empty(){return self.page_deletion.close_demand(body)}if !self.data_decision.terminal_is_empty(){return self.data_decision.close_demand(body)}if !self.frame_decision.terminal_is_empty(){return self.frame_decision.close_demand(body)}if !self.decision.terminal_is_empty(){return Ok(self.decision.close_demand())}if !self.retirement.is_empty()||self.diff.is_some(){return self.retirement.next_owner_demands::<LayoutDiff>(self.diff.is_some(),&None,body)}if let Some(owner)=self.mutation_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<FactoryVectorRetirement<LayoutMutation>>>()*2+256,depth:1,..Default::default()})}else{owner.demands(context.mutations,body)}}if self.payload_rows.is_some()||self.inverse_rows.is_some(){return Ok(FactoryVectorRetirement::<LayoutMutation>::constructor_demand())}if self.messages.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>()+size_of::<MutationMessageLedgerRetirement>(),depth:1,..Default::default()})}if let Some(ledger)=self.ledger.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_demand(ledger,body)}Ok(Default::default())}
 fn close_work_demand(&self,c:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<u64,ValueError>{
  if !self.batch.terminal_is_empty(){return self.batch.close_work_demand(c,body)}
  if !self.story_inverse.terminal_is_empty(){return self.story_inverse.close_work_demand(body)}
  if !self.story_output.terminal_is_empty(){return self.story_output.close_work_demand(body)}
   if !self.parent_inverse.terminal_is_empty(){return self.parent_inverse.close_work_demand(body)}
   if !self.parent_output.terminal_is_empty(){return self.parent_output.close_work_demand(body)}
  if !self.inverse.terminal_is_empty(){return self.inverse.close_work_demand(c,body)}
  if !self.payload.terminal_is_empty(){return if self.payload.close_started(){self.payload.close_work_demand(body)}else{Ok(self.payload.begin_close_work_demand())}}
  if !self.output.terminal_is_empty(){return self.output.close_work_demand(body)}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_work_demand(body)}
  if !self.document_decision.terminal_is_empty(){return self.document_decision.close_work_demand(body)}
  if !self.override_decision.terminal_is_empty(){return self.override_decision.close_work_demand(body)}
  if !self.story_decision.terminal_is_empty(){return self.story_decision.close_work_demand(body)}
   if !self.parent_content.terminal_is_empty(){return self.parent_content.close_work_demand(body)}
  if !self.page_deletion.terminal_is_empty(){return self.page_deletion.close_work_demand(body)}
  if !self.frame_decision.terminal_is_empty(){return self.frame_decision.close_work_demand(body)}
  if !self.data_decision.terminal_is_empty()||!self.decision.terminal_is_empty(){return Ok(1)}
  if !self.retirement.is_empty()||self.diff.is_some(){return self.retirement.next_owner_work::<LayoutDiff>(self.diff.is_some(),&None,body)}
  if let Some(rows)=self.mutation_retirement.as_ref(){return if rows.terminal_is_empty(){Ok(1)}else{rows.close_work_demand(c.mutations,body)}}
  if self.payload_rows.is_some()||self.inverse_rows.is_some()||self.messages.is_some(){return Ok(1)}
  if let Some(ledger)=self.ledger.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_computational_work(ledger,body)}
  Ok(0)
 }
 fn requires_original_birth(&self)->bool{
  if !self.flight.idle(){return false}
  if !self.batch.terminal_is_empty(){return self.batch.requires_original_birth()}
  if !self.story_inverse.terminal_is_empty(){return self.story_inverse.requires_original_birth()}
  if !self.story_output.terminal_is_empty(){return self.story_output.requires_original_birth()}
   if !self.parent_inverse.terminal_is_empty(){return self.parent_inverse.requires_original_birth()}
   if !self.parent_output.terminal_is_empty(){return self.parent_output.requires_original_birth()}
  if !self.inverse.terminal_is_empty(){return self.inverse.requires_original_birth()}
  if !self.payload.terminal_is_empty(){return self.payload.close_started()&&self.payload.requires_original_birth()}
  if !self.output.terminal_is_empty(){return self.output.requires_original_birth()}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.requires_original_birth()}
  if !self.document_decision.terminal_is_empty()||!self.override_decision.terminal_is_empty()||!self.story_decision.terminal_is_empty(){return false}
  if !self.parent_content.terminal_is_empty(){return self.parent_content.requires_original_birth()}
  if !self.page_deletion.terminal_is_empty(){return self.page_deletion.requires_original_birth()}
  if !self.data_decision.terminal_is_empty()||!self.frame_decision.terminal_is_empty()||!self.decision.terminal_is_empty(){return false}
  if !self.retirement.is_empty(){return self.retirement.requires_original_birth()}
  self.diff.is_none()&&self.mutation_retirement.as_ref().is_some_and(FactoryVectorRetirement::requires_original_birth)
 }
 fn requires_original_control(&self)->bool{
  if let Some(loaned)=self.flight.loaned(){return loaned}
  if !self.batch.terminal_is_empty(){return self.batch.requires_original_control()}
  if !self.story_inverse.terminal_is_empty(){return self.story_inverse.requires_original_control()}
  if !self.story_output.terminal_is_empty(){return self.story_output.requires_original_control()}
   if !self.parent_inverse.terminal_is_empty(){return self.parent_inverse.requires_original_control()}
   if !self.parent_output.terminal_is_empty(){return self.parent_output.requires_original_control()}
  if !self.inverse.terminal_is_empty(){return self.inverse.requires_original_control()}
  if !self.payload.terminal_is_empty(){return self.payload.close_started()&&self.payload.requires_original_control()}
  if !self.output.terminal_is_empty(){return self.output.requires_original_control()}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.requires_original_control()}
  if !self.document_decision.terminal_is_empty()||!self.override_decision.terminal_is_empty(){return false}
  if !self.story_decision.terminal_is_empty(){return self.story_decision.requires_original_control()}
   if !self.parent_content.terminal_is_empty(){return self.parent_content.requires_original_control()}
  if !self.page_deletion.terminal_is_empty(){return self.page_deletion.requires_original_control()}
  if !self.data_decision.terminal_is_empty()||!self.frame_decision.terminal_is_empty()||!self.decision.terminal_is_empty(){return false}
  if !self.retirement.is_empty(){return self.retirement.requires_original_control()}
  self.diff.is_none()&&self.mutation_retirement.as_ref().is_some_and(FactoryVectorRetirement::requires_original_control)
 }
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  if !self.batch.terminal_is_empty(){return self.batch.close_original_birth_step(original,grant)}
  if !self.story_inverse.terminal_is_empty(){return self.story_inverse.close_original_birth_step(original,grant)}
  if !self.story_output.terminal_is_empty(){return self.story_output.close_original_birth_step(original,grant)}
   if !self.parent_inverse.terminal_is_empty(){return self.parent_inverse.close_original_birth_step(original,grant)}
   if !self.parent_output.terminal_is_empty(){return self.parent_output.close_original_birth_step(original,grant)}
  if !self.inverse.terminal_is_empty(){return self.inverse.close_original_birth_step(original,grant)}
  if !self.payload.terminal_is_empty(){return self.payload.close_original_birth_step(original,grant)}
  if !self.output.terminal_is_empty(){return self.output.close_original_birth_step(original,grant)}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_original_birth_step(original,grant)}
  if !self.parent_content.terminal_is_empty(){return self.parent_content.close_original_birth_step(original,grant)}
  if !self.page_deletion.terminal_is_empty(){return self.page_deletion.close_original_birth_step(original,grant)}
  if !self.retirement.is_empty(){return self.retirement.step_original_birth(original,grant)}
  match self.mutation_retirement.as_mut(){
   Some(rows)=>rows.close_original_birth_step(original,grant),
   None=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(fault().into()),work:0}),
  }
 }
 fn close_reply<'r>(&mut self,c:RetainedMutationCloseContext<'_,LayoutMutation>,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let start=effect.performed_work();let g=effect.grant();let mut work=0;
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   if !self.closing{return Err(fault().into())}
   let d=if recover{Default::default()}else{
    let d=self.close_demand(c,body)?;
    if !protocol::retained_mutation_demand_fits(d,g)||self.close_work_demand(c,body)?>effect.remaining_work_units()?{return Ok(RetainedCloneStep::Progress(Default::default()))}
    d
   };
   let selected=if !self.batch.terminal_is_empty(){Some(self.batch.close_step(c,original.as_deref_mut(),body,effect,recover))}
   else if !self.story_inverse.terminal_is_empty(){Some(self.story_inverse.close_step(original.as_deref_mut(),body,effect,recover))}
   else if !self.story_output.terminal_is_empty(){Some(self.story_output.close_step(original.as_deref_mut(),body,effect,recover))}
    else if !self.parent_inverse.terminal_is_empty(){Some(self.parent_inverse.close_step(original.as_deref_mut(),body,effect,recover))}
    else if !self.parent_output.terminal_is_empty(){Some(self.parent_output.close_step(original.as_deref_mut(),body,effect,recover))}
   else if !self.inverse.terminal_is_empty(){Some(self.inverse.close_step(c,original.as_deref_mut(),body,effect,recover))}
   else if !self.payload.terminal_is_empty(){
    if !self.payload.close_started(){let turn=self.payload.begin_close_paid(effect);return Ok(RetainedCloneStep::Progress(turn.result?))}
    Some(protocol::retained_routed!(self.flight,effect,recover,3,body,0,original.as_deref_mut(),work,|r|
     match original.as_deref_mut(){Some(loan)=>self.payload.close_original_into(loan,body,effect,r),None=>self.payload.close_into(body,effect,r)},
     match original.as_deref_mut(){Some(loan)=>self.payload.recover_close_original_into(loan,body,effect,r),None=>self.payload.recover_close_into(body,effect,r)}
    ).map_err(Into::into))
   }else if !self.output.terminal_is_empty(){Some(self.output.close_step(original.as_deref_mut(),body,effect,recover))}
   else if !self.diagnostic.terminal_is_empty(){
    Some(protocol::retained_routed!(self.flight,effect,recover,4,body,0,original.as_deref_mut(),work,|r|
     match original.as_deref_mut(){Some(loan)=>self.diagnostic.close_original_into(loan,body,effect,r),None=>self.diagnostic.close_into(body,effect,r)},
     match original.as_deref_mut(){Some(loan)=>self.diagnostic.recover_close_original_into(loan,body,effect,r),None=>self.diagnostic.recover_close_into(body,effect,r)}
    ).map_err(Into::into))
   }else if !self.document_decision.terminal_is_empty(){Some(self.document_decision.close_step(body,effect).map_err(Into::into))}
   else if !self.override_decision.terminal_is_empty(){Some(self.override_decision.close_step(body,effect).map_err(Into::into))}
   else if !self.story_decision.terminal_is_empty(){Some(self.story_decision.close_step(original.as_deref_mut(),body,effect,recover))}
    else if !self.parent_content.terminal_is_empty(){Some(self.parent_content.close_step(original.as_deref_mut(),body,effect,recover))}
   else if !self.page_deletion.terminal_is_empty(){Some(self.page_deletion.close_step(original.as_deref_mut(),body,effect,recover))}
   else if !self.data_decision.terminal_is_empty(){if !effect.performed(1)?{return Err(fault().into())}Some(self.data_decision.close_step(effect).map_err(Into::into))}
   else if !self.frame_decision.terminal_is_empty(){Some(self.frame_decision.close_step(body,effect).map_err(Into::into))}
   else if !self.decision.terminal_is_empty(){if !effect.performed(1)?{return Err(fault().into())}Some(Ok(self.decision.close_step(g)))}
   else{None};
   if let Some(selected)=selected{
    work=effect.performed_work().saturating_sub(start).max(work);
    return Ok(RetainedCloneStep::Progress(selected?.progress()))
   }
   if !self.retirement.is_empty(){
    let step=polled_close(&mut self.flight,5,original,body,effect,recover,&mut work,|original,effect|{
     if let Some(loan)=original{self.retirement.step_original(loan,body,effect)}else{self.retirement.step_granted(body,effect)}
    })?;
    return Ok(RetainedCloneStep::Progress(step.progress()))
   }
   if self.diff.is_some(){
    let turn=self.retirement.begin_granted(&mut self.diff,effect);work+=turn.work;
    return Ok(RetainedCloneStep::Progress(turn.result?.ok_or_else(fault)?.progress()))
   }
   if let Some(rows)=self.mutation_retirement.as_mut(){
    if rows.terminal_is_empty(){if !effect.performed(1)?{return Err(fault().into())}work+=1;self.mutation_retirement=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
    let step=polled_close(&mut self.flight,6,original,body,effect,recover,&mut work,|original,effect|if let Some(loan)=original{rows.close_original_step(loan,body,effect)}else{rows.close_step(c.mutations,body,effect)})?;
    return Ok(RetainedCloneStep::Progress(step.progress()))
   }
   if self.payload_rows.is_some()||self.inverse_rows.is_some(){
    let payload=self.payload_rows.is_some();
    let rows=if payload{&mut self.payload_rows}else{&mut self.inverse_rows};
    let step=protocol::retained_polled(&mut self.flight,effect,recover,if payload{7}else{8},body,&mut work,|effect|FactoryVectorRetirement::admit(rows,&mut self.mutation_retirement,effect))??;
    return Ok(RetainedCloneStep::Progress(step.progress()))
   }
   if self.messages.is_some(){
    if !effect.performed(1)?{return Err(fault().into())}work+=1;
    *self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),self.messages.take().unwrap()));
    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
   }
   if let Some(ledger)=self.ledger.as_mut(){let step=ledger.close_step(g)?;work+=u128::from(ledger.last_turn_work());if ledger.terminal_is_empty(){*self.ledger=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
   Ok(RetainedCloneStep::Complete(Default::default()))
  })();
  let work=work.max(effect.performed_work().saturating_sub(start));
  conclude(effect,start,result,work,|step|step.progress(),reply)
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.batch.terminal_is_empty()&&self.document_decision.terminal_is_empty()&&self.override_decision.terminal_is_empty()&&self.story_decision.terminal_is_empty()&&self.page_deletion.terminal_is_empty()&&self.story_output.terminal_is_empty()&&self.story_inverse.terminal_is_empty()&&self.parent_inverse.terminal_is_empty()&&self.parent_output.terminal_is_empty()&&self.parent_content.terminal_is_empty()&&self.data_decision.terminal_is_empty()&&self.frame_decision.terminal_is_empty()&&self.inverse.terminal_is_empty()&&self.payload.terminal_is_empty()&&self.output.terminal_is_empty()&&self.diagnostic.terminal_is_empty()&&self.decision.terminal_is_empty()&&self.diff.is_none()&&self.retirement.is_empty()&&self.payload_rows.is_none()&&self.inverse_rows.is_none()&&self.mutation_retirement.is_none()&&self.messages.is_none()&&self.ledger.is_none()&&self.flight.idle()}
}
impl Drop for RetainedLayoutMutation{fn drop(&mut self){assert!(std::thread::panicking()||<Self as RetainedMutationPreparation<LayoutSnapshot,LayoutMutation>>::terminal_is_empty(self),"Layout semantic owner requires funded original cleanup")}}

#[cfg(test)]
#[test]
fn shared_reference_refusals_keep_error_severity(){let corpus:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🧳️retained/🎚️simple/🧪️tests/🔣️.json")).unwrap();let mut checked=0;for row in corpus["commands"].as_array().unwrap(){if row["errorLevel"]=="error"{assert!(matches!(error_level(row["error"].as_str().unwrap()),semio_framework_diagnostic::Severity::Error));checked+=1}}assert_eq!(checked,5);assert!(matches!(error_level("mutation.invariant"),semio_framework_diagnostic::Severity::Fatal));}
