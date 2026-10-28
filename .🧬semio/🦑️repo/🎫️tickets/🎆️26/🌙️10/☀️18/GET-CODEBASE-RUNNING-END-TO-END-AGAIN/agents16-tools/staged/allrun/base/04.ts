import {mkdirSync,readFileSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {settleBusyFilesystemSyncV1} from "../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {requireHostCapacityV1,isHostCapacityV1,type HostCapacityPolicyV1,type HostRequestV1} from "../📏️decision/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 🧾️ One finished run of a class: the peak its process tree held against the default size of that request, how long it was actively working and after how much of that active time the tree first came within 5 % of its peak (absent in samples written before the time to peak was learned: the peak then counts as reached at the end). */
export type HostEstimateSampleV1=Readonly<{atMs:number;ratioBasisPoints:number;activeMilliseconds:number;peakAtMilliseconds?:number;peakMegabytes?:number}>;
/** 🗃️ The learned peaks of every class (`EstimateStore` of the schema); a class is a key of the lineage `operation` ← `operation:kind` ← `operation:project:target`. */
export type HostEstimateStoreV1=Readonly<{version:1;classes:Readonly<Record<string,Readonly<{samples:readonly HostEstimateSampleV1[]}>>>}>;
/** 🏁️ What a finished grant reports: its peak megabytes against the default megabytes of the size it was admitted at. */
export type HostOutcomeV1=Readonly<{operation:string;jobs:number;peakMegabytes:number;defaultMegabytes:number;activeMilliseconds:number;peakAtMilliseconds:number;endedAtMs:number;keepPeak?:boolean}>;
/** 🎯️ The learned size of one class: a share of its default size, how long its runs need to reach their peak (the time their reservation must count in full), the effective samples behind it and whether it is a quick, interactive rebuild. */
export type HostEstimateV1=Readonly<{ratioBasisPoints:number;rampMilliseconds:number;samples:number;quick:boolean}>;
/** 🚦️ How a run ended: it completed, it failed, or it was cancelled. */
export type HostRunStatusV1="ok"|"failed"|"cancelled";

const UNIT=1_000_000,DEFAULT_BASIS_POINTS=10000;
const round=(value:number):number=>Math.floor(value+0.5);

/** 🌱️ The store before anything was learned. */
export function emptyHostEstimateStoreV1():HostEstimateStoreV1{return Object.freeze({version:1,classes:Object.freeze({})});}

/** 🪜️ Names the classes of one request from the general to the specific, so a class with few runs borrows its size from the class above it. */
export function hostEstimateLineageV1(operation:string,...parts:readonly string[]):readonly string[]{
 const lineage:string[]=[operation];
 for(const part of parts.filter(row=>row!==""))lineage.push(`${lineage[lineage.length-1]}:${part}`);
 return Object.freeze(lineage);
}

const weights=(policy:HostCapacityPolicyV1,samples:readonly HostEstimateSampleV1[],nowMs:number):number[]=>samples.map(sample=>round(UNIT*2**(-Math.max(0,nowMs-sample.atMs)/policy.estimates.halfLifeMilliseconds)));

/** 📊️ The weighted quantile of one sample field: the first sample, in ascending order, whose cumulative decayed weight reaches the share; integer weights make every language pick the same sample. */
function quantile(samples:readonly HostEstimateSampleV1[],weight:readonly number[],field:"ratioBasisPoints"|"activeMilliseconds"|"peakAtMilliseconds"|"peakMegabytes",basisPoints:number):number{
 const value=(sample:HostEstimateSampleV1):number=>field==="peakAtMilliseconds"?sample.peakAtMilliseconds??sample.activeMilliseconds:field==="peakMegabytes"?sample.peakMegabytes??0:sample[field];
 const rows=samples.map((sample,index)=>({sample,weight:weight[index]!})).sort((left,right)=>value(left.sample)-value(right.sample)||left.sample.atMs-right.sample.atMs),total=rows.reduce((sum,row)=>sum+row.weight,0),target=Math.ceil(total*basisPoints/10000);
 let cumulative=0;
 for(const row of rows){cumulative+=row.weight;if(cumulative>=target)return value(row.sample);}
 return value(rows[rows.length-1]!.sample);
}

export {weights as hostSampleWeightsV1,quantile as hostSampleQuantileV1};

/** 🧮️ Estimates a request from what its class learned: per class of the lineage the decayed quantile of the peak ratios plus a margin, blended with the class above by the share of trust the effective samples earn (full trust at `minimumSamples`); what a class borrows from the class above never exceeds the default, so an unknown class keeps the conservative 100 % and pooled evidence can only shave it, while a class's own evidence may rise above it; the answer stays between the authored bounds. The time to peak follows the same rule (quantile, margin, trust, an unknown class keeps `defaultRampMilliseconds`, what a class borrows never exceeds it) and stays between `memory.rampMilliseconds` and `maximumRampMilliseconds`, so the reservation of a run that peaks late is not forgotten after a minute. A class with enough samples whose median active time is under `quickMilliseconds` is a quick rebuild. */
export function estimateHostRatioV1(policy:HostCapacityPolicyV1,store:HostEstimateStoreV1,lineage:readonly string[],nowMs:number):HostEstimateV1{
 const rules=policy.estimates;
 let ratio=DEFAULT_BASIS_POINTS,ramp=rules.defaultRampMilliseconds,learned=0,quick=false;
 for(const key of lineage){
  const samples=store.classes[key]?.samples??[],weight=weights(policy,samples,nowMs),total=weight.reduce((sum,row)=>sum+row,0);
  if(samples.length===0||total===0){ratio=Math.min(DEFAULT_BASIS_POINTS,ratio);ramp=Math.min(rules.defaultRampMilliseconds,ramp);quick=false;continue;}
  const effective=total/UNIT,peak=round(quantile(samples,weight,"ratioBasisPoints",rules.quantileBasisPoints)*(DEFAULT_BASIS_POINTS+rules.marginBasisPoints)/DEFAULT_BASIS_POINTS),trust=Math.min(1,effective/rules.minimumSamples);
  const late=round(quantile(samples,weight,"peakAtMilliseconds",rules.quantileBasisPoints)*(DEFAULT_BASIS_POINTS+rules.marginBasisPoints)/DEFAULT_BASIS_POINTS);
  ratio=round(trust*peak+(1-trust)*Math.min(DEFAULT_BASIS_POINTS,ratio));ramp=round(trust*late+(1-trust)*Math.min(rules.defaultRampMilliseconds,ramp));learned=round(effective);
  quick=effective>=rules.minimumSamples&&rules.quickMilliseconds>0&&quantile(samples,weight,"activeMilliseconds",5000)<=rules.quickMilliseconds;
 }
 return Object.freeze({ratioBasisPoints:Math.max(rules.minimumRatioBasisPoints,Math.min(rules.maximumRatioBasisPoints,ratio)),rampMilliseconds:Math.max(policy.memory.rampMilliseconds,Math.min(rules.maximumRampMilliseconds,ramp)),samples:learned,quick});
}

/** 🧢️ The most a class may ask for by declaration: the smallest `megabytes` of the ceilings whose pattern matches the class key, undefined when none does. */
export function hostClassCeilingV1(policy:HostCapacityPolicyV1,key:string):number|undefined{
 const rows=policy.classCeilings.filter(row=>new RegExp(row.pattern,"u").test(key));
 return rows.length===0?undefined:Math.min(...rows.map(row=>row.megabytes));
}

/** 📏️ The most a class has measurably needed: the decayed quantile of the absolute peaks its OWN runs kept (the last element of the lineage; the pooled peaks of the classes above it are those of other products and never bound this one) plus the margin, only once `minimumSamples` effective runs back it; this bound does not depend on the number of jobs a run was granted, which the learned ratio does. */
export function measuredHostPeakV1(policy:HostCapacityPolicyV1,store:HostEstimateStoreV1,lineage:readonly string[],nowMs:number):number|undefined{
 const rules=policy.estimates,rows=(store.classes[lineage[lineage.length-1]!]?.samples??[]).filter(sample=>sample.peakMegabytes!==undefined);
 if(rows.length===0)return undefined;
 const weight=weights(policy,rows,nowMs);
 return weight.reduce((sum,row)=>sum+row,0)/UNIT<rules.minimumSamples?undefined:round(quantile(rows,weight,"peakMegabytes",rules.quantileBasisPoints)*(DEFAULT_BASIS_POINTS+rules.marginBasisPoints)/DEFAULT_BASIS_POINTS);
}

/** 🧱️ The most a class may ask for in full and where that number comes from: the peak the class measured (and its declared ceiling when that is smaller), the declared ceiling while it has measured too little, and `estimates.fallbackCeilingMegabytes` for a class that is neither measured nor declared; never 200 % of its default, so a run is admissible once its real peak fits the machine. */
export function hostClassBoundV1(policy:HostCapacityPolicyV1,store:HostEstimateStoreV1,lineage:readonly string[],nowMs:number):Readonly<{megabytes:number;source:"measured"|"declared"|"fallback"}>{
 const declared=hostClassCeilingV1(policy,lineage[lineage.length-1]!),measured=measuredHostPeakV1(policy,store,lineage,nowMs);
 return Object.freeze(measured!==undefined?{megabytes:Math.min(measured,declared??measured),source:"measured" as const}:declared!==undefined?{megabytes:declared,source:"declared" as const}:{megabytes:policy.estimates.fallbackCeilingMegabytes,source:"fallback" as const});
}

/** ✍️ Records one finished run in every class of its lineage, keeping the newest samples of each class and the most recently used classes of the store; a pure function. */
export function recordHostOutcomeV1(policy:HostCapacityPolicyV1,store:HostEstimateStoreV1,lineage:readonly string[],outcome:HostOutcomeV1):HostEstimateStoreV1{
 const rules=policy.estimates,ratio=Math.max(1,round(outcome.peakMegabytes*10000/outcome.defaultMegabytes)),classes:Record<string,{samples:HostEstimateSampleV1[]}>={};
 for(const [key,value] of Object.entries(store.classes))classes[key]={samples:[...value.samples]};
 for(const key of lineage){
  const rows=(classes[key]??={samples:[]}).samples;
  rows.push({atMs:outcome.endedAtMs,ratioBasisPoints:ratio,activeMilliseconds:outcome.activeMilliseconds,peakAtMilliseconds:Math.min(outcome.peakAtMilliseconds,outcome.activeMilliseconds),...outcome.keepPeak===true?{peakMegabytes:Math.max(1,round(outcome.peakMegabytes))}:{}});
  rows.sort((left,right)=>right.atMs-left.atMs||left.ratioBasisPoints-right.ratioBasisPoints||left.activeMilliseconds-right.activeMilliseconds||(left.peakAtMilliseconds??left.activeMilliseconds)-(right.peakAtMilliseconds??right.activeMilliseconds));
  rows.length=Math.min(rows.length,rules.maximumSamples);
 }
 const newest=(key:string)=>Math.max(...classes[key]!.samples.map(row=>row.atMs)),kept=Object.keys(classes).sort((left,right)=>newest(right)-newest(left)||(left<right?-1:1)).slice(0,rules.maximumClasses).sort();
 return Object.freeze({version:1,classes:Object.freeze(Object.fromEntries(kept.map(key=>[key,Object.freeze({samples:Object.freeze(classes[key]!.samples)})])))});
}

/** 📐️ Sizes a request at its learned share: the reservation (fixed block and per job) shrinks or grows with the ratio, never under `minimumMegabytes` (or the constant `floorMegabytes` the learned assembly and plan phases need, which does not scale with the jobs) in total and never over `maximumShareBasisPoints` of the memory above the floor (`capacityMegabytes`), so a learned peak can never ask for more than an idle machine could ever grant; the envelope the build tool may use per job stays at the default, so a tool is never starved by an estimate. */
export function scaleHostRequestV1(policy:HostCapacityPolicyV1,request:HostRequestV1,ratioBasisPoints:number,capacityMegabytes:number,floorMegabytes:number=0):HostRequestV1{
 const rules=policy.estimates,perJob=round(request.perJobMegabytes*ratioBasisPoints/10000);
 let fixed=round(request.fixedMegabytes*ratioBasisPoints/10000);
 fixed+=Math.max(0,Math.max(rules.minimumMegabytes,floorMegabytes)-(fixed+request.minimumJobs*perJob));
 const ceiling=Math.floor(capacityMegabytes*rules.maximumShareBasisPoints/10000),need=fixed+request.minimumJobs*perJob,shrink=need>ceiling?(value:number)=>Math.floor(value*ceiling/need):(value:number)=>value;
 return requireHostCapacityV1<HostRequestV1>("Request",Object.freeze({...request,fixedMegabytes:shrink(fixed),perJobMegabytes:shrink(perJob),envelopePerJobMegabytes:request.perJobMegabytes}));
}

/** ⏱️ Gives a scaled request the time it needs to reach its peak and, when its reservation was shrunk below the default (fixed block or per-job share), a ramp slot: only `memory.maximumRamping` such reservations may grow towards their peak at once, so a wrong estimate overshoots with a bounded number of runs. */
export function rampHostRequestV1(base:HostRequestV1,scaled:HostRequestV1,estimate:HostEstimateV1):HostRequestV1{
 return requireHostCapacityV1<HostRequestV1>("Request",Object.freeze({...scaled,rampMilliseconds:estimate.rampMilliseconds,rampSlot:scaled.fixedMegabytes<base.fixedMegabytes||scaled.perJobMegabytes<base.perJobMegabytes}));
}

/** 🧹️ Whether a finished run may teach its class: a completed run always, a failed or cancelled run only when the partial peak it reached already exceeds the current estimate, so it can only raise a reservation and never lower one with a run that died early. */
export function learnableHostRunV1(status:HostRunStatusV1,peakRatioBasisPoints:number,estimate:Pick<HostEstimateV1,"ratioBasisPoints">):boolean{
 return status==="ok"||peakRatioBasisPoints>estimate.ratioBasisPoints;
}

/** 🥇️ Who is served before the batch: a request the operation marks interactive, a process whose lease standing is interactive (a person at a terminal, the same class the lease queue serves first), or a class learned to be a quick rebuild; a stream of agent suites never delays them, and aging still keeps every batch request from starving. */
export function hostRequestPriorityV1(base:HostRequestV1["priority"],standing:"interactive"|"agent"|"background"|undefined,estimate:HostEstimateV1):HostRequestV1["priority"]{
 return base==="interactive"||standing==="interactive"||estimate.quick?"interactive":"batch";
}

const storePath=(directory:string)=>join(directory,schema.definitions.Protocol.const.estimatesFile);

/** 📖️ Reads the store of a gate directory; a missing, oversized or malformed file is an empty store, because learning is an optimisation that must never fail a run. */
export function readHostEstimateStoreV1(directory:string):HostEstimateStoreV1{
 try{
  if(statSync(storePath(directory)).size>schema.definitions.Protocol.const.maximumEstimateBytes)return emptyHostEstimateStoreV1();
  const value:unknown=JSON.parse(readFileSync(storePath(directory),"utf8"));
  return isHostCapacityV1("EstimateStore",value)&&(value as {measure?:string}).measure===schema.definitions.Observers.const.measure?value as HostEstimateStoreV1:emptyHostEstimateStoreV1();
 }catch{return emptyHostEstimateStoreV1();}
}

/** 🔏️ Applies one update to the stored estimates under a short directory lock and an atomic replace; when the lock cannot be taken within the bound the sample is dropped, never the run. The replace of a store another run holds open (a Windows sharing violation) is retried on the churn pauses; a store that stays busy drops the sample, removes the temporary file and says so once in every language. */
export async function updateHostEstimateStoreV1(directory:string,policy:HostCapacityPolicyV1,update:(store:HostEstimateStoreV1)=>HostEstimateStoreV1,ports:Readonly<{now?:()=>number;pid?:number;sleep?:(milliseconds:number)=>Promise<void>;platform?:string;sleepSync?:(milliseconds:number)=>void;rename?:(from:string,to:string)=>void}>={}):Promise<boolean>{
 const now=ports.now??Date.now,pause=ports.sleep??(milliseconds=>new Promise<void>(done=>setTimeout(done,milliseconds))),lock=join(directory,"estimates.lock"),deadline=now()+policy.wait.lockStaleMilliseconds;
 mkdirSync(directory,{recursive:true,mode:0o700});
 for(;;){
  try{mkdirSync(lock,{mode:0o700});break;}
  catch(error){
   if((error as NodeJS.ErrnoException).code!=="EEXIST")return false;
   try{if(now()-statSync(lock).mtimeMs>policy.wait.lockStaleMilliseconds)rmSync(lock,{recursive:true,force:true});}catch{}
   if(now()>=deadline)return false;
   await pause(25);
  }
 }
 try{
  const next=requireHostCapacityV1<HostEstimateStoreV1>("EstimateStore",update(readHostEstimateStoreV1(directory))),text=JSON.stringify({...next,measure:schema.definitions.Observers.const.measure}),temporary=`${storePath(directory)}.${ports.pid??process.pid}.tmp`;
  if(text.length>schema.definitions.Protocol.const.maximumEstimateBytes)return false;
  mkdirSync(dirname(temporary),{recursive:true});writeFileSync(temporary,text);
  const replaced=settleBusyFilesystemSyncV1(storePath(directory),()=>(ports.rename??renameSync)(temporary,storePath(directory)),{report:line=>{try{process.stderr.write(line+"\n");}catch{}},...ports.platform?{platform:ports.platform}:{},...ports.sleepSync?{sleepSync:ports.sleepSync}:{}});
  if(!replaced.settled){rmSync(temporary,{force:true});return false;}
  return true;
 }catch{return false;}
 finally{rmSync(lock,{recursive:true,force:true});}
}
