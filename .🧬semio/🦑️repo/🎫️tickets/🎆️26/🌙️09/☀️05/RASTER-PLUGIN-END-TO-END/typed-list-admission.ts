import {readFileSync,writeFileSync} from 'node:fs';
const file='🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📋️list/🦀️.rs';let text=readFileSync(file,'utf8');
const changes:[string,string][]=[
 ['allocation:crate::native_decoding::NativeReceivingEffect','allocation:crate::native_decoding::NativeSharedAdmission<T>'],
 ['pub fn receiving_effect(&mut self)->&mut crate::native_decoding::NativeReceivingEffect{&mut self.allocation}','pub fn shared_admission(&mut self)->&mut crate::native_decoding::NativeSharedAdmission<T>{&mut self.allocation}'],
 ['Ok(crate::native_decoding::NativeOwnedDecodeControl::shared_birth_demand::<T>())','Ok(self.allocation.shared_demand())'],
 ['return Err(phase_error())}if self.child.reserved()','return Err(phase_error())}if self.allocation.has_birth(){return self.allocation.close_demand()}if self.child.reserved()'],
 ['if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.child.reserved()','if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.allocation.has_birth(){let turn=self.allocation.close_step(g);work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress()))}if self.child.reserved()'],
 ['self.allocation.phase()==crate::native_decoding::NativeReceivingEffectPhase::Empty&&self.child.terminal_is_empty()','self.allocation.terminal_is_empty()&&self.child.terminal_is_empty()'],
];for(const [before,after] of changes){if(!text.includes(before))throw new Error(`list admission changed: ${before}`);text=text.replace(before,after)}writeFileSync(file,text);
