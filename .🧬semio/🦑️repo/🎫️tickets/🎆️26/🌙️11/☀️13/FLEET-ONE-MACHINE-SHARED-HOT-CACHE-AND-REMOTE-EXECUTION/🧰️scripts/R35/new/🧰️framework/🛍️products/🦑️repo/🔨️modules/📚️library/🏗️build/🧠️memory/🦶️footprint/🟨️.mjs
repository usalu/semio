import{platform as osPlatform,tmpdir}from"node:os";
import{appendFileSync,mkdirSync}from"node:fs";
import{join}from"node:path";
import{noticeLanguagesV1}from"../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import{runtimeCollectGarbageV1}from"../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import{parseFootprintV1,parseSmapsRollupV1,readHostTextV1,runHostCommandV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

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

/** 📏️ The bytes of the grant that stay before `hardPercent` is reached (negative once passed), Infinity without a known grant. */
export function footprintHeadroomBytesV1({footprintBytes,grantBytes,policy=P}){
 return grantBytes>0&&Number.isFinite(footprintBytes)?Math.floor(grantBytes*policy.hardPercent/100-footprintBytes):Infinity;
}

/** 🔬️ Reads the physical footprint of ONE process in bytes: macOS `footprint` (phys_footprint), Linux `smaps_rollup` (Pss plus SwapPss), every other platform its resident set (the working set on Windows); a reading that fails answers undefined, never the resident set, which counts freed pages the footprint does not. */
export async function readProcessFootprintBytesV1(pid,{platform=osPlatform(),run=runHostCommandV1,read=readHostTextV1,rss=()=>process.memoryUsage().rss}={}){
 if(platform!=="darwin"&&platform!=="linux")return rss();
 try{
  if(platform==="darwin")return parseFootprintV1(await run("footprint",["-f","bytes","--noCategories","-p",String(pid)])).get(pid);
  const text=await read(`/proc/${pid}/smaps_rollup`),kilobytes=text===null?undefined:parseSmapsRollupV1(text);
  return kilobytes===undefined?undefined:kilobytes*1024;
 }catch{return undefined;}
}

/** 🪞️ A cached reading of the footprint of this process: `bytes()` answers at once with the last reading (the resident set until the first one arrives) and starts one background reading when the last is older than `ttlMilliseconds` (one reading at a time, a failed one is ignored); `refresh()` waits for a fresh reading. */
export function createFootprintMeterV1({pid=process.pid,read=()=>readProcessFootprintBytesV1(pid),rss=()=>process.memoryUsage().rss,now=()=>Date.now(),ttlMilliseconds=P.ttlMilliseconds}={}){
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
export async function heapCensusV1({stats=async()=>(await import("bun:jsc")).heapStats(),types=P.censusTypes}={}){
 try{
  const row=await stats(),counts=Object.entries(row.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,types).map(([name,count])=>name+" "+count);
  return Object.freeze({heapMegabytes:megabytes(row.heapSize??0),extraMegabytes:megabytes(row.extraMemorySize??0),objects:row.objectCount??0,types:counts.join(", ")||"-"});
 }catch{return undefined;}
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
export function createFootprintGateV1({meter=createFootprintMeterV1(),grantBytes,release=releaseRuntimeMemoryV1,heapCensus=heapCensusV1,heartbeat,setTimer=setInterval,clearTimer=clearInterval,sleep=defaultSleep,now=()=>Date.now(),report,environment=process.env,policy=P}){
 const relievers=new Set(),stats={admitted:0,settled:0,waits:0,waitedMilliseconds:0,peakBytes:0};
 let lastSettle=-Infinity,said=-Infinity,calmUntil=-Infinity,beatAt=-Infinity,timer;
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
  if(report!==undefined&&before*100>=(grant()??Infinity)*policy.softPercent)await census(before);
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
   if(decision!=="wait"||now()<calmUntil)return decision;
   let waited=0;
   if(now()-said>=policy.waitBoundMilliseconds){said=now();say("slowed",{footprint:megabytes(measure()),grant:megabytes(limit),seconds:Math.round(policy.waitBoundMilliseconds/1000)});}
   stats.waits++;
   while(waited<policy.waitBoundMilliseconds){
    await sleep(policy.waitStepMilliseconds,signal);waited+=policy.waitStepMilliseconds;stats.waitedMilliseconds+=policy.waitStepMilliseconds;
    const verdict=decideFootprintV1({footprintBytes:await meter.refresh(),grantBytes:limit,policy});
    if(verdict!=="wait")return verdict;
   }
   if(decideFootprintV1({footprintBytes:meter.bytes(),grantBytes:limit,policy})==="wait")calmUntil=now()+policy.waitCooldownMilliseconds;
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
  },
  stop(){if(timer!==undefined){clearTimer(timer);timer=undefined;}},
  settle,
  headroomBytes:()=>footprintHeadroomBytesV1({footprintBytes:measure(),grantBytes:grant(),policy}),
  footprintBytes:measure,
  grantBytes:grant,
  stats:()=>Object.freeze({...stats}),
 });
}

let installed;
/** 📌️ Installs the gate of this process (undefined removes it): the closure settle and the governor find it here. */
export function useFootprintGateV1(gate){if(installed!==gate)installed?.stop?.();installed=gate;}

/** 🔎️ The gate of this process, undefined when none is installed (a process without a grant runs ungated). */
export function footprintGateV1(){return installed;}

/** 🎟️ The grant in bytes of a capacity source (`{handle,limit}` as the build script binds it): the declared memory limit, else the reserved megabytes, undefined when the process runs ungranted. */
export function grantBytesOfV1(source){
 const{handle,limit}=source?.()??{},value=limit?.memoryMegabytes??handle?.reservedMegabytes;
 return typeof value==="number"&&value>0?value*MEGABYTE:undefined;
}

/** 🌊️ Settles the memory of a process after a published wave and logs ONE line per language with the footprint before and after: `release` drops what the published wave already persisted (the store is the source of truth), then the runtime collects and shrinks. Answers the numbers in megabytes; nothing here can fail a read. */
export async function settleWaveMemoryV1({generation,release,meter=createFootprintMeterV1({ttlMilliseconds:0}),managed=()=>{const usage=process.memoryUsage();return usage.heapUsed+usage.external;},grantBytes=()=>footprintGateV1()?.grantBytes(),collect=releaseRuntimeMemoryV1,heapCensus=heapCensusV1,report=line=>{try{process.stderr.write(line+"\n");}catch{}},environment=process.env}){
 try{
  const before=await meter.refresh();
  await release?.();collect();
  const after=await meter.refresh(),result=Object.freeze({beforeMegabytes:megabytes(before),afterMegabytes:megabytes(after),managedMegabytes:megabytes(managed()),grantMegabytes:megabytes(grantBytes()??0)});
  if(report!==undefined&&result.afterMegabytes>0){const row=await heapCensus();if(row!==undefined)for(const line of renderFootprintMessageV1("census",{footprint:result.afterMegabytes,heap:row.heapMegabytes,extra:row.extraMegabytes,objects:row.objects,types:row.types},environment))report(line);}
  for(const line of renderFootprintMessageV1("waveSettled",{generation,before:result.beforeMegabytes,after:result.afterMegabytes,managed:result.managedMegabytes,grant:result.grantMegabytes},environment))report(line);
  return result;
 }catch{return undefined;}
}
