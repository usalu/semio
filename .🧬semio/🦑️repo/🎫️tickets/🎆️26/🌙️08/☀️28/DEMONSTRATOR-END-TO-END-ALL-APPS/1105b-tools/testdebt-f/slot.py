import sys
sys.path.insert(0,'.')
from rep import apply
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/📦️slot/🧪️tests/🦀️.rs"
LOAN="let mut port_loan_issuer=crate::native_encoding::NativeEncodeAllocationIssuer::new();let mut port_loan=port_loan_issuer.loan(&mut port);"
AUTH="let mut authority=crate::NativeAllocationAuthority::new(eff::Port(port));authority.with_send_loan(&mut (),|port_loan|{"
I="}).unwrap();assert!(authority.into_port().is_ok());"
apply(P,[
 ("use super::super::{NativeAllocationEvent,NativeEncodeAllocationIssuer};","use super::super::NativeAllocationEvent;use crate::eff;",1),
 ("#[path=\"../../../🧪️tests/♻️close/🦀️.rs\"]\nmod typed_close;\n","",1),
 ("fn close<T:crate::retirement::RetireOwned>(value:T){typed_close::close(value,crate::RetainedCloneGrant{maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},32768);}","fn close<T:crate::retirement::RetireOwned>(value:T){eff::close(value);}",1),
 ("let mut port=|event:NativeAllocationEvent|match event{NativeAllocationEvent::Admit(_)=>panic!(\"unexpected budget admission\"),NativeAllocationEvent::Quote(_)","let port=|event:NativeAllocationEvent|match event{NativeAllocationEvent::Admit(_)=>panic!(\"unexpected budget admission\"),NativeAllocationEvent::Quote(_)",3),
 ("let mut port=|event:NativeAllocationEvent|match event{NativeAllocationEvent::Admit(_)=>panic!(\"unexpected budget admission\"),NativeAllocationEvent::Quote(quote)","let port=|event:NativeAllocationEvent|match event{NativeAllocationEvent::Admit(_)=>panic!(\"unexpected budget admission\"),NativeAllocationEvent::Quote(quote)",1),
 # A
 (LOAN+"let mut native=NativeEncodeControl::new_forwarded(prior+request,&mut progress,&mut port_loan);let mut prior_owner=Vec::<u8>::new();let mut unwind=None;native.allocate_vec_into(prior,&mut prior_owner,&mut unwind).unwrap();deny.store(true,Ordering::Relaxed);let mut output=Vec::<u8>::new();",
  AUTH+"let mut native=NativeEncodeControl::new_forwarded(prior+request,&mut progress,port_loan);let mut prior_owner=Vec::<u8>::new();let mut unwind=None;let mut prior_actual=None;native.allocate_vec_into(prior,&mut prior_owner,&mut prior_actual,&mut unwind).unwrap();deny.store(true,Ordering::Relaxed);let mut output=Vec::<u8>::new();let mut actual=None;",1),
 ("native.allocate_vec_into(request,&mut output,&mut unwind)));","native.allocate_vec_into(request,&mut output,&mut actual,&mut unwind)));",1),
 ("assert_eq!(failure.forwarded_cancel.as_ref().unwrap().message.as_ref(),\"cancel refused\");","assert_eq!(failure.forwarded_cancel.as_ref().unwrap().owned().unwrap().message.as_ref(),\"cancel refused\");",1),
 ("drop(native);close(prior_owner);close(output);close(failure);","drop(native);close(prior_owner);close(output);close(failure);"+I,1),
 # B
 (LOAN+"let mut native=NativeEncodeControl::new_forwarded(5,&mut progress,&mut port_loan);let mut output=Vec::<u8>::new();let mut unwind=None;let failure=native.allocate_vec_into(5,&mut output,&mut unwind).unwrap_err();",
  AUTH+"let mut native=NativeEncodeControl::new_forwarded(5,&mut progress,port_loan);let mut output=Vec::<u8>::new();let mut unwind=None;let mut actual=None;let failure=native.allocate_vec_into(5,&mut output,&mut actual,&mut unwind).unwrap_err();",1),
 ("assert_eq!(failure.forwarded_post.as_ref().unwrap().message.as_ref(),\"post refused\");assert!(unwind.is_none());drop(native);close(output);close(failure);","assert_eq!(failure.forwarded_post.as_ref().unwrap().owned().unwrap().message.as_ref(),\"post refused\");assert!(unwind.is_none());drop(native);close(output);close(failure);"+I,1),
 # C
 (LOAN+"let mut native=NativeEncodeControl::new_forwarded(5,&mut progress,&mut port_loan);let mut output=String::new();let mut unwind=None;let failure=native.reserve_text_into(5,&mut output,&mut unwind).unwrap_err();",
  AUTH+"let mut native=NativeEncodeControl::new_forwarded(5,&mut progress,port_loan);let mut output=String::new();let mut unwind=None;let mut actual=None;let failure=native.reserve_text_into(5,&mut output,&mut actual,&mut unwind).unwrap_err();",1),
 ("assert_eq!(failure.forwarded_post.as_ref().unwrap().message.as_ref(),\"text post refused\");assert!(unwind.is_none());drop(native);close(output);close(failure);","assert_eq!(failure.forwarded_post.as_ref().unwrap().owned().unwrap().message.as_ref(),\"text post refused\");assert!(unwind.is_none());drop(native);close(output);close(failure);"+I,1),
 # D
 ("let mut original=|event:NativeAllocationEvent|","let original=|event:NativeAllocationEvent|",1),
 ("let mut scoped=|event:NativeAllocationEvent|match event{NativeAllocationEvent::Admit(_)=>panic!(\"unexpected budget admission\"),NativeAllocationEvent::Quote(_)=>Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,\"scoped quote refused\"))","let scoped=|event:NativeAllocationEvent|match event{NativeAllocationEvent::Admit(_)=>panic!(\"unexpected budget admission\"),NativeAllocationEvent::Quote(_)=>Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,\"scoped quote refused\"))",1),
 ("let mut original_loan_issuer=crate::native_encoding::NativeEncodeAllocationIssuer::new();let mut original_loan=original_loan_issuer.loan(&mut original);let mut scoped_loan_issuer=crate::native_encoding::NativeEncodeAllocationIssuer::new();let mut scoped_loan=scoped_loan_issuer.scoped_loan(&mut scoped);let mut native=NativeEncodeControl::new_forwarded(5,&mut progress,&mut original_loan);native.scoped_allocation=Some(&mut scoped_loan);let mut output=Vec::<u8>::new();let mut unwind=None;let failure=native.allocate_vec_into(5,&mut output,&mut unwind).unwrap_err();assert_eq!(failure.scoped_quote.as_ref().unwrap().message.as_ref(),\"scoped quote refused\");assert_eq!(failure.forwarded_cancel.as_ref().unwrap().message.as_ref(),\"original cancel refused\");",
  "let mut original_authority=crate::NativeAllocationAuthority::new(eff::Port(original));let mut scoped_authority=crate::NativeAllocationAuthority::new(eff::Port(scoped));original_authority.with_send_loan(&mut (),|original_loan|scoped_authority.with_send_loan(&mut (),|scoped_loan|{let mut native=NativeEncodeControl::new_forwarded(5,&mut progress,original_loan);native.scoped_allocation=Some(scoped_loan);let mut output=Vec::<u8>::new();let mut unwind=None;let mut actual=None;let failure=native.allocate_vec_into(5,&mut output,&mut actual,&mut unwind).unwrap_err();assert_eq!(failure.scoped_quote.as_ref().unwrap().owned().unwrap().message.as_ref(),\"scoped quote refused\");assert_eq!(failure.forwarded_cancel.as_ref().unwrap().owned().unwrap().message.as_ref(),\"original cancel refused\");",1),
 ("assert_eq!(output.capacity(),0);assert!(unwind.is_none());\n}","assert_eq!(output.capacity(),0);assert!(unwind.is_none());native.scoped_allocation=None;drop(native);close(failure);}).unwrap()).unwrap();assert!(original_authority.into_port().is_ok());assert!(scoped_authority.into_port().is_ok());\n}",1),
])
lines=open(P,encoding='utf-8').read().split('\n')
def find(prefix):
    idx=[i for i,l in enumerate(lines) if l.startswith(prefix)]
    assert len(idx)==1,(prefix,idx)
    return idx[0]
# E: unwind after recorded post verdict
i=find(" let verdict=std::sync::Mutex::new(None);let port=")
lines[i]=(" struct Interrupting;impl crate::NativeAllocationAuthorityPort for Interrupting{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{None}"
 "fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:NativeAllocationEvent,reply:crate::NativeAllocationReply<'reply>)->Result<crate::NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{match event{NativeAllocationEvent::Admit(_)=>panic!(\"unexpected budget admission\"),NativeAllocationEvent::Quote(_)=>Ok(reply.complete(Ok(()))),NativeAllocationEvent::Post{..}=>{let reply=reply.complete(Err(ValueError::literal(ValueRefusalKind::Canceled,\"original post refusal\").into()));std::hint::black_box(&reply);panic!(\"original post interrupted after recording verdict\")},_=>panic!(\"unexpected event\")}}"
 "fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:NativeAllocationEvent,reply:crate::NativeAllocationReply<'reply>)->Result<crate::NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{Ok(reply.pending())}}\n"
 " let mut observe=|_|true;let mut authority=crate::NativeAllocationAuthority::new(Interrupting);authority.with_send_loan(&mut (),|port_loan|{let mut native=NativeEncodeControl::new_forwarded(5,&mut observe,port_loan);let mut output=Vec::<u8>::new();let mut state=None;let mut actual=None;let unwind=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||native.allocate_vec_into(5,&mut output,&mut actual,&mut state)));assert!(unwind.is_err());assert_eq!(output.capacity(),5);let pending=state.as_mut().unwrap();let actual=pending.outcome().actual;assert_eq!(actual,Some(output.capacity()));"
 "native.settle_allocation_progress(pending,actual).unwrap();assert!(native.settle_allocation_progress(pending,actual).is_err());let failure=state.take().unwrap().into_outcome().ok().unwrap().into_allocation_failure(0).unwrap();assert_eq!(failure.forwarded_post.as_ref().unwrap().owned().unwrap().message.as_ref(),\"original post refusal\");assert_eq!(failure.progress.admitted_bytes,5);assert_eq!(failure.progress.actual_capacity_bytes,actual);drop(native);close(output);close(failure);}).unwrap();assert!(authority.into_port().is_ok());")
# F
i=find(" for (forwarded_ok,scoped_ok) in")
l=lines[i]
l=l.replace("use super::super::super::{NativeAllocationEvent,NativeEncodeAllocationIssuer};","")
l=l.replace("let mut forwarded=|event|","let forwarded=|event|").replace("let mut scoped=|event|","let scoped=|event|")
old="let mut issuer=NativeEncodeAllocationIssuer::new();let mut loan=issuer.loan(&mut forwarded);let mut scoped_issuer=NativeEncodeAllocationIssuer::new();let mut scoped_loan=scoped_issuer.scoped_loan(&mut scoped);let mut observer=|_|true;let mut control=NativeEncodeControl::new_forwarded(5,&mut observer,&mut loan);control.scoped_allocation=Some(&mut scoped_loan);"
assert l.count(old)==1
l=l.replace(old,"let mut observer=|_|true;let mut forwarded_authority=crate::NativeAllocationAuthority::new(eff::Port(forwarded));let mut scoped_authority=crate::NativeAllocationAuthority::new(eff::Port(scoped));forwarded_authority.with_send_loan(&mut (),|loan|scoped_authority.with_send_loan(&mut (),|scoped_loan|{let mut control=NativeEncodeControl::new_forwarded(5,&mut observer,loan);control.scoped_allocation=Some(scoped_loan);")
old2="control.scoped_allocation=None;drop(control)}"
assert l.count(old2)==1
l=l.replace(old2,"control.scoped_allocation=None;drop(control)}).unwrap()).unwrap();assert!(forwarded_authority.into_port().is_ok());assert!(scoped_authority.into_port().is_ok());}")
lines[i]=l
# G
i=find(" let verdict=std::sync::Mutex::new(None);let mut forwarded=")
lines[i]=(" struct Forwarded;impl crate::NativeAllocationAuthorityPort for Forwarded{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{None}"
 "fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:NativeAllocationEvent,reply:crate::NativeAllocationReply<'reply>)->Result<crate::NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{match event{NativeAllocationEvent::Admit(_)=>{let reply=reply.complete(Ok(()));std::hint::black_box(&reply);panic!(\"original admission interrupted\")},_=>panic!(\"unexpected physical event\")}}"
 "fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:NativeAllocationEvent,reply:crate::NativeAllocationReply<'reply>)->Result<crate::NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{Ok(reply.pending())}}\n"
 " let scoped=|event|match event{NativeAllocationEvent::Admit(_)=>Err(ValueError::literal(ValueRefusalKind::Canceled,\"scoped refused\")),_=>panic!(\"unexpected physical event\")};let mut observer=|_|true;let mut forwarded_authority=crate::NativeAllocationAuthority::new(Forwarded);let mut scoped_authority=crate::NativeAllocationAuthority::new(eff::Port(scoped));"
 "forwarded_authority.with_send_loan(&mut (),|loan|scoped_authority.with_send_loan(&mut (),|scoped_loan|{let mut control=NativeEncodeControl::new_forwarded(5,&mut observer,loan);control.scoped_allocation=Some(scoped_loan);let mut slot=None;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||control.begin_admission(5,&mut slot)));assert!(result.is_err());let state=slot.as_mut().unwrap();control.continue_admission(state).unwrap();assert_eq!(control.owned_bytes(),5);"
 "let failure=control.finish_admission(&mut slot).unwrap_err();assert_eq!(failure.progress.admitted_bytes,5);assert_eq!(failure.progress.actual_capacity_bytes,None);assert!(failure.forwarded_admission.is_none());assert_eq!(failure.scoped_admission.as_ref().unwrap().owned().unwrap().message.as_ref(),\"scoped refused\");control.scoped_allocation=None;drop(control);close(failure);}).unwrap()).unwrap();assert!(forwarded_authority.into_port().is_ok());assert!(scoped_authority.into_port().is_ok());")
open(P,'w',encoding='utf-8').write('\n'.join(lines))
print("ok")
