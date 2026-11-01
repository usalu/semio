D='/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/'
P='🌱️derived/🦀️.rs'
s=open(D+P).read()
a="semio_framework_value::retirement::admit_owned_retirement(error,grant){"
b="semio_framework_value::retirement::admit_owned_retirement(original,grant){"
assert s.count(a)==1 and s.count(b)==2
s=s.replace(a,"semio_framework_value::retirement::admit_owned_retirement(error,grant).result{").replace(b,"semio_framework_value::retirement::admit_owned_retirement(original,grant).result{")
old="fn close_step_controlled(&mut self,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(refusal(\"derived operation requires explicit cancellation before retirement\"));}self.step_turn(self.context.generation,self.context.base_revision,grant,DerivedOperationTurn::Close(Some(control)),&mut||false)}"
new="fn close_step_controlled(&mut self,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value::OriginalRetirementControl)->semio_framework_value::RetainedWorkOutcome{let result=if self.closing{self.step_turn(self.context.generation,self.context.base_revision,grant,DerivedOperationTurn::Close(Some(control)),&mut||false)}else{Err(refusal(\"derived operation requires explicit cancellation before retirement\"))};let work=u64::from(result.as_ref().is_ok_and(|step|step.progress()!=Default::default()));semio_framework_value::RetainedWorkOutcome{result,work}}"
assert s.count(old)==1
s=s.replace(old,new)
open(D+P,'w').write(s)
