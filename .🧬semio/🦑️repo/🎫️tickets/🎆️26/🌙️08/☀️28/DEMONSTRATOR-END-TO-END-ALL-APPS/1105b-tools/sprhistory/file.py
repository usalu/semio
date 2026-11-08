import sys
P="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🦀️.rs"
s=open(P,encoding="utf-8").read()
def once(old,new,count=1):
    global s
    n=s.count(old)
    assert n==count,(n,old[:90])
    s=s.replace(old,new)
once("use semio_framework_value::{ValueError, ValueRefusalKind, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ErasedSnapshotRetirement, admit_owned_retirement, owned_retirement_birth_bytes};","use semio_framework_value::{ValueError, ValueRefusalKind, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ErasedSnapshotRetirement, admit_owned_retirement, owned_retirement_birth_bytes};\nuse semio_framework_value::retained_clone::NativeAllocationIdentity;")
once("use semio_framework_value::RetainedWorkOutcome;\n","")
once("last_work:u64,source:OriginalWireSourceIdentity,control_identity:(u64,Option<u64>),","last_work:u128,source:OriginalWireSourceIdentity,control_identity:(NativeAllocationIdentity,Option<u64>),")
once("fn charge_work(work:&mut u64,","fn charge_work(work:&mut u128,")
lines=s.split("\n")
def find(prefix):
    idx=[i for i,l in enumerate(lines) if l.startswith(prefix)]
    assert len(idx)==1,(prefix,len(idx))
    return idx[0]
i=find(" pub fn admit<S:RetainedWireByteSource+?Sized>(source:&S,range:(usize,usize),limits:HistoryReadLimits,persisted_document:bool,")
del lines[i]
i=find(" fn admit_result<")
l=lines[i]
old_head=" fn admit_result<S:RetainedWireByteSource+?Sized>(source:&S,range:(usize,usize),limits:HistoryReadLimits,persisted_document:bool,control:&NativeDecodeControl<'_>,g:RetainedCloneGrant,work:&mut u64)->Result<Option<Self>,RetainedHistoryOpMetaFault>{limits.validate()?;"
assert l.startswith(old_head)
new_head=" /// 🧮️ Bounds the performed work of one decoder turn by the nesting depth its grant admits.\n pub fn work_demand(g:RetainedCloneGrant)->u64{u64::try_from(g.maximum_depth).unwrap_or(u64::MAX).saturating_mul(4).saturating_add(8)}\n pub fn admit<S:RetainedWireByteSource+?Sized>(source:&S,range:(usize,usize),limits:HistoryReadLimits,persisted_document:bool,control:&NativeDecodeControl<'_>,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<Self>,RetainedHistoryOpMetaFault>{let g=effect.grant();limits.validate()?;"
l=new_head+l[len(old_head):]
o="*work+=1;let(_,progress)=frames.take_receipt().unwrap();if !progress.fits(g){frames.cancel();frames.close_step(source,control,child)?;frames.take_receipt();return Err(fault(\"original History admission exceeded caller grant\").into())}"
assert o in l
l=l.replace(o,"let(_,progress)=frames.take_receipt().unwrap();let paid=effect.performed(1);if !matches!(paid,Ok(true))||!progress.fits(g){frames.cancel();frames.close_step(source,control,child)?;frames.take_receipt();return Err(match paid{Err(error)=>error,Ok(false)=>ValueError::literal(ValueRefusalKind::WorkLimit,\"original History admission exceeded its work ceiling\"),Ok(true)=>fault(\"original History admission exceeded caller grant\")}.into())}")
lines[i]=l
# publish/pay
i=find(" fn publish(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)")
lines.insert(i,"\n".join([" fn pay(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<(),RetainedHistoryOpMetaFault>{self.last_work=1;if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,\"original History step exceeded its work ceiling\").into())}}"," pub fn last_turn_work(&self)->u128{self.last_work}"]))
# step wrapper
i=find(" /// 🪙️ Reports the selected semantic frontier")
assert lines[i+1].startswith(" pub fn step<S:") and lines[i+3]==" }",(lines[i+3])
sig=lines[i+1]
head=sig[:sig.index("g:RetainedCloneGrant)")]
lines[i+1:i+4]=[head+"effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{let start=effect.performed_work();let outcome=self.step_result(source,control,effect);match crate::settle_since(effect,start,self.last_work){Ok(work)=>{self.last_work=work;outcome},Err(error)=>if outcome.is_ok(){Err(error.into())}else{outcome}}}"]
i=find(" fn step_result<")
l=lines[i]
o="control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{self.validate(source,control)?;"
assert o in l
l=l.replace(o,"control:&mut NativeDecodeControl<'_>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{let g=effect.grant();self.last_work=0;self.validate(source,control)?;")
o="match self.step_inner(source,control,g){"
assert o in l
l=l.replace(o,"match self.step_inner(source,control,effect){")
lines[i]=l
i=find(" fn auxiliary_step(")
lines[i]=" fn auxiliary_step(&mut self,control:&mut dyn NativeDecodeCloseControl,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedHistorySourceStep>,RetainedHistoryOpMetaFault>{let g=effect.grant();if self.retiring.is_some(){let d=self.retirement_demands(g.maximum_copy_bytes)?;if !permits(g,d){return Ok(Some(RetainedHistorySourceStep::Blocked))}let owner=self.retiring.as_mut().unwrap();if owner.terminal_is_empty(){self.last_work=1;drop(self.retiring.take());self.publish(g,RetainedCloneProgress{released_bytes:d.release_bytes,..metadata()})?;return Ok(Some(RetainedHistorySourceStep::Progress))}Self::charge_work(&mut self.last_work,control,d.capacity_bytes)?;let result=match owner.close_step(&mut effect.nested(RetainedCloneGrant{maximum_depth:g.maximum_depth.saturating_sub(2),..g})){semio_framework_value::RetainedWorkPoll::Pending{..}=>return Ok(Some(RetainedHistorySourceStep::Blocked)),semio_framework_value::RetainedWorkPoll::Complete(turn)=>{self.last_work=turn.work;turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)}};let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};self.publish(g,progress)?;result?;return Ok(Some(RetainedHistorySourceStep::Progress))}if self.partial.as_ref().is_some_and(|partial|partial.old_identity.is_some()){control.charge(owned_retirement_birth_bytes::<RetainedHistoryIdentityRecord>())?;self.last_work=1;let mut original=self.partial.as_mut().unwrap().old_identity.take();let turn=admit_owned_retirement(&mut original,&mut effect.nested(RetainedCloneGrant{maximum_depth:g.maximum_depth.saturating_sub(1),..g}));self.last_work=turn.work;if original.is_some(){self.partial.as_mut().unwrap().old_identity=original}match turn.result{Ok(Some((owner,p)))=>{*self.retiring=Some(owner);self.publish(g,p)?;return Ok(Some(RetainedHistorySourceStep::Progress))},Ok(None)=>return Ok(Some(RetainedHistorySourceStep::Blocked)),Err(error)=>return Err(error.into())}}Ok(None)}"
i=find(" fn step_inner<")
l=lines[i]
o="control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{if let Some(step)=self.auxiliary_step(control,g)?{return Ok(step)}"
assert o in l
l=l.replace(o,"control:&mut NativeDecodeControl<'_>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{let g=effect.grant();if let Some(step)=self.auxiliary_step(control,effect)?{return Ok(step)}")
for o,n in [("return self.child_step(source,control,g)}","return self.child_step(source,control,effect)}"),("validation.step(bytes,control,child).map(|_|())","validation.step(bytes,control,&mut effect.nested(child)).map(|_|())")]:
    assert l.count(o)==1,o
    l=l.replace(o,n)
lines[i]=l
for k in range(i,i+20):
    if lines[k].startswith("   9=>"):
        o="index.append_original(&mut p.pending_edit,child)"
        assert lines[k].count(o)==1
        lines[k]=lines[k].replace(o,"index.append_original(&mut p.pending_edit,&mut effect.nested(child))")
        break
else: raise SystemExit("phase 9")
for k in range(i,i+20):
    if lines[k].startswith("   14=>"):
        o="dictionary.close_step(source,child)"
        assert lines[k].count(o)==1
        lines[k]=lines[k].replace(o,"dictionary.close_step(source,&mut effect.nested(child))")
        break
else: raise SystemExit("phase 14")
i=find(" fn child_step<")
l=lines[i]
o="control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{self.last_work=1;"
assert o in l
lines[i]=l.replace(o,"control:&mut NativeDecodeControl<'_>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{let g=effect.grant();self.last_work=1;")
i=find(" fn child_close_step<")
l=lines[i]
o="control:&mut dyn NativeDecodeCloseControl,g:RetainedCloneGrant)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{"
assert o in l
lines[i]=l.replace(o,"control:&mut dyn NativeDecodeCloseControl,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{")
assert lines[i+1].startswith("  self.last_work=1;let child_grant")
lines[i+1]=lines[i+1].replace("  self.last_work=1;","  let g=effect.grant();self.last_work=1;",1)
s="\n".join(lines)
once("c.step(source,dictionary,prior,control,child_grant)","c.step(source,dictionary,prior,control,&mut effect.nested(child_grant))",2)
once("c.step(source,dictionary,control,child_grant)","c.step(source,dictionary,control,&mut effect.nested(child_grant))",2)
once("c.close_step(source,prior,control,child_grant)","c.close_step(source,prior,control,&mut effect.nested(child_grant))",2)
once("c.close_step(source,control,child_grant)","c.close_step(source,control,&mut effect.nested(child_grant))",2)
lines=s.split("\n")
i=find(" pub fn take_ready_into(")
assert lines[i+1].startswith(" fn take_ready_into_result(")
l=lines[i+1]
o=" fn take_ready_into_result(&mut self,recipient:&mut Option<HistoryLog>,g:RetainedCloneGrant)->Result<bool,RetainedHistoryOpMetaFault>{if"
assert l.startswith(o)
l=" pub fn take_ready_into(&mut self,recipient:&mut Option<HistoryLog>,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,RetainedHistoryOpMetaFault>{let g=effect.grant();self.last_work=0;if"+l[len(o):]
for a,b in [("let mut p=self.partial.take().unwrap();","self.pay(effect)?;let mut p=self.partial.take().unwrap();"),("self.closed=true;self.last_work=1;self.publish(g,metadata())?;Ok(true)}","self.closed=true;self.publish(g,metadata())?;Ok(true)}")]:
    assert l.count(a)==1,a
    l=l.replace(a,b)
lines[i:i+2]=[l]
i=find(" pub fn take_fault_into(")
assert lines[i+1].startswith(" fn take_fault_into_result(")
l=lines[i+1]
o=" fn take_fault_into_result(&mut self,recipient:&mut Option<RetainedHistoryOpMetaFault>,g:RetainedCloneGrant)->Result<bool,RetainedHistoryOpMetaFault>{if"
assert l.startswith(o)
l=" pub fn take_fault_into(&mut self,recipient:&mut Option<RetainedHistoryOpMetaFault>,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,RetainedHistoryOpMetaFault>{let g=effect.grant();self.last_work=0;if"+l[len(o):]
a="*recipient=self.fault.take();self.last_work=1;self.cancel();"
assert l.count(a)==1
l=l.replace(a,"self.pay(effect)?;*recipient=self.fault.take();self.cancel();")
lines[i:i+2]=[l]
i=find(" pub fn take_partial_into(")
assert lines[i+1].startswith(" fn take_partial_into_result(")
l=lines[i+1]
o=" fn take_partial_into_result(&mut self,recipient:&mut Option<RetainedHistorySourcePartial>,g:RetainedCloneGrant)->Result<bool,RetainedHistoryOpMetaFault>{if"
assert l.startswith(o)
l=" pub fn take_partial_into(&mut self,recipient:&mut Option<RetainedHistorySourcePartial>,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,RetainedHistoryOpMetaFault>{let g=effect.grant();self.last_work=0;if"+l[len(o):]
a="*recipient=self.partial.take();self.last_work=1;self.publish"
assert l.count(a)==1
l=l.replace(a,"self.pay(effect)?;*recipient=self.partial.take();self.publish")
lines[i:i+2]=[l]
i=find(" /// 🧹️ Closes one admitted frontier")
assert lines[i+1].startswith(" pub fn close_step<") and lines[i+3]==" }",lines[i+3][:40]
sig=lines[i+1]
head=sig[:sig.index("g:RetainedCloneGrant)")]
lines[i+1:i+4]=[head+"effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{let start=effect.performed_work();let outcome=self.close_result(source,control,effect);match crate::settle_since(effect,start,self.last_work){Ok(work)=>{self.last_work=work;outcome},Err(error)=>if outcome.is_ok(){Err(error.into())}else{outcome}}}"]
i=find(" fn close_result<")
l=lines[i]
o="control:&mut dyn NativeDecodeCloseControl,g:RetainedCloneGrant)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{self.validate(source,control)?;"
assert o in l
l=l.replace(o,"control:&mut dyn NativeDecodeCloseControl,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>{let g=effect.grant();self.last_work=0;self.validate(source,control)?;")
for a,b in [("if let Some(step)=self.auxiliary_step(control,g)?{","if let Some(step)=self.auxiliary_step(control,effect)?{"),("return self.child_close_step(source,control,g)}","return self.child_close_step(source,control,effect)}"),("validation.close_step(bytes,control,child)?;","validation.close_step(bytes,control,&mut effect.nested(child))?;")]:
    assert l.count(a)==1,a
    l=l.replace(a,b)
lines[i]=l
for k in range(i+1,i+6):
    if "dictionary.close_step(source,child)?;" in lines[k]:
        lines[k]=lines[k].replace("dictionary.close_step(source,child)?;","dictionary.close_step(source,&mut effect.nested(child))?;"); break
else: raise SystemExit("dict close")
s="\n".join(lines)
open(P,"w",encoding="utf-8").write(s)
print("ok")
