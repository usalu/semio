"""R36: the footprint gate is installed in the process that really grows (the sealed child of the granted CLI), a 2 s heap sampler writes the growth with its phase, the wave start is admitted by footprint. Anchored edits (rel path from the worktree root, old, new, count)."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))

FP=B+"🧠️memory/🦶️footprint/"
FM=FP+"🟨️.mjs"
add(FM,'import schema from"./🧬️schema/🔣️.json"with{type:"json"};\n','import schema from"./🧬️schema/🔣️.json"with{type:"json"};\nimport{readLiveHostCapacityReservationV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";\nimport{BUILD_HOST_CAPACITY_POLICY_V1}from"../../🌡️host-capacity/🎛️control/🟦️.ts";\nimport{profileCurrentPhaseV1}from"../../⏱️profile/🟨️.mjs";\n')
add(FM,'/** 📜️ The trace of this process:','''/** 🔭️ Samples the process while it grows: every `sampleMilliseconds` it looks at the resident set and the cached footprint (cheap) and, when either moved by `sampleGrowthBytes` since the last row, when `sampleQuietMilliseconds` passed or when the main thread was blocked for `sampleLagMilliseconds`, it counts the runtime heap (`bun:jsc` `heapStats`) and emits ONE row: the step it is in, footprint, resident set, managed heap, external memory, array buffers, objects, the loop lag and the `sampleTypes` most numerous object types, so a growth is attributed to objects, strings, buffers or memory outside the heap.
 * @param {{meter:{bytes:()=>number};stats?:()=>Promise<any>|any;usage?:()=>{rss:number;arrayBuffers?:number};now?:()=>number;phase?:()=>string|undefined;emit:(fields:Record<string,string|number>)=>void;policy?:typeof FOOTPRINT_POLICY_V1}} options */
export function createHeapSamplerV1({meter,stats=async()=>(await import("bun:jsc")).heapStats(),usage=()=>process.memoryUsage(),now=()=>Date.now(),phase=()=>"-",emit,policy=P}){
 let last=now(),wrote=-Infinity,rssAt=0,footprintAt=0,flight=false;
 return Object.freeze({async tick(){
  const at=now(),lag=Math.max(0,at-last-policy.sampleMilliseconds);last=at;
  if(flight)return;
  try{
   const memory=usage(),footprint=meter.bytes(),moved=Math.max(Math.abs(memory.rss-rssAt),Math.abs(footprint-footprintAt));
   if(moved<policy.sampleGrowthBytes&&at-wrote<policy.sampleQuietMilliseconds&&lag<policy.sampleLagMilliseconds)return;
   flight=true;
   const row=await stats(),types=Object.entries(row.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,policy.sampleTypes).map(([name,count])=>name+" "+count).join(", ")||"-";
   wrote=at;rssAt=memory.rss;footprintAt=footprint;
   emit({phase:phase()??"-",footprint:megabytes(footprint),moved:megabytes(moved),rss:megabytes(memory.rss),heap:megabytes(row.heapSize??0),extra:megabytes(row.extraMemorySize??0),buffers:megabytes(memory.arrayBuffers??0),objects:row.objectCount??0,lag,types});
  }catch{}finally{flight=false;}
 }});
}

/** 📜️ The trace of this process:''')
add(FM,'heartbeat,setTimer=setInterval,','heartbeat,sample,phase=()=>"-",setTimer=setInterval,')
add(FM,'beatAt=-Infinity,timer;','beatAt=-Infinity,timer,sampleTimer;')
add(FM,'   timer.unref?.();\n  },\n  stop(){if(timer!==undefined){clearTimer(timer);timer=undefined;}},','''   timer.unref?.();
   if(sample!==undefined&&sampleTimer===undefined){
    const sampler=createHeapSamplerV1({meter,now,phase,policy,emit:fields=>{for(const line of renderFootprintMessageV1("sampled",fields,environment))sample(line);}});
    sampleTimer=setTimer(()=>{void sampler.tick();},policy.sampleMilliseconds);sampleTimer.unref?.();
   }
  },
  stop(){if(timer!==undefined){clearTimer(timer);timer=undefined;}if(sampleTimer!==undefined){clearTimer(sampleTimer);sampleTimer=undefined;}},''')
add(FM,'''export async function settleWaveMemoryV1({generation,release,''','''export async function settleWaveMemoryV1({generation,release,''')
add(FM,' try{\n  const before=await meter.refresh();\n  await release?.();collect();',' try{\n  useEnvironmentGateV1(environment);\n  const before=await meter.refresh();\n  await release?.();collect();')
add(FM,'let installed;\n','''let installed,step;
/** 🧭️ Names the step the process is in (the resource of the running graph phase), so the heartbeat and the sampler can say what runs while the memory moves; a label is cut to 160 characters. */
export function noteFootprintStepV1(label){step=typeof label==="string"?label.slice(0,160):undefined;}

/** 🧭️ The step noted last, else the profile phase, `-` when neither is known. */
export function footprintStepV1(){return[profileCurrentPhaseV1(),step].filter(Boolean).join(" | ")||"-";}

''')
open_marker='/** 🌊️ Settles the memory of a process after a published wave'
add(FM,open_marker,'''/** 🔗️ Installs the gate of THIS process for a grant source: the process gets its trace file (`<tmp>/semio-footprint/<pid>.log`), the heartbeat and the sampler rows with the step it is in, and the lines of the gate also go to the read's stderr. @param {(() => unknown) | undefined} grant */
export function bindProcessGateV1(grant,{trace=createFootprintTraceV1(),write=line=>{try{process.stderr.write(line+"\\n");}catch{}},create=createFootprintGateV1}={}){
 const gate=grant===undefined?undefined:create({grantBytes:()=>grantBytesOfV1(grant),report:line=>{write(line);trace(line);},sample:trace,phase:footprintStepV1,heartbeat:row=>trace("heartbeat phase="+footprintStepV1()+" footprint="+megabytes(row.footprintBytes)+" MB grant="+megabytes(row.grantBytes)+" MB rss="+megabytes(row.rssBytes)+" MB")});
 if(gate!==undefined)trace("gate installed pid="+process.pid+" grant="+megabytes(gate.grantBytes()??0)+" MB");
 gate?.watch();useFootprintGateV1(gate);return gate;
}

/** 🧬️ Installs the gate of a process that holds no grant object but runs inside a granted run: the sealed child of the granted CLI, where the graph is really read, inherits only the grant id; its reserved megabytes are read live (cached `grantTtlMilliseconds`) from the scheduler, so a grown reservation counts. Does nothing when a gate is installed already, when the process runs ungranted or when the reservation cannot be read; answers the gate. */
export function useEnvironmentGateV1(environment=process.env,{read=()=>readLiveHostCapacityReservationV1(environment,BUILD_HOST_CAPACITY_POLICY_V1)?.reservedMegabytes,bind=bindProcessGateV1,now=()=>Date.now()}={}){
 if(installed!==undefined||!environment.SEMIO_HOST_CAPACITY_GRANT)return installed;
 let at=-Infinity,reserved=0;
 const source=()=>{if(now()-at>=P.grantTtlMilliseconds){at=now();try{const value=read();if(value>0)reserved=value;}catch{}}return reserved>0?{handle:{reservedMegabytes:reserved}}:undefined;};
 return source()===undefined?undefined:bind(source);
}

'''+open_marker)

FD=FP+"🟨️.d.mts"
add(FD,'readonly heartbeatMilliseconds:number;','readonly heartbeatMilliseconds:number;readonly grantTtlMilliseconds:number;readonly sampleMilliseconds:number;readonly sampleTypes:number;readonly sampleGrowthBytes:number;readonly sampleQuietMilliseconds:number;readonly sampleLagMilliseconds:number;')
add(FD,'FootprintMessageKeyV1="waveSettled"|"relieved"|"slowed"|"census";','FootprintMessageKeyV1="waveSettled"|"relieved"|"slowed"|"census"|"sampled";')
add(FD,'heartbeat?:(row:{footprintBytes:number;grantBytes:number;rssBytes:number})=>void;','heartbeat?:(row:{footprintBytes:number;grantBytes:number;rssBytes:number})=>void;sample?:(line:string)=>void;phase?:()=>string|undefined;')
add(FD,'export function useFootprintGateV1(','''export function createHeapSamplerV1(options:{meter:{bytes:()=>number};stats?:()=>unknown;usage?:()=>{rss:number;arrayBuffers?:number};now?:()=>number;phase?:()=>string|undefined;emit:(fields:Readonly<Record<string,string|number>>)=>void;policy?:FootprintPolicyV1}):Readonly<{tick():Promise<void>}>;
export function noteFootprintStepV1(label:string|undefined):void;
export function footprintStepV1():string;
export function bindProcessGateV1(grant:(()=>unknown)|undefined,ports?:{trace?:(line:string)=>void;write?:(line:string)=>void;create?:typeof createFootprintGateV1}):FootprintGateV1|undefined;
export function useEnvironmentGateV1(environment?:Readonly<Record<string,string|undefined>>,ports?:{read?:()=>number|undefined;bind?:(grant:()=>unknown)=>FootprintGateV1|undefined;now?:()=>number}):FootprintGateV1|undefined;
export function useFootprintGateV1(''')

FS=FP+"🧬️schema/🔣️.json"
add(FS,'"traceBytes": 2097152,','"traceBytes": 8388608,')
add(FS,'"heartbeatMilliseconds": 30000,','"heartbeatMilliseconds": 30000,\n        "grantTtlMilliseconds": 10000,\n        "sampleMilliseconds": 2000,\n        "sampleTypes": 15,\n        "sampleGrowthBytes": 268435456,\n        "sampleQuietMilliseconds": 20000,\n        "sampleLagMilliseconds": 1000,')
add(FS,'heartbeatMilliseconds between two heartbeat lines of the watcher; commandTimeoutMilliseconds of one reading.','heartbeatMilliseconds between two heartbeat lines of the watcher; grantTtlMilliseconds during which a reservation read from the scheduler is reused; sampleMilliseconds between two looks of the heap sampler, which writes a row when the resident set or the footprint moved by sampleGrowthBytes, after sampleQuietMilliseconds without a row or when the main thread was blocked for sampleLagMilliseconds, naming the sampleTypes most numerous object types; commandTimeoutMilliseconds of one reading.')
add(FS,'heartbeatMilliseconds zwischen zwei Herzschlagzeilen des Wächters; commandTimeoutMilliseconds einer Messung.','heartbeatMilliseconds zwischen zwei Herzschlagzeilen des Wächters; grantTtlMilliseconds, während der eine vom Planer gelesene Reservierung wiederverwendet wird; sampleMilliseconds zwischen zwei Blicken des Heap-Probennehmers, der eine Zeile schreibt, wenn der residente Satz oder der Fußabdruck sich um sampleGrowthBytes bewegt hat, nach sampleQuietMilliseconds ohne Zeile oder wenn der Hauptfaden sampleLagMilliseconds blockiert war, und die sampleTypes zahlreichsten Objekttypen nennt; commandTimeoutMilliseconds einer Messung.')
add(FS,'"slowed": "graph: the process holds','"sampled": "graph: sample step {phase}: footprint {footprint} MB (moved {moved} MB), resident {rss} MB, managed {heap} MB, external {extra} MB, buffers {buffers} MB, {objects} objects, loop lag {lag} ms; most numerous: {types}",\n          "slowed": "graph: the process holds')
add(FS,'"slowed": "Graph: der Prozess hält','"sampled": "Graph: Probe Schritt {phase}: Fußabdruck {footprint} MB (bewegt {moved} MB), resident {rss} MB, verwaltet {heap} MB, extern {extra} MB, Puffer {buffers} MB, {objects} Objekte, Schleifenverzug {lag} ms; am zahlreichsten: {types}",\n          "slowed": "Graph: der Prozess hält')

CK=B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs"
import os
_ck=open("/Users/Shared/semio/"+CK,encoding="utf8").read().split("\n")
_old60=next(l for l in _ck if l.startswith("export function useCapacityGrantV1(grant){grantOfProcess=grant;const trace="))
_old15=next(l for l in _ck if l.startswith("import{createFootprintGateV1,createFootprintTraceV1,footprintGateV1,"))
add(CK,_old60,'export function useCapacityGrantV1(grant){grantOfProcess=grant;bindProcessGateV1(grant);}')
add(CK,_old15,'import{bindProcessGateV1,footprintGateV1,releaseRuntimeMemoryV1,useEnvironmentGateV1}from"../../../🧠️memory/🦶️footprint/🟨️.mjs";')
add(CK,'import{profileCurrentPhaseV1}from"../../../⏱️profile/🟨️.mjs";\n','')
add(CK,' let scope,relief,chain=Promise.resolve();\n',' let scope,relief,chain=Promise.resolve();\n useEnvironmentGateV1();\n')

PH=B+"🕸️graph/⏱️phases/🟨️.mjs"
add(PH,'import {refuseBuildV1} from "../../📖️texts/🟦️.ts";\n','import {refuseBuildV1} from "../../📖️texts/🟦️.ts";\nimport {noteFootprintStepV1} from "../../🧠️memory/🦶️footprint/🟨️.mjs";\n')
add(PH,'const emit=(phase,state,done)=>{state.lastAt=now();state.lastDone=done;state.lastResource=resourceOf(phase,state,done);','const emit=(phase,state,done)=>{state.lastAt=now();state.lastDone=done;state.lastResource=resourceOf(phase,state,done);noteFootprintStepV1(state.lastResource);')

PN=B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
add(PN,'  const memoryPorts=createMemoryPortsV1(),parentBytes=','  useEnvironmentGateV1();\n  const memoryPorts=createMemoryPortsV1(),parentBytes=')
add(PN,'import { planFarmWorkersV1 } from','import { useEnvironmentGateV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";\nimport { planFarmWorkersV1 } from')

TF=FP+"🧪️tests/🟦️.ts"
add(TF,'import{readFileSync}from"node:fs";\n','import{readFileSync}from"node:fs";\nimport{bindProcessGateV1,createHeapSamplerV1,footprintStepV1,noteFootprintStepV1,useEnvironmentGateV1}from"../🟨️.mjs";\n')
_tests=r'''
test("the sampler writes a row on the first look, on growth, on a blocked loop and after the quiet interval, with the 15 most numerous types, and nothing in between",async()=>{
 const P=FOOTPRINT_POLICY_V1,time=clock(),rows:Record<string,string|number>[]=[];let rss=500*MB,footprint=400*MB;
 const counts=Object.fromEntries(Array.from({length:20},(_,index)=>["T"+index,1000-index]));
 const sampler=createHeapSamplerV1({meter:{bytes:()=>footprint},stats:async()=>({heapSize:300*MB,extraMemorySize:50*MB,objectCount:777,objectTypeCounts:counts}),usage:()=>({rss,arrayBuffers:3*MB}),now:time.now,phase:()=>"step-a",emit:fields=>rows.push(fields)});
 await sampler.tick();expect(rows.length).toBe(1);
 expect(rows[0]).toMatchObject({phase:"step-a",footprint:400,rss:500,heap:300,extra:50,buffers:3,objects:777,lag:0});
 expect(String(rows[0]!.types).split(", ").length).toBe(P.sampleTypes);expect(String(rows[0]!.types).startsWith("T0 1000, T1 999")).toBe(true);
 time.advance(P.sampleMilliseconds);await sampler.tick();expect(rows.length).toBe(1);
 rss+=P.sampleGrowthBytes;time.advance(P.sampleMilliseconds);await sampler.tick();expect(rows.length).toBe(2);expect(rows[1]!.moved).toBe(P.sampleGrowthBytes/MB);
 rss-=P.sampleGrowthBytes;time.advance(P.sampleMilliseconds);await sampler.tick();expect(rows.length).toBe(3);
 time.advance(P.sampleMilliseconds+P.sampleLagMilliseconds);await sampler.tick();expect(rows.length).toBe(4);expect(rows[3]!.lag).toBe(P.sampleLagMilliseconds);
 time.advance(P.sampleQuietMilliseconds);await sampler.tick();expect(rows.length).toBe(5);
});

test("the sampler survives a runtime without counters and never overlaps two countings",async()=>{
 const time=clock(),rows:unknown[]=[];let release:(()=>void)|undefined,calls=0;
 const failing=createHeapSamplerV1({meter:{bytes:()=>1},stats:async()=>{throw Error("none");},usage:()=>({rss:1}),now:time.now,emit:fields=>rows.push(fields)});
 await expect(failing.tick()).resolves.toBeUndefined();expect(rows.length).toBe(0);
 const slow=createHeapSamplerV1({meter:{bytes:()=>1},stats:()=>{calls++;return new Promise(resolve=>{release=()=>resolve({heapSize:1});});},usage:()=>({rss:1}),now:time.now,emit:fields=>rows.push(fields)});
 const first=slow.tick();await slow.tick();expect(calls).toBe(1);release!();await first;expect(rows.length).toBe(1);
});

test("the real runtime counters of the sampler equal the managed heap the runtime reports (independent reading)",async()=>{
 const rows:Record<string,string|number>[]=[],keep=Array.from({length:50000},(_,index)=>({index,text:"s"+index}));
 const sampler=createHeapSamplerV1({meter:{bytes:()=>process.memoryUsage().rss},emit:fields=>rows.push(fields)});
 await sampler.tick();const usage=process.memoryUsage();
 expect(rows.length).toBe(1);expect(Math.abs(Number(rows[0]!.heap)-usage.heapUsed/MB)).toBeLessThan(64);expect(String(rows[0]!.types)).toContain("Object ");expect(keep.length).toBe(50000);
});

test("the gate with a sample port runs a second timer for the sampler, in both languages with the step, and stops both timers",async()=>{
 const lines:string[]=[],callbacks:(()=>unknown)[]=[];let cleared=0;
 const meter=createFootprintMeterV1({read:async()=>100*MB,ttlMilliseconds:0});await meter.refresh();
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,sample:line=>lines.push(line),phase:()=>"inference 7 of 4774",environment:{SEMIO_LANGUAGE:"en,de"},setTimer:(callback:()=>unknown)=>{callbacks.push(callback);return{unref:()=>undefined};},clearTimer:()=>{cleared++;}});
 gate.watch();gate.watch();expect(callbacks.length).toBe(2);
 await callbacks[1]!();await new Promise(resolve=>setTimeout(resolve,50));
 expect(lines.length).toBe(2);expect(lines[0]!.startsWith("graph: sample step inference 7 of 4774: footprint 100 MB")).toBe(true);expect(lines[1]!.startsWith("Graph: Probe Schritt inference 7 of 4774: Fußabdruck 100 MB")).toBe(true);
 gate.stop();expect(cleared).toBe(2);
});

test("a gate without a sample port starts no sampler",()=>{
 const callbacks:unknown[]=[];const gate=createFootprintGateV1({meter:createFootprintMeterV1({read:async()=>1}),grantBytes:()=>1000*MB,setTimer:(callback:unknown)=>{callbacks.push(callback);return{unref:()=>undefined};},clearTimer:()=>undefined});
 gate.watch();expect(callbacks.length).toBe(1);gate.stop();
});

test("a process that holds no grant object but runs inside a granted run installs its gate from the reservation of the environment, once, and follows a grown reservation",()=>{
 let reserved:number|undefined=4096,at=0;const binds:(()=>unknown)[]=[];
 try{
  expect(useEnvironmentGateV1({},{read:()=>reserved})).toBeUndefined();
  const env={SEMIO_HOST_CAPACITY_GRANT:"abc"},ports={read:()=>reserved,now:()=>at,bind:(source:()=>unknown)=>{binds.push(source);return bindProcessGateV1(source,{trace:()=>undefined,write:()=>undefined});}};
  const gate=useEnvironmentGateV1(env,ports);
  expect(gate?.grantBytes()).toBe(4096*MB);expect(footprintGateV1()).toBe(gate);
  expect(useEnvironmentGateV1(env,ports)).toBe(gate);expect(binds.length).toBe(1);
  reserved=8192;expect(gate?.grantBytes()).toBe(4096*MB);at+=FOOTPRINT_POLICY_V1.grantTtlMilliseconds;expect(gate?.grantBytes()).toBe(8192*MB);
  useFootprintGateV1(undefined);reserved=undefined;at+=1;expect(useEnvironmentGateV1(env,{...ports,read:()=>{throw Error("none");}})).toBeUndefined();
  expect(useEnvironmentGateV1(env,{...ports,read:()=>undefined})).toBeUndefined();
 }finally{useFootprintGateV1(undefined);}
});

test("the step label names the running graph phase beside the profile phase and is cut",()=>{
 try{
  noteFootprintStepV1("build:graph-phase:owners:4:5102");expect(footprintStepV1().endsWith("build:graph-phase:owners:4:5102")).toBe(true);
  noteFootprintStepV1("x".repeat(500));expect(footprintStepV1().length).toBeLessThan(260);
 }finally{noteFootprintStepV1(undefined);}
});
'''
add(TF,'time.advance(FOOTPRINT_POLICY_V1.heartbeatMilliseconds);await tick!();expect(rows.length).toBe(2);gate.stop();\n});\n','time.advance(FOOTPRINT_POLICY_V1.heartbeatMilliseconds);await tick!();expect(rows.length).toBe(2);gate.stop();\n});\n'+_tests)
