#!/usr/bin/env python3
"""Idempotent patch for the retirement-queue stall in original_turn_tests.

Root cause (by reading; unverified by execution): FactoryOwnedRetirement::demands quotes the native
stage-0 setup step (1 byte, depth 1, 1 work) but the legacy close_step, which every owner without
resident_reply runs, never executes it and calls produce() directly, which needs the real producer
demand. Quote and execution disagree, so a 1-byte grant is admitted and then yields zero work.
Depth is consistent (queue adds len, owner adds its child level). Fix is on the execution side.

Edits: (1) factory close_step executes stage 0 (native_stage 0 -> 1, 1 work, quoted copy bytes);
(2) produce() resets native_stage to 0 on completion; (3) pinning test in the queue crate (the crate
that owns the effect bank), registered in the queue lib. Run from anywhere; re-running is a no-op.
"""
import pathlib, sys
R = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/♻️retirement")
OWNED = R / "🏭️factory/📦️owned/🦀️.rs"
QUEUE = R / "📋️queue/🦀️.rs"
TEST = R / "📋️queue/🧪️tests/🏭️legacy-owner.rs"

OLD1 = "if self.production.is_some()||self.unique.is_none()&&self.original.is_some(){return self.produce(effect)}let child="
NEW1 = ("if self.native_stage==0&&self.production.is_none()&&self.unique.is_none()&&self.original.is_some(){"
        "let work=match crate::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return done(Err(error.into()),work)};"
        "self.native_stage=1;return done(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work)}"
        + OLD1)
OLD2 = "*self.unique=source.output;crate::RetainedWorkPoll::Complete("
NEW2 = "*self.unique=source.output;self.native_stage=0;crate::RetainedWorkPoll::Complete("
MOD = '#[cfg(test)]\n#[path="🧪️tests/🏭️legacy-owner.rs"]\nmod factory_legacy_owner_tests;\n'
TESTSRC = '''//! 🏭️ A factory-owned frame without a resident reply executes exactly the step its quote names.
use crate::{observe_retirement_allocations,queue_bank::{bank,grant,poll}};
use semio_framework_value_retirement::{close_owner,retained_clone::RetainedCloneGrant};
use semio_framework_value_factory::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,OwnedValueRetirementFactory,owned::FactoryOwnedRetirement};
use std::sync::Arc;
#[test]
fn factory_legacy_close_executes_the_setup_step_it_quotes_and_terminates_on_exact_quotes(){
 let mut bank=bank();let mut original=Some(String::with_capacity(8192));let mut factory:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<String>>>=Some(Arc::new(OwnedValueRetirementFactory::<String>::default()));
 let admission=grant(FactoryOwnedRetirement::<String>::original_admission_demands(&original));let mut output=None;
 let((turn,_),heap)=observe_retirement_allocations(||poll(&mut bank,admission,1,|effect|FactoryOwnedRetirement::admit_original(&mut original,&mut factory,&mut output,effect)));let p=turn.result.unwrap().progress();bank.post(p,admission,heap);
 let mut frame=output.take().expect("admitted factory frame");
 let first=frame.next_demand(1).unwrap();assert_eq!((first.copy_bytes,first.depth),(1,1));
 let mut progressed=0;
 for _ in 0..64{
  if frame.terminal_is_empty(){break}
  let demand=frame.next_demand(1).unwrap();let units=frame.next_computational_work(1).unwrap();let exact:RetainedCloneGrant=grant(demand);
  let((turn,_),heap)=observe_retirement_allocations(||poll(&mut bank,exact,units,|effect|close_owner(frame.as_mut(),1,effect)));
  assert!(turn.work>0,"an exactly quoted grant must perform work: {demand:?}");let p=turn.result.unwrap().progress();assert!(p.copied_items>0||p.released_bytes>0);bank.post(p,exact,heap);progressed+=1;
 }
 assert!(frame.terminal_is_empty()&&progressed>1);
 drop(frame);
}
'''

def edit(path, old, new, marker):
    s = path.read_text(encoding="utf-8")
    if marker in s: return False
    if s.count(old) != 1: sys.exit(f"anchor not unique/found in {path}: {old[:60]}")
    path.write_text(s.replace(old, new), encoding="utf-8"); return True

a = edit(OWNED, OLD1, NEW1, "self.native_stage=1;return done(Ok(RetainedCloneStep::Progress(")
b = edit(OWNED, OLD2, NEW2, "*self.unique=source.output;self.native_stage=0;")
q = QUEUE.read_text(encoding="utf-8")
c = False
if "mod factory_legacy_owner_tests;" not in q:
    QUEUE.write_text(q.rstrip("\n") + "\n\n" + MOD, encoding="utf-8"); c = True
d = not TEST.exists() or TEST.read_text(encoding="utf-8") != TESTSRC
if d: TEST.write_text(TESTSRC, encoding="utf-8")
print("close_step:", a, "produce:", b, "queue mod:", c, "test file:", d)
