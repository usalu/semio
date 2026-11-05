import{readFileSync,writeFileSync,renameSync}from"node:fs";
const file="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🎟️unit/🦀️.rs";
let source=readFileSync(file,"utf8");
for(const[before,after]of[
 ["OwnedSemioExport,OwnedOriginalCallProgress,OwnedOriginalCallStatus,OriginalOperationReceiving","OwnedOriginalCallProgress,OwnedOriginalCallStatus"],
 ["Begin,Drive,Output,Acknowledge","Begin,Drive,AwaitBudget,Output,Acknowledge"],
 ["GuestRelayUnitStatus{Pending,Output,Complete,Refused}","GuestRelayUnitStatus{Pending,AwaitBudget,Output,Complete,Refused}"],
 ["runtime.begin_original_unit_step(instance,export,&mut*self.input,input_budget,cx)","runtime.begin_original_unit_step(instance,&mut*self.input,budget,cx)"],
 ["let result=if let Some(receiver)=receiver{runtime.advance_original_unit_step_receiving(instance,execution,receiver,cx)}else{runtime.advance_original_unit_step(instance,execution,cx)};","let result=runtime.advance_original_unit_step(instance,execution,cx);"],
 ["}Ok(if self.failure().is_some(){GuestRelayUnitStatus::Refused}","}if runtime.original_unit_waiting_for_budget(instance)?{self.phase=Phase::AwaitBudget;return Ok(GuestRelayUnitStatus::AwaitBudget)}Ok(if self.failure().is_some(){GuestRelayUnitStatus::Refused}"],
 ["if self.phase==Phase::Complete{return Ok(GuestRelayUnitStatus::Complete)}","if self.phase==Phase::AwaitBudget{return Ok(GuestRelayUnitStatus::AwaitBudget)}if self.phase==Phase::Complete{return Ok(GuestRelayUnitStatus::Complete)}"],
 ]){if(source.split(before).length!==2)throw Error(before);source=source.replace(before,after);}
source=source.split("\n").filter(line=>!line.includes("pub fn advance_step_receiving")).join("\n");
for(const[before,after]of[
 ["export:OwnedSemioExport,input_budget:Option<semio_framework_actor::OriginalJobBudget>,execution:","budget:semio_framework_actor::OriginalJobBudget,execution:"],
 ["self.advance(runtime,instance,export,input_budget,execution,None,cx)","self.advance(runtime,instance,budget,execution,cx)"],
 ["receiver:Option<&mut dyn OriginalOperationReceiving>,",""]
 ]){if(!source.includes(before))throw Error(before);source=source.replaceAll(before,after);}
const marker=" pub fn execution_receipt(&self)";if(source.split(marker).length!==2)throw Error(marker);
source=source.replace(marker," pub fn repeat_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,budget:semio_framework_actor::OriginalJobBudget,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.phase!=Phase::AwaitBudget||self.failure().is_some(){return Err(refused().into())}if !metadata(cx,false)?{return Ok(false)}self.progress.take();if runtime.repeat_original_unit_step(instance,budget,cx)?{self.phase=Phase::Drive;return Ok(true)}Ok(false)}\n"+marker);
const temp=import.meta.dirname+"/🗑️generated/guest-input-flow-edit.rs";writeFileSync(temp,source);renameSync(temp,file);
