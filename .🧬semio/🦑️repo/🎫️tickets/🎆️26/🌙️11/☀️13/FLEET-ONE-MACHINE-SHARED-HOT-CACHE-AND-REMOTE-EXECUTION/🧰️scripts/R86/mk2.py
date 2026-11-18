#!/usr/bin/env python3
"""R86 stage C: the slice worker reports the take counters and the plan stage as progress, so a stage that is taking owners is never silent. usage: mk2.py STAGE_DIR"""
import sys
S=sys.argv[1]
def sub(t,old,new):
    assert t.count(old)==1,(old[:70],t.count(old));return t.replace(old,new)
rd=lambda n:open(S+"/"+n,encoding="utf8").read()
def wr(n,t):open(S+"/"+n,"w",encoding="utf8").write(t)
t=rd("1.base")
t=sub(t,"/** 📍️ Notes the phase of the read",'''/** 📖️ What the source `name` answers now, or undefined while it is not registered or throws. */
export function readSliceReportSourceV1(name){const read=sources.get(name);if(read===undefined)return undefined;try{return read();}catch{return undefined;}}

/** 💓️ Tells `emit({phase:"taken",done,total})` every `milliseconds` while the owners source counts more owners decided (taken from the store or the fleet, or inferred) than the last time told: a worker that takes thousands of owners shows work to whoever watches its output, which a worker that says nothing for ten minutes never does. Returns the function that stops it; the timer never keeps the process alive.
 * @see ./🧬️schema/🔣️.json */
export function startSliceProgressV1({emit,read=readSliceReportSourceV1,milliseconds=P.progressMilliseconds}){
 let told;
 const timer=setInterval(()=>{const ledger=read("owners")?.ledger,done=Number(ledger?.done),total=Number(ledger?.total);if(!Number.isSafeInteger(done)||!Number.isSafeInteger(total)||total<=0||done===told)return;told=done;try{emit({phase:"taken",done,total});}catch{}},milliseconds);
 timer.unref?.();
 return()=>clearInterval(timer);
}

/** 📍️ Notes the phase of the read''')
wr("1.new",t)
t=rd("2.base")
t=sub(t,"topTypes:number;stopLines:number}>;","topTypes:number;stopLines:number;progressMilliseconds:number}>;")
t=sub(t,"export function registerSliceReportSourceV1(name:string,read:()=>unknown):()=>void;","export function registerSliceReportSourceV1(name:string,read:()=>unknown):()=>void;\nexport function readSliceReportSourceV1(name:string):unknown;\nexport function startSliceProgressV1(input:{emit:(event:{phase:string;done:number;total:number})=>void;read?:(name:string)=>unknown;milliseconds?:number}):()=>void;")
wr("2.new",t)
t=rd("3.base")
t=sub(t,'        "snapshotTopRetainers": 14\n','        "snapshotTopRetainers": 14,\n        "progressMilliseconds": 5000\n')
wr("3.new",t)
t=rd("4.base")
t=sub(t,'import{SLICE_REPORT_POLICY_V1 as P,summarizeHeapSnapshotV1,buildSliceReportV1,registerSliceReportSourceV1,writeSliceReportV1}from"../🟨️.mjs";','import{SLICE_REPORT_POLICY_V1 as P,summarizeHeapSnapshotV1,buildSliceReportV1,registerSliceReportSourceV1,startSliceProgressV1,writeSliceReportV1}from"../🟨️.mjs";')
t+='''
/** 💓️ A worker that takes owners tells the count that rose, once per count, and nothing while the source is absent or the count stands. */
test("the take counters are told as progress while they rise and not while they stand",async()=>{
 let ledger:{done:number;total:number}|undefined,throwing=false;const told:{phase:string;done:number;total:number}[]=[];
 const stop=startSliceProgressV1({emit:event=>told.push(event),read:()=>{if(throwing)throw new Error("nope");return ledger===undefined?undefined:{ledger};},milliseconds:10});
 try{
  await new Promise(resolve=>setTimeout(resolve,60));expect(told).toEqual([]);
  ledger={done:3,total:478};await new Promise(resolve=>setTimeout(resolve,60));
  ledger={done:9,total:478};await new Promise(resolve=>setTimeout(resolve,60));
  expect(told).toEqual([{phase:"taken",done:3,total:478},{phase:"taken",done:9,total:478}]);
  ledger={done:9,total:0};await new Promise(resolve=>setTimeout(resolve,40));expect(told.length).toBe(2);
 }finally{stop();}
 const after=told.length;ledger={done:11,total:478};await new Promise(resolve=>setTimeout(resolve,40));expect(told.length,"stopped").toBe(after);
 expect(P.progressMilliseconds).toBeGreaterThan(0);expect(P.progressMilliseconds).toBeLessThanOrEqual(10000);
});
'''
wr("4.new",t)
t=rd("5.base")
t=sub(t,'onGenerationProgressV1((event:{phase:string;done:number;total:number;running?:unknown;decision?:unknown})=>{report.noteSliceReportPhaseV1(event);parentPort!.postMessage({type:"progress",...event});});','onGenerationProgressV1((event:{phase:string;done:number;total:number;running?:unknown;decision?:unknown})=>{report.noteSliceReportPhaseV1(event);parentPort!.postMessage({type:"progress",...event});});\nreport.startSliceProgressV1({emit:(event:{phase:string;done:number;total:number})=>{report.noteSliceReportPhaseV1(event);parentPort!.postMessage({type:"progress",...event});}});')
t=sub(t,'onPlanStageV1(event=>{report.noteSliceReportStageV1(event);parentPort!.postMessage(event);});','onPlanStageV1(event=>{report.noteSliceReportStageV1(event);parentPort!.postMessage(event);const entered={phase:"stage "+String(event.stage),done:0,total:0};report.noteSliceReportPhaseV1(entered);parentPort!.postMessage({type:"progress",...entered});});')
wr("5.new",t)
print("staged C")
