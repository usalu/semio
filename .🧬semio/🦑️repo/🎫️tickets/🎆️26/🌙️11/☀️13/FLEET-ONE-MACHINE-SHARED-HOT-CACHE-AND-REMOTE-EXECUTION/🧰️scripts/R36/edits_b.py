"""R36b: the sampler also speaks while the main thread never returns to the event loop: hot paths of the inference pulse it (rate limited), and a starved loop yields a row with the stack and the counts."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
FP=B+"🧠️memory/🦶️footprint/"
FM=FP+"🟨️.mjs"
add(FM,'import{appendFileSync,mkdirSync}from"node:fs";\n','import{appendFileSync,mkdirSync}from"node:fs";\nimport{createRequire}from"node:module";\n')
add(FM,'export function createHeapSamplerV1({meter,stats=async()=>(await import("bun:jsc")).heapStats(),usage=','export function createHeapSamplerV1({meter,stats=async()=>(await import("bun:jsc")).heapStats(),statsSync=()=>createRequire(import.meta.url)("bun:jsc").heapStats(),stack=()=>String(new Error().stack).split("\\n").slice(3,3+P.pulseFrames).map(line=>line.trim().replace(/^at /u,"").slice(-110)).join(" < "),starved=fields=>fields,usage=')
add(FM,' return Object.freeze({async tick(){\n  const at=now(),lag=Math.max(0,at-last-policy.sampleMilliseconds);last=at;',''' return Object.freeze({starved(at){
  if(flight||at-last<policy.pulseStarvedMilliseconds)return;
  flight=true;
  try{
   const memory=usage(),row=statsSync(),types=Object.entries(row.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,policy.sampleTypes).map(([name,count])=>name+" "+count).join(", ")||"-";
   emit({phase:phase()??"-",blocked:at-last,rss:megabytes(memory.rss),heap:megabytes(row.heapSize??0),extra:megabytes(row.extraMemorySize??0),objects:row.objectCount??0,stack:stack(),types},"starved");
  }catch{}finally{flight=false;}
 },async tick(){
  const at=now(),lag=Math.max(0,at-last-policy.sampleMilliseconds);last=at;''')
add(FM,'emit:fields=>{for(const line of renderFootprintMessageV1("sampled",fields,environment))sample(line);}});','emit:(fields,key="sampled")=>{for(const line of renderFootprintMessageV1(key,fields,environment))sample(line);}});\n    pulser=at=>sampler.starved(at);')
add(FM,'beatAt=-Infinity,timer,sampleTimer;','beatAt=-Infinity,timer,sampleTimer;')
add(FM,'if(sampleTimer!==undefined){clearTimer(sampleTimer);sampleTimer=undefined;}}','if(sampleTimer!==undefined){clearTimer(sampleTimer);sampleTimer=undefined;pulser=undefined;}}')
add(FM,'let installed,step;\n','''let installed,step,pulser,pulses=0,pulsedAt=0;
/** 💓️ Called from the hot paths of the inference (one call per row, per interned value, per yield): every 256th call looks at the clock and, at most every `pulseMilliseconds`, lets the sampler speak when the event loop has been starved for `pulseStarvedMilliseconds` (a main thread that never returns to the loop cannot run a timer, so the growth would stay invisible exactly while it happens). */
export function footprintPulseV1(){
 if(pulser===undefined||(++pulses&255)!==0)return;
 const at=Date.now();
 if(at-pulsedAt<P.pulseMilliseconds)return;
 pulsedAt=at;pulser(at);
}

''')
FD=FP+"🟨️.d.mts"
add(FD,'readonly sampleLagMilliseconds:number;','readonly sampleLagMilliseconds:number;readonly pulseMilliseconds:number;readonly pulseStarvedMilliseconds:number;readonly pulseFrames:number;')
add(FD,'FootprintMessageKeyV1="waveSettled"|"relieved"|"slowed"|"census"|"sampled";','FootprintMessageKeyV1="waveSettled"|"relieved"|"slowed"|"census"|"sampled"|"starved";')
add(FD,'emit:(fields:Readonly<Record<string,string|number>>)=>void;policy?:FootprintPolicyV1}):Readonly<{tick():Promise<void>}>;','statsSync?:()=>unknown;stack?:()=>string;emit:(fields:Readonly<Record<string,string|number>>,key?:"sampled"|"starved")=>void;policy?:FootprintPolicyV1}):Readonly<{tick():Promise<void>;starved(at:number):void}>;\nexport function footprintPulseV1():void;')
FS=FP+"🧬️schema/🔣️.json"
add(FS,'"sampleLagMilliseconds": 1000,','"sampleLagMilliseconds": 1000,\n        "pulseMilliseconds": 3000,\n        "pulseStarvedMilliseconds": 1500,\n        "pulseFrames": 14,')
add(FS,'commandTimeoutMilliseconds of one reading.','pulseMilliseconds between two pulse looks of hot inference code, which write a starved row (the stack of pulseFrames frames and the runtime counts) when the event loop has not run for pulseStarvedMilliseconds; commandTimeoutMilliseconds of one reading.')
add(FS,'commandTimeoutMilliseconds einer Messung.','pulseMilliseconds zwischen zwei Pulsblicken heißen Inferenzcodes, die eine Hungerzeile (Stapel mit pulseFrames Rahmen und die Zählungen der Laufzeit) schreiben, wenn die Ereignisschleife pulseStarvedMilliseconds nicht lief; commandTimeoutMilliseconds einer Messung.')
add(FS,'"slowed": "graph: the process holds','"starved": "graph: starved sample step {phase}: blocked {blocked} ms, resident {rss} MB, managed {heap} MB, external {extra} MB, {objects} objects; stack {stack}; most numerous: {types}",\n          "slowed": "graph: the process holds')
add(FS,'"slowed": "Graph: der Prozess hält','"starved": "Graph: Hungerprobe Schritt {phase}: blockiert {blocked} ms, resident {rss} MB, verwaltet {heap} MB, extern {extra} MB, {objects} Objekte; Stapel {stack}; am zahlreichsten: {types}",\n          "slowed": "Graph: der Prozess hält')
M=B+"🧠️memory/🟨️.mjs"
add(M,'import {footprintGateV1} from "./🦶️footprint/🟨️.mjs";','import {footprintGateV1,footprintPulseV1} from "./🦶️footprint/🟨️.mjs";')
add(M,'export function internJsonStringsV1(value,interner){\n const pending=[value];','export function internJsonStringsV1(value,interner){\n footprintPulseV1();\n const pending=[value];')
O=B+"👁️observation/🟨️.mjs"
add(O,'import { GRAPH_MEMORY_POLICY_V1, graphStringInternerV1, internJsonStringsV1 } from "../🧠️memory/🟨️.mjs";\n','import { GRAPH_MEMORY_POLICY_V1, graphStringInternerV1, internJsonStringsV1 } from "../🧠️memory/🟨️.mjs";\nimport { footprintPulseV1 } from "../🧠️memory/🦶️footprint/🟨️.mjs";\n')
add(O,'function recordRowV1(row){\n','function recordRowV1(row){\n footprintPulseV1();\n')
add(O,'function record(kind, path, fact) {\n','function record(kind, path, fact) {\n  footprintPulseV1();\n')
SI=B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
add(SI,'export async function yieldInferenceV1(context,phase,index,force=false){\n throwIfOwnerExceededV1();','export async function yieldInferenceV1(context,phase,index,force=false){\n footprintPulseV1();\n throwIfOwnerExceededV1();')
TF=FP+"🧪️tests/🟦️.ts"
add(TF,'import{bindProcessGateV1,createHeapSamplerV1,','import{footprintPulseV1,bindProcessGateV1,createHeapSamplerV1,')
_t=r'''
test("a starved loop makes the sampler speak from hot code: stack, counts and the blocked time, rate limited, and nothing while the loop runs",()=>{
 const time=clock(),rows:{key?:string;fields:Record<string,string|number>}[]=[];let tick:(()=>unknown)|undefined,now=1_000_000;
 const realNow=Date.now;
 const meter=createFootprintMeterV1({read:async()=>100*MB,ttlMilliseconds:0});
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,sample:line=>rows.push({fields:{line}}),phase:()=>"owners 0 of 5102",now:()=>now,environment:{SEMIO_LANGUAGE:"en,de"},setTimer:(callback:()=>unknown)=>{tick=callback;return{unref:()=>undefined};},clearTimer:()=>undefined});
 try{
  gate.watch();expect(typeof tick).toBe("function");
  Date.now=()=>now;
  for(let index=0;index<1024;index++)footprintPulseV1();
  expect(rows.length).toBe(0);
  now+=FOOTPRINT_POLICY_V1.pulseMilliseconds+1;
  for(let index=0;index<256;index++)footprintPulseV1();
  expect(rows.length).toBe(2);
  expect(rows[0]!.fields.line).toContain("graph: starved sample step owners 0 of 5102: blocked ");expect(rows[0]!.fields.line).toContain("stack ");expect(String(rows[1]!.fields.line).startsWith("Graph: Hungerprobe Schritt owners 0 of 5102")).toBe(true);
  for(let index=0;index<512;index++)footprintPulseV1();
  expect(rows.length).toBe(2);
  now+=FOOTPRINT_POLICY_V1.pulseMilliseconds;for(let index=0;index<256;index++)footprintPulseV1();
  expect(rows.length).toBe(4);
 }finally{Date.now=realNow;gate.stop();for(let index=0;index<512;index++)footprintPulseV1();expect(time.now()).toBe(0);}
});

test("the sampler starved row carries the real runtime counts and a stack of at most pulseFrames frames",()=>{
 const rows:Record<string,string|number>[]=[];let at=10_000;
 const sampler=createHeapSamplerV1({meter:{bytes:()=>1},now:()=>0,emit:fields=>rows.push(fields)});
 sampler.starved(at);expect(rows.length).toBe(1);
 expect(Number(rows[0]!.objects)).toBeGreaterThan(0);expect(Number(rows[0]!.blocked)).toBe(at);expect(String(rows[0]!.stack).split(" < ").length).toBeLessThanOrEqual(FOOTPRINT_POLICY_V1.pulseFrames);
 sampler.starved(0);expect(rows.length).toBe(1);
});
'''
add(TF,'test("the step label names the running graph phase beside the profile phase and is cut",()=>{','//R36b\n'+_t+'\ntest("the step label names the running graph phase beside the profile phase and is cut",()=>{')
add(SI,'import { creditOwnerSharedV1, throwIfOwnerExceededV1 } from "../../../🧠️memory/🎯️owner-scope/🟨️.mjs";\n','import { creditOwnerSharedV1, throwIfOwnerExceededV1 } from "../../../🧠️memory/🎯️owner-scope/🟨️.mjs";\nimport { footprintPulseV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";\n')
