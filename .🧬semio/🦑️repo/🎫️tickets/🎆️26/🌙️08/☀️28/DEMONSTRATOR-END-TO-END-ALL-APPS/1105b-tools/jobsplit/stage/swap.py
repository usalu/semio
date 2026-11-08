#!/usr/bin/env python3
# Atomic swap: moves ⏸️park out of job-context into semio-framework-job-park. Fail-closed; run once after LOCK NEEDED is granted.
import os,sys,shutil
R='/Users/ueli/Documents/semio/'
F=R+'🧰️framework/'
J=F+'🔨️modules/🧵️job/'
C=J+'⏱️context/'
STAGE=os.path.dirname(os.path.abspath(__file__))
edits={}
def load(p):
    if p not in edits: edits[p]=open(p,encoding='utf8').read()
    return edits[p]
def rep(p,old,new,n=1):
    t=load(p); assert t.count(old)==n,(p[-60:],old[:70],t.count(old)); edits[p]=t.replace(old,new)
root=C+'🦀️.rs'
rep(root,'#[path="⏸️park/🦀️.rs"]\nmod original_owned_caller_turn;\npub use original_owned_caller_turn::{OriginalOwnedCallerTurn,OriginalOwnedCallerReturn,OriginalOwnedCallerOutcome};\n','')
rep(root,'struct OriginalRetainedLoanIdentity{','/// 🪪️ Pins the exact turn context a loan or parked wallet was lent from.\npub struct OriginalRetainedLoanIdentity{')
rep(root,' fn capture(cx:&StepContext<\'_>)->Self{',' pub fn capture(cx:&StepContext<\'_>)->Self{')
rep(root,' fn matches(&self,cx:&StepContext<\'_>)->bool{self.turn==',' pub fn matches(&self,cx:&StepContext<\'_>)->bool{self.turn==')
rep(root,'impl JobPayloadStep for StepContext<\'_> {','''/// 🔗️ The narrow mutable view of turn state that the parked-wallet crate moves between contexts.
pub struct ParkFields<'c>{pub fuel_remaining:&'c mut u64,pub retained:&'c mut RetainedCloneGrant,pub clock:&'c std::cell::Cell<ClockStride>,pub payload_page_granted:&'c mut bool,pub stage:&'c mut &'static str,pub preview_sequence:&'c mut u64}
impl StepContext<'_>{
    /// 🔗️ Lends the wallet-carrying fields of this turn.
    pub fn park_fields(&mut self)->ParkFields<'_>{ParkFields{fuel_remaining:&mut self.fuel_remaining,retained:&mut self.retained,clock:&self.clock,payload_page_granted:&mut self.payload_page_granted,stage:&mut self.stage,preview_sequence:&mut *self.preview_sequence}}
}

impl JobPayloadStep for StepContext<\'_> {''')
src=C+'🎟️source/🦀️.rs'
rep(src,' pub(crate) fn admit_owned_turn(&self,output:&mut Option<crate::OriginalOwnedCallerTurn>,cx:&mut StepContext<\'_>,epoch:u64)->Result<bool,OriginalTurnFailure>{',' pub fn admit_owned_turn<T>(&self,output:&mut Option<T>,cx:&mut StepContext<\'_>,epoch:u64,make:impl FnOnce(OriginalCallerSourceParent,&StepContext<\'_>,usize,u64,OriginalSourceTurn)->T)->Result<bool,OriginalTurnFailure>{')
rep(src,'*output=Some(crate::OriginalOwnedCallerTurn::from_admitted_source(loan.into_borrowed_parent(),cx,parent_identity,epoch,dispatch));','*output=Some(make(loan.into_borrowed_parent(),cx,parent_identity,epoch,dispatch));')
rep(src,' pub(crate) fn borrow_context_for_turn<',' pub fn borrow_context_for_turn<')
rep(src,' pub(crate) fn into_source_loan(',' pub fn into_source_loan(')
fac=J+'🦀️.rs'
rep(fac,'pub use semio_framework_job_context::*;\n','pub use semio_framework_job_context::*;\npub use semio_framework_job_park::*;\n')
facm=J+'📦️packages/🦀️rust/Cargo.toml'
rep(facm,'semio-framework-job-context = { workspace = true }\n','semio-framework-job-context = { workspace = true }\nsemio-framework-job-park = { workspace = true }\n')
cons=F+'🔨️modules/🎭️actor/🎟️retained-turn/💼️job/⏸️park/🦀️.rs'
rep(cons,'use semio_framework_job::{OriginalCallerSourceParent,OriginalOwnedCallerTurn,','use semio_framework_job::{OriginalCallerSourceParent,OriginalParkExt,OriginalOwnedCallerTurn,')
fw=F+'Cargo.toml'
rep(fw,'    "🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/📦️packages/🦀️rust",\n','    "🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/📦️packages/🦀️rust",\n    "🔨️modules/🧵️job/⏱️context/⏸️park/📦️packages/🦀️rust",\n')
rep(fw,'    [workspace.dependencies.semio-framework-job-testing-caller]\n','    [workspace.dependencies.semio-framework-job-park]\n    path = "🔨️modules/🧵️job/⏱️context/⏸️park/📦️packages/🦀️rust"\n\n    [workspace.dependencies.semio-framework-job-testing-caller]\n')
new=C+'⏸️park/'
assert not os.path.exists(new+'📦️packages'),'already swapped'
# all preconditions held: write everything
if os.environ.get('DRY'):
    print('dry run ok',len(edits),'files');sys.exit(0)
os.makedirs(new+'📦️packages/🦀️rust')
shutil.copy(STAGE+'/Cargo.toml',new+'📦️packages/🦀️rust/Cargo.toml')
shutil.copy(STAGE+'/project.json',new+'📦️packages/🦀️rust/📋️project.json')
shutil.copy(STAGE+'/park.rs',new+'🦀️.rs')
for p,t in edits.items(): open(p,'w',encoding='utf8').write(t)
print('swapped',len(edits),'files')
