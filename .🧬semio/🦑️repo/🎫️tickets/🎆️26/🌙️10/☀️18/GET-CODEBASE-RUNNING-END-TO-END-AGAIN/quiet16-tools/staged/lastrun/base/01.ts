import {timingSafeEqual} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {resolveLanguageV1,type OsLanguageListV1} from "../../🗣️language/🟦️.ts";

const protocol=schema.definitions.Protocol.const;

/** 📚️ The authored policy of the measured-demand estimates: decay half-life, the quantile of the learned peaks and the margin on it, how many effective samples earn full trust, the bounds of the store, the bounds of a learned size and the active time under which a class counts as a quick, interactive rebuild. */
export type HostEstimatePolicyV1=Readonly<{halfLifeMilliseconds:number;quantileBasisPoints:number;marginBasisPoints:number;minimumSamples:number;maximumSamples:number;maximumClasses:number;minimumRatioBasisPoints:number;maximumRatioBasisPoints:number;minimumMegabytes:number;quickMilliseconds:number;maximumShareBasisPoints:number;fallbackCeilingMegabytes:number;defaultRampMilliseconds:number;maximumRampMilliseconds:number;phaseReferenceMegabytes:number;rebuildPriorMegabytes:number;rebuildShareBasisPoints:number;maximumRebuildSlots:number;quietReferenceMegabytes:number;quietShareBasisPoints:number;quietMinimumSamples:number;quietMaximumMegabytes:number;quietFollowUpMilliseconds:number}>;
/** 🌡️ The authored action kinds a job estimate is keyed by; identical to the worker pool vocabulary, never a build tool name. */
export type HostActionKindV1="test"|"check"|"generate"|"build"|"script"|"service";
/** 📐️ One kind's estimate: cores it keeps busy and megabytes it holds at its peak. */
export type HostResourceV1=Readonly<{cpu:number;megabytes:number}>;
/** 🧰️ The resources a target holds at its peak when it runs several workers of its own (end-to-end suites): a target whose name matches `pattern` is sized at least at `cpu` cores and `megabytes` per job, whatever kind the worker pool classifies it as (`Envelope` of the schema). */
export type HostEnvelopeV1=Readonly<{name:string;pattern:string;cpu:number;megabytes:number}>;
/** 🧢️ The most a class of runs may ever ask for (`Ceiling` of the schema): a declared bound on the class key that holds until the class measured enough runs to be capped by its own peaks. */
export type HostCeilingV1=Readonly<{name:string;pattern:string;megabytes:number}>;
/** 🧭️ How one operation sizes its request: `tasks` multiplies per-job estimates by the job count, `fixed` reserves one block; `forecast` lets a run of the operation hold only a small quiet reservation until work has to run (only where the tool gates its executors). */
export type HostOperationV1=Readonly<{name:string;shape:"tasks"|"fixed";kind?:HostActionKindV1;fixedMegabytes:number;cpu:number;priority:"batch"|"interactive";forecast?:boolean}>;
/** 📜️ The authored host capacity policy (`Policy` of the schema). */
export type HostCapacityPolicyV1=Readonly<{version:1;enabled:boolean;memory:Readonly<{floorBasisPoints:number;floorMinimumMegabytes:number;floorMaximumMegabytes:number;budgetBasisPoints:number;rampMilliseconds:number;maximumRamping:number;maximumQuiet:number;quietSoftStopGrants:number}>;swap:Readonly<{ceilingBasisPoints:number;outMegabytesPerSecond:number}>;pressure:Readonly<{psiWarnPercent:number;psiCriticalPercent:number;availableWarnBasisPoints:number;availableCriticalBasisPoints:number;compressedWarnBasisPoints:number;compressedCriticalBasisPoints:number}>;load:Readonly<{perCoreCeiling:number;overrideAfterMilliseconds:number}>;files:Readonly<{ceilingBasisPoints:number}>;attaches:Readonly<{maximum:number}>;wait:Readonly<{pollMilliseconds:number;progressMilliseconds:number;heartbeatMilliseconds:number;staleMilliseconds:number;lockStaleMilliseconds:number;sampleMaximumAgeMilliseconds:number;maximumWaiters:number;bypassMilliseconds:number;maximumBypasses:number;agingMilliseconds:number;lendAfterMilliseconds:number;jitterBasisPoints:number}>;kinds:Readonly<Record<HostActionKindV1,HostResourceV1>>;operations:readonly HostOperationV1[];estimates:HostEstimatePolicyV1;envelopes:readonly HostEnvelopeV1[];classCeilings:readonly HostCeilingV1[]}>;
/** 🖥️ One sample of the host (`Host` of the schema); every platform reduces to these fields, `null` marking a counter the platform does not offer, `counters:"partial"` marking a sample whose platform probe failed (pressure and swap unknown, treated as raised pressure): the machine-wide open file table (used and maximum) and the number of attached disk images join memory, swap and load. */
export type HostSampleV1=Readonly<{version:1;atMs:number;platform:"darwin"|"linux"|"win32";cores:number;totalMegabytes:number;availableMegabytes:number;pressure:"normal"|"warn"|"critical";swapUsedMegabytes:number|null;swapOutMegabytesPerSecond:number|null;load:number|null;openFiles:number|null;openFilesMaximum:number|null;attachedImages:number|null;counters?:"partial"}>;
/** 🎫️ What one operation asks for (`Request` of the schema); `label` names what it builds (the target of `repo run`), so the progress lines of others can say whom they stand behind; `elastic` marks a supplementary request of a run that already holds a grant and goes on without it (the project farm): it is decided after every other request and never makes one wait; `envelopePerJobMegabytes` is the per-job memory a tool may use when it differs from the reserved `perJobMegabytes`; `rampMilliseconds` is how long the run needs to reach its peak (what its class learned, else the policy ramp) and `rampSlot` marks a reservation sized below the default, which may only ramp in a bounded number of slots at once; `quiet` marks the small forecast request of a run expected to execute nothing, which passes the queue barrier (within `memory.maximumQuiet` such grants) because it can starve nobody and is upgraded to its full request if work has to run. */
export type HostRequestV1=Readonly<{id:string;operation:string;priority:"batch"|"interactive";minimumJobs:number;preferredJobs:number;fixedMegabytes:number;perJobMegabytes:number;cpuPerJob:number;envelopePerJobMegabytes?:number;rampMilliseconds?:number;rampSlot?:boolean;quiet?:boolean;elastic?:boolean;label?:string}>;
/** ⏳️ A request in the queue (`Waiter` of the schema); `upgrade` marks the request of a held quiet grant to grow into its full request: it replaces the grant of the same id and keeps the queue position the run arrived with; `grow` (with `upgrade`) marks the request of a running grant admitted with fewer jobs than it asked for to grow its jobs now that memory freed up. */
export type HostWaiterV1=Readonly<{request:HostRequestV1;arrivedAtMs:number;pid:number;session?:string;weight?:number;upgrade?:boolean;grow?:boolean}>;
/** 🔐️ A live admission (`Grant` of the schema); `phase` is the phase the run is in once it reports phases; `quiet` marks a forecast grant that holds only the small reservation of a run expected to execute nothing, `arrivedAtMs` is the queue position it keeps for its upgrade and `upgradedAtMs` when it grew into its full request; `observedMegabytes` is what its process tree holds right now; `lentSinceMs` marks a grant whose process only waits in a queue (its reservation and cores are lent back to the ledger); `resumedAtMs` restarts the ramp when that process continues; `rampMilliseconds` is the time the grant counts its full reservation (the time its class needs to reach its peak) and `rampSlot` marks a shrunk reservation that occupies a ramp slot while it ramps; `grownAtMs` is when a grant admitted with fewer jobs than it asked for last grew its jobs. */
export type HostGrantV1=Readonly<{id:string;operation:string;jobs:number;cpu:number;reservedMegabytes:number;admittedAtMs:number;observedMegabytes:number|null;pid:number;bootAtMs:number;session?:string;lentSinceMs?:number;resumedAtMs?:number;handover?:HostHandoverV1;rampMilliseconds?:number;rampSlot?:boolean;quiet?:boolean;arrivedAtMs?:number;upgradedAtMs?:number;grownAtMs?:number;label?:string;phase?:"assembly"|"plan"|"rebuild"|"execution"}>;
/** 🤝️ The offer a process makes while its grant waits for the command that replaces it: a one-time token and the identity (id and start time) of the offering process, so only its own replacement can take the grant over. */
export type HostHandoverV1=Readonly<{token:string;fromPid:number;fromStarted:string;offeredAtMs:number}>;
/** 🧾️ Why a request was admitted or kept waiting. */
export type HostReasonV1="fits"|"reduced"|"override"|"memory"|"swap"|"swap-activity"|"pressure-critical"|"pressure-warn"|"files"|"attaches"|"load"|"queue-order"|"ramp"|"quiet-limit";
/** ⚖️ The verdict for one request. */
export type HostDecisionV1=Readonly<{id:string;verdict:"admit"|"wait";jobs:number;reservedMegabytes:number;reason:HostReasonV1}>;
/** 📊️ The whole evaluation: the derived headroom and one decision per waiter in service order. */
export type HostEvaluationV1=Readonly<{floorMegabytes:number;unrealizedMegabytes:number;headroomMegabytes:number;binding:"available"|"budget";loadRoom:number|null;decisions:readonly HostDecisionV1[]}>;
/** 🧩️ The tool-neutral resource envelope a build tool renders into its own flags. */
export type HostPlanV1=Readonly<{version:1;id:string;jobs:number;cpu:number;memoryMegabytes:number;fixedMegabytes:number;floorMegabytes:number}>;
/** 🏷️ Names of the closed definitions a value can be required to match. */
export type HostDefinitionV1="Policy"|"Host"|"Request"|"Waiter"|"Grant"|"Decision"|"Evaluation"|"Plan"|"GrantLiveness"|"Handover"|"HandoverFacts"|"Estimates"|"EstimateStore"|"Outcome"|"Estimate";

/** 🛂️ Requires a value to match one closed definition of the host capacity schema. */
export function requireHostCapacityV1<T>(definition:HostDefinitionV1,value:unknown):T{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value);
 if(errors.length)throw hostCapacityErrorV1("definitionRequired",{definition,detail:errors[0]!});
 return value as T;
}

/** 🔍️ Answers whether a value matches one closed definition without throwing. */
export function isHostCapacityV1(definition:HostDefinitionV1,value:unknown):boolean{
 return validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value).length===0;
}

/** 📕️ Closed liveness facts shared by ordinary accounting and inherited admission. */
export type HostGrantLivenessV1=Readonly<{grant:HostGrantV1;expectedId:string;pidAlive:boolean;nowMs:number;bootAtMs:number;modifiedAtMs:number}>;

/** 🚀️ Requires the same live instance, boot and heartbeat before a grant can carry admission. */
export function isCurrentHostGrantV1(value:unknown,policy:HostCapacityPolicyV1):boolean{
 if(!isHostCapacityV1("GrantLiveness",value))return false;
 const facts=value as HostGrantLivenessV1;
 return facts.pidAlive&&facts.grant.id===facts.expectedId&&Math.abs(facts.grant.bootAtMs-facts.bootAtMs)<=schema.definitions.Protocol.const.bootToleranceMilliseconds&&facts.nowMs-facts.modifiedAtMs<=policy.wait.staleMilliseconds;
}

/** ⚓️ The boot instant of this boot, written once beside the grants: `bootAtMs` (the wall clock minus the uptime at the first reader, to the second) and the uptime of that reader. */
export type HostBootAnchorV1=Readonly<{bootAtMs:number;uptimeMs:number}>;

/** 🕰️ Decides the anchor of the boot a process runs in. The instant a grant carries must not follow the wall clock (a step of the clock, by the user, a time service or a resumed virtual machine, moves `now - uptime` by the step wherever the uptime is monotonic, and made every running grant look like a grant of another boot): an anchor taken at an uptime that is not later than ours belongs to this boot and stays as it is; none, or one taken later than now (the uptime ran backwards, so the machine restarted since), is replaced by one taken from the wall clock and the uptime. A restart that is quicker than a heartbeat of the grants is also refused by their age. Pure and deterministic. */
export function hostBootAnchorV1(anchor:HostBootAnchorV1|undefined,wallMs:number,uptimeMs:number):HostBootAnchorV1{
 return anchor!==undefined&&uptimeMs>=anchor.uptimeMs?anchor:Object.freeze({bootAtMs:Math.round((wallMs-uptimeMs)/1000)*1000,uptimeMs});
}

/** 🛑️ Why a grant offered by a replaced command cannot be taken over. */
export type HostHandoverRefusalV1="gone"|"token"|"expired"|"identity"|"operation";
/** 🏭️ What a process knows when it is offered the grant of the command it replaces: the grant row it read (null when gone), whether that row is a current live grant, the offered text of its environment (`<id>.<token>`), the operation it asks for, the identity of its parent process and the clock. */
export type HostHandoverFactsV1=Readonly<{grant:HostGrantV1|null;grantCurrent:boolean;offered:string;operation:string;parent:Readonly<{pid:number;started:string}>|null;nowMs:number}>;
/** ☁️ The verdict on one offer: take the grant over or the first closed reason it is refused. */
export type HostHandoverVerdictV1=Readonly<{adopt:true;id:string}>|Readonly<{adopt:false;reason:HostHandoverRefusalV1}>;

/** ⚙️ Decides whether the offered grant may pass to this process: the offer must be well formed and name a current grant carrying exactly that one-time token, be younger than the declared bound, have been made by this process's own parent (id and start time, which a recycled id cannot fake) and be for the same operation; the first failing check names the reason, so an unrelated process never receives a grant. */
export function decideHostHandoverV1(facts:HostHandoverFactsV1):HostHandoverVerdictV1{
 const protocol=schema.definitions.Protocol.const,match=new RegExp(`^(${schema.definitions.Grant.properties.id.pattern.slice(1,-1)})\\.(${schema.definitions.Handover.properties.token.pattern.slice(1,-1)})$`,"u").exec(facts.offered);
 if(!match)return{adopt:false,reason:"token"};
 const grant=facts.grant,id=match[1]!,token=match[2]!;
 if(grant===null||!facts.grantCurrent||grant.id!==id)return{adopt:false,reason:"gone"};
 const offer=grant.handover,left=Buffer.from(token),right=Buffer.from(offer?.token??"");
 if(offer===undefined||left.length!==right.length||!timingSafeEqual(left,right))return{adopt:false,reason:"token"};
 if(facts.nowMs-offer.offeredAtMs>protocol.handoverMaximumAgeMilliseconds)return{adopt:false,reason:"expired"};
 if(facts.parent===null||facts.parent.pid!==offer.fromPid||facts.parent.started!==offer.fromStarted||grant.pid!==offer.fromPid)return{adopt:false,reason:"identity"};
 if(grant.operation!==facts.operation)return{adopt:false,reason:"operation"};
 return{adopt:true,id};
}

/** 💬️ Names a handover refusal in the chosen language. */
export function describeHostHandoverRefusalV1(language:"en"|"de",reason:HostHandoverRefusalV1):string{
 return schema.definitions.Messages.const[language].handoverReasons[reason];
}

/** 📖️ Reads the authored policy and refuses anything the closed schema or its ordering rules reject. */
export function readHostCapacityPolicyV1(value:unknown):HostCapacityPolicyV1{
 const policy=requireHostCapacityV1<HostCapacityPolicyV1>("Policy",value);
 if(policy.memory.floorMinimumMegabytes>policy.memory.floorMaximumMegabytes)throw hostCapacityErrorV1("floorOrder");
 if(policy.pressure.psiWarnPercent>policy.pressure.psiCriticalPercent||policy.pressure.availableCriticalBasisPoints>policy.pressure.availableWarnBasisPoints||policy.pressure.compressedWarnBasisPoints>policy.pressure.compressedCriticalBasisPoints)throw hostCapacityErrorV1("pressureOrder");
 return policy;
}

/** 🧱️ The free-memory floor of one host: a share of its memory between the authored bounds, never above what the host has. */
export function hostFloorMegabytesV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes">):number{
 const share=Math.floor(host.totalMegabytes*policy.memory.floorBasisPoints/10000);
 return Math.min(host.totalMegabytes,Math.max(policy.memory.floorMinimumMegabytes,Math.min(policy.memory.floorMaximumMegabytes,share)));
}

/** 🔢️ Orders waiters for service: interactive before batch, then the agent session holding the least memory per unit of budget weight (weighted fair share between sessions; waiters without a session count as holding nothing), then by arrival, then by id; grow requests of running grants and elastic requests come after every other request. */
export function orderHostWaitersV1(waiters:readonly HostWaiterV1[],ledger:readonly Pick<HostGrantV1,"session"|"reservedMegabytes">[]=[]):HostWaiterV1[]{
 const rank=(waiter:HostWaiterV1)=>waiter.grow===true||waiter.request.elastic===true?2:waiter.request.priority==="interactive"?0:1,held=(waiter:HostWaiterV1)=>waiter.session===undefined?0:ledger.reduce((sum,grant)=>sum+(grant.session===waiter.session?grant.reservedMegabytes:0),0),weight=(waiter:HostWaiterV1)=>waiter.weight??1;
 return [...waiters].sort((left,right)=>rank(left)-rank(right)||held(left)*weight(right)-held(right)*weight(left)||left.arrivedAtMs-right.arrivedAtMs||(left.request.id<right.request.id?-1:left.request.id>right.request.id?1:0));
}

/** 🚧️ Names what holds every admission back without being a hard stop: raised memory pressure, an open file table above its ceiling share or as many attached disk images as the maximum; undefined when none applies. These can be waited out, and are overridden only when nothing of ours is running. */
export function softHostReasonV1(policy:HostCapacityPolicyV1,host:HostSampleV1):"pressure-warn"|"files"|"attaches"|undefined{
 if(host.pressure==="warn")return "pressure-warn";
 if(host.openFiles!==null&&host.openFilesMaximum!==null&&host.openFiles*10000>host.openFilesMaximum*policy.files.ceilingBasisPoints)return "files";
 if(host.attachedImages!==null&&host.attachedImages>=policy.attaches.maximum)return "attaches";
 return undefined;
}

/** 🧮️ Decides, for a host sample, the live grants and the queue, which requests start now and with how many jobs; a pure function of its arguments. Reservations count in full (or by what the grant already holds when that is more) only while a grant ramps up, i.e. for the time its class needs to reach its peak (`rampMilliseconds` of the grant, else `memory.rampMilliseconds`; restarted by `resumedAtMs`), then the memory it really holds speaks for itself; at most `memory.maximumRamping` grants whose reservation was shrunk below the default (`rampSlot`) may ramp at once, so a wrong estimate can overshoot with a bounded number of runs only. A lent grant (its process only waits in a queue) counts neither memory nor cores nor fair-share weight and does not count as running for the override. A waiter refused for memory or load does not hold later ones back: they pass while they fit. It becomes a strict barrier only if finishing our own grants would admit it (foreign pressure never creates a barrier, draining cannot help it) and it has waited `wait.bypassMilliseconds` or been overtaken `wait.maximumBypasses` times, counting grants admitted since it arrived; so it cannot starve. A batch waiter older than `wait.agingMilliseconds` ranks as interactive, so a stream of interactive requests cannot starve it. The load only scales jobs down: foreign load (editors, indexers) never refuses a request whose memory fits, it is admitted with its minimum jobs; the cores of our own grants plus the request are what must stay under `load.perCoreCeiling` per core. An upgrade request (`upgrade`) replaces the live grant of its id: it is decided as the same request in the world where that grant is released and the memory it observably holds is returned, so only the difference has to fit (an upgrade that grows nothing always fits), and its cores, ramp slot and the override count without the old grant; except that an upgrade is never refused for memory while every other live grant is a quiet grant that itself waits to grow (it executes nothing and releases nothing before it grows, so waiting for it is a circular wait that also blocks the queue behind the barrier): nothing of ours competes, so the first such upgrade in queue order is admitted at the largest size that fits now (never under what the grant holds, reason `override`) with as many jobs as that memory carries at the learned footprint of one job (`fixedMegabytes` + jobs x `perJobMegabytes`, never fewer than one), and the run goes on, instead of waiting for memory that foreign processes or the other waiting runs hold. A quiet request passes the strict barrier while fewer than `memory.maximumQuiet` quiet grants are live, never becomes a barrier head and does not count as overtaking the heads, because it is small, bounded and every hard stop still applies to it. A grow request (`grow`, which only the holder of a grant that runs with fewer jobs than it asked for evaluates, never written to the queue) is decided after every waiting request and takes nothing from them: it waits `queue-order` while any other request waits or the queue is held behind a barrier, `ramp` while the reservation of its previous growth still ramps up (not yet observed, so it cannot grow twice on the same memory; the admission reservation counts in full while it ramps, so a run may grow right after it was admitted), a soft stop like any request, and otherwise grows to the most jobs whose memory fits with the memory of `growSpareJobs` more jobs left free (the protocol's hysteresis margin) and whose cores fit the load room; it never shrinks a reservation, never becomes a barrier and never takes the partial path of an upgrade. An elastic request (`elastic`, the workers a running build asks for beside its own grant and does without) is decided after every other request like a grow request: it waits `queue-order` while any other request waits or a barrier holds, takes only what fits now and never becomes a barrier head or an override. Our own booked cores never keep an agent session from its first job: a request whose session holds no other live grant (and was given none earlier in the same evaluation) is admitted with its minimum jobs whenever its memory fits and no soft stop applies, even when the cores of our grants already reach `load.perCoreCeiling` per core (while they stay below that ceiling plus one core per core of the host), so every agent progresses and the ceiling bounds everything beyond one job per session. An upgrade whose whole request fits the memory but not the cores our other grants book is admitted (`reduced`) with the most jobs those cores leave once the minimum jobs of every fresh or quiet request served after it are set aside (so a growth never takes the cores a later small request needs, with or without an agent session), or with one job under the session rule, and keeps the memory of its whole request (a run's memory does not shrink with its jobs; the grower adds jobs when cores free up), instead of waiting for another run to end. */
export function evaluateHostCapacityV1(policy:HostCapacityPolicyV1,host:HostSampleV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],nowMs:number):HostEvaluationV1{
 const floor=hostFloorMegabytesV1(policy,host),cap=Math.max(0,host.totalMegabytes-floor);
 const ramping=(grant:HostGrantV1)=>nowMs-(grant.resumedAtMs??grant.admittedAtMs)<(grant.rampMilliseconds??policy.memory.rampMilliseconds),active=ledger.filter(grant=>grant.lentSinceMs===undefined);
 const unrealizedOf=(grant:HostGrantV1)=>ramping(grant)?Math.max(0,grant.reservedMegabytes-(grant.observedMegabytes??0)):0,committedOf=(grant:HostGrantV1)=>ramping(grant)?Math.max(grant.reservedMegabytes,grant.observedMegabytes??0):grant.observedMegabytes??grant.reservedMegabytes;
 const unrealized=active.reduce((sum,grant)=>sum+unrealizedOf(grant),0),committed=active.reduce((sum,grant)=>sum+committedOf(grant),0);
 const observed=host.availableMegabytes-floor-unrealized,budget=Math.floor(cap*policy.memory.budgetBasisPoints/10000)-committed,binding=budget<observed?"budget" as const:"available" as const;
 let roomObserved=observed,roomBudget=budget;
 const headroom=Math.min(observed,budget),ledgerCpu=active.reduce((sum,grant)=>sum+grant.cpu,0),ceiling=policy.load.perCoreCeiling*host.cores;
 let loadRoom=host.load===null&&ledgerCpu===0?Infinity:ceiling-Math.max(host.load??0,ledgerCpu);
 const reportedLoadRoom=loadRoom===Infinity?null:loadRoom;
 const swapCeiling=Math.floor(host.totalMegabytes*policy.swap.ceilingBasisPoints/10000);
 const swapping=host.swapOutMegabytesPerSecond===null||host.swapOutMegabytesPerSecond>0;
 const soft=softHostReasonV1(policy,host);
 const hard:HostReasonV1|undefined=host.pressure==="critical"?"pressure-critical":host.swapUsedMegabytes!==null&&host.swapUsedMegabytes>swapCeiling&&swapping?"swap":host.swapOutMegabytesPerSecond!==null&&host.swapOutMegabytesPerSecond>policy.swap.outMegabytesPerSecond?"swap-activity":undefined;
 const drainedRoom=Math.min(host.availableMegabytes+ledger.reduce((sum,grant)=>sum+(grant.observedMegabytes??0),0),host.totalMegabytes)-floor,drainedBudget=Math.floor(cap*policy.memory.budgetBasisPoints/10000);
 const aged=waiters.map(waiter=>waiter.request.priority==="batch"&&nowMs-waiter.arrivedAtMs>=policy.wait.agingMilliseconds?Object.freeze({...waiter,request:Object.freeze({...waiter.request,priority:"interactive" as const})}):waiter);
 const growing=new Set(waiters.filter(waiter=>waiter.upgrade===true).map(waiter=>waiter.request.id)),decisions:HostDecisionV1[]=[],heads:{bypasses:number}[]=[],sessions=new Set<string>();let blocked=false,waits=false,admitted=0,own=ledgerCpu,softLeft=policy.memory.quietSoftStopGrants-active.filter(grant=>grant.quiet===true).length,rampers=active.filter(grant=>grant.rampSlot===true&&ramping(grant)).length,quietLeft=policy.memory.maximumQuiet-active.filter(grant=>grant.quiet===true).length;
 const ordered=orderHostWaitersV1(aged,active);
 for(const [at,waiter] of ordered.entries()){
  const request=waiter.request,need=(jobs:number)=>Math.min(request.fixedMegabytes+jobs*request.perJobMegabytes,cap),waited=nowMs-waiter.arrivedAtMs,quiet=request.quiet===true,grows=waiter.grow===true&&waiter.upgrade===true,elastic=request.elastic===true,avoids=quiet&&softLeft>0,pause=avoids?undefined:soft;
  const replaced=waiter.upgrade===true?active.find(grant=>grant.id===request.id):undefined,held=replaced?.observedMegabytes??0,freeObserved=roomObserved+(replaced?unrealizedOf(replaced):0),freeBudget=roomBudget+(replaced?committedOf(replaced):0),freeLoad=loadRoom===Infinity||!replaced?loadRoom:loadRoom+Math.max(host.load??0,ledgerCpu)-Math.max(host.load??0,ledgerCpu-replaced.cpu),freeRampers=rampers-(replaced?.rampSlot===true&&ramping(replaced)?1:0);
  const costObserved=(jobs:number)=>Math.max(0,need(jobs)-held),costBudget=(jobs:number)=>Math.max(need(jobs),held),fits=(jobs:number)=>costObserved(jobs)<=freeObserved&&costBudget(jobs)<=freeBudget||replaced!==undefined&&costObserved(jobs)<=unrealizedOf(replaced)&&costBudget(jobs)<=committedOf(replaced);
  const refuse=(reason:HostReasonV1):void=>{if(!grows&&!elastic)waits=true;decisions.push(Object.freeze({id:request.id,verdict:"wait",jobs:0,reservedMegabytes:0,reason}));};
  const admit=(jobs:number,reason:HostReasonV1,megabytes?:number):void=>{const reserved=megabytes??need(jobs);roomObserved=freeObserved-Math.max(0,reserved-held);roomBudget=freeBudget-Math.max(reserved,held);if(loadRoom!==Infinity)loadRoom=freeLoad-jobs*request.cpuPerJob;own+=jobs*request.cpuPerJob-(replaced?.cpu??0);admitted++;if(waiter.session!==undefined)sessions.add(waiter.session);rampers=freeRampers+(request.rampSlot===true?1:0);if(quiet){quietLeft--;softLeft--;}else for(const head of heads){head.bypasses++;if(head.bypasses>=policy.wait.maximumBypasses)blocked=true;}decisions.push(Object.freeze({id:request.id,verdict:"admit",jobs,reservedMegabytes:reserved,reason}));};
  if(hard){refuse(hard);continue;}
  if(grows){
   if(blocked||waits){refuse("queue-order");continue;}
   if(replaced===undefined||replaced.grownAtMs!==undefined&&ramping(replaced)){refuse("ramp");continue;}
   if(soft!==undefined){refuse(soft);continue;}
   const roomy=(jobs:number)=>{const wanted=Math.min(request.fixedMegabytes+(jobs+protocol.growSpareJobs)*request.perJobMegabytes,cap);return Math.max(0,wanted-held)<=freeObserved&&Math.max(wanted,held)<=freeBudget;};
   let jobs=0;for(let candidate=request.preferredJobs;candidate>=request.minimumJobs;candidate--)if(roomy(candidate)&&candidate*request.cpuPerJob<=freeLoad){jobs=candidate;break;}
   if(jobs>0)admit(jobs,jobs<request.preferredJobs?"reduced":"fits",Math.max(replaced.reservedMegabytes,need(jobs)));else refuse(roomy(request.minimumJobs)?"load":"memory");
   continue;
  }
  if(elastic&&(blocked||waits)||blocked&&!(quiet&&quietLeft>0)){refuse("queue-order");continue;}
  if(quiet&&quietLeft<=0){refuse("quiet-limit");continue;}
  if(request.rampSlot===true&&freeRampers>=policy.memory.maximumRamping){refuse("ramp");continue;}
  const memoryFits=fits(request.minimumJobs),first=!elastic&&waiter.session!==undefined&&own<ceiling+host.cores&&!sessions.has(waiter.session)&&active.every(grant=>grant.id===request.id||grant.session!==waiter.session);
  let jobs=0;
  if(pause===undefined)for(let candidate=request.preferredJobs;candidate>=request.minimumJobs;candidate--)if(fits(candidate)&&(avoids||candidate*request.cpuPerJob<=freeLoad)){jobs=candidate;break;}
  if(jobs===0&&pause===undefined&&memoryFits&&(avoids||first&&replaced===undefined||own-(replaced?.cpu??0)+request.minimumJobs*request.cpuPerJob<=ceiling))jobs=request.minimumJobs;
  if(jobs>0){admit(jobs,jobs<request.preferredJobs?"reduced":"fits");continue;}
  if(pause===undefined&&memoryFits&&replaced!==undefined){const cores=Math.min(request.preferredJobs,Math.floor((ceiling-own+replaced.cpu-ordered.slice(at+1).filter(later=>later.upgrade!==true&&later.grow!==true&&later.request.elastic!==true).reduce((sum,later)=>sum+later.request.minimumJobs*later.request.cpuPerJob,0))/request.cpuPerJob));if(cores>=1||first){admit(Math.max(1,cores),"reduced",need(request.minimumJobs));continue;}}
  const reason:HostReasonV1=!memoryFits?"memory":pause??"load";
  if(reason==="memory"&&replaced!==undefined&&active.every(grant=>grant.id===request.id||grant.quiet===true&&growing.has(grant.id))&&admitted===0&&!quiet){const fitted=Math.min(need(request.minimumJobs),held+freeObserved,freeBudget),units=request.perJobMegabytes>0?Math.max(1,Math.min(request.preferredJobs,Math.floor((fitted-request.fixedMegabytes)/request.perJobMegabytes))):request.preferredJobs;admit(units,"override",Math.max(replaced.reservedMegabytes,Math.min(fitted,need(units))));}
  else if(reason!=="memory"&&!elastic&&active.length-(replaced?1:0)===0&&admitted===0&&waited>=policy.load.overrideAfterMilliseconds)admit(request.minimumJobs,"override");
  else{
   refuse(reason);
   if((reason==="memory"||reason==="load")&&!quiet&&!elastic){
    const drains=soft===undefined&&need(request.minimumJobs)<=Math.min(drainedRoom,drainedBudget)&&request.minimumJobs*request.cpuPerJob<=ceiling,bypasses=ledger.filter(grant=>grant.id!==request.id&&grant.quiet!==true&&grant.admittedAtMs>=waiter.arrivedAtMs).length;
    if(drains&&(waited>=policy.wait.bypassMilliseconds||bypasses>=policy.wait.maximumBypasses))blocked=true;else if(drains)heads.push({bypasses});
   }
  }
 }
 return Object.freeze({floorMegabytes:floor,unrealizedMegabytes:unrealized,headroomMegabytes:headroom,binding,loadRoom:reportedLoadRoom,decisions:Object.freeze(decisions)});
}

/** 🪴️ How many consecutive grow attempts of one grant were admitted and the fewest jobs any of them admitted. */
export type HostGrowthSettleV1=Readonly<{attempts:number;jobs:number}>;

/** ⏲️ The time hysteresis of a growth: a grant grows only after `growSettleAttempts` consecutive attempts (one per heartbeat) were admitted, and then to the fewest jobs any of them admitted; one refused attempt starts the count again, so memory that frees up for a moment never grows a run.
 * @see ../🧪️tests/🔼️upgrade/🌱️grow/🐍️oracle/🐍️.py */
export function settleHostGrowthV1(previous:HostGrowthSettleV1|undefined,decision:Pick<HostDecisionV1,"verdict"|"jobs">):Readonly<{next:HostGrowthSettleV1|undefined;jobs:number|undefined}>{
 if(decision.verdict!=="admit")return Object.freeze({next:undefined,jobs:undefined});
 const next=Object.freeze({attempts:(previous?.attempts??0)+1,jobs:Math.min(previous?.jobs??decision.jobs,decision.jobs)});
 return next.attempts>=protocol.growSettleAttempts?Object.freeze({next:undefined,jobs:next.jobs}):Object.freeze({next,jobs:undefined});
}

/** 🧭️ The plan a process carries, followed to the grant row as it is now: fewer jobs after a partial growth, more after a grant admitted with fewer jobs grew, the cores of the grant and the memory the jobs may hold in proportion; the same plan when the row is quiet, belongs to another grant or holds the same jobs. */
export function followHostGrantPlanV1(plan:HostPlanV1,grant:Pick<HostGrantV1,"id"|"jobs"|"cpu"|"quiet">):HostPlanV1{
 if(grant.id!==plan.id||grant.quiet===true||grant.jobs===plan.jobs)return plan;
 return requireHostCapacityV1<HostPlanV1>("Plan",Object.freeze({...plan,jobs:grant.jobs,cpu:grant.cpu,memoryMegabytes:Math.floor(plan.memoryMegabytes*grant.jobs/plan.jobs)}));
}

/** 📏️ Sizes the request of one named operation, or answers undefined when the operation is light and passes ungated. */
export function sizeHostRequestV1(policy:HostCapacityPolicyV1,input:Readonly<{id:string;operation:string;preferredJobs?:number;kinds?:readonly HostActionKindV1[];targets?:readonly string[]}>):HostRequestV1|undefined{
 const operation=policy.operations.find(row=>row.name===input.operation);
 if(!operation)return undefined;
 if(operation.shape==="fixed")return requireHostCapacityV1<HostRequestV1>("Request",Object.freeze({id:input.id,operation:operation.name,priority:operation.priority,minimumJobs:1,preferredJobs:1,fixedMegabytes:operation.fixedMegabytes,perJobMegabytes:0,cpuPerJob:operation.cpu}));
 const kinds=[...new Set<HostActionKindV1>([...operation.kind?[operation.kind]:input.kinds??[],"build"])],rows:HostResourceV1[]=[...kinds.map(kind=>policy.kinds[kind]),...policy.envelopes.filter(row=>(input.targets??[]).some(target=>new RegExp(row.pattern,"u").test(target)))];
 return requireHostCapacityV1<HostRequestV1>("Request",Object.freeze({id:input.id,operation:operation.name,priority:operation.priority,minimumJobs:1,preferredJobs:Math.max(1,Math.min(input.preferredJobs??1,1024)),fixedMegabytes:operation.fixedMegabytes,perJobMegabytes:Math.max(...rows.map(row=>row.megabytes)),cpuPerJob:Math.max(...rows.map(row=>row.cpu))}));
}

/** 🎁️ Renders an admitted request into the tool-neutral envelope: jobs, cores, and the memory the jobs may hold beside the fixed block; the envelope per job is the default size (`envelopePerJobMegabytes`) when a learned estimate shrank the reservation, so a build tool is never starved by an estimate. */
export function planHostCapacityV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes">,request:HostRequestV1,jobs:number):HostPlanV1{
 if(!Number.isSafeInteger(jobs)||jobs<request.minimumJobs||jobs>request.preferredJobs)throw hostCapacityErrorV1("planJobs");
 return requireHostCapacityV1<HostPlanV1>("Plan",Object.freeze({version:1,id:request.id,jobs,cpu:jobs*request.cpuPerJob,memoryMegabytes:jobs*(request.envelopePerJobMegabytes??request.perJobMegabytes),fixedMegabytes:request.fixedMegabytes,floorMegabytes:hostFloorMegabytesV1(policy,host)}));
}

/** 🗣️ Picks the message language by the shared language rule; no language is assumed. */
export function resolveHostCapacityLanguageV1(environment:Readonly<Record<string,string|undefined>>,osLanguages?:OsLanguageListV1):"en"|"de"{
 return resolveLanguageV1(["en","de"],[],environment,osLanguages).language as "en"|"de";
}

/** 🖨️ Fills one catalogue message of the chosen language; unknown placeholders stay visible. */
export function formatHostCapacityV1(language:"en"|"de",name:"waiting"|"admitted"|"estimated"|"phased"|"quiet"|"quietFollowUp"|"upgrading"|"upgraded"|"upgradedPartly"|"upgradedReduced"|"upgradeWithdrawn"|"grantQuiet"|"waiterUpgrade"|"reduced"|"reducedGrows"|"grown"|"override"|"bypassed"|"status"|"grant"|"grantLent"|"waiter"|"handover"|"adopted"|"handoverRefused"|"treeUnobserved"|"footprintFallback"|"countersPartial"|"blocker"|"blockerHost",values:Readonly<Record<string,string|number>>):string{
 return fillHostCapacityV1(schema.definitions.Messages.const[language][name],values);
}

/** 🔤️ Fills the placeholders of one catalogue text; unknown placeholders stay visible. */
function fillHostCapacityV1(text:string,values:Readonly<Record<string,string|number>>):string{
 return text.replace(/\{([a-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);
}

/** 🚨️ Names of the catalogue errors a refused host capacity value, plan or platform raises. */
export type HostCapacityErrorV1=keyof typeof schema.definitions.Messages.const.en.errors;

/** 🌐️ Fills one catalogue error of the chosen language; `{detail}` carries the first finding of the schema validator. */
export function formatHostCapacityErrorV1(language:"en"|"de",name:HostCapacityErrorV1,values:Readonly<Record<string,string|number>>={}):string{
 return fillHostCapacityV1(schema.definitions.Messages.const[language].errors[name],values);
}

/** ❌️ Builds the error of one refusal in the language of the environment, so every diagnostic of the gate speaks English or German like its progress lines. */
export function hostCapacityErrorV1(name:HostCapacityErrorV1,values:Readonly<Record<string,string|number>>={},environment:Readonly<Record<string,string|undefined>>=process.env):Error{
 return Error(formatHostCapacityErrorV1(resolveHostCapacityLanguageV1(environment),name,values));
}

/** 🧾️ One thing of ours a waiting request stands behind: the label of its request, whether it runs, is quiet, waits to grow or waits ahead in the queue, its jobs and the megabytes it holds (committed) or asks for. */
export type HostBlockerV1=Readonly<{label:string;state:"running"|"growing"|"quiet"|"waiting";jobs:number;megabytes:number}>;

/** 🧭️ Names what a request that waits in `evaluation` stands behind, decided by its reason: for `memory` the live grants of ours by the megabytes they count (committed: the reservation while it ramps, then what the tree holds), for `load` the grants by the cores they book, for `ramp` the grants that ramp in a slot (each with the requests admitted before it in the same evaluation, which run from now on), for `quiet-limit` the quiet grants, for `queue-order` the requests served before it that wait; a lent grant and the request's own grant never count; at most `limit`, largest first, then by label; empty for a reason that is the host's own (pressure, swap, files, images). A pure function.
 * @see ../🧪️tests/🧾️blockers/🐍️oracle/🐍️.py */
export function hostBlockersV1(policy:HostCapacityPolicyV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],evaluation:HostEvaluationV1,id:string,nowMs:number,limit:number=3):readonly HostBlockerV1[]{
 const position=evaluation.decisions.findIndex(row=>row.id===id),mine=evaluation.decisions[position];
 if(mine===undefined||mine.verdict!=="wait")return Object.freeze([]);
 const ramping=(grant:HostGrantV1)=>nowMs-(grant.resumedAtMs??grant.admittedAtMs)<(grant.rampMilliseconds??policy.memory.rampMilliseconds),committed=(grant:HostGrantV1)=>ramping(grant)?Math.max(grant.reservedMegabytes,grant.observedMegabytes??0):grant.observedMegabytes??grant.reservedMegabytes;
 const growing=new Set(waiters.filter(waiter=>waiter.upgrade===true).map(waiter=>waiter.request.id)),grants=ledger.filter(grant=>grant.lentSinceMs===undefined&&grant.id!==id);
 const ofGrant=(grant:HostGrantV1,megabytes:number):HostBlockerV1=>Object.freeze({label:grant.label??`${grant.operation} ${grant.id.slice(0,6)}`,state:grant.quiet===true?growing.has(grant.id)?"growing" as const:"quiet" as const:"running" as const,jobs:grant.jobs,megabytes});
 const ahead=evaluation.decisions.slice(0,position).filter(row=>row.verdict==="wait").flatMap(row=>waiters.filter(waiter=>waiter.request.id===row.id).slice(0,1)).map(waiter=>Object.freeze({label:waiter.request.label??`${waiter.request.operation} ${waiter.request.id.slice(0,6)}`,state:"waiting" as const,jobs:waiter.request.minimumJobs,megabytes:waiter.request.fixedMegabytes+waiter.request.minimumJobs*waiter.request.perJobMegabytes}));
 const admitted=evaluation.decisions.slice(0,position).filter(row=>row.verdict==="admit"&&!ledger.some(grant=>grant.id===row.id)).flatMap(row=>waiters.filter(waiter=>waiter.request.id===row.id).slice(0,1).map(waiter=>Object.freeze({blocker:Object.freeze({label:waiter.request.label??`${waiter.request.operation} ${waiter.request.id.slice(0,6)}`,state:"running" as const,jobs:row.jobs,megabytes:row.reservedMegabytes}),slot:waiter.request.rampSlot===true})));
 const rows=mine.reason==="memory"?[...grants.map(grant=>ofGrant(grant,committed(grant))),...admitted.map(row=>row.blocker)]:mine.reason==="load"?[...grants.filter(grant=>grant.cpu>0).map(grant=>ofGrant(grant,committed(grant))),...admitted.map(row=>row.blocker)]:mine.reason==="ramp"?[...grants.filter(grant=>grant.rampSlot===true&&ramping(grant)).map(grant=>ofGrant(grant,committed(grant))),...admitted.filter(row=>row.slot).map(row=>row.blocker)]:mine.reason==="quiet-limit"?grants.filter(grant=>grant.quiet===true).map(grant=>ofGrant(grant,committed(grant))):mine.reason==="queue-order"?ahead:[];
 const key=(row:HostBlockerV1)=>mine.reason==="load"?row.jobs:row.megabytes;
 return Object.freeze([...rows].sort((left,right)=>key(right)-key(left)||(left.label<right.label?-1:left.label>right.label?1:0)).slice(0,limit));
}

/** 🗂️ Renders what a waiter stands behind in the chosen language: each blocker as `label (state, jobs, megabytes)`, or the host itself when nothing of ours holds it back. */
export function describeHostBlockersV1(language:"en"|"de",blockers:readonly HostBlockerV1[]):string{
 const messages=schema.definitions.Messages.const[language];
 return blockers.length===0?messages.blockerHost:blockers.map(row=>fillHostCapacityV1(messages.blocker,{label:row.label,state:messages.blockerStates[row.state],jobs:row.jobs,megabytes:row.megabytes})).join(", ");
}

/** ⌛️ Names a wait or override reason in the chosen language. */
export function describeHostReasonV1(language:"en"|"de",reason:Exclude<HostReasonV1,"fits"|"reduced"|"override">):string{
 return schema.definitions.Messages.const[language].reasons[reason];
}
