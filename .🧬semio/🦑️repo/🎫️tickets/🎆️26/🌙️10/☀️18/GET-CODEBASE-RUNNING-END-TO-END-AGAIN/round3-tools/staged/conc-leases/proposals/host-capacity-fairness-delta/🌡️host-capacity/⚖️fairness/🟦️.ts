import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {evaluateHostCapacityV1,orderHostWaitersV1,type HostCapacityPolicyV1,type HostDecisionV1,type HostEvaluationV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../📏️decision/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** ⚖️ The bounds of the bypass: how long, and how many later requests, a refused request may be overtaken before it becomes a barrier, and when a waiting batch request is promoted. */
export type HostFairnessV1=Readonly<{version:1;bypassMilliseconds:number;maximumBypasses:number;agingMilliseconds:number}>;

/** 📜️ The authored bounds; validated against the closed schema when read. */
export const HOST_FAIRNESS_V1:HostFairnessV1=readHostFairnessV1(schema.definitions.Authored.const);

/** 📖️ Reads bounds and refuses anything the closed schema rejects. */
export function readHostFairnessV1(value:unknown):HostFairnessV1{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:"#/definitions/Policy"},value);
 if(errors.length)throw Error(`Closed host capacity fairness required: ${errors[0]}`);
 return value as HostFairnessV1;
}

const drained=(host:HostSampleV1,ledger:readonly HostGrantV1[]):HostSampleV1=>{
 const held=ledger.reduce((sum,grant)=>sum+(grant.observedMegabytes??0),0),cpu=ledger.reduce((sum,grant)=>sum+grant.cpu,0);
 return Object.freeze({...host,availableMegabytes:Math.min(host.totalMegabytes,host.availableMegabytes+held),load:host.load===null?null:Math.max(0,host.load-cpu)});
};

/** 🚰️ Whether a refused request would be admitted if everything of ours finished: only then can a barrier help it; when foreign pressure alone holds it back, draining changes nothing and later requests may pass. */
export function hostRequestDrainsV1(policy:HostCapacityPolicyV1,host:HostSampleV1,ledger:readonly HostGrantV1[],waiter:HostWaiterV1,nowMs:number):boolean{
 return evaluateHostCapacityV1(policy,drained(host,ledger),[],[waiter],nowMs).decisions[0]?.verdict==="admit";
}

/** 🧮️ Decides like `evaluateHostCapacityV1`, but a request that does not fit no longer holds back every later one. A refused request is overtaken by later requests that fit, until it has waited `bypassMilliseconds` or been overtaken `maximumBypasses` times by grants admitted after it arrived; from then on, and only if finishing our own grants would admit it, it is a barrier and everything behind it waits. Foreign pressure never creates a barrier. A batch request waiting `agingMilliseconds` ranks as interactive so a stream of interactive requests cannot starve it. A pure function of its arguments. */
export function evaluateFairHostCapacityV1(policy:HostCapacityPolicyV1,host:HostSampleV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],nowMs:number,fairness:HostFairnessV1=HOST_FAIRNESS_V1):HostEvaluationV1{
 const header=evaluateHostCapacityV1(policy,host,ledger,[],nowMs);
 const ordered=orderHostWaitersV1(waiters.map(waiter=>waiter.request.priority==="batch"&&nowMs-waiter.arrivedAtMs>=fairness.agingMilliseconds?Object.freeze({...waiter,request:Object.freeze({...waiter.request,priority:"interactive" as const})}):waiter));
 const active:HostWaiterV1[]=[],decisions=new Map<string,HostDecisionV1>(),heads:{bypasses:number;drains:boolean}[]=[];
 let barrier=false;
 for(const waiter of ordered){
  const id=waiter.request.id;
  if(barrier){decisions.set(id,Object.freeze({id,verdict:"wait",jobs:0,reservedMegabytes:0,reason:"queue-order"}));continue;}
  const decision=evaluateHostCapacityV1(policy,host,ledger,[...active,waiter],nowMs).decisions.find(row=>row.id===id)!;
  decisions.set(id,decision);
  if(decision.verdict==="admit"){
   active.push(waiter);
   for(const head of heads){head.bypasses++;if(head.drains&&head.bypasses>=fairness.maximumBypasses)barrier=true;}
   continue;
  }
  if(decision.reason!=="memory"&&decision.reason!=="load")continue;
  const drains=hostRequestDrainsV1(policy,host,ledger,waiter,nowMs),bypasses=ledger.filter(grant=>grant.admittedAtMs>=waiter.arrivedAtMs).length;
  if(drains&&(nowMs-waiter.arrivedAtMs>=fairness.bypassMilliseconds||bypasses>=fairness.maximumBypasses))barrier=true;
  else heads.push({bypasses,drains});
 }
 return Object.freeze({...header,decisions:Object.freeze(ordered.map(waiter=>decisions.get(waiter.request.id)!))});
}
