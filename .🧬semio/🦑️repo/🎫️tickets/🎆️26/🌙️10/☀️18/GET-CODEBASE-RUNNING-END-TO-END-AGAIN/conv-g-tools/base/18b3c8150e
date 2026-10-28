import {existsSync,readFileSync,rmSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../🔒️leases/🎛️control/🟦️.ts";
import {beginAdmissionWaitV1,waitingDelayV1 as delay} from "../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import {describeHostReasonV1,evaluateHostCapacityV1,formatHostCapacityV1,isHostCapacityV1,requireHostCapacityV1,type HostCapacityPolicyV1,type HostGrantV1,type HostReasonV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from "../📏️decision/🟦️.ts";
import {hostCapacityDirectoryV1,hostUpgradePathV1} from "../🎫️gate/🟦️.ts";
import type {HostLedgerPrimitivesV1} from "../🎫️gate/📋️contract/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.definitions.Protocol.const;

/** 🧭️ How an upgrade request ended for the process that asked: no grant to upgrade (the gate is off or the run holds none), the grant was already complete, or it grew into its full request. */
export type HostUpgradeOutcomeV1="absent"|"held"|"upgraded";

const asking=new Map<string,number>(),settled=new Map<string,HostUpgradeOutcomeV1>();
const settle=(key:string,outcome:HostUpgradeOutcomeV1):HostUpgradeOutcomeV1=>{settled.set(key,outcome);if(settled.size>256)settled.delete(settled.keys().next().value!);return outcome;};

/** 🙋️ Says that work has to run now and returns when the run may start it: a process of the run (any child of the holder: the planner, the graph rebuild, the first action of a build tool) writes the request file beside its grant and waits until the holder has grown the quiet reservation into the full request in the fair queue, polling the grant row every `upgradePollMilliseconds` and counting as a wait for machine admission (so a deadline whose clock stops during admission waits is credited, exactly as at the door); a grant that is already complete only records that the run executed (once per process and run, so the thousands of actions of a build cost one read). Cancelling the wait (the control's signal or deadline) withdraws the request unless another asker of this process still waits, and rejects with the cancellation reason; a request file that vanished (another asker cancelled) is written again, so the holder never loses a live asker. */
export async function requestHostCapacityUpgradeV1(environment:Readonly<Record<string,string|undefined>>,control:ResourceLeaseControl,ports:Readonly<{now?:()=>number;pollMilliseconds?:number}>={}):Promise<HostUpgradeOutcomeV1>{
 const id=environment[protocol.grantEnvironment];
 if(!id||!new RegExp(schema.definitions.Grant.properties.id.pattern,"u").test(id))return "absent";
 const directory=hostCapacityDirectoryV1(environment),row=join(directory,"grants",`${id}.json`),request=hostUpgradePathV1(directory,id),key=request,now=ports.now??Date.now,poll=ports.pollMilliseconds??protocol.upgradePollMilliseconds;
 const known=settled.get(key);
 if(known!==undefined)return known;
 const read=():HostGrantV1|undefined=>{try{const value:unknown=JSON.parse(readFileSync(row,"utf8"));return isHostCapacityV1("Grant",value)?value as HostGrantV1:undefined;}catch{return undefined;}};
 const write=():void=>{try{writeFileSync(request,JSON.stringify({pid:process.pid,askedAtMs:now()}));}catch{}};
 const first=read();
 if(!first)return "absent";
 if(first.quiet!==true){if(!existsSync(request))write();return settle(key,"held");}
 asking.set(key,(asking.get(key)??0)+1);
 const endAdmissionWait=beginAdmissionWaitV1();
 try{
  for(;;){
   checkResourceLeaseControl(control);
   const current=read();
   if(!current)return "absent";
   if(current.quiet!==true)return settle(key,"upgraded");
   if(!existsSync(request))write();
   try{await delay(poll,undefined,{signal:control.signal});}catch(error){throw control.signal.aborted?control.signal.reason:error;}
  }
 }finally{
  endAdmissionWait();
  const left=(asking.get(key)??1)-1;
  if(left>0)asking.set(key,left);else{asking.delete(key);if(read()?.quiet===true)try{unlinkSync(request);}catch{}}
 }
}

/** 🔌️ What the holder of a grant hands to the upgrade: the ledger primitives of the gate plus the readers of its ledger, the queue file name and the host sample, by value so this module imports nothing of the gate's internals. */
export type HostUpgradeLedgerV1=HostLedgerPrimitivesV1&Readonly<{readGrants:(directory:string,now:number,policy:HostCapacityPolicyV1)=>HostGrantV1[];readWaiters:(directory:string,now:number,policy:HostCapacityPolicyV1)=>HostWaiterV1[];queuePath:(directory:string,arrivedAtMs:number,id:string)=>string;sample:()=>Promise<HostSampleV1>}>;
/** 🧳️ Everything the holder knows when it upgrades its grant. */
export type HostUpgradeContextV1=Readonly<{ledger:HostUpgradeLedgerV1;directory:string;policy:HostCapacityPolicyV1;request:HostRequestV1;control:ResourceLeaseControl;state:()=>HostGrantV1;publish:(next:HostGrantV1)=>void;requested:()=>boolean;stopped:()=>boolean;now:()=>number;pid:number;language:"en"|"de";write:(line:string)=>void;session?:Readonly<{address:string;weight:number}>}>;

/** 🔼️ Grows a held quiet grant into its full request in the fair queue: the run waits as an upgrade request that keeps the queue position the run arrived with (so a forecast never costs it its turn and never gives it a better one than asking for the full grant at the door would have), the decision replaces the held reservation by the full one (only the difference has to fit, the freed small reservation counts) and the grant row is rewritten under the gate lock in one atomic replace, with a restarted ramp for the new reservation; so the reservation never drops to nothing and never counts twice. Progress lines come in the language of the run; the wait ends when the asker cancelled (the request file vanished or the control fired), in which case the small grant is kept. */
export async function upgradeHostGrantV1(context:HostUpgradeContextV1):Promise<"upgraded"|"withdrawn">{
 const {ledger,directory,policy,request,control,now,pid,language,write}=context,{takeLock,writeJson,remove}=ledger,started=now(),first=context.state(),arrivedAtMs=first.arrivedAtMs??first.admittedAtMs,waiterPath=ledger.queuePath(directory,arrivedAtMs,request.id);
 const record:HostWaiterV1=requireHostCapacityV1<HostWaiterV1>("Waiter",{request,arrivedAtMs,pid,upgrade:true,...context.session===undefined?{}:{session:context.session.address,weight:context.session.weight}});
 let lastProgress=-Infinity,done=false;
 writeJson(waiterPath,record);
 try{
  for(;;){
   if(context.stopped())return "withdrawn";
   if(!context.requested()){write(formatHostCapacityV1(language,"upgradeWithdrawn",{operation:request.operation,seconds:Math.round((now()-started)/1000),reserved:context.state().reservedMegabytes}));return "withdrawn";}
   checkResourceLeaseControl(control);
   const host=await context.ledger.sample();
   try{utimesSync(waiterPath,new Date(),new Date());}catch{}
   if(takeLock(directory,now(),policy,pid)){
    try{
     const at=now(),grants=ledger.readGrants(directory,at,policy),waiters=ledger.readWaiters(directory,at,policy);
     if(!waiters.some(row=>row.request.id===request.id&&row.upgrade===true)){writeJson(waiterPath,record);waiters.push(record);}
     const evaluation=evaluateHostCapacityV1(policy,host,grants,waiters,at),mine=evaluation.decisions.find(row=>row.id===request.id)!,state=context.state();
     if(mine.verdict==="admit"){
      const{quiet:_,...rest}=state;
      context.publish(requireHostCapacityV1<HostGrantV1>("Grant",{...rest,jobs:mine.jobs,cpu:mine.jobs*request.cpuPerJob,reservedMegabytes:mine.reservedMegabytes,resumedAtMs:at,upgradedAtMs:at,...request.rampMilliseconds===undefined?{}:{rampMilliseconds:request.rampMilliseconds},...request.rampSlot===undefined?{}:{rampSlot:request.rampSlot}}));
      remove(waiterPath);done=true;
      write(formatHostCapacityV1(language,"upgraded",{operation:request.operation,from:state.reservedMegabytes,to:mine.reservedMegabytes,seconds:Math.round((at-started)/1000),jobs:mine.jobs,available:host.availableMegabytes,floor:evaluation.floorMegabytes}));
      return "upgraded";
     }
     if(at-lastProgress>=policy.wait.progressMilliseconds){
      lastProgress=at;
      const position=evaluation.decisions.findIndex(row=>row.id===request.id),before=evaluation.decisions.slice(0,position),spare=evaluation.headroomMegabytes-before.reduce((sum,row)=>sum+row.reservedMegabytes,0);
      write(formatHostCapacityV1(language,"upgrading",{operation:request.operation,seconds:Math.round((at-started)/1000),from:state.reservedMegabytes,to:Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,Math.max(0,host.totalMegabytes-evaluation.floorMegabytes)),reason:describeHostReasonV1(language,mine.reason as Exclude<HostReasonV1,"fits"|"reduced"|"override">),spare,floor:evaluation.floorMegabytes,running:grants.length,ahead:position}));
      await control.publish({resource:protocol.resource,mode:"shared",elapsedMs:at-started});checkResourceLeaseControl(control);await control.yieldContinuation();
     }
    }finally{rmSync(join(directory,"lock"),{recursive:true,force:true});}
   }
   if(done)return "upgraded";
   const budget=checkResourceLeaseControl(control);
   try{await delay(Math.max(1,Math.min(policy.wait.pollMilliseconds*(0.75+Math.random()*0.5),budget??Infinity)),undefined,{signal:control.signal});}catch(error){throw control.signal.aborted?control.signal.reason:error;}
  }
 }finally{if(!done)remove(waiterPath);}
}
