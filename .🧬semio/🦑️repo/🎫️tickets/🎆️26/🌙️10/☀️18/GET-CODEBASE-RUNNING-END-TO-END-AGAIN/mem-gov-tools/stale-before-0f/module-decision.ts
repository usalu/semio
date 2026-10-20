import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 🌡️ The authored action kinds a job estimate is keyed by; identical to the worker pool vocabulary, never a build tool name. */
export type HostActionKindV1="test"|"check"|"generate"|"build"|"script"|"service";
/** 📐️ One kind's estimate: cores it keeps busy and megabytes it holds at its peak. */
export type HostResourceV1=Readonly<{cpu:number;megabytes:number}>;
/** 🧭️ How one operation sizes its request: `tasks` multiplies per-job estimates by the job count, `fixed` reserves one block. */
export type HostOperationV1=Readonly<{name:string;shape:"tasks"|"fixed";kind?:HostActionKindV1;fixedMegabytes:number;cpu:number;priority:"batch"|"interactive"}>;
/** 📜️ The authored host capacity policy (`Policy` of the schema). */
export type HostCapacityPolicyV1=Readonly<{version:1;enabled:boolean;memory:Readonly<{floorBasisPoints:number;floorMinimumMegabytes:number;floorMaximumMegabytes:number;budgetBasisPoints:number;rampMilliseconds:number}>;swap:Readonly<{ceilingBasisPoints:number;outMegabytesPerSecond:number}>;pressure:Readonly<{psiWarnPercent:number;psiCriticalPercent:number;availableWarnBasisPoints:number;availableCriticalBasisPoints:number}>;load:Readonly<{perCoreCeiling:number;overrideAfterMilliseconds:number}>;files:Readonly<{ceilingBasisPoints:number}>;attaches:Readonly<{maximum:number}>;wait:Readonly<{pollMilliseconds:number;progressMilliseconds:number;heartbeatMilliseconds:number;staleMilliseconds:number;lockStaleMilliseconds:number;sampleMaximumAgeMilliseconds:number;maximumWaiters:number;bypassMilliseconds:number;maximumBypasses:number;agingMilliseconds:number}>;kinds:Readonly<Record<HostActionKindV1,HostResourceV1>>;operations:readonly HostOperationV1[]}>;
/** 🖥️ One sample of the host (`Host` of the schema); every platform reduces to these fields, `null` marking a counter the platform does not offer: the machine-wide open file table (used and maximum) and the number of attached disk images join memory, swap and load. */
export type HostSampleV1=Readonly<{version:1;atMs:number;platform:"darwin"|"linux"|"win32";cores:number;totalMegabytes:number;availableMegabytes:number;pressure:"normal"|"warn"|"critical";swapUsedMegabytes:number|null;swapOutMegabytesPerSecond:number|null;load:number|null;openFiles:number|null;openFilesMaximum:number|null;attachedImages:number|null}>;
/** 🎫️ What one operation asks for (`Request` of the schema). */
export type HostRequestV1=Readonly<{id:string;operation:string;priority:"batch"|"interactive";minimumJobs:number;preferredJobs:number;fixedMegabytes:number;perJobMegabytes:number;cpuPerJob:number}>;
/** ⏳️ A request in the queue (`Waiter` of the schema). */
export type HostWaiterV1=Readonly<{request:HostRequestV1;arrivedAtMs:number;pid:number}>;
/** 🔐️ A live admission (`Grant` of the schema); `observedMegabytes` is what its process tree holds right now. */
export type HostGrantV1=Readonly<{id:string;operation:string;jobs:number;cpu:number;reservedMegabytes:number;admittedAtMs:number;observedMegabytes:number|null;pid:number;bootAtMs:number}>;
/** 🧾️ Why a request was admitted or kept waiting. */
export type HostReasonV1="fits"|"reduced"|"override"|"memory"|"swap"|"swap-activity"|"pressure-critical"|"pressure-warn"|"files"|"attaches"|"load"|"queue-order";
/** ⚖️ The verdict for one request. */
export type HostDecisionV1=Readonly<{id:string;verdict:"admit"|"wait";jobs:number;reservedMegabytes:number;reason:HostReasonV1}>;
/** 📊️ The whole evaluation: the derived headroom and one decision per waiter in service order. */
export type HostEvaluationV1=Readonly<{floorMegabytes:number;unrealizedMegabytes:number;headroomMegabytes:number;binding:"available"|"budget";loadRoom:number|null;decisions:readonly HostDecisionV1[]}>;
/** 🧩️ The tool-neutral resource envelope a build tool renders into its own flags. */
export type HostPlanV1=Readonly<{version:1;id:string;jobs:number;cpu:number;memoryMegabytes:number;fixedMegabytes:number;floorMegabytes:number}>;
/** 🏷️ Names of the closed definitions a value can be required to match. */
export type HostDefinitionV1="Policy"|"Host"|"Request"|"Waiter"|"Grant"|"Decision"|"Evaluation"|"Plan";

/** 🛂️ Requires a value to match one closed definition of the host capacity schema. */
export function requireHostCapacityV1<T>(definition:HostDefinitionV1,value:unknown):T{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value);
 if(errors.length)throw Error(`Closed host capacity ${definition} required: ${errors[0]}`);
 return value as T;
}

/** 🔍️ Answers whether a value matches one closed definition without throwing. */
export function isHostCapacityV1(definition:HostDefinitionV1,value:unknown):boolean{
 return validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value).length===0;
}

/** 📖️ Reads the authored policy and refuses anything the closed schema or its ordering rules reject. */
export function readHostCapacityPolicyV1(value:unknown):HostCapacityPolicyV1{
 const policy=requireHostCapacityV1<HostCapacityPolicyV1>("Policy",value);
 if(policy.memory.floorMinimumMegabytes>policy.memory.floorMaximumMegabytes)throw Error("Host capacity floor minimum exceeds its maximum");
 if(policy.pressure.psiWarnPercent>policy.pressure.psiCriticalPercent||policy.pressure.availableCriticalBasisPoints>policy.pressure.availableWarnBasisPoints)throw Error("Host capacity pressure thresholds must rise from warn to critical");
 return policy;
}

/** 🧱️ The free-memory floor of one host: a share of its memory between the authored bounds, never above what the host has. */
export function hostFloorMegabytesV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes">):number{
 const share=Math.floor(host.totalMegabytes*policy.memory.floorBasisPoints/10000);
 return Math.min(host.totalMegabytes,Math.max(policy.memory.floorMinimumMegabytes,Math.min(policy.memory.floorMaximumMegabytes,share)));
}

/** 🔢️ Orders waiters for service: interactive before batch, then by arrival, then by id. */
export function orderHostWaitersV1(waiters:readonly HostWaiterV1[]):HostWaiterV1[]{
 const rank=(waiter:HostWaiterV1)=>waiter.request.priority==="interactive"?0:1;
 return [...waiters].sort((left,right)=>rank(left)-rank(right)||left.arrivedAtMs-right.arrivedAtMs||(left.request.id<right.request.id?-1:left.request.id>right.request.id?1:0));
}

/** 🚧️ Names what holds every admission back without being a hard stop: raised memory pressure, an open file table above its ceiling share or as many attached disk images as the maximum; undefined when none applies. These can be waited out, and are overridden only when nothing of ours is running. */
export function softHostReasonV1(policy:HostCapacityPolicyV1,host:HostSampleV1):"pressure-warn"|"files"|"attaches"|undefined{
 if(host.pressure==="warn")return "pressure-warn";
 if(host.openFiles!==null&&host.openFilesMaximum!==null&&host.openFiles*10000>host.openFilesMaximum*policy.files.ceilingBasisPoints)return "files";
 if(host.attachedImages!==null&&host.attachedImages>=policy.attaches.maximum)return "attaches";
 return undefined;
}

/** 🧮️ Decides, for a host sample, the live grants and the queue, which requests start now and with how many jobs; a pure function of its arguments. Reservations count in full only while a grant ramps up (`memory.rampMilliseconds`), then the memory it really holds speaks for itself. A waiter refused for memory or load does not hold later ones back: they pass while they fit. It becomes a strict barrier only if finishing our own grants would admit it (foreign pressure never creates a barrier, draining cannot help it) and it has waited `wait.bypassMilliseconds` or been overtaken `wait.maximumBypasses` times, counting grants admitted since it arrived; so it cannot starve. A batch waiter older than `wait.agingMilliseconds` ranks as interactive, so a stream of interactive requests cannot starve it. */
export function evaluateHostCapacityV1(policy:HostCapacityPolicyV1,host:HostSampleV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],nowMs:number):HostEvaluationV1{
 const floor=hostFloorMegabytesV1(policy,host),cap=Math.max(0,host.totalMegabytes-floor);
 const ramping=(grant:HostGrantV1)=>nowMs-grant.admittedAtMs<policy.memory.rampMilliseconds;
 const unrealized=ledger.reduce((sum,grant)=>sum+(ramping(grant)?Math.max(0,grant.reservedMegabytes-(grant.observedMegabytes??0)):0),0),committed=ledger.reduce((sum,grant)=>sum+(ramping(grant)?grant.reservedMegabytes:grant.observedMegabytes??grant.reservedMegabytes),0);
 const observed=host.availableMegabytes-floor-unrealized,budget=Math.floor(cap*policy.memory.budgetBasisPoints/10000)-committed,binding=budget<observed?"budget" as const:"available" as const;
 let room=Math.min(observed,budget);
 const headroom=room,ledgerCpu=ledger.reduce((sum,grant)=>sum+grant.cpu,0),ceiling=policy.load.perCoreCeiling*host.cores;
 let loadRoom=host.load===null&&ledgerCpu===0?Infinity:ceiling-Math.max(host.load??0,ledgerCpu);
 const reportedLoadRoom=loadRoom===Infinity?null:loadRoom;
 const swapCeiling=Math.floor(host.totalMegabytes*policy.swap.ceilingBasisPoints/10000);
 const swapping=host.swapOutMegabytesPerSecond===null||host.swapOutMegabytesPerSecond>0;
 const soft=softHostReasonV1(policy,host);
 const hard:HostReasonV1|undefined=host.pressure==="critical"?"pressure-critical":host.swapUsedMegabytes!==null&&host.swapUsedMegabytes>swapCeiling&&swapping?"swap":host.swapOutMegabytesPerSecond!==null&&host.swapOutMegabytesPerSecond>policy.swap.outMegabytesPerSecond?"swap-activity":undefined;
 const drainedRoom=Math.min(host.availableMegabytes+ledger.reduce((sum,grant)=>sum+(grant.observedMegabytes??0),0),host.totalMegabytes)-floor,drainedBudget=Math.floor(cap*policy.memory.budgetBasisPoints/10000),drainedLoadRoom=host.load===null?Infinity:ceiling-Math.max(0,host.load-ledgerCpu);
 const aged=waiters.map(waiter=>waiter.request.priority==="batch"&&nowMs-waiter.arrivedAtMs>=policy.wait.agingMilliseconds?Object.freeze({...waiter,request:Object.freeze({...waiter.request,priority:"interactive" as const})}):waiter);
 const decisions:HostDecisionV1[]=[],heads:{bypasses:number}[]=[];let blocked=false,admitted=0;
 for(const waiter of orderHostWaitersV1(aged)){
  const request=waiter.request,need=(jobs:number)=>Math.min(request.fixedMegabytes+jobs*request.perJobMegabytes,cap),waited=nowMs-waiter.arrivedAtMs;
  const refuse=(reason:HostReasonV1):void=>{decisions.push(Object.freeze({id:request.id,verdict:"wait",jobs:0,reservedMegabytes:0,reason}));};
  const admit=(jobs:number,reason:HostReasonV1):void=>{const reserved=need(jobs);room-=reserved;if(loadRoom!==Infinity)loadRoom-=jobs*request.cpuPerJob;admitted++;for(const head of heads){head.bypasses++;if(head.bypasses>=policy.wait.maximumBypasses)blocked=true;}decisions.push(Object.freeze({id:request.id,verdict:"admit",jobs,reservedMegabytes:reserved,reason}));};
  if(hard){refuse(hard);continue;}
  if(blocked){refuse("queue-order");continue;}
  const memoryFits=need(request.minimumJobs)<=room;
  let jobs=0;
  if(soft===undefined)for(let candidate=request.preferredJobs;candidate>=request.minimumJobs;candidate--)if(need(candidate)<=room&&candidate*request.cpuPerJob<=loadRoom){jobs=candidate;break;}
  if(jobs>0){admit(jobs,jobs<request.preferredJobs?"reduced":"fits");continue;}
  const reason:HostReasonV1=!memoryFits?"memory":soft??"load";
  if(reason!=="memory"&&ledger.length===0&&admitted===0&&waited>=policy.load.overrideAfterMilliseconds)admit(request.minimumJobs,"override");
  else{
   refuse(reason);
   if(reason==="memory"||reason==="load"){
    const drains=soft===undefined&&need(request.minimumJobs)<=Math.min(drainedRoom,drainedBudget)&&request.minimumJobs*request.cpuPerJob<=drainedLoadRoom,bypasses=ledger.filter(grant=>grant.admittedAtMs>=waiter.arrivedAtMs).length;
    if(drains&&(waited>=policy.wait.bypassMilliseconds||bypasses>=policy.wait.maximumBypasses))blocked=true;else if(drains)heads.push({bypasses});
   }
  }
 }
 return Object.freeze({floorMegabytes:floor,unrealizedMegabytes:unrealized,headroomMegabytes:headroom,binding,loadRoom:reportedLoadRoom,decisions:Object.freeze(decisions)});
}

/** 📏️ Sizes the request of one named operation, or answers undefined when the operation is light and passes ungated. */
export function sizeHostRequestV1(policy:HostCapacityPolicyV1,input:Readonly<{id:string;operation:string;preferredJobs?:number;kinds?:readonly HostActionKindV1[]}>):HostRequestV1|undefined{
 const operation=policy.operations.find(row=>row.name===input.operation);
 if(!operation)return undefined;
 if(operation.shape==="fixed")return requireHostCapacityV1<HostRequestV1>("Request",Object.freeze({id:input.id,operation:operation.name,priority:operation.priority,minimumJobs:1,preferredJobs:1,fixedMegabytes:operation.fixedMegabytes,perJobMegabytes:0,cpuPerJob:operation.cpu}));
 const kinds=[...new Set<HostActionKindV1>([...operation.kind?[operation.kind]:input.kinds??[],"build"])],rows=kinds.map(kind=>policy.kinds[kind]);
 return requireHostCapacityV1<HostRequestV1>("Request",Object.freeze({id:input.id,operation:operation.name,priority:operation.priority,minimumJobs:1,preferredJobs:Math.max(1,Math.min(input.preferredJobs??1,1024)),fixedMegabytes:operation.fixedMegabytes,perJobMegabytes:Math.max(...rows.map(row=>row.megabytes)),cpuPerJob:Math.max(...rows.map(row=>row.cpu))}));
}

/** 🎁️ Renders an admitted request into the tool-neutral envelope: jobs, cores, and the memory the jobs may hold beside the fixed block. */
export function planHostCapacityV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes">,request:HostRequestV1,jobs:number):HostPlanV1{
 if(!Number.isSafeInteger(jobs)||jobs<request.minimumJobs||jobs>request.preferredJobs)throw Error("Host capacity plan needs jobs between the request bounds");
 return requireHostCapacityV1<HostPlanV1>("Plan",Object.freeze({version:1,id:request.id,jobs,cpu:jobs*request.cpuPerJob,memoryMegabytes:jobs*request.perJobMegabytes,fixedMegabytes:request.fixedMegabytes,floorMegabytes:hostFloorMegabytesV1(policy,host)}));
}

/** 🗣️ Picks the message language: `SEMIO_LANGUAGE`, then the locale, then the first catalogue language. */
export function resolveHostCapacityLanguageV1(environment:Readonly<Record<string,string|undefined>>):"en"|"de"{
 for(const value of [environment.SEMIO_LANGUAGE,environment.LC_ALL,environment.LC_MESSAGES,environment.LANG]){const tag=value?.toLowerCase().split(/[._@-]/u)[0];if(tag==="en"||tag==="de")return tag;}
 return "en";
}

/** 🖨️ Fills one catalogue message of the chosen language; unknown placeholders stay visible. */
export function formatHostCapacityV1(language:"en"|"de",name:"waiting"|"admitted"|"reduced"|"override"|"bypassed"|"status"|"grant"|"waiter",values:Readonly<Record<string,string|number>>):string{
 return schema.definitions.Messages.const[language][name].replace(/\{([a-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);
}

/** 💬️ Names a wait or override reason in the chosen language. */
export function describeHostReasonV1(language:"en"|"de",reason:Exclude<HostReasonV1,"fits"|"reduced"|"override">):string{
 return schema.definitions.Messages.const[language].reasons[reason];
}
