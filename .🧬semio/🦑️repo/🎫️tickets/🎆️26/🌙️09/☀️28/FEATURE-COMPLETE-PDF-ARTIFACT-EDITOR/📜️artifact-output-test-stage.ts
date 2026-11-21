/** ♻️ Rebases the existing producer retirement tests onto the current Original effect caller. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const output=join(import.meta.dir,"🗑️generated","artifact-output-queue");
const overlay=join(output,"overlay.json");
const rows=JSON.parse(readFileSync(overlay,"utf8")) as {path:string;before:string|null;after:string}[];
const test=rows.find(row=>row.path.endsWith("📤️producer/🧪️tests/🦀️.rs"));
assert.ok(test);
const one=(s:string,old:string,next:string)=>{
  assert.equal(s.split(old).length,2,old);
  return s.replace(old,next);
};
const shift=(s:string,old:string,next:string,count:number)=>{
  assert.equal(s.split(old).length,count+1,old);
  return s.replaceAll(old,next);
};
const section=(s:string,a:string,b:string)=>{
  const start=s.indexOf(a),end=s.indexOf(b,start);
  assert.ok(start>=0&&end>start);
  return s.slice(start,end);
};
let source=one(test.after,"use semio_framework_job::{OriginalCallerSourceParent,OperationId,Generation,StepBudget,close_original_retained_step};\nuse semio_framework_job_payload::JobPayloadAuthority;","use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepBudget,close_original_effect_step};");
source=one(source,section(source,"fn parent()->OriginalCallerSourceParent","#[test]"),"fn parent(fuel: u64) -> OriginalJobFixtureCaller {\n    OriginalJobFixtureCaller::admit(OperationId(807),Generation(3),stock(),now_us,1500,fuel)\n}\n\nfn close<T: RetireOwned>(value: T, cx: &mut StepContext<'_>) {\n    let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!(\"original producer owners support controlled retirement\"));\n    let maximum=stock().maximum_copy_bytes;\n    for _ in 0..65536 {\n        if owner.terminal_is_empty() { return; }\n        let demand=owner.selected_demand(maximum).unwrap();\n        if demand.release_bytes!=0 {\n            let denied=RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..cx.retained_grant()};\n            let before=cx.retained_progress();\n            let fuel=cx.fuel_remaining();\n            let step=cx.with_retained_loan(denied,|same|close_original_effect_step(same,demand,|effect|owner.step(maximum,effect))).unwrap();\n            assert_eq!(step.progress(),Default::default());\n            assert_eq!(cx.retained_progress(),before);\n            assert_eq!(cx.fuel_remaining(),fuel);\n        }\n        let before=cx.retained_progress();\n        let step=close_original_effect_step(cx,demand,|effect|owner.step(maximum,effect)).unwrap();\n        assert_eq!(cx.retained_progress(),before.checked_add(step.progress()).unwrap());\n    }\n    panic!(\"finite original producer retirement\");\n}\n\n");
source=shift(source,"let mut parent=parent();",'let mut parent=parent(if row["grant"]=="fuel-zero"{0}else{32768});',2+1);
source=one(source,'let mut parent=parent(if row["grant"]=="fuel-zero"{0}else{32768});\n    let mut sequence=0;\n    let mut recipient=Default::default();\n    let mut cx=parent.borrow_context(StepBudget::new(65536,1500,stock()),now_us,&mut sequence,&mut recipient).unwrap();',"let mut parent=parent(65536);\n    let mut cx=parent.lend_context();");
source=shift(source,"parent.cancellation().unwrap().cancel_now()","parent.cancel().cancel_now()",2);
source=shift(source,'let mut sequence=0;let mut recipient=Default::default();let fuel=if row["grant"]=="fuel-zero"{0}else{32768};let mut cx=parent.borrow_context(StepBudget::new(fuel,1500,stock()),now_us,&mut sequence,&mut recipient).unwrap();',"let mut cx=parent.lend_context();",2);
source=shift(source,"let mut close_receipt=Default::default();let mut close_cx=parent.borrow_context(StepBudget::new(65536,1500,stock()),now_us,&mut sequence,&mut close_receipt).unwrap();","parent.fund_fuel(65536);let mut close_cx=parent.lend_context();",2);
const loops=/for turn in 0\.\.256\{if parent\.terminal_is_empty\(\)\{break\}[^\n]*?assert!\(turn<255\)\}assert!\(parent\.terminal_is_empty\(\)\);/g;
assert.equal(Array.from(source.matchAll(loops)).length,2);
source=source.replace(loops,"parent.retire(stock());");
source=one(source,"assert_eq!(after.released_bytes-before.released_bytes,paid+after.retained_capacity_bytes-before.retained_capacity_bytes);\n}","assert_eq!(after.released_bytes-before.released_bytes,paid+after.retained_capacity_bytes-before.retained_capacity_bytes);\n    drop(cx);\n    parent.retire(stock());\n}");
test.after=source;
const custody=test.path.replace("/🦀️.rs","/📥️custody/🦀️.rs");
const before=readFileSync(custody,"utf8");
let next=one(before,"use semio_framework_value::retained_clone::RetainedCloneClose;\n","");
const helper="fn retire<T:semio_framework_value::retirement::RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T){let mut cx=caller.lend_context();super::close(value,&mut cx);let remaining=cx.fuel_remaining();drop(cx);caller.settle_lent_fuel(remaining);}\n";
next=one(next,section(next,"fn retire<T:","#[test]"),helper);
next=shift(next,"ArtifactOutputChunks::original_producer_custody_demands()","ArtifactOutputChunks::original_push_demands()",2);
rows.push({path:custody,before:createHash("sha256").update(before).digest("hex"),after:next});
writeFileSync(overlay,JSON.stringify(rows,null,2)+"\n");
for(const [i,row] of rows.entries())writeFileSync(join(output,"source-"+i+".rs"),row.after);
console.log("[DEBUG] Rebased "+rows.length+" paged queue source and test paths");
