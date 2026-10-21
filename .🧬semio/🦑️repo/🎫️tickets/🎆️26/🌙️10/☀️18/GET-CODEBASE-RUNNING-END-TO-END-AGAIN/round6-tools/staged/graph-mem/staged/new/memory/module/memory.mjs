import v8 from "node:v8";

/** 🧠️ The declared limits of the graph memory governor; the schema file declares the same constants and a law keeps both equal. */
export const GRAPH_MEMORY_POLICY_V1=Object.freeze({
 version:1,
 defaultBudgetMegabytes:3072,minimumBudgetMegabytes:1024,grantShare:0.5,resumeRatio:0.85,
 pollMilliseconds:50,sampleMilliseconds:25,admitMinimumMilliseconds:100,collectMinimumMilliseconds:2000,retainedMilliseconds:30000,progressMilliseconds:5000,
 retainedImageBytes:33554432,minimumInternedLength:24,maximumInternedCharacters:67108864,
 budgetEnvironment:"SEMIO_GRAPH_MEMORY_MB",planEnvironment:"SEMIO_HOST_CAPACITY_PLAN",
});
const P=GRAPH_MEMORY_POLICY_V1,MEGABYTE=1048576;

/** 🗣️ The bilingual texts of the governor; the schema file declares the same table and a law keeps both equal. */
export const GRAPH_MEMORY_MESSAGES_V1=Object.freeze({
 en:Object.freeze({
  waiting:"graph: waiting for memory ({used} of {budget} MB in use, {running} projects running)",
  serial:"graph: memory above the budget of {budget} MB ({used} MB in use), inferring one project at a time",
  resumed:"graph: memory below the budget again ({used} of {budget} MB), resuming",
 }),
 de:Object.freeze({
  waiting:"Graph: wartet auf Speicher ({used} von {budget} MB belegt, {running} Projekte laufen)",
  serial:"Graph: Speicher über dem Budget von {budget} MB ({used} MB belegt), Projekte werden einzeln abgeleitet",
  resumed:"Graph: Speicher wieder unter dem Budget ({used} von {budget} MB), es geht weiter",
 }),
});

/** 🌐️ Picks the language of the process like every other progress text: dashboard locale, then the system locale, English first and German second. */
export function selectGraphMemoryLanguageV1(environment){
 return["SEMIO_DASHBOARD_LOCALE","LC_ALL","LC_MESSAGES","LANG"].map(name=>environment[name]?.split(/[_.@-]/u)[0]?.toLowerCase()).find(value=>value==="en"||value==="de")??"en";
}

/** 🗣️ Renders one governor message in the language of the environment. */
export function renderGraphMemoryMessageV1(key,fields,environment){
 const template=GRAPH_MEMORY_MESSAGES_V1[selectGraphMemoryLanguageV1(environment)][key];
 if(template===undefined)throw Error("Closed graph memory message required: "+key);
 return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
}

/** 📏️ The managed memory budget of one graph construction in bytes: the explicit environment value, else a share of the host capacity grant the process inherited, else the declared default; never below the declared minimum and never above the declared default. */
export function graphMemoryBudgetBytesV1(environment,policy=P){
 const text=environment[policy.budgetEnvironment];
 if(text!==undefined){
  const explicit=Number(text);
  if(!Number.isFinite(explicit)||explicit<policy.minimumBudgetMegabytes)throw Error("Graph memory budget of at least "+policy.minimumBudgetMegabytes+" MB required: "+policy.budgetEnvironment);
  return Math.floor(explicit)*MEGABYTE;
 }
 let granted;try{granted=JSON.parse(environment[policy.planEnvironment]??"null")?.memoryMegabytes;}catch{}
 const share=typeof granted==="number"&&Number.isFinite(granted)&&granted>0?Math.floor(granted*policy.grantShare):policy.defaultBudgetMegabytes;
 return Math.max(policy.minimumBudgetMegabytes,Math.min(policy.defaultBudgetMegabytes,share))*MEGABYTE;
}

/**
 * ⚖️ Decides whether the next project may start: never beyond the worker count, otherwise while the managed memory is within the budget (within `resumeRatio` of it after it was exceeded, so admission does not flap).
 * Memory above the budget is answered with one collection (the caller collects and decides again with `collected` set); what a collection cannot release is retained data, which degrades the run to one project at a time (`forced` while nothing runs) and never deadlocks.
 * @param {typeof GRAPH_MEMORY_POLICY_V1} policy
 * @param {{used:number;budget:number;running:number;workers:number;blocked:boolean;collected:boolean}} state
 * @returns {{admit:boolean;collect:boolean;reason:"workers"|"within"|"collect"|"over"|"forced"}}
 */
export function decideMemoryAdmissionV1(policy,{used,budget,running,workers,blocked,collected}){
 if(running>=workers)return{admit:false,collect:false,reason:"workers"};
 if(used<=(blocked?budget*policy.resumeRatio:budget))return{admit:true,collect:false,reason:"within"};
 if(!collected)return{admit:false,collect:true,reason:"collect"};
 return running===0?{admit:true,collect:false,reason:"forced"}:{admit:false,collect:false,reason:"over"};
}

/** 🔌️ The live managed bytes of the JavaScript heap and the collection that releases them; both are ports so every platform and every test supplies its own. */
export function createMemoryPortsV1(){
 return Object.freeze({
  sample:()=>{const statistics=v8.getHeapStatistics();return statistics.used_heap_size+statistics.external_memory;},
  collect:()=>{if(globalThis.Bun?.gc)globalThis.Bun.gc(true);else globalThis.gc?.();},
 });
}

const defaultSleep=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

/**
 * 🚦️ Gates the start of concurrent project inferences by the managed memory: at most `workers` run, a new one starts only while the heap is within `budgetBytes` (after one rate limited collection), waiters start in arrival order, progress is reported at most every `progressMilliseconds` and an abort rejects every waiter.
 * @param {{budgetBytes:number;workers:number;sample?:()=>number;collect?:()=>void;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;signal?:AbortSignal;report?:(line:string)=>void;environment?:Record<string,string|undefined>;policy?:typeof GRAPH_MEMORY_POLICY_V1}} options
 */
export function createMemoryGovernorV1({budgetBytes,workers,sample,collect,sleep=defaultSleep,now=()=>performance.now(),signal,report,environment={},policy=P}){
 if(!Number.isSafeInteger(workers)||workers<1||!(budgetBytes>0))throw Error("Positive worker count and memory budget required");
 const defaults=createMemoryPortsV1(),read=sample??defaults.sample,collectNow=collect??defaults.collect;
 const queue=[],stats={waits:0,waitedMilliseconds:0,collections:0,peakBytes:0};
 let running=0,blocked=false,pumping=false,serialSaid=false,retained=false,lastCollect=-Infinity,lastReport=-Infinity,lastSampleAt=-Infinity,lastAdmit=-Infinity,lastRelease=0,lastUsed=0;
 const say=(key,fields)=>report?.(renderGraphMemoryMessageV1(key,{budget:Math.round(budgetBytes/MEGABYTE),running,...fields},environment));
 const observe=fresh=>{const at=now();if(fresh||at-lastSampleAt>=policy.sampleMilliseconds){lastUsed=read();lastSampleAt=at;if(lastUsed>stats.peakBytes)stats.peakBytes=lastUsed;}return lastUsed;};
 const settled=()=>lastCollect>=lastRelease||retained&&now()-lastCollect<policy.retainedMilliseconds;
 const mayCollect=()=>now()-lastCollect>=policy.collectMinimumMilliseconds&&!settled();
 const fail=error=>{for(const waiter of queue.splice(0))waiter.reject(error);};
 const pump=async()=>{
  if(pumping)return;pumping=true;
  try{
   while(queue.length){
    signal?.throwIfAborted();
    const spacing=lastAdmit+policy.admitMinimumMilliseconds-now();
    if(running>0&&spacing>0){await sleep(spacing,signal);continue;}
    let used=observe(false),verdict=decideMemoryAdmissionV1(policy,{used,budget:budgetBytes,running,workers,blocked,collected:false});
    if(verdict.collect){
     if(mayCollect()){collectNow();lastCollect=now();stats.collections++;retained=running===0&&observe(true)>budgetBytes;}
     used=observe(true);verdict=decideMemoryAdmissionV1(policy,{used,budget:budgetBytes,running,workers,blocked,collected:settled()});
     if(verdict.collect)verdict={admit:false,collect:false,reason:"over"};
    }
    if(verdict.admit){
     if(verdict.reason==="within"){if(blocked){blocked=false;say("resumed",{used:Math.round(used/MEGABYTE)});}serialSaid=false;}
     else{blocked=true;if(!serialSaid){serialSaid=true;say("serial",{used:Math.round(used/MEGABYTE)});}}
     const waiter=queue.shift();running++;lastAdmit=now();
     if(waiter.waited){stats.waits++;stats.waitedMilliseconds+=now()-waiter.since;}
     waiter.resolve();
     continue;
    }
    if(verdict.reason==="workers")return;
    blocked=true;for(const waiter of queue)waiter.waited=true;
    if(now()-lastReport>=policy.progressMilliseconds){lastReport=now();say("waiting",{used:Math.round(used/MEGABYTE)});}
    await sleep(policy.pollMilliseconds,signal);
   }
  }catch(error){fail(error);}
  finally{pumping=false;}
 };
 signal?.addEventListener("abort",()=>fail(signal.reason),{once:true});
 return Object.freeze({
  async run(task){
   signal?.throwIfAborted();
   await new Promise((resolve,reject)=>{queue.push({resolve,reject,since:now(),waited:false});pump();});
   try{return await task();}
   finally{running--;lastRelease=now();if(queue.length)pump();}
  },
  stats:()=>({waits:stats.waits,waitedMilliseconds:Math.round(stats.waitedMilliseconds),collections:stats.collections,peakMegabytes:Math.round(stats.peakBytes/MEGABYTE)}),
 });
}

/**
 * 🧶️ Creates a bounded table that makes equal long strings one instance: equal paths, input lists and keys of thousands of projects cost memory once. A table over its character bound is cleared, never grown.
 * @param {{maximumCharacters?:number;minimumLength?:number}} [options]
 */
export function createStringInternerV1({maximumCharacters=P.maximumInternedCharacters,minimumLength=P.minimumInternedLength}={}){
 const table=new Map();let characters=0,hits=0,clears=0;
 return Object.freeze({
  share(text){
   if(text.length<minimumLength)return text;
   const known=table.get(text);if(known!==undefined){hits++;return known;}
   if(characters+text.length>maximumCharacters){table.clear();characters=0;clears++;}
   table.set(text,text);characters+=text.length;return text;
  },
  stats:()=>({entries:table.size,characters,hits,clears}),
 });
}

/**
 * 🧶️ Replaces every long string value of a freshly parsed JSON value by the shared instance of `interner`, in place and without touching object keys, array lengths or any non-string value, so the value serializes to the identical bytes.
 * @template T
 * @param {T} value
 * @param {ReturnType<typeof createStringInternerV1>} interner
 * @returns {T}
 */
export function internJsonStringsV1(value,interner){
 const pending=[value];
 while(pending.length){
  const item=pending.pop();
  if(Array.isArray(item)){
   for(let index=0;index<item.length;index++){const child=item[index];if(typeof child==="string")item[index]=interner.share(child);else if(child!==null&&typeof child==="object")pending.push(child);}
  }else if(item!==null&&typeof item==="object"){
   for(const key of Object.keys(item)){const child=item[key];if(typeof child==="string")item[key]=interner.share(child);else if(child!==null&&typeof child==="object")pending.push(child);}
  }
 }
 return value;
}

/** 🌍️ The process wide table that shares strings between inference results, stored fragments and expanded input lists. */
export const graphStringInternerV1=createStringInternerV1();
