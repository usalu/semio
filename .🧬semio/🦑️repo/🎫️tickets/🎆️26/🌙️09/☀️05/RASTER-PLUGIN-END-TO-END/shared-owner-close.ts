import {readFileSync,writeFileSync} from 'node:fs';
const file='🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔗️sealed-shared-receiver/🦀️.rs';
let text=readFileSync(file,'utf8');
const replacements:[string,string][]=[
 ['return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared allocation callback requires resolution"))}if self.child.reserved()','return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared allocation callback requires resolution"))}if self.allocation.has_birth(){return self.allocation.close_demand()}if self.child.reserved()'],
 ['if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.child.reserved()','if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.allocation.has_birth(){let turn=self.allocation.close_step(g);work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress()))}if self.child.reserved()'],
 ['self.allocation.phase()==crate::native_decoding::NativeReceivingEffectPhase::Empty&&self.child.terminal_is_empty()','self.allocation.terminal_is_empty()&&self.child.terminal_is_empty()'],
];
for(const [before,after] of replacements){if(!text.includes(before))throw new Error(`shared owner changed: ${before}`);text=text.replace(before,after)}
writeFileSync(file,text);
