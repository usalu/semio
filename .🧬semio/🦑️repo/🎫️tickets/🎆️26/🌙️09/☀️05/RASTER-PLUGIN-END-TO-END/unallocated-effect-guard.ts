import {readFileSync,writeFileSync} from 'node:fs';
const file='🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️effect/🦀️.rs';let text=readFileSync(file,'utf8');
const changes:[string,string][]=[
 ['actual:Option<usize>,allocation:ManuallyDrop','actual:Option<usize>,unallocated_started:bool,allocation:ManuallyDrop'],
 ['grant:None,actual:None,allocation:ManuallyDrop','grant:None,actual:None,unallocated_started:false,allocation:ManuallyDrop'],
 ['self.phase=NativeReceivingEffectPhase::EffectStarted;*self.child=Some(operation(grant));','self.phase=NativeReceivingEffectPhase::EffectStarted;self.unallocated_started=true;*self.child=Some(operation(grant));'],
 ['self.demand.capacity_bytes!=0||self.allocation.is_some()||self.child.is_some()||Self::recipient','self.demand.capacity_bytes!=0||!self.unallocated_started||self.allocation.is_some()||self.child.is_some()||Self::recipient'],
 ['||self.allocation.is_some()||self.outcome.is_some()||self.child.is_some()||self.actual.is_some(){return Err(Self::invalid())}self.phase=NativeReceivingEffectPhase::Granted','||self.unallocated_started||self.allocation.is_some()||self.outcome.is_some()||self.child.is_some()||self.actual.is_some(){return Err(Self::invalid())}self.phase=NativeReceivingEffectPhase::Granted'],
 ['self.grant=None;self.actual=None;Ok(RetainedWorkOutcome{result,work:turn.work})','self.grant=None;self.actual=None;self.unallocated_started=false;Ok(RetainedWorkOutcome{result,work:turn.work})'],
];for(const [before,after] of changes){if(!text.includes(before))throw new Error(`effect changed: ${before}`);text=text.replace(before,after)}writeFileSync(file,text);
