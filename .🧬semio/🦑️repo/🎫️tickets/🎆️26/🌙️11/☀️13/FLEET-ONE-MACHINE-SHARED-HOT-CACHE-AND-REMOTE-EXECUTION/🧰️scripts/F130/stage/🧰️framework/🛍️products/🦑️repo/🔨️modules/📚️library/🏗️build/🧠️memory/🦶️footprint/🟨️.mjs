import{platform as osPlatform,tmpdir}from"node:os";
import{appendFileSync,mkdirSync}from"node:fs";
import{createRequire}from"node:module";
import{join}from"node:path";
import{noticeLanguagesV1}from"../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import{runtimeCollectGarbageV1}from"../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import{observeTreeMegabytesV1,parseFootprintV1,parseSmapsRollupV1,readHostTextV1,runHostCommandV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import{systemWindowsProcessWorkingSetV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪟️native-memory/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import memorySchema from"../🧬️schema/🔣️.json"with{type:"json"};
import{readLiveHostCapacityReservationV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import{BUILD_HOST_CAPACITY_POLICY_V1}from"../../🌡️host-capacity/🎛️control/🟦️.ts";
import{profileCurrentPhaseV1}from"../../⏱️profile/🟨️.mjs";

const freeze=value=>{if(value!==null&&typeof value==="object"){for(const child of Object.values(value))freeze(child);Object.freeze(value);}return value;};

/** 🦶️ The declared numbers of the footprint gate: the schema file is the single source. */
export const FOOTPRINT_POLICY_V1=freeze(structuredClone(schema.definitions.Policy.const));

/** 🗣️ The bilingual lines of the footprint gate, read from the schema file. */
export const FOOTPRINT_MESSAGES_V1=freeze(structuredClone(schema.definitions.Messages.const));
const P=FOOTPRINT_POLICY_V1,MEGABYTE=1048576,megabytes=bytes=>Math.round(bytes/MEGABYTE);

/** 💬️ Renders one footprint line as one line per language of the environment. */
export function renderFootprintMessageV1(key,fields,environment=process.env){
 return[...noticeLanguagesV1(P.languages,environment)].map(language=>{
  const template=FOOTPRINT_MESSAGES_V1[language][key];
  if(template===undefined)throw Error("Closed footprint message required: "+key);
  return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
 });
}

/** ⚖️ What a footprint asks of the admission of new work: `run` below `softPercent` of the grant (or without a known grant), `settle` from there, `wait` from `hardPercent`. */
export function decideFootprintV1({footprintBytes,grantBytes,policy=P}){
 if(!(grantBytes>0)||!Number.isFinite(footprintBytes))return"run";
 const share=footprintBytes*100/grantBytes;
 return share>=policy.hardPercent?"wait":share>=policy.softPercent?"settle":"run";
}

/** 🚨️ True from `overPercent` of the grant (the hard over-grant state: a process that still admits work here only deepens the overshoot); false without a known grant. */
export function footprintOverGrantV1({footprintBytes,grantBytes,policy=P}){
 return grantBytes>0&&Number.isFinite(footprintBytes)&&footprintBytes*100>=grantBytes*policy.overPercent;
}

/** 📏️ The bytes of the grant that stay before `hardPercent` is reached (negative once passed), Infinity without a known grant. */
export function footprintHeadroomBytesV1({footprintBytes,grantBytes,policy=P}){
 return grantBytes>0&&Number.isFinite(footprintBytes)?Math.floor(grantBytes*policy.hardPercent/100-footprintBytes):Infinity;
}

/** 🐚️ Collects neighboring host footprint requests into one bounded Darwin probe. */
export function createDarwinFootprintProbeV1(run=runHostCommandV1){
 const pending=new Set();
 let timer,running=false;
 const schedule=()=>{
  if(!running&&timer===undefined&&pending.size)timer=setTimeout(flush,P.processProbeBatchMilliseconds);
 };
 async function flush(){
  timer=undefined;
  if(!pending.size)return;
  const batch=[...pending],active={controller:new AbortController(),remaining:batch.length};
  pending.clear();running=true;
  for(const row of batch)row.active=active;
  let measured;
  try{
   const pids=[...new Set(batch.map(row=>row.pid))];
   measured=parseFootprintV1(await run("footprint",["-f","bytes","--noCategories",...pids.flatMap(pid=>["-p",String(pid)])],active.controller.signal));
  }catch{}
  for(const row of batch)row.finish(measured?.get(row.pid));
  running=false;schedule();
 }
 return(pid,signal)=>{
  if(!Number.isInteger(pid)||pid<1||pid>0x7fffffff||signal?.aborted||pending.size>=P.processProbeBatchRequests)return Promise.resolve(undefined);
  return new Promise(resolve=>{
   const row={pid,active:undefined,done:false,finish(bytes,cancelled=false){
    if(row.done)return;
    row.done=true;signal?.removeEventListener("abort",cancel);
    pending.delete(row);
    if(row.active&&--row.active.remaining===0&&cancelled)row.active.controller.abort();
    resolve(bytes);
   }};
   const cancel=()=>row.finish(undefined,true);
   pending.add(row);signal?.addEventListener("abort",cancel,{once:true});schedule();
  });
 };
}

const darwinProcessFootprintV1=createDarwinFootprintProbeV1();

/** 🔬️ Reads one process's physical footprint in bytes: macOS phys_footprint, Linux Pss plus SwapPss, Windows working set and the reader's own resident set on other platforms. Unavailable readings are undefined. */
export async function readProcessFootprintBytesV1(pid,{platform=osPlatform(),run=runHostCommandV1,read=readHostTextV1,rss=()=>process.memoryUsage().rss,workingSet=systemWindowsProcessWorkingSetV1,signal}={}){
 if(signal?.aborted||!Number.isInteger(pid)||pid<1||pid>0xffffffff)return undefined;
 try{
  if(platform==="darwin"){
   if(pid!==process.pid&&run===runHostCommandV1)return await darwinProcessFootprintV1(pid,signal);
   const text=await run("footprint",["-f","bytes","--noCategories","-p",String(pid)],signal);
   return signal?.aborted?undefined:parseFootprintV1(text).get(pid);
  }
  if(platform==="linux"){
   const text=await read(`/proc/${pid}/smaps_rollup`),kilobytes=signal?.aborted||text===null?undefined:parseSmapsRollupV1(text);
   return kilobytes===undefined?undefined:kilobytes*1024;
  }
  const bytes=pid===process.pid?rss():platform==="win32"?workingSet(pid):undefined;
  return !signal?.aborted&&typeof bytes==="number"&&Number.isSafeInteger(bytes)&&bytes>=0?bytes:undefined;
 }catch{return undefined;}
}

/** 🪫️ The bytes the runtime of THIS process holds (heap in use plus external memory). */
export const managedBytesV1=()=>{const row=process.memoryUsage();return row.heapUsed+row.external;};

/** 🧱️ A footprint reading of THIS process never stands below what its own runtime holds: a host that compresses the pages of a busy worker read 623 MB for a worker that held 4447 MB, and a gate that trusted that reading admitted every owner and never released what they left behind until the plan guard stopped the plan. A reading that failed (undefined) stays a failure. */
export function footprintAtLeastManagedV1(bytes,managed=managedBytesV1){return Number.isFinite(bytes)?Math.max(bytes,managed()):bytes;}

/** 🪞️ A cached reading of the footprint of this process: `bytes()` answers at once with the last reading (the resident set until the first one arrives) and starts one background reading when the last is older than `ttlMilliseconds` (one reading at a time, a failed one is ignored); `refresh()` waits for a fresh reading. */
export function createFootprintMeterV1({pid=process.pid,read=async()=>{const bytes=await readProcessFootprintBytesV1(pid);return pid===process.pid?footprintAtLeastManagedV1(bytes):bytes;},rss=()=>process.memoryUsage().rss,now=()=>Date.now(),ttlMilliseconds=P.ttlMilliseconds}={}){
 let last=0,at=-Infinity,flight,peak=0,measured=false;
 const begin=()=>flight??=(async()=>{try{const bytes=await read();if(Number.isFinite(bytes)&&bytes>=0){last=bytes;measured=true;at=now();peak=Math.max(peak,bytes);}}catch{}finally{flight=undefined;}return last;})(),refresh=async()=>{if(flight!==undefined)await flight;return begin();};
 return Object.freeze({
  bytes(){if(now()-at>=ttlMilliseconds)void begin();return measured?last:rss();},
  refresh:async()=>{const bytes=await refresh();return measured?bytes:rss();},
  fresh:async()=>{if(measured&&now()-at<ttlMilliseconds)return last;const bytes=await refresh();return measured?bytes:rss();},
  measured:()=>measured,
  peakBytes:()=>peak,
 });
}

/** 🔢️ Counts what the runtime heap holds without walking it (`bun:jsc` `heapStats`): the managed and the external megabytes, the number of objects and the most numerous object types; undefined where the runtime has no such counters. */
export async function heapCensusV1({stats=async()=>(await import(["bun","jsc"].join(":"))).heapStats(),types=P.censusTypes}={}){
 try{
  const row=await stats(),counts=Object.entries(row.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,types).map(([name,count])=>name+" "+count);
  return Object.freeze({heapMegabytes:megabytes(row.heapSize??0),extraMegabytes:megabytes(row.extraMemorySize??0),objects:row.objectCount??0,types:counts.join(", ")||"-"});
 }catch{return undefined;}
}

/** 🔭️ Samples the process while it grows: every `sampleMilliseconds` it looks at the resident set and the cached footprint (cheap) and, when either moved by `sampleGrowthBytes` since the last row, when `sampleQuietMilliseconds` passed or when the main thread was blocked for `sampleLagMilliseconds`, it counts the runtime heap (`bun:jsc` `heapStats`) and emits ONE row: the step it is in, footprint, resident set, managed heap, external memory, array buffers, objects, the loop lag and the `sampleTypes` most numerous object types, so a growth is attributed to objects, strings, buffers or memory outside the heap. Counting walks the whole heap (seconds on a heap of ten million objects whose pages are partly swapped out) and a starved loop asks again every pulse, so the counting itself is bounded: after a count that took C milliseconds the next waits C * (100 - censusDutyPercent) / censusDutyPercent milliseconds, a starved row in between carries the last counts, and a regular row waits.
 * @param {{meter:{bytes:()=>number};stats?:()=>Promise<any>|any;usage?:()=>{rss:number;arrayBuffers?:number};now?:()=>number;phase?:()=>string|undefined;emit:(fields:Record<string,string|number>)=>void;policy?:typeof FOOTPRINT_POLICY_V1}} options */
export function createHeapSamplerV1({meter,stats=async()=>(await import(["bun","jsc"].join(":"))).heapStats(),statsSync=()=>createRequire(import.meta.url)("bun:jsc").heapStats(),stack=()=>String(new Error().stack).split("\n").slice(3,3+P.pulseFrames).map(line=>{const row=/^\s*(?:at )?(.*?)\s*\((?:.*?)([^/]*):(\d+):\d+\)$/u.exec(line);return row===null?line.trim().slice(-80):(row[1]||"?")+"@"+row[2]+":"+row[3];}).join(" < "),starved=fields=>fields,usage=()=>process.memoryUsage(),now=()=>Date.now(),phase=()=>"-",emit,policy=P}){
 let last=now(),wrote=-Infinity,rssAt=0,footprintAt=0,flight=false,quietUntil=-Infinity,known={heap:0,extra:0,objects:0,types:"-"};
 const counted=row=>{known={heap:megabytes(row.heapSize??0),extra:megabytes(row.extraMemorySize??0),objects:row.objectCount??0,types:Object.entries(row.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,policy.sampleTypes).map(([name,count])=>name+" "+count).join(", ")||"-"};return known;};
 const walked=began=>{const took=Math.max(0,now()-began);quietUntil=now()+took*(100-policy.censusDutyPercent)/policy.censusDutyPercent;};
 return Object.freeze({starved(at){
  if(flight||at-last<policy.pulseStarvedMilliseconds)return;
  flight=true;
  try{
   const memory=usage();
   if(now()>=quietUntil){const began=now();try{counted(statsSync());}finally{walked(began);}}
   emit({phase:phase()??"-",blocked:at-last,rss:megabytes(memory.rss),heap:known.heap,extra:known.extra,objects:known.objects,stack:stack(),types:known.types},"starved");
  }catch{}finally{flight=false;}
 },async tick(){
  const at=now(),lag=Math.max(0,at-last-policy.sampleMilliseconds);last=at;
  if(flight)return;
  try{
   const memory=usage(),footprint=meter.bytes(),moved=Math.max(Math.abs(memory.rss-rssAt),Math.abs(footprint-footprintAt));
   if(moved<policy.sampleGrowthBytes&&at-wrote<policy.sampleQuietMilliseconds&&lag<policy.sampleLagMilliseconds)return;
   if(now()<quietUntil)return;
   flight=true;
   const began=now();let counts;
   try{counts=counted(await stats());}finally{walked(began);}
   wrote=at;rssAt=memory.rss;footprintAt=footprint;
   emit({phase:phase()??"-",footprint:megabytes(footprint),moved:megabytes(moved),rss:megabytes(memory.rss),heap:counts.heap,extra:counts.extra,buffers:megabytes(memory.arrayBuffers??0),objects:counts.objects,lag,types:counts.types});
  }catch{}finally{flight=false;}
 }});
}

/** 📜️ The trace of this process: every footprint line with its time is appended to `<tmp>/semio-footprint/<pid>.log` (stops at `traceBytes`), so a stall dump that keeps only the last output line can still be explained; the path is answered, nothing here can fail a read. */
export function createFootprintTraceV1({directory=join(tmpdir(),"semio-footprint"),pid=process.pid,maximumBytes=P.traceBytes,now=()=>new Date().toISOString()}={}){
 const path=join(directory,pid+".log");let written=0,ready=false;
 return Object.assign(line=>{try{if(written>=maximumBytes)return;if(!ready){mkdirSync(directory,{recursive:true});ready=true;}const text=now()+" "+line+"\n";appendFileSync(path,text);written+=text.length;}catch{}},{path});
}

/** 🧯️ Gives memory back at once: a full collection, `Bun.shrink` (returns freed pages to the operating system; a collection alone leaves them in the footprint), a second collection for what the shrink released. */
export function releaseRuntimeMemoryV1({collect=()=>runtimeCollectGarbageV1(true),shrink=()=>{try{globalThis.Bun?.shrink?.();}catch{}}}={}){collect();shrink();collect();}

const defaultSleep=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

/**
 * 🚧️ The gate of one process: `admit(signal)` returns at once below `softPercent` of the grant; from there it collects and shrinks (at most once per `settleMinimumMilliseconds`), when the footprint still stands at `hardPercent` it lets the registered relievers drop what a pass can re-read and, when that does not help either, waits in steps for the footprint to fall, at most `waitBoundMilliseconds` per call, never again within `waitCooldownMilliseconds` of a wait that found no relief, and always cancellable, so the admission slows before the grant is overrun and a read can never wait forever. `headroomBytes()` is what stays before `hardPercent`.
 * @param {{meter?:ReturnType<typeof createFootprintMeterV1>;grantBytes:()=>number|undefined;release?:()=>void;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;report?:(line:string)=>void;environment?:Record<string,string|undefined>;policy?:typeof FOOTPRINT_POLICY_V1}} options
 */
export function createFootprintGateV1({meter=createFootprintMeterV1(),grantBytes,release=releaseRuntimeMemoryV1,heapCensus=heapCensusV1,heartbeat,sample,phase=()=>"-",setTimer=setInterval,clearTimer=clearInterval,sleep=defaultSleep,now=()=>Date.now(),report,environment=process.env,policy=P}){
 const relievers=new Set(),stats={admitted:0,settled:0,waits:0,waitedMilliseconds:0,peakBytes:0};
 let lastSettle=-Infinity,said=-Infinity,calmUntil=-Infinity,beatAt=-Infinity,timer,sampleTimer;
 const grant=()=>{const bytes=grantBytes();return bytes>0?bytes:undefined;};
 const say=(key,fields)=>{if(report!==undefined)for(const line of renderFootprintMessageV1(key,fields,environment))report(line);};
 const measure=()=>{const bytes=meter.bytes();stats.peakBytes=Math.max(stats.peakBytes,bytes);return bytes;};
 const census=async at=>{const row=await heapCensus();if(row!==undefined)say("census",{footprint:megabytes(at),heap:row.heapMegabytes,extra:row.extraMegabytes,objects:row.objects,types:row.types});};
 const settle=async(relieve=true)=>{
  const before=measure();
  if(relieve)for(const reliever of[...relievers])try{await reliever();}catch{}
  release();lastSettle=now();stats.settled++;
  const after=await meter.refresh();
  say("relieved",{before:megabytes(before),after:megabytes(after),grant:megabytes(grant()??0)});
  if(report!==undefined&&before*100>=(grant()??Infinity)*policy.softPercent)await census(after);
  return after;
 };
 return Object.freeze({
  async admit(signal){
   stats.admitted++;
   const limit=grant();if(limit===undefined)return"run";
   let decision=decideFootprintV1({footprintBytes:meter.measured()?measure():await meter.refresh(),grantBytes:limit,policy});
   if(decision==="run")return"run";
   signal?.throwIfAborted();
   decision=decideFootprintV1({footprintBytes:await meter.fresh(),grantBytes:limit,policy});
   if(decision==="run")return"run";
   if(now()-lastSettle>=policy.settleMinimumMilliseconds){
    let after=await settle(false);
    if(decideFootprintV1({footprintBytes:after,grantBytes:limit,policy})==="wait")after=await settle(true);
    decision=decideFootprintV1({footprintBytes:after,grantBytes:limit,policy});
   }
   const overNow=()=>footprintOverGrantV1({footprintBytes:measure(),grantBytes:limit,policy});
   if(decision!=="wait"||(now()<calmUntil&&!overNow()))return decision;
   let waited=0;
   if(now()-said>=policy.waitBoundMilliseconds){said=now();const at=measure();say(overNow()?"overGrant":"slowed",{footprint:megabytes(at),grant:megabytes(limit),percent:Math.round(at*100/limit),seconds:Math.round(policy.waitBoundMilliseconds/1000)});}
   stats.waits++;
   while(waited<policy.waitBoundMilliseconds){
    await sleep(policy.waitStepMilliseconds,signal);waited+=policy.waitStepMilliseconds;stats.waitedMilliseconds+=policy.waitStepMilliseconds;
    const verdict=decideFootprintV1({footprintBytes:await meter.refresh(),grantBytes:limit,policy});
    if(verdict!=="wait")return verdict;
   }
   if(decideFootprintV1({footprintBytes:meter.bytes(),grantBytes:limit,policy})==="wait"&&!overNow())calmUntil=now()+policy.waitCooldownMilliseconds;
   return"wait";
  },
  relieve(reliever){relievers.add(reliever);return()=>relievers.delete(reliever);},
  watch(){
   if(timer!==undefined)return;
   let looking=false;
   timer=setTimer(async()=>{
    if(looking)return;looking=true;
    try{const limit=grant();if(limit===undefined)return;const bytes=measure();if(heartbeat!==undefined&&now()-beatAt>=policy.heartbeatMilliseconds){beatAt=now();heartbeat({footprintBytes:bytes,grantBytes:limit,rssBytes:process.memoryUsage().rss});}const at=decideFootprintV1({footprintBytes:bytes,grantBytes:limit,policy});if(at==="run"||now()-lastSettle<policy.settleMinimumMilliseconds)return;await settle(at==="wait");}catch{}finally{looking=false;}
   },policy.watchMilliseconds);
   timer.unref?.();
   if(sample!==undefined&&sampleTimer===undefined){
    const sampler=createHeapSamplerV1({meter,now,phase,policy,emit:(fields,key="sampled")=>{for(const line of renderFootprintMessageV1(key,fields,environment))sample(line);}});
    pulser=at=>sampler.starved(at);
    sampleTimer=setTimer(()=>{void sampler.tick();},policy.sampleMilliseconds);sampleTimer.unref?.();
   }
  },
  stop(){if(timer!==undefined){clearTimer(timer);timer=undefined;}if(sampleTimer!==undefined){clearTimer(sampleTimer);sampleTimer=undefined;pulser=undefined;}},
  settle,
  overGrant:()=>footprintOverGrantV1({footprintBytes:measure(),grantBytes:grant(),policy}),
  headroomBytes:()=>footprintHeadroomBytesV1({footprintBytes:measure(),grantBytes:grant(),policy}),
  footprintBytes:measure,
  grantBytes:grant,
  stats:()=>Object.freeze({...stats}),
 });
}

let slowTrace;
/** ⏱️ Names a slow stretch in the trace of this process: `label` and the milliseconds since `startedAt`, written when they reach `thresholdMilliseconds` (the pulse cannot speak inside a stretch that calls no instrumented function, so the code that is known to be heavy reports itself afterwards). */
export function noteSlowV1(label,startedAt,thresholdMilliseconds=2000,trace=(slowTrace??=createFootprintTraceV1())){
 const milliseconds=Date.now()-startedAt;
 if(milliseconds>=thresholdMilliseconds)trace("slow "+label+" "+milliseconds+" ms");
 return milliseconds;
}

/** ⏲️ Writes one line to the trace of this process (the settlement stages and the owner steps report themselves here; a trace that cannot be written costs nothing). */
export function noteTraceV1(line,trace=(slowTrace??=createFootprintTraceV1())){trace(line);}

/** ⏲️ The milliseconds of named stages summed over many calls: `time` runs and measures one piece of work, `add` takes a measurement made elsewhere, `text` is `name=Nms/calls` for every stage in the order it first ran. */
export function createStageClockV1({now=Date.now}={}){
 const sums=new Map(),calls=new Map();
 const add=(name,milliseconds)=>{sums.set(name,(sums.get(name)??0)+milliseconds);calls.set(name,(calls.get(name)??0)+1);};
 return Object.freeze({add,async time(name,work){const began=now();try{return await work();}finally{add(name,now()-began);}},totals:()=>Object.fromEntries(sums),text:()=>[...sums].map(([name,milliseconds])=>name+"="+Math.round(milliseconds)+"ms/"+calls.get(name)).join(" ")});
}

/** ⏲️ Where the wall time of an owner goes: `begin` opens an owner, `add` charges a named step to it, `finish` closes it and writes `owner <name> wall=Nms step=Nms ...` when it took `slowMilliseconds`, and every `rollupOwners` owners one `owners n=... wall.avg=... running.avg=... step=avg...` line with the average milliseconds of every step per owner and how many owners ran at once (`running` is what the caller counts at that moment). The steps are the ones the caller names; the part of the wall time no step covers is `other`. */
export function createOwnerClockV1({now=Date.now,trace=(line=>noteTraceV1(line)),slowMilliseconds=3000,rollupOwners=50}={}){
 const open=new Map();let window=[];
 const render=(row,count)=>[...row].map(([name,milliseconds])=>name+"="+Math.round(milliseconds/count)+"ms").join(" ");
 return Object.freeze({
  begin(name){open.set(name,{began:now(),steps:new Map()});},
  add(name,step,milliseconds){const row=open.get(name);if(row!==undefined)row.steps.set(step,(row.steps.get(step)??0)+milliseconds);},
  get(name,step){return open.get(name)?.steps.get(step)??0;},
  finish(name,{running=0}={}){
   const row=open.get(name);if(row===undefined)return;open.delete(name);
   const wall=now()-row.began,covered=[...row.steps.values()].reduce((sum,value)=>sum+value,0);row.steps.set("other",Math.max(0,wall-covered));
   if(wall>=slowMilliseconds)trace("owner "+name+" wall="+Math.round(wall)+"ms "+render(row.steps,1));
   window.push({wall,running,steps:row.steps});
   if(window.length>=rollupOwners){
    const sums=new Map();for(const item of window)for(const[step,milliseconds]of item.steps)sums.set(step,(sums.get(step)??0)+milliseconds);
    const count=window.length,walls=window.map(item=>item.wall).sort((a,b)=>a-b);
    trace("owners n="+count+" wall.avg="+Math.round(walls.reduce((sum,value)=>sum+value,0)/count)+"ms wall.p90="+Math.round(walls[Math.floor(count*0.9)])+"ms wall.max="+Math.round(walls[count-1])+"ms running.avg="+(window.reduce((sum,item)=>sum+item.running,0)/count).toFixed(1)+" running.max="+Math.max(...window.map(item=>item.running))+" "+render(sums,count));
    window=[];
   }
  },
 });
}

let installed,step,pulser,pulses=0,pulsedAt=0,releaseCalls=0;
const releaseStateV1={quietUntil:-Infinity,releasedAt:0};
const footprintDecisionV1=()=>{const grant=installed?.grantBytes();return grant>0?decideFootprintV1({footprintBytes:installed.footprintBytes(),grantBytes:grant}):undefined;};
/** 🧹️ Returns the memory a long synchronous stretch of garbage left behind: when the managed memory (heap in use plus external) grew by `releaseGrowthBytes` since the last release, it collects, shrinks and collects again; a call that finds no such growth costs one counter step in `every - 1` of `every` calls. Compiling a TypeScript program leaves about 1 GB of unreleased garbage in the runtime (measured: 5 programs of 4468 files, footprint 1.7 GB to 4.9 GB; with a release between the programs 1.3 to 2.0 GB), so the declared closure calls it between its programs and the inference at its yield points. A release is a walk of the whole heap, and the walk of a heap whose pages are partly swapped out takes tens of seconds, so it is bounded twice: it does not run while the process holds less than `softPercent` of its grant (the gate settles from there), and after a release that took C milliseconds the next waits C * (100 - releaseDutyPercent) / releaseDutyPercent milliseconds (the share of the wall time that forced collections may take). @returns {boolean} whether it released */
export function releaseIfGrownV1({usage=()=>{const row=process.memoryUsage();return row.heapUsed+row.external;},release=releaseRuntimeMemoryV1,threshold=P.releaseGrowthBytes,every=1,now=()=>Date.now(),decision=footprintDecisionV1,state=releaseStateV1}={}){
 if(every>1&&(++releaseCalls%every)!==0)return false;
 if(usage()-state.releasedAt<threshold)return false;
 if(now()<state.quietUntil||decision()==="run")return false;
 const began=now();release();const ended=now();state.releasedAt=usage();
 state.quietUntil=ended+Math.max(0,ended-began)*(100-P.releaseDutyPercent)/P.releaseDutyPercent;return true;
}

/** 💓️ Called from the hot paths of the inference (one call per row, per interned value, per yield): every 256th call looks at the clock and, at most every `pulseMilliseconds`, lets the sampler speak when the event loop has been starved for `pulseStarvedMilliseconds` (a main thread that never returns to the loop cannot run a timer, so the growth would stay invisible exactly while it happens). */
export function footprintPulseV1(){
 if(pulser===undefined||(++pulses&255)!==0)return;
 const at=Date.now();
 if(at-pulsedAt<P.pulseMilliseconds)return;
 pulsedAt=at;pulser(at);
}

/** 🧭️ Names the step the process is in (the resource of the running graph phase), so the heartbeat and the sampler can say what runs while the memory moves; a label is cut to 160 characters. */
export function noteFootprintStepV1(label){step=typeof label==="string"?label.slice(0,160):undefined;}

/** 🧭️ The step noted last, else the profile phase, `-` when neither is known. */
export function footprintStepV1(){return[profileCurrentPhaseV1(),step].filter(Boolean).join(" | ")||"-";}

/** 📌️ Installs the gate of this process (undefined removes it): the closure settle and the governor find it here. */
export function useFootprintGateV1(gate){if(installed!==gate)installed?.stop?.();installed=gate;}

/** 🔎️ The gate of this process, undefined when none is installed (a process without a grant runs ungated). */
export function footprintGateV1(){return installed;}

/** 🎟️ The grant in bytes of a capacity source (`{handle,limit}` as the build script binds it): the declared memory limit, else the reserved megabytes, undefined when the process runs ungranted. */
export function grantBytesOfV1(source){
 const{handle,limit}=source?.()??{},value=limit?.memoryMegabytes??handle?.reservedMegabytes;
 return typeof value==="number"&&value>0?value*MEGABYTE:undefined;
}

/** 🔗️ Installs the gate of THIS process for a grant source: the process gets its trace file (`<tmp>/semio-footprint/<pid>.log`), the heartbeat and the sampler rows with the step it is in, and the lines of the gate also go to the read's stderr. @param {(() => unknown) | undefined} grant */
export function bindProcessGateV1(grant,{trace=createFootprintTraceV1(),write=line=>{try{process.stderr.write(line+"\n");}catch{}},create=createFootprintGateV1}={}){
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

/** 🌳️ Reads the footprint in bytes of the whole process tree under `pid` (the process and every worker it started), the measure the slice pool and the capacity gate judge a plan by; undefined when the tree cannot be read. */
export async function readTreeFootprintBytesV1(pid,{observe=observeTreeMegabytesV1}={}){
 try{const value=await observe(pid);return Number.isFinite(value)&&value>=0?value*MEGABYTE:undefined;}catch{return undefined;}
}

/** 🧱️ The ceiling in bytes a slice worker was told to keep its process tree under (`planCeilingEnvironment` of the memory policy, set by the slice pool from the declared plan limit), undefined when none was named. */
export function planCeilingBytesV1(environment=process.env,policy=memorySchema.definitions.Policy.const){
 const named=Number(environment[policy.planCeilingEnvironment]);
 return Number.isFinite(named)&&named>0?Math.floor(named)*MEGABYTE:undefined;
}

/** 🏔️ Installs the gate of a slice worker: it holds no capacity grant of its own (the service books the plan), so without this gate nothing in the worker knew its footprint and `releaseOwnerStateV1` never shrank, the governor never held and only the slice pool, reacting to a sample taken after the growth, bounded the plan. The grant of the gate is the declared plan ceiling and the footprint it compares is the whole tree of the worker, so the owner state is released from `softPercent`, shrunk and spilled to the store from `hardPercent` and no further owner is admitted while the tree stands there, long before the pool would have to refuse. Does nothing when a gate is installed already or the pool named no ceiling; answers the gate. */
export function useSliceCeilingGateV1(environment=process.env,{bind=bindProcessGateV1,pid=process.pid,read=readTreeFootprintBytesV1}={}){
 if(installed!==undefined)return installed;
 const bytes=planCeilingBytesV1(environment);
 if(bytes===undefined)return undefined;
 return bind(()=>({limit:{memoryMegabytes:bytes/MEGABYTE}}),{create:options=>createFootprintGateV1({...options,meter:createFootprintMeterV1({read:async()=>{const tree=await read(pid);return pid===process.pid?footprintAtLeastManagedV1(tree):tree;}})})});
}

/** 🌊️ Settles the memory of a process after a published wave and logs ONE line per language with the footprint before and after: `release` drops what the published wave already persisted (the store is the source of truth), then the runtime collects and shrinks. Answers the numbers in megabytes; nothing here can fail a read. */
export async function settleWaveMemoryV1({generation,release,meter=createFootprintMeterV1({ttlMilliseconds:0}),managed=()=>{const usage=process.memoryUsage();return usage.heapUsed+usage.external;},grantBytes=()=>footprintGateV1()?.grantBytes(),collect=releaseRuntimeMemoryV1,heapCensus=heapCensusV1,report=line=>{try{process.stderr.write(line+"\n");}catch{}},environment=process.env}){
 try{
  useEnvironmentGateV1(environment);
  const before=await meter.refresh();
  await release?.();collect();
  const after=await meter.refresh(),result=Object.freeze({beforeMegabytes:megabytes(before),afterMegabytes:megabytes(after),managedMegabytes:megabytes(managed()),grantMegabytes:megabytes(grantBytes()??0)});
  if(report!==undefined&&result.afterMegabytes>0){const row=await heapCensus();if(row!==undefined)for(const line of renderFootprintMessageV1("census",{footprint:result.afterMegabytes,heap:row.heapMegabytes,extra:row.extraMegabytes,objects:row.objects,types:row.types},environment))report(line);}
  for(const line of renderFootprintMessageV1("waveSettled",{generation,before:result.beforeMegabytes,after:result.afterMegabytes,managed:result.managedMegabytes,grant:result.grantMegabytes},environment))report(line);
  return result;
 }catch{return undefined;}
}
