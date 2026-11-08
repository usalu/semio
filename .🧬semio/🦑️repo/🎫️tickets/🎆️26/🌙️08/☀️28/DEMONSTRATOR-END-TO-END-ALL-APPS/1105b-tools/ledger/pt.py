p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📝️text/📦️paged/🦀️.rs"
s=open(p,encoding='utf8').read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:110])
    s=s.replace(a,b)
def wrap_in(start_marker,end_marker=None):
    """wrap every `crate::RetainedWorkOutcome{..}` struct literal after start_marker (until end of line) with done(...)"""
    global s
    i=s.index(start_marker)
    j=s.index("\n",i)
    seg=s[i:j]
    out="";k=0;lit="crate::RetainedWorkOutcome{"
    first=seg.index("{")  # skip signature (no literal there anyway)
    while True:
        m=seg.find(lit,k)
        if m<0: out+=seg[k:];break
        out+=seg[k:m]
        d=0;e=m+len(lit)-1
        while True:
            c=seg[e]
            if c=="{":d+=1
            elif c=="}":
                d-=1
                if d==0:break
            e+=1
        out+="done("+seg[m:e+1]+")";k=e+1
    s=s[:i]+out+s[j:]
# a) reserve_decoded_effect
i=s.index("fn reserve_decoded_effect<const N:usize>")
j=s.index("\n",i)
new="""fn reserve_decoded_effect<const N:usize>(bytes:&mut PagedList<u8,N>,limit:Option<usize>,required:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep,ValueError>{let turn=match limit{Some(limit)=>bytes.reserve_exact_capacity_one(limit,effect),None=>bytes.reserve_one(effect)};let work=turn.work;let result=match turn.result{Ok(crate::list::PagedListReservationOutcome::Denied)=>Ok(RetainedCloneStep::Progress(Default::default())),Ok(crate::list::PagedListReservationOutcome::Ready(step))=>{let progress=RetainedCloneProgress{copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,..Default::default()};if step.progressed&&step.allocated_bytes==required{Ok(RetainedCloneStep::Complete(progress))}else{Err(ValueError::invariant("paged text allocation differs from its exact admission").with_retained_progress(progress))}},Err(crate::list::PagedListReservationFailure::Work(failure))=>Err(failure),Err(crate::list::PagedListReservationFailure::Allocation{error:failure,copied_bytes})=>{let progress=RetainedCloneProgress{copied_bytes,retained_capacity_bytes:failure.allocated_bytes,..Default::default()};Err(error(failure.refusal()).with_retained_progress(progress))}};crate::RetainedWorkOutcome{result,work}}"""
s=s[:i]+new+s[j:]
# b) mut effect
rep("read:&mut TextSourceReadFrame,effect:Option<&mut crate::RetainedAllocationEffect<'_>>,phase:&mut u8,length:&Option<usize>,complete:&mut bool","read:&mut TextSourceReadFrame,mut effect:Option<&mut crate::RetainedAllocationEffect<'_>>,phase:&mut u8,length:&Option<usize>,complete:&mut bool")
# c) cursor close_steps (two identical lines 81,106)
sig=" pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementOutcome{if let Err(error)=self.close_demand(effect.grant().maximum_copy_bytes){return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}if self.binding.is_none()&&self.read.have!=0{"
assert s.count(sig)==2
s=s.replace(sig,sig.replace("->semio_framework_value_retirement::OriginalRetirementOutcome{","->semio_framework_value_clone_source::OriginalRetirementPoll{"))
# wrap literals in those two lines
newsig=sig.replace("->semio_framework_value_retirement::OriginalRetirementOutcome{","->semio_framework_value_clone_source::OriginalRetirementPoll{")
for _ in range(2):
    # find next unprocessed occurrence (no 'done(' in line)
    pos=0
    while True:
        i=s.index(newsig,pos);j=s.index("\n",i)
        if "done(" in s[i:j]: pos=j;continue
        wrap_in(s[i:i+60]);break
# 153/194 shapes
sig2=" pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementOutcome{if let Err(error)=self.close_demand(effect.grant().maximum_copy_bytes){return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}if !self.state.terminal_is_empty(){"
assert s.count(sig2)==2
a1="let turn=self.state.close_step(effect);return crate::RetainedWorkOutcome{result:turn.result.map(|step|if self.owner.is_some(){RetainedCloneStep::Progress(step.progress())}else{step}),work:turn.work}}"
assert s.count(a1)==2
s=s.replace(a1,"return self.state.close_step(effect).map(|step|if self.owner.is_some(){RetainedCloneStep::Progress(step.progress())}else{step})}")
a2="let turn=owner.close_step(effect);if owner.terminal_is_empty(){self.owner=None}return crate::RetainedWorkOutcome{result:turn.result.map(|step|if self.inline.is_some(){RetainedCloneStep::Progress(step.progress())}else{step}),work:turn.work}}"
assert s.count(a2)==2
s=s.replace(a2,"let turn=owner.close_step(effect);if owner.terminal_is_empty(){self.owner=None}return turn.map(|step|if self.inline.is_some(){RetainedCloneStep::Progress(step.progress())}else{step})}")
newsig2=sig2.replace("->semio_framework_value_retirement::OriginalRetirementOutcome{","->semio_framework_value_clone_source::OriginalRetirementPoll{")
s=s.replace(sig2,newsig2)
pos=0
for _ in range(2):
    while True:
        i=s.index(newsig2,pos);j=s.index("\n",i)
        if "done(" in s[i:j]: pos=j;continue
        wrap_in(s[i:i+60]);break
# helper done
rep("pub use semio_framework_value_clone_source::RetainedCloneRef;","pub use semio_framework_value_clone_source::RetainedCloneRef;\nfn done(turn:crate::RetainedWorkOutcome<RetainedCloneStep,semio_framework_value_clone_source::OriginalRetirementFailure>)->semio_framework_value_clone_source::OriginalRetirementPoll{semio_framework_value_clone_source::RetainedWorkPoll::Complete(turn)}")
# d) settle/retry
n=s.count("if let Some(work)=self.publication.child_work(){")
s=s.replace("if let Some(work)=self.publication.child_work(){","if self.publication.child_work().is_some(){")
s=s.replace("if let Some(work)=frame.effect.child_work(){","if frame.effect.child_work().is_some(){")
s=s.replace("self.publication.retry_original_work(control)?}else{self.publication.settle_original_work(control,work)?}","{self.publication.retry_original_work(control)?;}}else{self.publication.settle_original_work(control)?;}")
s=s.replace("frame.effect.retry_original_work(control)?}else{frame.effect.settle_original_work(control,work)?}","{frame.effect.retry_original_work(control)?;}}else{frame.effect.settle_original_work(control)?;}")
# e) decode bind
rep("let actual_grant=effect.grant();match selected{PagedTextReadOperation::Bind=>{let turn=input.bind(binding,actual_grant);","match selected{PagedTextReadOperation::Bind=>{let turn=input.bind(binding,effect);")
# f) close_retained
rep("let step=self.bytes.release_empty_page(grant.maximum_release_bytes).map_err(error)?;work+=step.work;","let turn=self.bytes.release_empty_page(effect);work+=turn.work;let step=match turn.result{Ok(crate::list::PagedListReleaseOutcome::Denied)=>return Ok(RetainedCloneStep::Progress(empty)),Ok(crate::list::PagedListReleaseOutcome::Ready(step))=>step,Err(crate::list::PagedListReleaseFailure::Work(failure))=>return Err(failure),Err(crate::list::PagedListReleaseFailure::Release(failure))=>return Err(error(failure))};")
rep("fn close_retained(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let grant=effect.grant();let mut work=0;","fn close_retained(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let grant=effect.grant();let mut work:u128=0;")
# g) trait close_step
rep("fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementOutcome{let turn=self.close_retained(effect);crate::RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}","fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_clone_source::OriginalRetirementPoll{let turn=self.close_retained(effect);done(crate::RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}")
open(p,'w',encoding='utf8').write(s);print("ok",n)
