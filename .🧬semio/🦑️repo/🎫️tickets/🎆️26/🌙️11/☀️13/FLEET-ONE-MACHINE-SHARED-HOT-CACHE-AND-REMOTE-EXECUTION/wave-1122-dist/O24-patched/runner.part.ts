/** 🌐️ Why a pass may or may not start on a fleet slot: a worker slot is free and the small local footprint fits, the fleet does not answer, every worker slot that fits a final is taken, the authored cap of fleet jobs is reached, memory pressure is critical or the free memory above the floor does not carry another fleet job. */
export type FleetJobReasonV1="fleet-fits"|"fleet-none"|"fleet-busy"|"fleet-cap"|"fleet-pressure"|"fleet-memory";
/** 🧭️ Which kind of slot a waiting pass is waiting for: a `local` one (no worker slot can take it), a `fleet` one (the local jobs cannot grow) or `either`, whichever frees first. */
export type FleetJobWaitV1="local"|"fleet"|"either";
/** 📣️ What the runner tells about a fleet slot: a pass starts on it and reserves its footprint, or it waits (repeated as progress) for `waitsFor`. `slots` are the worker slots that fit one more final now (`null` when the fleet does not answer), `running` the fleet jobs held here. */
export type FleetJobEventV1=Readonly<{kind:"waiting"|"started";waitsFor:FleetJobWaitV1|null;running:number;slots:number|null;footprintMegabytes:number;peakMegabytes:number;freeMegabytes:number;floorMegabytes:number;fleetReason:FleetJobReasonV1|null;localReason:LocalJobReasonV1|null;waitedMilliseconds:number}>;
/** 🌐️ How a lane runs component jobs on the FLEET: up to `maximum` fleet jobs at once, each charging `footprintMegabytes` of this machine (its cargo process, the rustc wrappers and the input shipping; its units and final run on workers), started only while `slots` (the worker slots that fit one more final, re-read every `pollMilliseconds`) are free and the memory `sample` shows the footprints above the hard `floorMegabytes`; local memory pressure does not stop them, critical pressure does. */
export type FleetJobsOptionsV1<T>=Readonly<{maximum:number;slots:()=>Promise<number|null|undefined>;footprintMegabytes:number;floorMegabytes:number;sample:()=>Promise<LocalJobSampleV1>;report?:(pass:T,event:FleetJobEventV1)=>void;pollMilliseconds?:number;progressMilliseconds?:number;now?:()=>number}>;
/** 🛑️ Raised by the body of a pass that holds a fleet slot when the worker placement of its final failed: the runner gives the fleet slot back and queues the pass for a LOCAL slot, where it waits for its reservation instead of compiling in memory nobody reserved. */
export class FleetPlacementFailedErrorV1 extends Error{constructor(message:string){super(message);this.name="FleetPlacementFailedErrorV1";}}

const pause=(milliseconds:number,wake:Iterable<Promise<void>>,signal?:AbortSignal):Promise<void>=>new Promise<void>(resolve=>{
 let timer:ReturnType<typeof setTimeout>|undefined;
 const done=()=>{if(timer!==undefined)clearTimeout(timer);signal?.removeEventListener("abort",done);resolve();};
 timer=setTimeout(done,milliseconds);signal?.addEventListener("abort",done,{once:true});
 if(signal?.aborted)done();
 for(const task of wake)void task.then(done,done);
});

/** 🧵️ Runs the passes of one lane in order with up to `jobs` offloadable passes at once, each holding ONE of two kinds of slot. The first running pass holds a local slot, always. A `fleet` slot (option `fleet`) reserves only the small local footprint of a pass whose units and final run on workers, and starts while a worker slot is free, re-read while the lane runs; without `fleet` the other `jobs` less the local jobs are fleet slots without a charge (the strictly sequential behaviour of a build without a fleet when `jobs` is one). A local slot beyond the first (option `local`, up to `local.maximum`) holds a real reservation of the predicted peak, starts only while the free memory above the floor carries it beside the running reservations and the fleet footprints, and re-reads the memory before each start. A pass starts as soon as EITHER kind is free (fleet first, it is the cheaper one); otherwise it waits, with progress that names the kind it waits for, woken by a settling job, the poll, `check` or `signal`. A fleet pass whose body throws `FleetPlacementFailedErrorV1` is queued for a LOCAL slot only, ahead of the passes not yet started, and does not hold them back: while it waits for its reservation the next passes still take free fleet slots. A pass that is not offloadable waits for every running pass and runs alone. The first failure stops launching, is handed to `fail` at once (the driver ends the sibling cargo children) and is thrown after every running pass settled.
 * @see ./🧪️tests/🟦️.ts
 * @see ./🧪️tests/🌐️fleet/🟦️.ts */
export async function runPassesInJobSlotsV1<T extends Pick<NativeBuildPassV1,"target">>(passes:readonly T[],options:Readonly<{jobs:number;check:()=>void;run:(pass:T,slot:PassJobSlotV1)=>Promise<void>;fail?:(error:unknown)=>void;offloadable?:(pass:T)=>boolean;local?:LocalJobsOptionsV1<T>;fleet?:FleetJobsOptionsV1<T>;signal?:AbortSignal}>):Promise<void>{
 const running=new Set<Promise<void>>(),reservations=new Map<Promise<void>,number>(),footprints=new Map<Promise<void>,number>(),offloadable=options.offloadable??nativeBuildPassOffloadableV1,local=options.local,fleet=options.fleet,localMaximum=Math.max(1,local?.maximum??1),staticFleet=Math.max(0,options.jobs-localMaximum),fleetMaximum=fleet?Math.max(0,Math.min(fleet.maximum,options.jobs-1)):staticFleet,now=local?.now??fleet?.now??Date.now,pending=[...passes],retries:T[]=[],states=new Map<T,{peak:number|undefined;waitedSince:number|undefined;reportedAt:number}>();
 let failure:{error:unknown}|undefined,localHeld=0,fleetHeld=0,idle:number|null|undefined,idleAt=-Infinity;
 const sum=(rows:Map<Promise<void>,number>)=>{let total=0;for(const value of rows.values())total+=value;return total;};
 const idleNow=async():Promise<number|null>=>{
  if(!fleet)return Math.max(0,staticFleet-fleetHeld);
  const at=now();
  if(idle===undefined||at-idleAt>=(fleet.pollMilliseconds??policy.fleetPollMilliseconds)){idleAt=at;const value=await fleet.slots().catch(()=>null);idle=value===null||value===undefined?null:Math.max(0,Math.floor(value));}
  return idle;
 };
 const launch=(pass:T,slot:PassJobSlotV1,peak:number,footprint:number):Promise<void>=>{
  if(slot==="local")localHeld++;else{fleetHeld++;if(typeof idle==="number")idle=Math.max(0,idle-1);}
  const task:Promise<void>=(async()=>options.run(pass,slot))().catch(error=>{
   if(slot==="fleet"&&error instanceof FleetPlacementFailedErrorV1&&!failure){states.delete(pass);retries.push(pass);return;}
   if(!failure){failure={error};options.fail?.(error);}
  }).finally(()=>{running.delete(task);reservations.delete(task);footprints.delete(task);if(slot==="local")localHeld--;else{fleetHeld--;idleAt=-Infinity;}});
  running.add(task);if(slot==="local")reservations.set(task,peak);else footprints.set(task,footprint);return task;
 };
 const attempt=async(pass:T,localOnly:boolean):Promise<"started"|"poll"|"settle">=>{
  let state=states.get(pass);if(!state){state={peak:undefined,waitedSince:undefined,reportedAt:-Infinity};states.set(pass,state);}
  let slot:PassJobSlotV1|undefined,verdict:LocalJobAdmitDecisionV1|undefined,fleetVerdict:FleetJobAdmitDecisionV1|undefined,slots:number|null=null;
  if(running.size<options.jobs){
   if(localHeld===0)slot="local";
   else{
    if(!localOnly){
     if(fleet){
      slots=await idleNow();
      const sample=await fleet.sample().catch(()=>({availableMegabytes:null,ownedMegabytes:0,pressure:null}) as LocalJobSampleV1);
      fleetVerdict=admitFleetComponentJobV1({running:fleetHeld,slots,maximum:fleetMaximum,availableMegabytes:sample.availableMegabytes,reservedMegabytes:Math.ceil(sum(footprints)),floorMegabytes:fleet.floorMegabytes,pressure:sample.pressure,footprintMegabytes:fleet.footprintMegabytes});
      if(fleetVerdict.admit)slot="fleet";
     }else if(fleetHeld<staticFleet)slot="fleet";
    }
    if(!slot&&local&&localHeld<localMaximum){
     state.peak??=await local.peak(pass);
     const sample=await local.sample(),held=sum(reservations);
     verdict=admitLocalComponentJobV1({running:localHeld,maximum:localMaximum,availableMegabytes:sample.availableMegabytes,ownedMegabytes:Math.min(sample.ownedMegabytes,held),reservedMegabytes:Math.ceil(held+sum(footprints)),floorMegabytes:local.floorMegabytes,pressure:sample.pressure,peakMegabytes:state.peak});
     if(verdict.admit)slot="local";
    }
   }
  }
  if(failure)return"settle";
  const waited=state.waitedSince===undefined?0:now()-state.waitedSince;
  if(slot){
   if(slot==="local"&&local&&state.peak===undefined)state.peak=await local.peak(pass);
   if(failure)return"settle";
   const extra=slot==="local"&&localHeld>0,peak=state.peak??0;
   if(extra&&local&&verdict)local.report?.(pass,{kind:"started",running:localHeld,peakMegabytes:peak,freeMegabytes:verdict.freeMegabytes,floorMegabytes:local.floorMegabytes,reason:verdict.reason,waitedMilliseconds:waited});
   if(slot==="fleet"&&fleet)fleet.report?.(pass,{kind:"started",waitsFor:null,running:fleetHeld,slots,footprintMegabytes:fleet.footprintMegabytes,peakMegabytes:peak,freeMegabytes:fleetVerdict?.freeMegabytes??0,floorMegabytes:fleet.floorMegabytes,fleetReason:fleetVerdict?.reason??null,localReason:verdict?.reason??null,waitedMilliseconds:waited});
   (localOnly?retries:pending).shift();states.delete(pass);launch(pass,slot,peak,fleet?.footprintMegabytes??0);
   return"started";
  }
  if(!fleetVerdict&&!(verdict&&local))return"settle";
  const at=now();state.waitedSince??=at;
  if(at-state.reportedAt>=(local?.progressMilliseconds??fleet?.progressMilliseconds??policy.localJobsProgressMilliseconds)){
   state.reportedAt=at;
   if(fleet){const fleetWait=fleetVerdict!==undefined&&fleetVerdict.reason!=="fleet-none",localWait=localOnly||local!==undefined,waitsFor:FleetJobWaitV1=fleetWait&&localWait?"either":fleetWait?"fleet":"local";fleet.report?.(pass,{kind:"waiting",waitsFor,running:fleetHeld,slots,footprintMegabytes:fleet.footprintMegabytes,peakMegabytes:state.peak??0,freeMegabytes:verdict?.freeMegabytes??fleetVerdict?.freeMegabytes??0,floorMegabytes:local?.floorMegabytes??fleet.floorMegabytes,fleetReason:fleetVerdict?.reason??null,localReason:verdict?.reason??null,waitedMilliseconds:at-state.waitedSince});}
   else if(verdict&&local)local.report?.(pass,{kind:"waiting",running:localHeld,peakMegabytes:state.peak!,freeMegabytes:verdict.freeMegabytes,floorMegabytes:local.floorMegabytes,reason:verdict.reason,waitedMilliseconds:at-state.waitedSince});
  }
  return"poll";
 };
 try{while(!failure){
  const retry=retries[0],next=pending[0];
  if(retry===undefined&&next===undefined){if(!running.size)break;await Promise.race(running);continue;}
  options.check();
  const sequential=next!==undefined&&(options.jobs<=1||!offloadable(next));
  if(retry===undefined&&sequential){if(running.size){await Promise.race(running);continue;}pending.shift();await launch(next,"local",0,0);continue;}
  let outcome:"started"|"poll"|"settle"="settle";
  if(retry!==undefined)outcome=await attempt(retry,true);
  if(failure)break;
  if(outcome!=="started"&&next!==undefined&&!sequential){const second=await attempt(next,false);outcome=second==="started"?"started":second==="poll"||outcome==="poll"?"poll":"settle";}
  if(failure)break;
  if(outcome==="poll"||outcome==="settle"&&!running.size)await pause(local?.pollMilliseconds??fleet?.pollMilliseconds??policy.localJobsPollMilliseconds,running,options.signal);
  else if(outcome==="settle")await Promise.race(running);
 }}catch(error){if(!failure){failure={error};options.fail?.(error);}}
 while(running.size)await Promise.allSettled([...running]);
 if(failure)throw failure.error;
}
