import os
FW="/Users/ueli/Documents/semio/🧰️framework"
LED="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/🗑️generated/debug-ledger.rs"
MOD='#[path="%s"]#[allow(dead_code)]mod dbg;\n'%LED
MODP=MOD.replace("mod dbg","pub(crate) mod dbg")
def edit(p,f):
    p=os.path.join(FW,p);s=open(p).read()
    if "debug-ledger" in s: return
    s=f(s);open(p,'w').write(s)
def op(s):
    sig=" fn step(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<MemberSnapshotOpenStep,OriginalTurnFailure>{\n"
    assert s.count(sig)==1
    i=s.index(sig)+len(sig);j=s.index("\n }\n fn take_ready",i);body=s[i:j]
    new=sig+"  let ph=self.phase;let w0=cx.original_work_units_spent();let r0=cx.retained_progress();\n  let out=(||->Result<MemberSnapshotOpenStep,OriginalTurnFailure>{\n"+body+"\n  })();\n  if dbg::on(){let w=cx.original_work_units_spent()-w0;let r1=cx.retained_progress();dbg::add(format!(\"open:{ph:?}\"),w,(r1.copied_items-r0.copied_items) as u128,(r1.copied_bytes-r0.copied_bytes) as u128)}\n  out"
    s=s[:i-len(sig)]+new+s[j:]
    return s.replace("use semio_framework_value_native_receiving_original",MOD+"use semio_framework_value_native_receiving_original",1)
edit("🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs",op)
def orig(s):
    a="Ok(step)=>{let progress=step.progress();let idle="
    assert s.count(a)==1
    s=s.replace(a,"Ok(step)=>{let progress=step.progress();if dbg::on(){dbg::add(\"recvroot\".into(),turn.work,progress.copied_items as u128,progress.copied_bytes as u128)}let idle=")
    a="  if self.close_reply.is_none(){let mut answer=None;"
    assert s.count(a)==1
    s=s.replace(a,"  let dbg_branch=if dbg::on(){Some(if !self.backing.terminal_is_empty()||self.receiver.reserved()||self.retired_header.is_some(){\"backing\"}else if !self.retirement.is_empty(){\"retirement\"}else if self.output.is_some(){\"output\"}else if self.authority.is_some(){\"authority\"}else{\"original\"})}else{None};\n"+a)
    a="let result=self.close_reply.take().expect(\"original close reply selected\");self.close_route=None;"
    assert s.count(a)==1
    s=s.replace(a,a+"if let Some(b)=dbg_branch{let(i,y)=match &result{Ok(RetainedCloneStep::Progress(p)|RetainedCloneStep::Complete(p))=>(p.copied_items as u128,p.copied_bytes as u128),_=>(0,0)};dbg::add(format!(\"closeroot:{b}\"),work,i,y)}")
    return s.replace("use std::mem::{ManuallyDrop,size_of};",MOD+"use std::mem::{ManuallyDrop,size_of};",1)
edit("🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs",orig)
def rm(s):
    L=s.split('\n');i=[k for k,l in enumerate(L) if l==" Ok(Some(step))"];assert len(i)==1
    L.insert(i[0]," if dbg::on(){dbg::add(format!(\"leaf:{}:a{}\",dbg::short::<N>(),action),work.max(recorded),1,d.copy_bytes as u128)}")
    s='\n'.join(L)
    sig="fn close_poll<I:NativeValueSource+?Sized,N:RetireOwned,const C:usize>(c:&mut NativeRecordCore<I,N,C>,a:&mut dyn NativeRecordArms<I,N,C>,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close{\n"
    assert s.count(sig)==1
    s=s.replace(sig,"""fn close_poll<I:NativeValueSource+?Sized,N:RetireOwned,const C:usize>(c:&mut NativeRecordCore<I,N,C>,a:&mut dyn NativeRecordArms<I,N,C>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close{
 if !dbg::on(){return close_poll_inner(c,a,original,body,effect)}
 let before=effect.performed_work();DBG_INNER.with(|f|f.set(false));let poll=close_poll_inner(c,a,original,body,effect);let inner=DBG_INNER.with(|f|f.replace(true));
 if !inner{let(i,y)=match &poll{RetainedWorkPoll::Complete(o)=>match &o.result{Ok(Step::Progress(p)|Step::Complete(p))=>(p.copied_items as u128,p.copied_bytes as u128),_=>(0,0)},_=>(0,0)};dbg::add(format!("close:{}:p{}",dbg::short::<N>(),c.close_field.min(99)),effect.performed_work()-before,i,y)}
 poll
}
"""+sig.replace("fn close_poll<","fn close_poll_inner<"))
    s=s.replace("\nfn close_poll<","\nthread_local!{static DBG_INNER:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};}\nfn close_poll<",1)
    return s.replace("type Projection=",MODP+"type Projection=",1)
edit("🔨️modules/🌱️value/🛬️decode/🧳️owned/🏗️record-machine/🦀️.rs",rm)
def pr(s):
    a="let selected=*self.demand.get_or_insert(demand);"
    assert s.count(a)==1
    s=s.replace(a,'if dbg::on()&&self.demand.is_none(){let n=dbg::strip(std::any::type_name::<P>());let n:String=n.replace("NativeSlotProducer<dynNativeValueSource,","").chars().take(60).collect();dbg::add(format!("poll:{n}:d{}",demand.copy_bytes),work as u128,1,demand.copy_bytes as u128)}'+a)
    i=s.index("\n");return s[:i+1]+MOD+s[i+1:]
edit("🔨️modules/🌱️value/🛬️decode/🧳️owned/🧬️protocol/📦️packages/🦀️rust/🦀️.rs",pr)
def en(s):
    a="  let result=(||->Result<Option<Step>,OriginalReceivingPrimary>{\n   let(d,q,action)=self.plan(input,body,maximum)?;"
    assert s.count(a)==1
    s=s.replace(a,a+"\n   if dbg::on(){dbg::add(format!(\"enum:{}:a{}:d{}\",dbg::short::<N>(),action,d.copy_bytes),q as u128,1,d.copy_bytes as u128)}")
    a2="   let done=if action==23{native_relay"
    assert s.count(a2)==1
    s=s.replace(a2,"   if dbg::on(){dbg::add(format!(\"enum:{}:a{}:d{}\",dbg::short::<N>(),action,d.copy_bytes),q as u128,1,d.copy_bytes as u128)}\n"+a2)
    return s.replace("use super::*;","use super::*;\n"+MODP,1)
edit("🔨️modules/🌱️value/🛬️decode/🧳️owned/🏗️record-machine/🔀️enum/🦀️.rs",en)
