import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {leasePriorityClassV1,type LeasePriorityV1} from "./⚖️priority/🟦️.ts";
import {createScriptProcessActivitySamplerV1} from "../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";

export {leasePriorityClassV1,leaseTicketArrivalV1,leaseRealArrivalV1,leaseOvertakeWindowV1,simulateLeaseQueueV1,type LeasePriorityV1} from "./⚖️priority/🟦️.ts";
export {leaseEtaV1,leaseMedianV1,type LeaseEtaV1} from "./⏱️eta/🟦️.ts";

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const;
/** 🪪️ What a process knows about the code it runs: a digest of its declared format-stamp components and when it loaded them. */
export type LeaseImplementationV1=Readonly<{digest:string;loadedAtMs:number;components:Readonly<Record<string,string>>}>;
/** 📈️ What a lease holder publishes about itself in the lease registry. */
export type LeaseStandingV1=Readonly<{version:1;priority:LeasePriorityV1;implementation:LeaseImplementationV1;phase?:string;done?:number;total?:number;phaseStartedAtMs?:number;progressAtMs:number;parentPid?:number;chain?:readonly number[]}>;
/** 🚦️ The judgement of a holder: legacy, progressing, silent, stuck or stale, with the silent time or the ids of the moved components. */
export type LeaseHolderStateV1=Readonly<{state:"legacy"|"progressing"|"silent"|"stuck"|"stale";silentMs?:number;staleIds?:readonly string[]}>;
/** 📜️ One progress line of this process: its stage or the resource of its current activity. */
export type LeaseProgressLineV1=Readonly<{stage?:string;activity?:Readonly<{resource:string}>}>;

/** 🔌️ The owner of the format stamps tells the lease layer which components identify the running code, which of them bind a resource (its result is discarded when they moved) and which of those moved on disk since this process loaded them (`moved`, read from the disk now; a table that cannot be read answers none, so nothing is ever yielded on a guess). */
export type LeaseImplementationProviderV1=Readonly<{components():Readonly<Record<string,string>>;bound(resource:string):readonly string[];moved?(resource:string):readonly string[]}>;
const providerKey=Symbol.for("semio.leaseImplementation.v1");
const global=globalThis as Record<symbol,LeaseImplementationProviderV1|undefined>;
/** 📝️ Registers the implementation provider of this process. */
export function registerLeaseImplementationProviderV1(provider:LeaseImplementationProviderV1):void{global[providerKey]=provider;}
const loadedAtMs=Math.round(performance.timeOrigin);

/** 🧬️ The implementation identity of this process; without a provider it has no components and is never older than anything. */
export function leaseImplementationV1():LeaseImplementationV1{
 const components=Object.freeze({...global[providerKey]?.components()}),sorted=Object.fromEntries(Object.keys(components).sort().map(key=>[key,components[key]]));
 return Object.freeze({digest:createHash("sha256").update(JSON.stringify(sorted)).digest("hex").slice(0,16),loadedAtMs,components});
}
/** 🔗️ The component ids that bind a resource: the stamps whose movement makes a holder's result obsolete. */
export function leaseBoundComponentsV1(resource:string):readonly string[]{return global[providerKey]?.bound(resource)??[];}
/** 💽️ The bound component ids of a resource whose stamp on disk differs from the value this process loaded: what a holder re-reads itself instead of trusting a requester's claim. */
export function leaseMovedBoundV1(resource:string):readonly string[]{try{return global[providerKey]?.moved?.(resource)??[];}catch{return [];}}

const families=policy.progress.families.map(family=>({prefix:family.prefix,pattern:new RegExp(family.pattern,"u")})),counter=new RegExp(policy.progress.counter,"u");
/** 🔎️ Reads a progress line into phase and counters: the declared families first, then a trailing counter, else the resource name itself. */
export function parseLeaseProgressV1(resource:string):Readonly<{phase:string;done?:number;total?:number}>|undefined{
 const rules=policy.progress;
 if(rules.ignoredPrefixes.some(prefix=>resource.startsWith(prefix)))return undefined;
 for(const family of families)if(resource.startsWith(family.prefix)){const match=family.pattern.exec(resource);if(match)return Object.freeze({phase:match[1]!,done:Number(match[2]),total:Number(match[3])});}
 const match=counter.exec(resource);
 if(match)return Object.freeze({phase:match[1]!.slice(0,rules.maximumPhaseCharacters),done:Number(match[2])});
 return Object.freeze({phase:resource.slice(0,rules.maximumPhaseCharacters)});
}

type State={phase?:string;done?:number;total?:number;phaseStartedAtMs?:number;progressAtMs:number;notifiedAtMs:number;probedAtMs?:number;listener?:()=>void;priority?:LeasePriorityV1};
const state:State={progressAtMs:Date.now(),notifiedAtMs:0};
/** 👂️ The registry asks to hear about every published change of the standing. */
export function observeLeaseStandingV1(listener:()=>void):void{state.listener=listener;}

/** 📣️ Notes one progress line of this process: any line is liveness, a changed phase or counter updates the report, and listeners hear about it at most once per publish interval (at once for a new phase). */
export function noteLeaseProgressV1(line:LeaseProgressLineV1,nowMs=Date.now()):void{
 state.progressAtMs=nowMs;
 const parsed=line.activity?parseLeaseProgressV1(line.activity.resource):line.stage&&state.phase===undefined?{phase:line.stage}:undefined;
 let changed=false;
 if(parsed){
  if(parsed.phase!==state.phase){state.phase=parsed.phase;state.phaseStartedAtMs=nowMs;changed=true;}
  if(parsed.done!==state.done||parsed.total!==state.total){state.done=parsed.done;state.total=parsed.total;}
 }
 if(state.listener&&(changed||nowMs-state.notifiedAtMs>=policy.progress.publishIntervalMilliseconds)){state.notifiedAtMs=nowMs;state.listener();}
}

/** 🌡️ What one activity probe saw: whether the descendant tree of this process used CPU time since the last probe, and how many milliseconds of CPU this process itself used. */
export type LeaseActivitySampleV1=Readonly<{treeActive:boolean;cpuDeltaMilliseconds:number}>;
/** 🔬️ The pure rule of liveness by work: a probe is due once `freshness.activityProbeMilliseconds` passed since the last one; a tree that used CPU, or a process that used at least `activityMinimumCpuMilliseconds` itself, counts as progress made now, so a holder compiling for an hour without a line of output is never stuck, while a holder whose whole tree idles ages like before. The sample is read only when a probe is due. */
export function leaseActivityV1(input:Readonly<{nowMs:number;progressAtMs:number;lastProbeAtMs?:number;sample:()=>LeaseActivitySampleV1}>):Readonly<{sampled:boolean;progressAtMs:number}>{
 const rules=policy.freshness;
 if(input.lastProbeAtMs!==undefined&&input.nowMs-input.lastProbeAtMs<rules.activityProbeMilliseconds)return Object.freeze({sampled:false,progressAtMs:input.progressAtMs});
 const seen=input.sample();
 return Object.freeze({sampled:true,progressAtMs:seen.treeActive||seen.cpuDeltaMilliseconds>=rules.activityMinimumCpuMilliseconds?Math.max(input.progressAtMs,input.nowMs):input.progressAtMs});
}
/** 🔧️ The two things a probe reads: the descendant tree's CPU activity (the watchdog's own sampler, so both notions of liveness are one) and the CPU time of this process in milliseconds; tests replace them. */
export type LeaseActivityPortsV1=Readonly<{tree():boolean;cpuMilliseconds():number}>;
let activityPorts:LeaseActivityPortsV1|undefined,lastCpuMs:number|undefined;
/** ♻️ Replaces the probe ports (undefined restores the real ones) and forgets the CPU baseline. */
export function setLeaseActivityPortsV1(ports:LeaseActivityPortsV1|undefined):void{activityPorts=ports;lastCpuMs=undefined;state.probedAtMs=undefined;}
/** 📡️ Probes the work of this process and renews its liveness when it did some; the registry calls it before every publish of the standing and the holder before it re-judges itself, so the clock that decides `stuck` is the work the machine sees, not only the lines a process prints. */
export function probeLeaseActivityV1(nowMs=Date.now()):void{
 const ports=activityPorts??={tree:createScriptProcessActivitySamplerV1({note:()=>{}}),cpuMilliseconds:()=>{const used=process.cpuUsage();return (used.user+used.system)/1000;}};
 const result=leaseActivityV1({nowMs,progressAtMs:state.progressAtMs,...state.probedAtMs===undefined?{}:{lastProbeAtMs:state.probedAtMs},sample:()=>{const cpu=ports.cpuMilliseconds(),delta=lastCpuMs===undefined?0:Math.max(0,cpu-lastCpuMs);lastCpuMs=cpu;return {treeActive:ports.tree(),cpuDeltaMilliseconds:delta};}});
 if(result.sampled){state.probedAtMs=nowMs;state.progressAtMs=result.progressAtMs;}
}

/** 🏷️ Overrides the resolved class for this process (a caller that knows its work is unattended). */
export function setLeasePriorityV1(priority:LeasePriorityV1|undefined):void{state.priority=priority;}
/** 🎖️ The class of this process; `agentSession` says whether it runs inside an agent session. */
export function currentLeasePriorityV1(agentSession:boolean,environment:Readonly<Record<string,string|undefined>>=process.env):LeasePriorityV1{
 return state.priority??leasePriorityClassV1(environment,{agentSession,interactiveTerminal:process.stdin.isTTY===true&&process.stdout.isTTY===true});
}

/** 📸️ The standing this process publishes now. */
export function leaseStandingSnapshotV1(agentSession:boolean,nowMs=Date.now(),environment:Readonly<Record<string,string|undefined>>=process.env):LeaseStandingV1{
 const chain=(environment.SEMIO_BUILD_INVOCATION_PIDS??"").split(",").map(Number).filter(pid=>Number.isSafeInteger(pid)&&pid>0).slice(-16);
 return Object.freeze({version:1,priority:currentLeasePriorityV1(agentSession,environment),implementation:leaseImplementationV1(),
  ...state.phase===undefined?{}:{phase:state.phase},...state.done===undefined?{}:{done:state.done},...state.total===undefined?{}:{total:state.total},...state.phaseStartedAtMs===undefined?{}:{phaseStartedAtMs:state.phaseStartedAtMs},
  progressAtMs:Math.min(state.progressAtMs,nowMs),...process.ppid>1?{parentPid:process.ppid}:{},...chain.length?{chain}:{}});
}

/** 🔑️ The kind of a hold for the ETA history: the resource family (name up to its first colon) and the command kind (the first two words of the command that are not flags, paths or labels). */
export function leaseHistoryKeyV1(resource:string,command:string):string{
 const words=command.split(/\s+/u).filter(word=>word.length>0&&!/^[-/@.]/u.test(word)&&!/\.(?:ts|mjs|js)$/u.test(word)).slice(0,2);
 return `${resource.split(":")[0]!.slice(0,48)} ${words.join(" ")}`.trim();
}

/** 🕰️ After how long without progress a holder of this resource counts as stuck: the longest declared prefix wins. */
export function leaseStuckAfterMsV1(resource:string):number{
 return policy.freshness.stuckAfterMilliseconds.filter(row=>resource.startsWith(row.prefix)).sort((left,right)=>right.prefix.length-left.prefix.length)[0]!.milliseconds;
}

/** 📥️ What judging a holder needs: the clock, the hold start, the resource, the holder's standing and the requester's identity with the components that bind the resource. */
export type LeaseHolderStateInputV1=Readonly<{nowMs:number;holdSinceMs:number;resource:string;standing?:LeaseStandingV1;waiter?:LeaseImplementationV1;bound?:readonly string[]}>;
/** 🩺️ Judges a holder: stale when a requester that loaded later runs a different value of a component that binds the resource, stuck or silent when nothing was reported for the declared time, legacy when it publishes no standing at all. Stale wins over stuck wins over silent. */
export function leaseHolderStateV1(input:LeaseHolderStateInputV1):LeaseHolderStateV1{
 const holder=input.standing;
 if(!holder)return Object.freeze({state:"legacy"});
 if(input.waiter&&input.waiter.loadedAtMs>holder.implementation.loadedAtMs){
  const ids=(input.bound??[]).filter(id=>holder.implementation.components[id]!==undefined&&input.waiter!.components[id]!==undefined&&holder.implementation.components[id]!==input.waiter!.components[id]).sort();
  if(ids.length)return Object.freeze({state:"stale",staleIds:ids});
 }
 const silentMs=Math.max(0,input.nowMs-Math.max(holder.progressAtMs,input.holdSinceMs));
 if(silentMs>=leaseStuckAfterMsV1(input.resource))return Object.freeze({state:"stuck",silentMs});
 if(silentMs>=policy.freshness.silentAfterMilliseconds)return Object.freeze({state:"silent",silentMs});
 return Object.freeze({state:"progressing",silentMs});
}

/** 🌐️ One declared message in one language with its placeholders filled. */
export function leaseStandingTextV1(language:"en"|"de",key:keyof typeof messages,values:Readonly<Record<string,string|number>>={}):string{
 return messages[key][language].replace(/\{([A-Za-z]+)\}/gu,(token,name:string)=>name in values?String(values[name]):token);
}

/** 🚪️ What a holder does when asked to give a resource up. The build layer registers one that ends the process the way an outdated implementation ends (exit 75, outermost command restarts). */
export type LeaseYieldHandlerV1=(request:Readonly<{reason:"stale"|"stuck";resource:string;requesterPid:number;message:string}>)=>void;
const handlerKey=Symbol.for("semio.leaseYieldHandler.v1"),rootKey=Symbol.for("semio.leaseRootAbort.v1");
const slots=globalThis as Record<symbol,unknown>;
/** 🪝️ Registers the yield handler of this process. */
export function registerLeaseYieldHandlerV1(handler:LeaseYieldHandlerV1):void{slots[handlerKey]=handler;}
/** 🔍️ The registered yield handler, if any. */
export function leaseYieldHandlerV1():LeaseYieldHandlerV1|undefined{return slots[handlerKey] as LeaseYieldHandlerV1|undefined;}
/** 🛑️ The root invocation tells the lease layer how to abort it with a reason. */
export function registerLeaseRootAbortV1(abort:(reason:unknown)=>void):void{slots[rootKey]=abort;}
/** ⛔️ Aborts the root invocation of this process with the reason; false when none registered. */
export function abortLeaseRootV1(reason:unknown):boolean{const abort=slots[rootKey] as ((reason:unknown)=>void)|undefined;if(!abort)return false;abort(reason);return true;}
