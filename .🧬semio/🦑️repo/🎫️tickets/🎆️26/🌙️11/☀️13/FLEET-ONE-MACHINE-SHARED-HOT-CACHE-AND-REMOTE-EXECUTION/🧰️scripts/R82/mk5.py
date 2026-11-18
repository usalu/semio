import sys,json;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
OBS=B+"👁️observation/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/"
s=Stage(SCR+"stage5")
s.edit(OBS+"🟨️.mjs",[
('/** 📤️ With `deferOffer` a record that is found stored here','''/** ⏱️ Where the lookups of stored project records spend their time in this process (milliseconds summed over the looks, concurrent looks count each their own wall time): reading the record, settling its closure rows, probing its rows, the reuse predicate, the offer to the fleet and the gate that fetches a missing record. */
const lookupClockV1={count:0,read:0,settle:0,hold:0,reusable:0,offer:0,gate:0};
function lookupLapV1(key,work){const at=performance.now();try{return work();}finally{lookupClockV1[key]+=performance.now()-at;}}
async function lookupLapAsyncV1(key,work){const at=performance.now();try{return await work();}finally{lookupClockV1[key]+=performance.now()-at;}}
/** ⏱️ The lookup clock of this process, in whole milliseconds. */
export function observationLookupClockV1(){return Object.fromEntries(Object.entries(lookupClockV1).map(([key,value])=>[key,Math.round(value)]));}
/** 📤️ With `deferOffer` a record that is found stored here''',1),
('  let candidate;\n  const record=readInferenceRecordOrMissV1(cachePath);','  let candidate;lookupClockV1.count++;\n  const record=lookupLapV1("read",()=>readInferenceRecordOrMissV1(cachePath));',1),
('await settleClosureRowsV1(header.observations,record.value);','await lookupLapAsyncV1("settle",()=>settleClosureRowsV1(header.observations,record.value));',2),
('if(nestedProject?sharedObservationsHoldV1(header.observations,parent):current(header.observations))candidate=','if(lookupLapV1("hold",()=>nestedProject?sharedObservationsHoldV1(header.observations,parent):current(header.observations)))candidate=',1),
('if(candidate&&await inferenceReusableV1(reusable,candidate.value)&&(nestedProject?sharedObservationsHoldV1(candidate.header.observations,parent):current(candidate.header.observations)))','if(candidate&&await lookupLapAsyncV1("reusable",()=>inferenceReusableV1(reusable,candidate.value))&&lookupLapV1("hold",()=>nestedProject?sharedObservationsHoldV1(candidate.header.observations,parent):current(candidate.header.observations)))',1),
('if(reused&&port&&!deferOffer)await port.offer(info);','if(reused&&port&&!deferOffer)await lookupLapAsyncV1("offer",()=>port.offer(info));',1),
('gate=await port.gate(info).catch(()=>undefined);','gate=await lookupLapAsyncV1("gate",()=>port.gate(info)).catch(()=>undefined);',1),
])
s.edit(OBS+"🟨️.d.mts",[('export function observationLayerHoldsV1(layer:readonly unknown[]):boolean;','export function observationLayerHoldsV1(layer:readonly unknown[]):boolean;\n/** ⏱️ Where the lookups of stored project records spent their milliseconds in this process. */\nexport function observationLookupClockV1():Record<string,number>;',1)])
s.edit(PN+"🟨️.mjs",[
('releaseProjectInferenceRetentionV1, useProjectFleetV1 }','releaseProjectInferenceRetentionV1, observationLookupClockV1, useProjectFleetV1 }',1),
('phases:Object.fromEntries(Object.entries(lookPhases).map(([key,value])=>[key,Math.round(value)]))}','phases:Object.fromEntries([...Object.entries(lookPhases).map(([key,value])=>[key,Math.round(value)]),...Object.entries(observationLookupClockV1()).map(([key,value])=>["lookup."+key,value])])}',1),
])
test='''/** ⏱️ Contract: every lookup of a stored project record is counted and its time is split into the steps of the lookup, so a slow take shows which step it waits in. @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../🟨️.mjs") as any;

test("a found record is counted and its read and row probes are timed",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"lookup-clock-")),file=join(root,"input.txt"),cachePath=join(root,"np","record.json");
 try{
  writeFileSync(file,"one");await new Promise(done=>setTimeout(done,2200));
  const compute=async()=>({text:observation.observedFs.readFileSync(file,"utf8")}),before=observation.observationLookupClockV1();
  await observation.observeInference({cachePath,key:["clock"]},compute);
  const middle=observation.observationLookupClockV1();
  await observation.observeInference({cachePath,key:["clock"]},compute);
  const after=observation.observationLookupClockV1();
  expect(middle.count-before.count).toBe(1);expect(after.count-middle.count).toBe(1);
  for(const key of["read","settle","hold","reusable","offer","gate"])expect(after[key]).toBeGreaterThanOrEqual(middle[key]);
  expect(after.read+after.hold).toBeGreaterThan(before.read+before.hold-1);
 }finally{rmSync(root,{recursive:true,force:true});}
});
'''
s.new(OBS+"🧪️tests/⏱️lookup-clock/🟦️.ts",test)
s.finish([OBS+"🧪️tests/⏱️lookup-clock/🟦️.ts",OBS+"🧪️tests/🧾️record-header/🟦️.ts",OBS+"🧪️tests/🧠️check-memo/🟦️.ts",OBS+"🧪️tests/🔂️validation-round/🟦️.ts",PN+"🧪️tests/📥️take/🟦️.ts"],"- Step 4 (R82 diagnostic): the lookup of a stored project record is timed per step (`observationLookupClockV1`: read, settle, hold, reusable, offer, gate) and the worker report shows it in `owners.looks.phases` as `lookup.*`, next to the look phases authority, observe, refs, rows.")
open(s.S+"/before.txt","w",encoding="utf8").write(OBS+"🧪️tests/⏱️lookup-clock/🟦️.ts\n")
