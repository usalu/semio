p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/👥️tuple/🦀️.rs"
s=open(p,encoding='utf8').read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:100])
    s=s.replace(a,b)
# demands: stage 2 publishes through the slot's own quote
rep("$index=>match self.stage{0|2|4=>Ok(fixed(0)),1=>","$index=>match self.stage{0|4=>Ok(fixed(0)),2=>self.$slot.take_demand(),1=>")
i=s.index("   fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>")
j=s.index("   fn output_ready(&self)->bool{")
new="""   fn select_advance(&self,input:RetainedCloneRef<'_,I>,body:usize,_maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{let bound=input.check_binding(&self.binding)?;if self.closing||self.phase>=3{return Err(invalid())}if self.phase==0{return Ok(NativeAdvanceSelection::Local{work:if bound{1}else{input.binding_work_demand(&self.binding)?}})}if self.phase==1||self.index==$count{return Ok(NativeAdvanceSelection::Local{work:1})}match self.index{$($index=>match self.stage{0|4=>Ok(NativeAdvanceSelection::Local{work:1}),1=>Ok(if self.$slot.ready(){NativeAdvanceSelection::Local{work:1}}else{NativeAdvanceSelection::Nested}),2=>Ok(NativeAdvanceSelection::Local{work:self.$slot.take_work_demand()?}),3=>Ok(if self.$slot.terminal_is_empty(){NativeAdvanceSelection::Local{work:1}}else{NativeAdvanceSelection::Local{work:self.$slot.close_work_demand(body)?}}),_=>Err(invalid())},)+_=>Err(invalid())}}
   fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{if self.closing||self.phase!=2||self.index>=$count{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}let rank=self.index;let source=input.project(rank,move|v|v.sequence_child(rank).unwrap());match self.index{$($index=>if self.stage==1&&!self.$slot.ready(){self.$slot.advance(source,control,body,maximum_work_units,child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress()))}else{crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})},)+_=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}}
   fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_receiver_protocol::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<semio_framework_value_native_receiver_protocol::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
    let g=effect.grant();let maximum_work_units=effect.maximum_work_units();
    macro_rules! perform{()=>{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native tuple action exceeded its original work").into())}}}
    let result:Result<Option<RetainedCloneStep>,OriginalReceivingPrimary>=(||{
     let d=self.demands(input,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
     if self.phase==0{if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|Some(RetainedCloneStep::Progress(p.unwrap_or_default()))).map_err(Into::into)}perform!();self.phase=1;return Ok(Some(RetainedCloneStep::Progress(receipt(fixed(0)))))}
     if self.phase==1{perform!();if input.get().sequence_len()? != $count{return Err(invalid().into())}self.phase=2;return Ok(Some(RetainedCloneStep::Progress(receipt(fixed(0)))))}
     if self.index==$count{perform!();self.phase=3;return Ok(Some(RetainedCloneStep::Complete(receipt(fixed(0)))))}
     match self.index{$($index=>match self.stage{
      0=>{perform!();self.stage=1;},
      1=>{if !self.$slot.ready(){return Err(invalid().into())}perform!();self.stage=2;},
      2=>{match self.$slot.take(&mut *self.$field,effect){crate::RetainedWorkPoll::Pending{..}=>return Ok(None),crate::RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;if self.$field.is_none(){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}self.$slot.begin_close();self.stage=3;return Ok(Some(RetainedCloneStep::Progress(step.progress())))}}},
      3=>{if self.$slot.terminal_is_empty(){perform!();self.stage=4;}else{return match self.$slot.close_step(&mut effect.reborrow(child_grant(g,0),effect.maximum_work_units())?){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>turn.result.map(|s|Some(RetainedCloneStep::Progress(s.progress()))).map_err(Into::into)}}},
      4=>{perform!();self.$slot=Default::default();self.index+=1;self.stage=0;},
      _=>return Err(invalid().into())},)+_=>return Err(invalid().into())}
     Ok(Some(RetainedCloneStep::Progress(receipt(fixed(0)))))
    })();
    Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
   }
"""
s=s[:i]+new+s[j:]
open(p,'w',encoding='utf8').write(s);print("ok")
