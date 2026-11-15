import{GRAPH_MEMORY_POLICY_V1,createMemoryGovernorV1,createMemoryPortsV1,hostMemoryMegabytesV1,selectGraphMemoryLanguagesV1}from"../🟨️.mjs";
import{ownerTooHeavyOfV1,runInOwnerScopeV1,throwIfOwnerExceededV1}from"../🎯️owner-scope/🟨️.mjs";
import{createRequire}from"node:module";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const nativeRequire=createRequire(import.meta.url);

const freeze=value=>{if(value!==null&&typeof value==="object"){for(const child of Object.values(value))freeze(child);Object.freeze(value);}return value;};

/** 🪜️ The declared limits and stages of a graph plan: the schema file is the single source. */
export const PLAN_STAGE_POLICY_V1=freeze(structuredClone(schema.definitions.Policy.const));

/** 🗣️ The bilingual lines of the plan stages, read from the schema file. */
export const PLAN_STAGE_MESSAGES_V1=freeze(structuredClone(schema.definitions.Messages.const));
const P=PLAN_STAGE_POLICY_V1,MEGABYTE=1048576,megabytes=bytes=>Math.round(bytes/MEGABYTE);

/** 🌐️ The listeners of the stage events of this process (a slice worker forwards them to its service). */
const sinks=new Set();

/** 📡️ Registers a listener of the stage events (`StageEvent` of the schema) of every plan of this process; returns the function that removes it. */
export function onPlanStageV1(listener){sinks.add(listener);return()=>sinks.delete(listener);}

/** 💬️ Renders one plan stage line as one line per language of the environment. */
export function renderPlanStageMessageV1(key,fields,environment){
 return selectGraphMemoryLanguagesV1(environment).map(language=>{
  const template=PLAN_STAGE_MESSAGES_V1[language][key];
  if(template===undefined)throw Error("Closed plan stage message required: "+key);
  return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
 });
}

/** 📏️ The managed bytes a plan may add to what its process holds when it starts: `hostShare` of the physical memory between the declared minimum and maximum, none on a machine that serves the fleet (it holds the whole graph and its plans are measured, not limited), none when the environment says `off`, and the megabytes the environment names when it names a number. */
export function planStageLimitBytesV1(environment=process.env,hostMegabytes=hostMemoryMegabytesV1(),policy=PLAN_STAGE_POLICY_V1){
 const text=environment[policy.limitEnvironment];
 if(text===policy.off)return Infinity;
 const named=Number(text);
 if(text!==undefined&&text!==""&&Number.isFinite(named)&&named>0)return Math.floor(named)*MEGABYTE;
 if(!(hostMegabytes>0)||hostMegabytes>=policy.serveHostMegabytes)return Infinity;
 return Math.min(policy.maximumMegabytes,Math.max(policy.minimumMegabytes,Math.floor(hostMegabytes*policy.hostShare)))*MEGABYTE;
}

/**
 * 🪜️ Guards one plan (the assembly of a wave after its projects are inferred) as ONE owner of the memory governor, the same mechanism that stops one project: `run(work)` runs `work` inside the owner scope `plan`, `mark(stage)` names the stage that starts (and ends the one before it with its seconds and growth), `operation(name)` names the operation inside a stage and is a throw point for long operations, and a plan whose managed bytes grew past `limitBytes` (or a stage past its own budget `stageMegabytes`), confirmed by a full collection (garbage is no growth), is stopped at its next yield point, operation or stage boundary, `refuse({stage,operation,megabytes,limit})` building the typed error the caller throws. A sampler every `sampleMilliseconds` keeps the last samples (heap, external, array buffers, resident) and enforces the limits between yield points; when it stops a plan it writes ONE detail line measured at that moment (counters, fresh heap statistics, pending native requests, the recent samples), so the log says whether the growth is heap or buffers and when it began. Every stage start and end is one bilingual line to `report` (on a machine that has a limit, or with `SEMIO_GRAPH_PHASES=1`; a serving machine that has none stays quiet), so a plan that dies inside a stage names that stage in the log; `run` ends the last stage when the plan ends, whatever the outcome.
 * @param {{limitBytes?:number;ports?:{sample:()=>number;collect:()=>void};environment?:Record<string,string|undefined>;report?:(line:string)=>void;now?:()=>number;resident?:()=>number;usage?:()=>Record<string,number>;heap?:()=>Record<string,unknown>|undefined;resources?:()=>readonly string[]|undefined;stageMegabytes?:Record<string,number>;refuse?:(detail:{stage:string;operation:string;megabytes:number;limit:number})=>Error}} [options]
 */
export function createPlanStageGuardV1({limitBytes=planStageLimitBytesV1(),ports=createMemoryPortsV1(),environment=process.env,report=line=>{try{process.stderr.write(line+"\n");}catch{}},now=()=>performance.now(),resident=()=>process.memoryUsage().rss,usage=()=>process.memoryUsage(),heap=()=>{try{return nativeRequire("bun:jsc").heapStats();}catch{return undefined;}},resources=()=>process.getActiveResourcesInfo?.(),stageMegabytes=P.stageMegabytes,refuse}={}){
 let stage,operationName="-",startedAt=0,startedBytes=0,baseline=0,current,stoppedIn,stoppedDuring,told=false,ring=[];
 const say=(key,fields)=>{for(const line of renderPlanStageMessageV1(key,fields,environment))report(line);},tell=limitBytes<Infinity||environment.SEMIO_GRAPH_PHASES==="1",narrate=(key,fields)=>{if(tell)say(key,fields);};
 const emit=()=>{const event={type:"stage",stage,operation:operationName,managedMegabytes:megabytes(ports.sample()),residentMegabytes:megabytes(resident())};for(const sink of sinks)try{sink(event);}catch{}return event;};
 const close=()=>{
  if(stage===undefined)return;
  const bytes=ports.sample();
  narrate("finished",{stage,seconds:Math.round((now()-startedAt)/100)/10,managed:megabytes(bytes),delta:megabytes(bytes-startedBytes),resident:megabytes(resident())});
  stage=undefined;
 };
 const detail=()=>{
  try{
   const counters=usage(),statistics=heap(),pending={};
   for(const name of resources()??[])pending[name]=(pending[name]??0)+1;
   const row={stage:stoppedIn,operation:stoppedDuring,memory:Object.fromEntries(["rss","heapTotal","heapUsed","external","arrayBuffers"].map(name=>[name,megabytes(counters[name]??0)])),...statistics?{heap:{heapMb:Math.round(statistics.heapSize/MEGABYTE*10)/10,extraMb:Math.round(statistics.extraMemorySize/MEGABYTE*10)/10,objectCount:statistics.objectCount,objectTypes:Object.entries(statistics.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,8).map(([type,count])=>({type,count}))}}:{},resources:pending,recent:ring};
   for(const line of renderPlanStageMessageV1("detail",{json:JSON.stringify(row)},environment))report(line);
  }catch{}
 };
 const stopped=(grown,limit)=>{
  stoppedIn??=stage??"?";stoppedDuring??=operationName;
  if(told)return;told=true;
  say("stopped",{stage:stoppedIn,operation:stoppedDuring,used:megabytes(grown),budget:megabytes(limit)});
  detail();
 };
 const over=()=>{
  const bytes=ports.sample(),plan=bytes-baseline,own=stage===undefined||stageMegabytes[stage]===undefined?undefined:stageMegabytes[stage]*MEGABYTE;
  if(plan>limitBytes)return[plan,limitBytes];
  if(own!==undefined&&bytes-startedBytes>own)return[bytes-startedBytes,own];
  return undefined;
 };
 const check=()=>{
  if(current===undefined||current.exceeded!==undefined||!(limitBytes<Infinity))return;
  if(over()===undefined)return;
  ports.collect();
  const hit=over();
  if(hit===undefined)return;
  current.exceed({takenBytes:hit[0],limitBytes:hit[1]});stopped(hit[0],hit[1]);
 };
 const sample=()=>{
  const counters=usage();
  ring.push({atMs:Math.round(now()),heapUsed:megabytes(counters.heapUsed??0),external:megabytes(counters.external??0),arrayBuffers:megabytes(counters.arrayBuffers??0),rss:megabytes(counters.rss??0),operation:stage===undefined?"-":stage+"/"+operationName});
  if(ring.length>P.recentSamples)ring.shift();
  check();
 };
 return Object.freeze({
  mark(name){
   throwIfOwnerExceededV1();check();throwIfOwnerExceededV1();close();
   stage=name;operationName=name;startedAt=now();startedBytes=ports.sample();
   const event=emit();
   narrate("started",{stage:name,managed:event.managedMegabytes,resident:event.residentMegabytes});
  },
  operation(name){
   throwIfOwnerExceededV1();check();throwIfOwnerExceededV1();
   if(name!==operationName){operationName=name;emit();}
  },
  async run(work){
   baseline=ports.sample();told=false;stoppedIn=undefined;stoppedDuring=undefined;ring=[];
   const governor=createMemoryGovernorV1({workers:1,headroom:()=>Number.MAX_SAFE_INTEGER,measure:ports.sample,collect:ports.collect,ownerLimitBytes:limitBytes,onStop:({takenBytes,limitBytes:limit})=>stopped(takenBytes,limit),report:()=>undefined,environment}),timer=setInterval(sample,P.sampleMilliseconds);
   timer.unref?.();
   try{
    return await governor.run(({scope})=>{current=scope;return runInOwnerScopeV1(scope,work);},{kind:P.owner,key:P.owner});
   }catch(error){
    const found=ownerTooHeavyOfV1(error);
    if(found===undefined||found.name!==P.owner||refuse===undefined)throw error;
    throw refuse({stage:stoppedIn??stage??"?",operation:stoppedDuring??operationName,megabytes:megabytes(found.takenBytes),limit:megabytes(found.limitBytes)});
   }finally{clearInterval(timer);close();governor.dispose();current=undefined;}
  },
 });
}
