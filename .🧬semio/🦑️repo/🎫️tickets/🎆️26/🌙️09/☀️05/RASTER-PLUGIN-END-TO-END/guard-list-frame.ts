import {readFileSync,writeFileSync} from 'node:fs';
const path='🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📋️list/🦀️.rs';
const text=readFileSync(path,'utf8');
const before='fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.child.reserved()';
const after='fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.allocation.phase()!=crate::native_decoding::NativeReceivingEffectPhase::Empty{return Err(phase_error())}if self.child.reserved()';
if(!text.includes(before))throw new Error('list close frontier changed');
writeFileSync(path,text.replace(before,after));
