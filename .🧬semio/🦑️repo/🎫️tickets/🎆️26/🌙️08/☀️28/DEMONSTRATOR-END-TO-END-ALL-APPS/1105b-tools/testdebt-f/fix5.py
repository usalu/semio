import sys
sys.path.insert(0,'.')
from rep import apply
D="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛫️encode/"
A=D+"🛂️allocation/🧪️tests/🦀️.rs"
apply(A,[
 ("let receipt=guest.pause().unwrap();","let receipt=guest.pause().ok().unwrap();",1),
 ("allocate_loan).pause().unwrap();","allocate_loan).pause().ok().unwrap();",1),
 ("let mut receipt=control.pause().unwrap();let mut next=","let mut receipt=control.pause().ok().unwrap();let mut next=",1),
 (".forwarded_refusal.as_ref().unwrap().message",".forwarded_refusal.as_ref().unwrap().owned().unwrap().message",2),
 (".scoped_refusal.as_ref().unwrap().message",".scoped_refusal.as_ref().unwrap().owned().unwrap().message",1),
 (".scoped_quote.as_ref().unwrap().message",".scoped_quote.as_ref().unwrap().owned().unwrap().message",2),
 ("fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementOutcome{let grant=effect.grant();let result=(||->Result<RetainedCloneStep,ValueError>{","fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{let grant=effect.grant();let result=(||->Result<RetainedCloneStep,ValueError>{",1),
 ("crate::RetainedWorkOutcome{result:result.map_err(Into::into),work:1}}\n  fn terminal_is_empty","crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:result.map_err(Into::into),work:1})}\n  fn terminal_is_empty",1),
])
apply(D+"🛂️allocation/🧪️tests/🔒️receipt/🦀️.rs",[
 ("|control|control.charge(1)).unwrap_err().kind","|control|control.charge(1)).map_err(crate::eff::cause).unwrap_err().kind",1),
])
apply(D+"🛂️allocation/📦️slot/🧪️tests/🦀️.rs",[
 ("use super::super::super::{NativeAllocationEvent,NativeEncodeAllocationIssuer};","",1),
])
apply(D+"🧪️tests/🎟️eff/🦀️.rs",[
 ("check.complete(self.validate_context(context))}\n fn recover_partial<'r,'a,'b>(&mut self,context:&mut Self::Context<'a,'b>,check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 'b:'a{check.complete(self.validate_context(context))}","check.complete(OriginalWorkRecipient::validate_context(self,context))}\n fn recover_partial<'r,'a,'b>(&mut self,context:&mut Self::Context<'a,'b>,check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 'b:'a{check.complete(OriginalWorkRecipient::validate_context(self,context))}",1),
 ("/// 🧾️ Generous per-turn stock","/// 🔎️ Names the one cause that a native call failure retains.\npub fn cause<T>(failure:semio_framework_value_call::NativeCallFailure<T,ValueError>)->ValueError{failure.operation.or(failure.context).expect(\"native call failure retains its cause\")}\n/// 🧾️ Generous per-turn stock",1),
])
apply(D+"🧭️span/🦀️.rs",[
 ("check:crate::OriginalWorkPartialCheck<'r>)->crate::OriginalWorkPartialCheckComplete<'r> where 'b:'a{check.complete(self.validate_context(context))}\n  fn recover_partial","check:crate::OriginalWorkPartialCheck<'r>)->crate::OriginalWorkPartialCheckComplete<'r> where 'b:'a{check.complete(crate::OriginalWorkRecipient::validate_context(self,context))}\n  fn recover_partial",1),
 ("check:crate::OriginalWorkPartialCheck<'r>)->crate::OriginalWorkPartialCheckComplete<'r> where 'b:'a{check.complete(self.validate_context(context))}\n  fn admits","check:crate::OriginalWorkPartialCheck<'r>)->crate::OriginalWorkPartialCheckComplete<'r> where 'b:'a{check.complete(crate::OriginalWorkRecipient::validate_context(self,context))}\n  fn admits",1),
])
# roots
apply(D+"🧪️tests/🥣️action/🦀️.rs",[("#[cfg(test)]\n#[path=\"../../🎟️action/🧪️tests/🦀️.rs\"]","#[cfg(test)]\n#[path=\"../🎟️eff/🦀️.rs\"]\nmod eff;\n#[cfg(test)]\n#[path=\"../../🎟️action/🧪️tests/🦀️.rs\"]",1)])
apply(D+"🧪️tests/🎟️effect/🦀️.rs",[("#[cfg(test)]\n#[path=\"../../🎟️effect/🧪️tests/🦀️.rs\"]","#[cfg(test)]\n#[path=\"../🎟️eff/🦀️.rs\"]\nmod eff;\n#[cfg(test)]\n#[path=\"../../🎟️effect/🧪️tests/🦀️.rs\"]",1)])
print("ok")
