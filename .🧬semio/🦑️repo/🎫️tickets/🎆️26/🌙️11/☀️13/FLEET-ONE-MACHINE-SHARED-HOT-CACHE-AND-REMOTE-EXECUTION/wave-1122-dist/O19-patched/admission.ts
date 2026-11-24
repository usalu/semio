import {existsSync,mkdirSync,readdirSync,readFileSync,renameSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {randomBytes} from "node:crypto";
import {availableParallelism} from "node:os";
import {evaluateHostCapacityV1,type HostCapacityPolicyV1,type HostReasonV1,type HostRequestV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import {currentHostProbeV1,readHostSampleV1,type HostProbeV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import {claimHeldV1,processAliveV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts";
import {remoteExecutionPlatformMatchesV1} from "../../🟦️.ts";
import {OFFLOAD_POLICY_V1,offloadExpenseV1,offloadFamilyCrateV1,offloadFamilyMillisecondsV1,offloadFamilyPeakMegabytesV1,offloadLearnPeakMegabytesV1,offloadModeV1,offloadPeakMegabytesV1,offloadUnitPeakMegabytesV1,type OffloadExpenseV1} from "../📐️policy/🟦️.ts";

const limits=OFFLOAD_POLICY_V1.limits;
/** 🚦️ The machine's own answer to "may one more compiler unit start here now". */
export type OffloadAdmissionV1=Readonly<{verdict:"admit"|"wait";reason:HostReasonV1;atMs:number}>;
/** 🔌️ The seam of the host-capacity decision: tests hand in a probe that reports a pressured machine or a policy, production uses the machine and the authored policy. */
export type OffloadAdmissionPortsV1=Readonly<{probe?:HostProbeV1;policy?:HostCapacityPolicyV1;now?:()=>number;signal?:AbortSignal;doorRefused?:boolean;peakMegabytes?:number;fleet?:Readonly<{fetch?:typeof fetch;timeoutMilliseconds?:number}>}>;

/** 🚦️ Asks the single admission decision of the host (`evaluateHostCapacityV1`) about one more unit: a sample of the machine, an empty ledger (what is committed by other builds already shows in the sample's free memory, swap, pressure and load) and one waiting request for one job of the unit's size. `wait` means the unit would queue here. */
export async function localUnitAdmissionV1(ports:OffloadAdmissionPortsV1={}):Promise<OffloadAdmissionV1>{
 const now=(ports.now??Date.now)(),policy=ports.policy??(await import("../../../🌡️host-capacity/🎛️control/🟦️.ts")).BUILD_HOST_CAPACITY_POLICY_V1;
 const signal=ports.signal,probe=ports.probe??currentHostProbeV1(signal);
 signal?.throwIfAborted();
 const record=await readHostSampleV1(policy,signal===undefined||probe.signal===signal?probe:{...probe,signal:probe.signal===undefined?signal:AbortSignal.any([probe.signal,signal])},undefined,()=>now),host:HostSampleV1=record.host;
 signal?.throwIfAborted();
 const request:HostRequestV1={id:"offload-"+process.pid,operation:"rustc",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:0,perJobMegabytes:limits.unitMegabytes,cpuPerJob:1};
 const decision=evaluateHostCapacityV1(policy,host,[],[{request,arrivedAtMs:now,pid:process.pid}],now).decisions[0]!;
 return Object.freeze({verdict:decision.verdict,reason:decision.reason,atMs:now});
}

const memoPath=(root:string,name:string)=>join(root,"offload-"+name+".json");
/** 📝️ Reads a short-lived verdict shared by every compiler call of the machine, so a build of hundreds of units samples the host and asks the fleet about once per memo window. */
export function readOffloadMemoV1<T extends {atMs:number}>(root:string,name:string,now:number,windowMilliseconds:number):T|undefined{
 try{const value=JSON.parse(readFileSync(memoPath(root,name),"utf8")) as T;return now-value.atMs<windowMilliseconds&&now>=value.atMs?value:undefined;}catch{return undefined;}
}
/** 🖊️ Writes the shared verdict atomically; a failure only costs the next caller one more question. */
export function writeOffloadMemoV1<T extends {atMs:number}>(root:string,name:string,value:T):void{
 try{mkdirSync(root,{recursive:true});const path=memoPath(root,name),temporary=path+"."+randomBytes(3).toString("hex");writeFileSync(temporary,JSON.stringify(value));renameSync(temporary,path);}catch{}
}
/** 🚦️ The host's verdict, shared within the memo window. */
export async function sharedUnitAdmissionV1(root:string,ports:OffloadAdmissionPortsV1={}):Promise<OffloadAdmissionV1>{
 const now=(ports.now??Date.now)(),memo=readOffloadMemoV1<OffloadAdmissionV1>(root,"admission",now,limits.admissionMemoMilliseconds);
 if(memo)return memo;
 const fresh=await localUnitAdmissionV1(ports);writeOffloadMemoV1(root,"admission",fresh);return fresh;
}

/** ⏳️ How long a unit would queue for a compile slot of this machine, from the slot files of the unit cache without joining the queue: `0` while a slot is free, else the age of the oldest live waiter (at least 1). A unit with no slot limit never queues. */
export function slotQueueMillisecondsV1(root:string,slots:number,now:number=Date.now()):number{
 if(slots<1)return 0;
 const layout=UNIT_CACHE_POLICY_V1.layout;let free=false;
 for(let index=0;index<slots;index++)if(!claimHeldV1(join(root,layout.slots),String(index))){free=true;break;}
 if(free)return 0;
 let oldest=0;
 try{
  for(const name of readdirSync(join(root,layout.tickets)).sort()){
   const match=/^(\d{15})-[0-9a-f]+-(\d+)$/u.exec(name);
   if(match&&processAliveV1(Number(match[2]))){oldest=Math.max(0,now-Number(match[1]));break;}
  }
 }catch{}
 return Math.max(1,oldest);
}

/** 🌡️ How many other machines could take a unit of this machine right now, asked of the scheduler itself (`/v1/fleet/idle?origin=`: signed like every fleet call, one request with a time limit): the answer is the counting the scheduler runs when a unit is submitted (platform key with toolchain and target, action format, headroom, slots, refusal cool-down, never the requester) over its live table at that moment, so the door and the submission cannot disagree about a table; only time can move between the two. `0` when no scheduler is published, it does not answer in time or nobody qualifies; never throws. */
export async function fleetHeadroomV1(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1={}):Promise<number>{
 const answer=await fleetIdleAnswerV1(environment,ports);
 return Number.isInteger(answer?.candidates)&&answer!.candidates!>0?answer!.candidates!:0;
}

/** 🛰️ How many action slots other machines of the same platform and toolchain have free right now, the width a build sizes its jobs to (`planFleetJobsV1`): the `slots` of the scheduler's idle answer (counted at `peakMegabytes` when the caller sizes the unit it asks for: a wasm final), else its candidate count for a scheduler that does not count slots yet; `null` when offloading is off, no scheduler is published or it does not answer in time, so the build stays at its local grant. Never throws. */
export async function fleetIdleSlotsV1(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1={}):Promise<number|null>{
 if(offloadModeV1(environment)==="never")return null;
 const answer=await fleetIdleAnswerV1(environment,ports);
 if(answer===undefined)return null;
 const value=Number.isInteger(answer.slots)?answer.slots:answer.candidates;
 return Number.isInteger(value)&&value!>=0?value!:null;
}

/** 📡️ One signed GET of the scheduler this machine's fleet names (`/v1/fleet/idle`, `/status`), with the idle question's time limit; the decoded body, `undefined` when no scheduler is published, it does not answer in time or refuses. Never throws. */
async function fleetSchedulerGetV1<T>(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1,route:(origin:string)=>string):Promise<T|undefined>{
 try{
  const fleet=await import("../../../🌐️remote-cache/🛰️fleet/🟦️.ts"),{resolveRemoteExecutionEndpointV1,resolveRemoteExecutionSchedulerV1}=await import("../../🧭️endpoint/🟦️.ts"),{remoteExecutionOriginV1}=await import("../../🌡️load/🟦️.ts");
  const root=environment.SEMIO_WORKSPACE_ROOT??(environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined)??fleet.fleetCacheRootV1(environment)??workspaceRootFromV1(process.cwd());
  const published=resolveRemoteExecutionEndpointV1({environment,...root===undefined?{}:{root}})?.endpoint;
  if(!published)return undefined;
  const scheduler=await resolveRemoteExecutionSchedulerV1({environment,...root===undefined?{}:{root}},published);
  if(scheduler===undefined)return undefined;
  const call=ports.fleet?.fetch??(root===undefined?fetch:fleet.remoteCacheFetchV1({...environment,SEMIO_WORKSPACE_ROOT:root},{root,audience:"execution",roleOf:()=>"submit"})),own=root===undefined?"":remoteExecutionOriginV1(root,environment);
  const response=await call(scheduler.url.replace(/\/+$/u,"")+route(own),{signal:AbortSignal.any([...ports.signal?[ports.signal]:[],AbortSignal.timeout(ports.fleet?.timeoutMilliseconds??limits.fleetStatusMilliseconds)])});
  if(!response.ok){await response.body?.cancel();return undefined;}
  return await response.json() as T;
 }catch{return undefined;}
}

/** 📡️ The scheduler's own idle counting for this machine (`/v1/fleet/idle?origin=`, signed, one request with a time limit); `undefined` when none is published, it does not answer in time or refuses. Never throws. */
async function fleetIdleAnswerV1(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1):Promise<{candidates?:number;slots?:number}|undefined>{
 return fleetSchedulerGetV1<{candidates?:number;slots?:number}>(environment,ports,own=>"/v1/fleet/idle?origin="+encodeURIComponent(own)+(ports.peakMegabytes!==undefined&&ports.peakMegabytes>0?"&peak="+Math.ceil(ports.peakMegabytes):""));
}

/** 👷️ A registered worker as a requester needs it to judge capability: who it is, what it advertises, whether it is draining and how many action slots it has and uses. */
export type FleetWorkerV1=Readonly<{origin:string;properties:readonly Readonly<{name:string;value:string}>[];concurrency:number;running:number;draining:boolean}>;
/** 👷️ The workers the scheduler lists (`/status`, signed) as a requester judges them, shared by every compiler call of the machine for `workersMemoMilliseconds` so a build asks about once per window; the workers and the machine's own origin; `undefined` when no scheduler answers (callers then stay as they were: capability is only ever used to decline, never to offload). Never throws. */
export async function fleetWorkersV1(environment:Readonly<Record<string,string|undefined>>,root:string,ports:OffloadAdmissionPortsV1={}):Promise<Readonly<{own:string;workers:readonly FleetWorkerV1[]}>|undefined>{
 const now=(ports.now??Date.now)(),memo=readOffloadMemoV1<{atMs:number;own:string;workers:readonly FleetWorkerV1[]}>(root,"workers",now,limits.workersMemoMilliseconds);
 if(memo!==undefined)return{own:memo.own,workers:memo.workers};
 let own="";
 const body=await fleetSchedulerGetV1<{workers?:readonly Record<string,unknown>[]}>(environment,ports,origin=>{own=origin;return"/status";});
 if(body===undefined||!Array.isArray(body.workers))return undefined;
 const workers=body.workers.flatMap((row):FleetWorkerV1[]=>{
  const properties=Array.isArray(row.properties)?(row.properties as readonly Record<string,unknown>[]).flatMap(item=>typeof item.name==="string"&&typeof item.value==="string"?[{name:item.name,value:item.value}]:[]):[];
  return typeof row.origin==="string"&&Number.isFinite(row.concurrency)&&Number.isFinite(row.running)?[{origin:row.origin,properties,concurrency:Number(row.concurrency),running:Number(row.running),draining:row.draining===true}]:[];
 });
 writeOffloadMemoV1(root,"workers",{atMs:now,own,workers});
 return{own,workers};
}

/** 🤝️ The workers that could ever take an action requiring `required`: not this machine, not draining, and advertising every required property (the scheduler's own rule, `remoteExecutionPlatformMatchesV1`, so the requester and the scheduler cannot disagree about capability); `slots` is their free action slots and `unmet` the required `name=value` rows that no other worker advertises at all. Capability, not headroom: a busy worker still counts. Pure.
 * @see ../🧪️tests/🤝️eligible/🟦️.ts */
export function eligibleFleetWorkersV1(fleet:Readonly<{own:string;workers:readonly FleetWorkerV1[]}>,required:readonly Readonly<{name:string;value:string}>[]):Readonly<{eligible:number;slots:number;total:number;unmet:readonly string[]}>{
 const others=fleet.workers.filter(worker=>worker.origin!==fleet.own),fit=others.filter(worker=>!worker.draining&&remoteExecutionPlatformMatchesV1(required,worker.properties));
 const unmet=required.filter(row=>!others.some(worker=>worker.properties.some(offered=>offered.name===row.name&&offered.value===row.value))).map(row=>row.name+"="+row.value);
 return Object.freeze({eligible:fit.length,slots:fit.reduce((sum,worker)=>sum+Math.max(0,worker.concurrency-worker.running),0),total:others.length,unmet:Object.freeze(unmet)});
}

/** 🔗️ The linker identity this machine's compiler asks workers for (`offload-linker.json` of the unit cache, written when the first linked unit asked); `undefined` when none was asked yet. */
export function readOffloadLinkerV1(root:string):string|undefined{
 try{const identity=(JSON.parse(readFileSync(join(root,"offload-linker.json"),"utf8")) as {identity?:unknown}).identity;return typeof identity==="string"&&/^[0-9a-f]{16}$/u.test(identity)?identity:undefined;}catch{return undefined;}
}

/** 🛰️ How many action slots can take a linked unit placed with `peakMegabytes` right now: what the scheduler's idle counting says at that peak, never more than the free slots of the workers that advertise this machine's linker (`semio-linker`), so a worker that cannot link what this compiler wrote is no capacity. Without a known linker or a worker list the scheduler's count stands; `null` as `fleetIdleSlotsV1`. Never throws. */
export async function fleetLinkedSlotsV1(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1={}):Promise<number|null>{
 const idle=await fleetIdleSlotsV1(environment,ports),root=environment.SEMIO_UNIT_CACHE_ROOT;
 if(idle===null||idle===0||root===undefined||root==="")return idle;
 const linker=readOffloadLinkerV1(root),fleet=linker===undefined?undefined:await fleetWorkersV1(environment,root,ports);
 return fleet===undefined?idle:Math.min(idle,eligibleFleetWorkersV1(fleet,[{name:"semio-linker",value:linker!}]).slots);
}

/** 🌊️ Returns the fleet fanout ceiling when offloading can relieve this machine: a scheduler is published, at least one other machine has headroom (`fleetHeadroomV1`), and either the door refused the whole build (`doorRefused`, the one verdict the build then hands to every compiler call through `SEMIO_OFFLOAD_PRESSURE`) or a local unit would queue. `undefined` keeps the caller waiting as before: no scheduler, nobody with headroom, or a calm machine. Under `pin` every unit goes to the named machine, so the ceiling is granted without pressure. */
export async function offloadFanOutJobsV1(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1={}):Promise<number|undefined>{
 if(offloadModeV1(environment)==="never")return undefined;
 const fleet=await import("../../../🌐️remote-cache/🛰️fleet/🟦️.ts"),{resolveRemoteExecutionEndpointV1}=await import("../../🧭️endpoint/🟦️.ts"),root=environment.SEMIO_WORKSPACE_ROOT??fleet.fleetCacheRootV1(environment)??workspaceRootFromV1(process.cwd());
 const published=resolveRemoteExecutionEndpointV1({environment,...root===undefined?{}:{root}})?.endpoint;
 if(!published)return undefined;
 if(offloadModeV1(environment)==="pin")return Math.min(limits.fanOutJobs,availableParallelism());
 if(ports.doorRefused!==true&&(await localUnitAdmissionV1(ports)).verdict!=="wait")return undefined;
 return await fleetHeadroomV1(environment,ports)>0?Math.min(limits.fanOutJobs,availableParallelism()):undefined;
}

/** 🧭️ The checkout that holds `cwd`: the nearest directory with the replica's `.cache/f/machine.json` (the build gate runs before the launcher states `SEMIO_WORKSPACE_ROOT`). */
export function workspaceRootFromV1(cwd:string):string|undefined{
 for(let at=cwd;;at=dirname(at)){if(existsSync(join(at,".cache","f","machine.json")))return at;if(dirname(at)===at)return undefined;}
}

const costPath=(root:string,crate:string)=>join(root,"c",crate.replace(/[^A-Za-z0-9_-]/gu,"_").slice(0,120));
/** 🧮️ What compiling a unit of this crate took here the last time (an average of the recorded local compiles), in milliseconds; `undefined` when none was recorded. Cheap: one small file per crate beside the unit cache. */
export function readUnitCostV1(root:string,crate:string):number|undefined{
 try{const value=Number(readFileSync(costPath(root,crate),"utf8"));return Number.isFinite(value)&&value>=0?value:undefined;}catch{return undefined;}
}
/** 🖊️ Records a local compile of a unit: the cost moves halfway to the new measurement. */
export function writeUnitCostV1(root:string,crate:string,milliseconds:number):void{
 try{const before=readUnitCostV1(root,crate),next=before===undefined?milliseconds:Math.round((before+milliseconds)/2),path=costPath(root,crate),temporary=path+"."+randomBytes(3).toString("hex");mkdirSync(join(root,"c"),{recursive:true});writeFileSync(temporary,String(next));renameSync(temporary,path);}catch{}
}
/** 🏋️ The peak memory a compile of this crate needed here the last times, in megabytes (a file `<crate>.peak` beside the cost record); `undefined` when none was measured. */
export function readUnitPeakV1(root:string,crate:string):number|undefined{
 try{const value=Number(readFileSync(costPath(root,crate)+".peak","utf8"));return Number.isFinite(value)&&value>0?value:undefined;}catch{return undefined;}
}
/** 👪️ The peak that a crate of the family is expected to need when none was measured for it: the quantile of the recorded peaks of every family crate in the cost folder of this root (a crate's own file `<crate>.peak`, one sample each, kills included), `undefined` while too few crates were measured. Reads only the names of the folder that end in the family suffix. */
export function readFamilyPeakV1(root:string):number|undefined{
 try{
  const suffix=OFFLOAD_POLICY_V1.peak.family.crateSuffix+".peak",samples:number[]=[];
  for(const name of readdirSync(join(root,"c")))if(name.endsWith(suffix)&&offloadFamilyCrateV1(name.slice(0,-5))){const value=readUnitPeakV1(root,name.slice(0,-5));if(value!==undefined)samples.push(value);}
  return offloadFamilyPeakMegabytesV1(samples);
 }catch{return undefined;}
}
/** 🕰️ The compile time that a crate of the family is expected to need when none was recorded for it: the quantile of the recorded local compile times of every family crate in the cost folder of this root (`offloadFamilyMillisecondsV1`), `undefined` while too few crates were measured. */
export function readFamilyCostV1(root:string):number|undefined{
 try{
  const samples:number[]=[];
  for(const name of readdirSync(join(root,"c")))if(offloadFamilyCrateV1(name)){const value=readUnitCostV1(root,name);if(value!==undefined)samples.push(value);}
  return offloadFamilyMillisecondsV1(samples);
 }catch{return undefined;}
}
/** 💎️ What placement and the expensive-unit rule know about a unit of this crate before it compiles: the megabytes it is placed with (`peak`: the learned peak, else for a linked wasm unit the family estimate, else the unknown default) and whether it is expensive (`expense`, see `offloadExpenseV1`). The one reading of the cost folder that the placement of the unit and the gate share. */
export function readUnitExpenseV1(root:string,crate:string,types:readonly string[],target:string|undefined):Readonly<{peak:number;expense:OffloadExpenseV1}>{
 const linked=offloadPeakMegabytesV1(types,target)>0,learned=readUnitPeakV1(root,crate),recorded=readUnitCostV1(root,crate),family=recorded===undefined&&linked?readFamilyCostV1(root):undefined;
 const peak=offloadUnitPeakMegabytesV1(learned,types,target,learned===undefined&&linked?readFamilyPeakV1(root):undefined);
 return Object.freeze({peak,expense:offloadExpenseV1({...recorded===undefined?{}:{recordedMilliseconds:recorded},...family===undefined?{}:{familyMilliseconds:family},...learned===undefined?{}:{learnedMegabytes:learned},placedMegabytes:peak,linked})});
}
/** 🖊️ Records the peak of a local compile (`megabytes`) by the rule of `offloadLearnPeakMegabytesV1`, or a peak a worker's kill taught (`force`), which replaces the record. */
export function writeUnitPeakV1(root:string,crate:string,megabytes:number,force:boolean=false):void{
 try{
  if(!Number.isFinite(megabytes)||megabytes<=0)return;
  const next=force?Math.ceil(megabytes):offloadLearnPeakMegabytesV1(readUnitPeakV1(root,crate),megabytes),path=costPath(root,crate)+".peak",temporary=path+"."+randomBytes(3).toString("hex");
  mkdirSync(join(root,"c"),{recursive:true});writeFileSync(temporary,String(next));renameSync(temporary,path);
 }catch{}
}
/** 🎯️ What a round trip costs a unit beyond its own compile, in milliseconds: the average of the measured overheads of the units that ran on a worker (the whole round trip minus the compile time the worker reports, so queue, input fetch, output upload, download and token waits are in it), else the authored floor `roundTripMilliseconds`. */
export function readOffloadOverheadV1(root:string):number{
 try{const value=Number(readFileSync(join(root,"offload-overhead"),"utf8"));if(Number.isFinite(value)&&value>=0)return value;}catch{}
 return limits.roundTripMilliseconds;
}
/** 🖊️ Records the overhead of one offloaded unit: the estimate moves halfway to the new measurement, so a fleet that gets slower or faster is followed within a few units. */
export function writeOffloadOverheadV1(root:string,milliseconds:number):void{
 try{
  if(!Number.isFinite(milliseconds)||milliseconds<0)return;
  let before:number|undefined;try{const value=Number(readFileSync(join(root,"offload-overhead"),"utf8"));if(Number.isFinite(value)&&value>=0)before=value;}catch{}
  const next=before===undefined?Math.round(milliseconds):Math.round((before+milliseconds)/2),path=join(root,"offload-overhead"),temporary=path+"."+randomBytes(3).toString("hex");
  mkdirSync(root,{recursive:true});writeFileSync(temporary,String(next));renameSync(temporary,path);
 }catch{}
}
/** 🎯️ The answer of the round trip test. */
export type OffloadWorthV1="worth"|"cheap"|"unmeasured";
/** 🎯️ Whether a unit is worth a round trip: `worth` when its recorded local compile time (`readUnitCostV1`, the run of the compiler without any wait for a slot) is not below the expected overhead of a round trip (`readOffloadOverheadV1`), `cheap` when it is, `unmeasured` when none was recorded (the unit compiles here first, which teaches the cost). */
export function worthARoundTripV1(root:string,crate:string):OffloadWorthV1{
 const cost=readUnitCostV1(root,crate);return cost===undefined?"unmeasured":cost>=readOffloadOverheadV1(root)?"worth":"cheap";
}

/** 🏁️ Whether a worker has room for an expensive unit placed with `peakMegabytes`, waiting for one for up to `waitMilliseconds` (asked every `pollMilliseconds`, the scheduler's own idle counting at that peak: `fleetIdleSlotsV1`, so the answer already counts the units that run on workers now). `false` at once when offloading is off or no scheduler answers; never throws.
 * @see ../🧪️tests/🏁️finals/🟦️.ts */
export async function awaitFleetFitV1(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1&Readonly<{waitMilliseconds:number;pollMilliseconds:number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>}>):Promise<boolean>{
 const now=ports.now??Date.now,sleep=ports.sleep??((milliseconds,signal)=>new Promise<void>(resolve=>{const timer=setTimeout(resolve,milliseconds);signal?.addEventListener("abort",()=>{clearTimeout(timer);resolve();},{once:true});})),until=now()+ports.waitMilliseconds;
 for(;;){
  const slots=await fleetLinkedSlotsV1(environment,ports);
  if(slots===null)return false;
  if(slots>0)return true;
  if(ports.signal?.aborted===true||now()+ports.pollMilliseconds>until)return false;
  await sleep(ports.pollMilliseconds,ports.signal);
 }
}

/** 🧭️ The signals one unit's offload decision reads, each asked only when the cheaper ones before it did not decide: an expensive unit (`expensive`, see `offloadExpenseV1`) asks only for a worker with room (`workerFits`, which waits at most the given milliseconds), a free compile slot of this machine taken at once (`localSlot(0)`, the caller keeps it and compiles in it), the round trip test (`worth`: a unit cheaper than the overhead of a round trip stays; a unit without a recorded cost waits up to `unmeasuredWaitMilliseconds` for a slot here, so it is measured, and goes away only when none became free), the door's verdict on the whole build (`SEMIO_OFFLOAD_PRESSURE=door`), the unit-cache slot queue, the host-capacity verdict and the build's own admission broker (`wait` when the unit would queue behind the memory budget or a frozen broker, `unknown` when no broker answers). */
export type OffloadGateInputV1=Readonly<{mode:"distribute"|"pin"|"never";door:boolean;expensive:()=>boolean;workerFits:(waitMilliseconds:number)=>Promise<boolean>;localSlot:(waitMilliseconds:number)=>Promise<boolean>;worth:()=>OffloadWorthV1;slotQueueMilliseconds:()=>number;host:()=>Promise<"admit"|"wait">;broker:()=>Promise<"admit"|"wait"|"unknown">}>;
/** 🧭️ Why a unit is offered to the fleet or stays: the first signal that decides (`final`: an expensive unit and a worker has room for it, whatever this machine's slots are; `final-wait`: an expensive unit but no worker had room in time, it compiles here; `local-slot`: a compile slot here was free, local first; `unmeasured`: its cost is unknown and a slot came free soon; `headroom`: every slot here is busy, the unit goes where another machine has room). */
export type OffloadGateReasonV1="never"|"pin"|"final"|"final-wait"|"local-slot"|"cheap"|"unmeasured"|"door"|"slot-queue"|"host-wait"|"broker-wait"|"headroom";
/** 🚪️ Whether a unit is offered to the fleet, and whether it queues there. Local first, except for expensive units: an expensive unit (a linked component final: 10 to 60 minutes, up to 6.5 GB) goes to a worker that has the room for it even while a slot here is free (it frees this machine for the next unit and runs with the worker's own cores), queued there for a short while if another unit took the room first, and compiles here when no worker has room within `finals.waitMilliseconds`; a cheaper unit that finds a free compile slot compiles here at once, the fleet only takes units that would wait for one. `pin` offers every unit and queues it on the named machine. `distribute` offers a unit that would wait here and is worth a round trip: where this machine is saturated (the door refused the build, a slot queue, the host or the build's memory budget would make the unit wait) the unit queues on the fleet because waiting here would be no better; where the wait is short, the fleet takes it only if another machine has headroom now, else it compiles here. Unknown signals count as calm. */
export async function offloadGateV1(input:OffloadGateInputV1):Promise<Readonly<{offload:boolean;queue:boolean;reason:OffloadGateReasonV1}>>{
 if(input.mode==="never")return{offload:false,queue:false,reason:"never"};
 if(input.mode==="pin")return{offload:true,queue:true,reason:"pin"};
 if(input.expensive())return await input.workerFits(OFFLOAD_POLICY_V1.finals.waitMilliseconds)?{offload:true,queue:true,reason:"final"}:{offload:false,queue:false,reason:"final-wait"};
 if(await input.localSlot(0))return{offload:false,queue:false,reason:"local-slot"};
 const worth=input.worth();
 if(worth==="cheap")return{offload:false,queue:false,reason:"cheap"};
 if(worth==="unmeasured"&&await input.localSlot(limits.unmeasuredWaitMilliseconds))return{offload:false,queue:false,reason:"unmeasured"};
 if(input.door)return{offload:true,queue:true,reason:"door"};
 if(input.slotQueueMilliseconds()>=limits.queueThresholdMilliseconds)return{offload:true,queue:true,reason:"slot-queue"};
 if(await input.host()==="wait")return{offload:true,queue:true,reason:"host-wait"};
 if(await input.broker()==="wait")return{offload:true,queue:true,reason:"broker-wait"};
 return{offload:true,queue:false,reason:"headroom"};
}
