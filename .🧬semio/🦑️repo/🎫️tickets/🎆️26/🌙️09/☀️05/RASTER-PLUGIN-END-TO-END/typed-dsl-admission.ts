import {readFileSync,writeFileSync} from 'node:fs';
const file='🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/📋️persistent/🦀️.rs';let text=readFileSync(file,'utf8');
const changes:[string,string][]=[
 ['allocation:semio_framework_value::native_decoding::NativeReceivingEffect','allocation:semio_framework_value::native_decoding::NativeSharedAdmission<T>'],
 ['pub fn receiving_effect(&mut self)->&mut semio_framework_value::native_decoding::NativeReceivingEffect{&mut self.allocation}','pub fn shared_admission(&mut self)->&mut semio_framework_value::native_decoding::NativeSharedAdmission<T>{&mut self.allocation}'],
 ['Ok(semio_framework_value_native_owned::native_decoding::NativeOwnedDecodeControl::shared_birth_demand::<T>())','Ok(self.allocation.shared_demand())'],
 ['return Err(invariant())}if self.terminal_is_empty()','return Err(invariant())}if self.allocation.has_birth(){return self.allocation.close_demand()}if self.terminal_is_empty()'],
 ['if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}if !self.closing','if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}if self.allocation.has_birth(){let turn=self.allocation.close_step(g);return RetainedWorkOutcome{result:turn.result.map(|step|step.progress()),work:turn.work}}if !self.closing'],
 ['self.allocation.phase()==semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty&&self.child.terminal_is_empty()','self.allocation.terminal_is_empty()&&self.child.terminal_is_empty()'],
];for(const [before,after] of changes){if(!text.includes(before))throw new Error(`DSL admission changed: ${before}`);text=text.replace(before,after)}writeFileSync(file,text);
