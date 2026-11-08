import sys
sys.path.insert(0,'.')
from rep import apply
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/🧪️tests/🦀️.rs"
LOAN="let mut allocate_loan_issuer=crate::native_encoding::NativeEncodeAllocationIssuer::new();let mut allocate_loan=allocate_loan_issuer.loan(&mut allocate);"
AUTH="let mut authority=crate::NativeAllocationAuthority::new(eff::Port(allocate));authority.with_send_loan(&mut (),|allocate_loan|{"
edits=[
 ("let mut allocate=|event","let allocate=|event",7),
 (LOAN,AUTH,7),
 ("&mut allocate_loan","allocate_loan",7),
 ("drop(resumed);}\n let host","drop(resumed);}).unwrap();}\n let host",1),
 ("assert_eq!(source.as_ptr(),pointer);}\n let calls","assert_eq!(source.as_ptr(),pointer);}).unwrap();}\n let calls",1),
 ("assert_eq!(zero.owned_bytes(),0);\n}","assert_eq!(zero.owned_bytes(),0);}).unwrap();\n}",1),
 ("drop(receipt);}\n let host","drop(receipt);}).unwrap();}\n let host",1),
 ("assert_eq!(original_progress.load(Ordering::Relaxed),4);assert_eq!(phase_progress.load(Ordering::Relaxed),3);\n}","assert_eq!(original_progress.load(Ordering::Relaxed),4);assert_eq!(phase_progress.load(Ordering::Relaxed),3);}).unwrap();\n}",1),
 ("drop(receipt);assert!(!recipient.has_owner());\n}","drop(receipt);}).unwrap();assert!(!recipient.has_owner());\n}",1),
 ("drop(receipt);assert!(recipient.has_owner());","drop(receipt);}).unwrap();assert!(recipient.has_owner());",1),
 ("let mut retirement_actual=Some(73);let(turn,heap)=crate::observe_retirement_allocations(||recipient.close_step(denied,&mut retirement_actual));","let mut retirement_actual=Some(73);let(turn,heap)=crate::observe_retirement_allocations(||eff::settled(eff::with_effect(denied,1<<20,&mut retirement_actual,|effect|recipient.close_step(effect))));",1),
 ("crate::observe_retirement_allocations(||recipient.close_step(remaining,&mut retirement_actual));","crate::observe_retirement_allocations(||eff::settled(eff::with_effect(remaining,1<<20,&mut retirement_actual,|effect|recipient.close_step(effect))));",1),
 ("let result=control.close_retirement_recipient(grant,&mut retirement_actual).result;","let result=eff::settled(eff::with_effect(grant,1<<20,&mut retirement_actual,|effect|control.close_retirement_recipient(effect))).result;",1),
 ("crate::observe_retirement_allocations(||control.close_retirement_recipient(grant,&mut retirement_actual));","crate::observe_retirement_allocations(||eff::settled(eff::with_effect(grant,1<<20,&mut retirement_actual,|effect|control.close_retirement_recipient(effect))));",1),
 ("crate::observe_retirement_allocations(||recipient.close_step(grant,&mut retirement_actual));","crate::observe_retirement_allocations(||eff::settled(eff::with_effect(grant,1<<20,&mut retirement_actual,|effect|recipient.close_step(effect))));",1),
 ("struct Original;\n impl crate::NativeAllocationAuthorityPort for Original{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}fn receive<'turn,'source>(&mut self,_:&mut (),_:crate::NativeAllocationEvent)->Result<(),crate::NativeAllocationRefusal> where 'source:'turn{Ok(())}fn recover<'turn,'source>(&mut self,_:&mut (),_:crate::NativeAllocationEvent)->Result<Option<Result<(),crate::NativeAllocationRefusal>>,ValueError> where 'source:'turn{Ok(None)}}\n let mut authority=crate::NativeAllocationAuthority::new(Original);","let mut authority=crate::NativeAllocationAuthority::new(eff::Port(|_|Ok(())));",1),
]
apply(P,edits)
lines=open(P,encoding='utf-8').read().split('\n')
def find(prefix):
    idx=[i for i,l in enumerate(lines) if l.startswith(prefix)]
    assert len(idx)==1,(prefix,idx)
    return idx[0]
def between(l,a,b):
    i=l.index(a);j=l.index(b,i);return l[i:j]
i=find(" use crate::native_encoding::NativeAllocationEvent;let mut first_pending=None;")
l=lines[i]
first=between(l,"let mut first=|event","let mut second=").replace("let mut first=","let first=",1)
second=between(l,"let mut second=|event","let mut observe=").replace("let mut second=","let second=",1)
op=between(l,"&mut |actual|{let layout=std::alloc::Layout::array::<u64>(2)","});assert_eq!(native.owned_bytes(),16)")
op=op.replace("&mut |actual|","&mut |actual:&mut Option<usize>,_|",1)
lines[i]=(" use crate::native_encoding::NativeAllocationEvent;let mut first_pending=None;let mut second_pending=None;let mut first_bytes=0usize;let mut second_bytes=0usize;"+first+second+
 "let mut observe=|_|true;let mut output=None;let mut first_authority=crate::NativeAllocationAuthority::new(eff::Port(first));let mut second_authority=crate::NativeAllocationAuthority::new(eff::Port(second));"
 "let result=first_authority.with_send_loan(&mut (),|first_loan|second_authority.with_send_loan(&mut (),|second_loan|{let mut native=NativeEncodeControl::new_forwarded(16,&mut observe,first_loan);native.scoped_allocation=Some(second_loan);let mut actual=None;let mut unwind=None;"
 "let result=native.allocate_effect(16,&mut actual,&mut unwind,"+op+"}).unwrap().expect(\"original allocation settled\");"
 "assert_eq!(native.owned_bytes(),16);assert_eq!(result.actual,Some(16));assert_eq!(result.forwarded_refusal.as_ref().unwrap().message.as_ref(),\"forwarded post refused\");assert_eq!(result.scoped_refusal.as_ref().unwrap().message.as_ref(),\"scoped post refused\");assert_eq!(result.primary.as_ref().unwrap().message.as_ref(),\"operation refused after allocation\");assert!(result.extent_refusal.is_none());assert!(unwind.is_none());native.scoped_allocation=None;drop(native);result}).unwrap()).unwrap();"
 "assert!(first_authority.into_port().is_ok());assert!(second_authority.into_port().is_ok());assert_eq!((first_bytes,second_bytes),(16,16));assert!(first_pending.is_none()&&second_pending.is_none());eff::close(output.take().unwrap());eff::close(result);")
i=find(" use crate::native_encoding::NativeAllocationEvent;let mut pending=None;let mut submitted=0usize;")
l=lines[i]
port=between(l,"let mut port=|event","let mut observe=").replace("let mut port=","let port=",1)
op1=between(l,"&mut |actual|{let layout=std::alloc::Layout::array::<u8>(8)","});assert!(first.primary").replace("&mut |actual|","&mut |actual:&mut Option<usize>,_|",1).replace("*actual=Some(8);Ok(())","*actual=Some(8);Ok(true)")
lines[i]=(" use crate::native_encoding::NativeAllocationEvent;let mut pending=None;let mut submitted=0usize;let mut admitted=0usize;let mut quotes=0;"+port+
 "let mut observe=|_|true;let mut output=None;let mut authority=crate::NativeAllocationAuthority::new(eff::Port(port));"
 "let(first,second)=authority.with_send_loan(&mut (),|port_loan|{let mut native=NativeEncodeControl::new_forwarded(32,&mut observe,port_loan);let mut actual=None;let mut unwind=None;"
 "let first=native.allocate_effect(16,&mut actual,&mut unwind,"+op1+"}).unwrap().expect(\"original allocation settled\");"
 "assert!(first.primary.is_none()&&first.extent_refusal.is_none()&&first.forwarded_refusal.is_none());assert_eq!(first.request.unwrap().bytes,16);assert_eq!(first.actual,Some(8));assert_eq!(native.owned_bytes(),8);actual=None;"
 "let second=native.allocate_effect(16,&mut actual,&mut unwind,&mut |_,_|Err(ValueError::literal(ValueRefusalKind::Canceled,\"no effect\"))).unwrap().expect(\"original allocation settled\");assert_eq!(native.owned_bytes(),8);drop(native);(first,second)}).unwrap();"
 "assert!(authority.into_port().is_ok());assert_eq!(submitted,8);assert_eq!(quotes,2);assert!(pending.is_none());for error in [first,second]{eff::close(error);}eff::close(output.take().unwrap());")
for name in ["refused_second_allocation_quote_cancels_first_original_quote","refused_second_quote_retains_first_cancel_refusal"]:
    i=find("#[test]fn "+name)
    l=lines[i]
    first=between(l,"let mut first=|event","let mut second=").replace("let mut first=","let first=",1)
    second=between(l,"let mut second=|event","let mut observe=").replace("let mut second=","let second=",1)
    head=l[:l.index("let mut first=|event")]
    assertions=between(l,"assert_eq!(outcome.scoped_quote","native.scoped_allocation=None;")
    lines[i]=(head+"let mut called=false;"+first+second+"let mut observe=|_|true;let mut first_authority=crate::NativeAllocationAuthority::new(eff::Port(first));let mut second_authority=crate::NativeAllocationAuthority::new(eff::Port(second));"
     "let outcome=first_authority.with_send_loan(&mut (),|first_loan|second_authority.with_send_loan(&mut (),|second_loan|{let mut native=NativeEncodeControl::new_forwarded(16,&mut observe,first_loan);native.scoped_allocation=Some(second_loan);let mut actual=None;let mut unwind=None;"
     "let outcome=native.allocate_effect(16,&mut actual,&mut unwind,&mut |_,_|{called=true;Ok(true)}).unwrap().expect(\"original allocation settled\");"+assertions+"native.scoped_allocation=None;drop(native);outcome}).unwrap()).unwrap();"
     "assert!(first_authority.into_port().is_ok());assert!(second_authority.into_port().is_ok());assert!(!called);assert_eq!(canceled,1);assert!(pending.is_none());eff::close(outcome);}")
i=find("#[test]fn outcome_slot_retains_refusal_before_the_next_action")
l=lines[i]
l=l.replace("&mut |_|{calls+=1;Err(","&mut |_,_|{calls+=1;Err(").replace("&mut |_|{calls+=1;Ok(())}","&mut |_,_|{calls+=1;Ok(true)}")
l=l.replace("&mut |actual|{let layout=std::alloc::Layout::array::<u8>(1)","&mut |actual:&mut Option<usize>,_|{let layout=std::alloc::Layout::array::<u8>(1)").replace("*actual=Some(1);Ok(())})","*actual=Some(1);Ok(true)})")
a=l.index("let grant=RetainedCloneGrant{");b=l.index("close(outcome.take().unwrap(),grant);close(refusal,grant);")
l=l[:a]+l[b:]
l=l.replace("close(outcome.take().unwrap(),grant);close(refusal,grant);","eff::close(outcome.take().unwrap());eff::close(refusal);").replace("close(outcome.take().unwrap(),grant);close(storage.take().unwrap(),grant);","eff::close(outcome.take().unwrap());eff::close(storage.take().unwrap());")
lines[i]=l.replace("use crate::RetainedCloneGrant;","")
open(P,'w',encoding='utf-8').write('\n'.join(lines))
print("ok")
