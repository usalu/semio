import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname,join}from"node:path";
const b="🧰️framework/🔨️modules/🌱️value/🛬️decode/",t=dirname(import.meta.path);
function edit(f:string,fn:(s:string)=>string){const s=readFileSync(f,"utf8"),n=fn(s),tmp=join(t,"🗑️generated/intrinsic-"+Date.now()+".rs");if(n===s)throw Error("unchanged "+f);writeFileSync(tmp,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent "+f);renameSync(tmp,f)}
function rep(s:string,a:string,z:string){if(s.split(a).length!==2)throw Error("anchor "+a);return s.replace(a,z)}
function region(s:string,a:string,b:string,z:string){const i=s.indexOf(a),j=s.indexOf(b,i+a.length);if(i<0||j<0)throw Error("region "+a);return s.slice(0,i)+z+s.slice(j)}
edit(b+"🧳️owned/🧬️contract/🦀️.rs",s=>rep(s,"fn close_step(&mut self,grant:RetainedCloneGrant)->OriginalRetirementOutcome;","fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->OriginalRetirementOutcome;"));
edit(b+"🧳️owned/🪡️field/🦀️.rs",s=>{
s=rep(s,"use crate::OriginalReceivingPrimary;","use crate::{OriginalReceivingPrimary,OriginalRetirementFailure,native_decoding::NativeReceivingEffect};");
s=rep(s,"retirement:RetainedCloneClose,phase:","retirement:RetainedCloneClose,effect:NativeReceivingEffect,retirement_effect:NativeReceivingEffect<OriginalRetirementFailure>,phase:");
s=rep(s,"retirement:Default::default(),phase:","retirement:Default::default(),effect:Default::default(),retirement_effect:Default::default(),phase:");
s=region(s," pub fn has_pending_recovery("," pub fn ready(",` pub fn has_pending_recovery(&self)->bool{self.effect.has_pending_recovery()||self.retirement_effect.has_pending_recovery()||self.slots.has_pending_recovery()}
 pub fn pending_recovery(&mut self)->Option<crate::owned::NativeRecoveryLoan<'_>>{if self.effect.has_pending_recovery(){return self.effect.pending_recovery().map(crate::owned::NativeRecoveryLoan::Value)}if self.retirement_effect.has_pending_recovery(){return self.retirement_effect.pending_recovery().map(crate::owned::NativeRecoveryLoan::Retirement)}self.slots.admission().pending_recovery().map(crate::owned::NativeRecoveryLoan::Value)}
 pub fn resume_recovery(&mut self,control:&mut NativeDecodeControl<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{if self.effect.has_pending_recovery(){control.finish_receiving(&mut self.effect)}else if self.retirement_effect.has_pending_recovery(){control.finish_retirement_receiving(&mut self.retirement_effect)}else{self.slots.resume_recovery(control)}}
`);
s=region(s," pub fn demands("," pub fn take_demand(",` pub fn demands(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=4{return Err(invalid())}if maximum_work_units==0{return Ok(Default::default())}
  if !original.check_binding(&self.binding)?{return Ok(fixed(original.binding_copy_bytes()))}
  if self.phase==0{return if self.cursor.output_ready(){self.cursor.take_demand()}else{self.cursor.advance_demands(original,body,maximum_work_units)}}
  if self.phase==1{return if self.output.is_some(){Ok(fixed(0))}else if self.raw.is_some(){Ok(super::NativeSharedSlots::<T,SealedShared<T>>::capture_demand())}else if self.slots.output_ready(){Ok(super::NativeSharedSlots::<T,SealedShared<T>>::take_demand())}else{Ok(self.slots.demand())}}
  if self.cursor.terminal_is_empty(){return Ok(fixed(0))}self.cursor.close_demands(body)
 }
 pub fn advance_work_demand(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
  self.demands(original,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}
  let work=if !original.check_binding(&self.binding)?{1}else if self.phase==0{if self.cursor.output_ready(){self.cursor.take_work_demand()?}else{self.cursor.advance_work_demand(original,body,maximum_work_units)?}}else if self.phase==1{if self.output.is_some(){crate::owned::work_sum(self.cursor.begin_close_work_demand()?,1)?}else if self.raw.is_some()||self.slots.output_ready(){1}else{self.slots.advance_work_demand()?}}else if self.cursor.terminal_is_empty(){1}else{self.cursor.close_work_demand(body)?};
  Ok(crate::owned::selected_native_work(work,maximum_work_units))
 }
 pub fn advance(&mut self,original:RetainedCloneRef<'_,T>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{let result=(||{
  let maximum=crate::owned::native_work_limit(control,maximum_work_units)?;let q=self.advance_work_demand(original,body,maximum)?;let d=self.demands(original,body,maximum)?;if q==0||!retained_clone_grant_admits(grant,d){return Ok(None)}control.checkpoint()?;crate::owned::receiving_capacity(control,d.capacity_bytes)?;
  let turn=if !original.check_binding(&self.binding)?{control.receive_allocation(d,&mut self.effect,false,q,|g,_|{let turn=original.bind(&mut self.binding,g);RetainedWorkOutcome{result:turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:turn.work}})}
  else if self.phase==0{
   if !self.cursor.output_ready(){let cursor=&mut self.cursor;control.receive_retirement(d,&mut self.retirement_effect,d.capacity_bytes!=0,q,|g,actual|{let mut none=None;cursor.advance(original,body,maximum,g,actual.unwrap_or(&mut none))})}
   else{let cursor=&mut self.cursor;let raw=&mut self.raw;let phase=&mut self.phase;control.receive_allocation(d,&mut self.effect,false,q,|g,_|{let turn=cursor.take(g);RetainedWorkOutcome{result:turn.result.map(|v|{if let Some((value,p))=v{**raw=Some(value);*phase=1;RetainedCloneStep::Progress(p)}else{RetainedCloneStep::Progress(Default::default())}}),work:turn.work}})}
  }else if self.phase==1{
   if self.output.is_some(){let cursor=&mut self.cursor;let phase=&mut self.phase;control.receive_allocation(d,&mut self.effect,false,q,|_,_|{let turn=cursor.begin_close_paid();RetainedWorkOutcome{result:turn.result.map(|()|{*phase=2;RetainedCloneStep::Progress(receipt(d))}),work:turn.work+1}})}
   else if self.raw.is_some(){let slots=&mut self.slots;let raw=&mut self.raw;control.receive_allocation(d,&mut self.effect,false,q,|g,_|slots.capture(raw,g))}
   else if self.slots.output_ready(){let slots=&mut self.slots;let output=&mut self.output;control.receive_allocation(d,&mut self.effect,false,q,|g,_|slots.take(output,g))}
   else{self.slots.advance(control,SharedIssuer::owned())}
  }else if self.cursor.terminal_is_empty(){let phase=&mut self.phase;control.receive_allocation(d,&mut self.effect,false,q,|_,_|{*phase=4;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}})}
  else{let cursor=&mut self.cursor;control.receive_retirement(d,&mut self.retirement_effect,d.capacity_bytes!=0,q,|g,actual|{let mut none=None;cursor.close_step(g,actual.unwrap_or(&mut none))})};
  Ok(Some(turn))
 })();match result{Ok(Some(turn))=>RetainedWorkOutcome{result:turn.result.map(|p|if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)}),work:turn.work},Ok(None)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>RetainedWorkOutcome{result:Err(error),work:0}}}
 pub fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;Ok(1)}
`);
s=rep(s," pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome",` pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  self.close_demands(body)?;if !self.slots.terminal_is_empty(){return self.slots.close_work_demand(body)}if !self.cursor.terminal_is_empty(){return self.cursor.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.raw.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<SealedShared<T>>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::OriginalRetirementOutcome`);
s=s.replaceAll("self.slots.close_step(grant)","self.slots.close_step(grant,actual)").replaceAll("self.cursor.close_step(grant)","self.cursor.close_step(grant,actual)").replaceAll("self.retirement.step_granted(grant)","self.retirement.step_granted(grant,actual)").replaceAll("self.retirement.begin_granted(&mut self.raw,grant)","self.retirement.begin_granted(&mut self.raw,grant,actual)").replaceAll("self.retirement.begin_granted(&mut self.output,grant)","self.retirement.begin_granted(&mut self.output,grant,actual)");
s=rep(s,"self.slots.terminal_is_empty()&&self.binding.is_none()","self.effect.is_empty()&&self.retirement_effect.is_empty()&&self.slots.terminal_is_empty()&&self.binding.is_none()");
return s});
console.log("Intrinsic work gates and stable effects authored; marker primitive pending Server");
