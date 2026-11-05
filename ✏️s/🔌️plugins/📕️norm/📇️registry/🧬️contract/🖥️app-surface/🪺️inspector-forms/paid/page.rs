//! 📄️ Original retained schema/value sources fund one bounded metadata step before a page can be borrowed.
use super::{original::*,walk::{self,Candidate,Kind,Part,Seed,Walk,WalkStep},NormInspectorChoicePage};
use semio_framework::ArgSchema;
use semio_framework_value::{NativeDecodeControl,OriginalRetirementControl,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneRef,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"norm.inspector.page-source")}
#[derive(Clone,Copy)]
pub(crate) struct Control{
 pub candidate:Candidate,pub path:[Part;64],pub values:[usize;64],pub choice:Option<NormInspectorChoicePage>,pub selected:Option<usize>,
}
#[derive(Clone,Copy)]
enum Phase{Check,Walk,Choice,Ready}
/// 🪟️ Page metadata and navigation borrow one exact original owner without cloning descriptors or entered values.
pub struct NormInspectorPreparedPage{
 original:ManuallyDrop<Option<NormInspectorOriginalSource>>,
 pub(crate) controls:[Option<Control>;16],pub(crate) len:usize,
 pub field_offset:usize,pub previous:Option<usize>,pub next:Option<usize>,
}
impl NormInspectorPreparedPage{
 pub fn borrow_original(&self)->Result<RetainedCloneRef<'_,NormInspectorPageOriginal>,ValueError>{Ok(self.original.as_ref().ok_or_else(invalid)?.borrow())}
 pub fn content_controls(&self)->usize{self.len}
 /// 🎁️ Create and Add borrow their exact prepared native values, including the real missing-array singleton.
 pub fn fixed_value_source(&self,index:usize)->Result<Option<RetainedCloneRef<'_,semio_framework_value::DslValue>>,ValueError>{
  let control=self.controls.get(index).and_then(Option::as_ref).ok_or_else(invalid)?;
  let original=self.borrow_original()?;
  let seed=match control.candidate.kind{
   Kind::Create{seed}=>seed,
   Kind::Append{absent:true,..}=>return super::norm_inspector_singleton_source(original,control.candidate.node).map(Some),
   Kind::Append{seed,..}=>seed,
   _=>return Ok(None),
  };
  match seed{
   Seed::Static(index)=>super::norm_inspector_seed_source(original,index).map(Some),
   Seed::Initial=>super::norm_inspector_initial_source(original).map(Some),
   Seed::EmptyText=>super::norm_inspector_singleton_source(original,control.candidate.node)?.try_project(0,|value|value.as_array().and_then(|values|values.first()).ok_or_else(invalid)).map(Some),
  }
 }
 /// 🔀️ Resident UI option indices map directly to the genuine prepared complete-parent replacement roster.
 pub fn variant_choice_source(&self,index:usize,visible:usize)->Result<Option<RetainedCloneRef<'_,semio_framework_value::DslValue>>,ValueError>{
  let control=self.controls.get(index).and_then(Option::as_ref).ok_or_else(invalid)?;
  if !matches!(control.candidate.kind,Kind::Choice{replace_parent:true,..}){return Ok(None)}
  let option=control.choice.ok_or_else(invalid)?.index(visible).ok_or_else(invalid)?;
  super::norm_inspector_variant_source(self.borrow_original()?,control.candidate.variant_group.ok_or_else(invalid)?,option).map(Some)
 }

 pub fn next_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.original.as_ref(){Some(source)if source.terminal_is_empty()=>Ok(RetirementDemand{copy_bytes:size_of::<Option<NormInspectorOriginalSource>>(),depth:1,..Default::default()}),Some(source)=>source.next_close_demand(body),None=>Ok(Default::default())}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let Some(source)=self.original.as_mut()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  if source.terminal_is_empty(){
   let bytes=size_of::<Option<NormInspectorOriginalSource>>();
   if !retained_clone_grant_admits(grant,RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
   self.original.take();self.len=0;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})),work:1}
  }source.close_step(grant)
 }
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()}
}
impl semio_framework_value::retirement::RetireOwned for NormInspectorPreparedPage{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
 fn controlled_retirement_supported()->bool{true}
}
impl semio_framework_value::retirement::RetirementCursor for NormInspectorPreparedPage{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.next_close_demand(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{let turn=Self::close_step(self,grant);match turn.result{Ok(step)=>semio_framework_value::retirement::RetirementStep::Progress(step.progress(),turn.work),Err(error)=>semio_framework_value::retirement::RetirementStep::Failure(error,turn.work)}}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl Drop for NormInspectorPreparedPage{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"norm inspector page requires original funded retirement")}}
/// 🚶️ The original native caller advances one checked field/value edge and retains at most sixteen resident controls.
pub struct NormInspectorPagePreparation{
 page:ManuallyDrop<Option<NormInspectorPreparedPage>>,walk:Walk,phase:Phase,
 index:usize,choice_scan:usize,choice_selected:Option<usize>,choice_offset:usize,
 ledger:u64,recipient:Option<u64>,closing:bool,
}
impl NormInspectorPagePreparation{
 pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 pub fn admit(source:&mut Option<NormInspectorOriginalSource>,control:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  let demand=Self::constructor_demand();if !retained_clone_grant_admits(grant,demand){return Ok(None)}
  let source=source.take().ok_or_else(invalid)?;
  Ok(Some((Self{page:ManuallyDrop::new(Some(NormInspectorPreparedPage{original:ManuallyDrop::new(Some(source)),controls:[None;16],len:0,field_offset:0,previous:None,next:None})),walk:Walk::new(),phase:Phase::Check,index:0,choice_scan:0,choice_selected:None,choice_offset:0,ledger:control.ledger_identity(),recipient:control.retirement_recipient_identity(),closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
 }
 fn same(&self,control:&NativeDecodeControl<'_>)->Result<(),ValueError>{if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity(){return Err(invalid())}Ok(())}
 pub fn ready(&self)->bool{!self.closing&&matches!(self.phase,Phase::Ready)}
 pub fn demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:if matches!(self.phase,Phase::Ready){size_of::<Option<NormInspectorPreparedPage>>()+size_of::<bool>()}else{size_of::<Control>()+Walk::step_copy_bytes()+size_of::<usize>()*8+size_of::<Phase>()},depth:1,..Default::default()}}
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let mut work=0;let result=(||{
   self.same(control)?;if self.closing||self.ready(){return Err(invalid())}let demand=self.demands();
   if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;
   let page=self.page.as_mut().ok_or_else(invalid)?;let original=page.original.as_ref().ok_or_else(invalid)?.borrow();let original=original.get();
   match self.phase{
    Phase::Check=>{
     original.validate()?;
     page.field_offset=original.request.page.as_ref().map_or(Ok(0),|page|page.field_offset.parse::<usize>().map_err(|_|invalid()))?;
     page.previous=(page.field_offset>0).then(||page.field_offset.saturating_sub(16));self.phase=Phase::Walk;
    },
    Phase::Walk=>match self.walk.step(original)?{
     WalkStep::Progress=>{},
     WalkStep::Complete=>{if self.index<page.field_offset||self.index==page.field_offset&&self.index!=0{return Err(invalid())}self.phase=Phase::Ready},
     WalkStep::Candidate(candidate)=>{
      let large=matches!(candidate.kind,Kind::Choice{count,..}if count>32);
      if self.index>=page.field_offset{
       if page.len==16||large&&page.len>0||page.controls[0].is_some_and(|control|matches!(control.candidate.kind,Kind::Choice{count,..}if count>32)){page.next=Some(self.index);self.phase=Phase::Ready}
       else{
        page.controls[page.len]=Some(Control{candidate,path:self.walk.path,values:self.walk.values,choice:None,selected:None});page.len+=1;
        if let Kind::Choice{..}=candidate.kind{
         self.choice_offset=0;self.choice_scan=0;self.choice_selected=None;
         if large{
          if let Some(choice)=original.request.page.as_ref().and_then(|page|page.choice.as_ref()){
           if choice.path.len()==candidate.path_len&&choice.path.iter().zip(&self.walk.path).all(|(text,part)|part.text(original)==Some(text.as_str())){
            self.choice_offset=choice.offset.parse().map_err(|_|invalid())?;
           }
          }
         }
         self.phase=Phase::Choice;
        }
       }
      }self.index=self.index.checked_add(1).ok_or_else(invalid)?;
     }
    },
    Phase::Choice=>{
     let selected=page.controls[page.len-1].as_mut().ok_or_else(invalid)?;
     let Kind::Choice{count,..}=selected.candidate.kind else{return Err(invalid())};
     if self.choice_scan<count{
      let(_,schema)=walk::schema(original,selected.candidate.node)?;
      let ArgSchema::String{options,..}=schema else{return Err(invalid())};
      if let Some(group)=selected.candidate.variant_group{
       let group=original.semantics.variants.get(group).ok_or_else(invalid)?;
       if group.schema_row!=selected.candidate.node||group.value_path.as_slice()!=&selected.values[..selected.candidate.value_len-1]||group.options.len()!=count||group.options[self.choice_scan].option_index!=self.choice_scan{return Err(invalid())}
      }
      let current=selected.candidate.present.then(||walk::value(original,&selected.values[..selected.candidate.value_len])).flatten().and_then(|value|value.as_str());
      if current==Some(options[self.choice_scan].value.as_str()){self.choice_selected=Some(self.choice_scan)}self.choice_scan+=1;
     }else{
      selected.choice=Some(NormInspectorChoicePage::admit(count,self.choice_offset,self.choice_selected)?);selected.selected=self.choice_selected;
      self.phase=Phase::Walk
     }
    },
    Phase::Ready=>return Err(invalid()),
   }work=1;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
  })();RetainedWorkOutcome{result,work}
 }
 pub fn take(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(NormInspectorPreparedPage,RetainedCloneProgress)>>{
  let mut work=0;let result=(||{self.same(control)?;if !self.ready(){return Err(invalid())}let demand=self.demands();if !retained_clone_grant_admits(grant,demand){return Ok(None)}control.checkpoint()?;let page=self.page.take().ok_or_else(invalid)?;self.closing=true;work=1;Ok(Some((page,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))})();RetainedWorkOutcome{result,work}
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}match self.page.as_ref(){Some(page)if page.terminal_is_empty()=>Ok(RetirementDemand{copy_bytes:size_of::<Option<NormInspectorPreparedPage>>(),depth:1,..Default::default()}),Some(page)=>page.next_close_demand(body),None=>Ok(Default::default())}}
 pub fn close_step(&mut self,control:&mut dyn OriginalRetirementControl,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing||self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  let demand=match self.close_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if !retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if demand.capacity_bytes>0{if let Err(error)=control.charge(demand.capacity_bytes){return RetainedWorkOutcome{result:Err(error),work:0}}}
  let Some(page)=self.page.as_mut()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  if page.terminal_is_empty(){
   let bytes=size_of::<Option<NormInspectorPreparedPage>>();
   if !retained_clone_grant_admits(grant,RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
   self.page.take();return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})),work:1}
  }page.close_step(grant)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.page.is_none()}
}
impl Drop for NormInspectorPagePreparation{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"norm inspector page preparation requires funded closure")}}
