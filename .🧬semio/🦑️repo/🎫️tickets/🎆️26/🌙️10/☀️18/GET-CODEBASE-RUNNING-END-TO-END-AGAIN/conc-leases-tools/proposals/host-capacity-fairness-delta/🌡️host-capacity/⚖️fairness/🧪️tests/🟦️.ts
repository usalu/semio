import {expect,test} from "bun:test";
import fixture from "../../🧪️tests/🧫️fixtures/🔣️.json";
import {evaluateHostCapacityV1,readHostCapacityPolicyV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../📏️decision/🟦️.ts";
import {HOST_FAIRNESS_V1,evaluateFairHostCapacityV1,hostRequestDrainsV1,readHostFairnessV1} from "../🟦️.ts";

const policy=readHostCapacityPolicyV1(fixture.policy);
const host=(availableMegabytes:number,load:number|null=0):HostSampleV1=>({version:1,atMs:0,platform:"darwin",cores:8,totalMegabytes:32768,availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load,openFiles:null,openFilesMaximum:null,attachedImages:null});
const waiter=(id:string,megabytes:number,arrivedAtMs:number,priority:"batch"|"interactive"="batch"):HostWaiterV1=>({arrivedAtMs,pid:100,request:{id:id.padEnd(8,"x"),operation:"command",priority,minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1}});
const grant=(id:string,observed:number,admittedAtMs:number):HostGrantV1=>({id:id.padEnd(8,"g"),operation:"command",jobs:1,cpu:1,reservedMegabytes:observed,admittedAtMs,observedMegabytes:observed,pid:200,bootAtMs:0});
const verdict=(evaluation:ReturnType<typeof evaluateFairHostCapacityV1>,id:string)=>evaluation.decisions.find(row=>row.id===id.padEnd(8,"x"))!;

test("the authored bounds satisfy the closed schema and refuse unknown fields",()=>{
 expect(readHostFairnessV1(HOST_FAIRNESS_V1)).toEqual(HOST_FAIRNESS_V1);
 expect(()=>readHostFairnessV1({...HOST_FAIRNESS_V1,extra:1})).toThrow("Closed host capacity fairness");
});

test("a head held back only by foreign pressure never blocks smaller requests (the strict barrier starves them)",()=>{
 const sample=host(10000),queue=[waiter("big",5200,0),waiter("small",500,1000)];
 expect(verdict(evaluateHostCapacityV1(policy,sample,[],queue,1000) as never,"small")).toMatchObject({verdict:"wait",reason:"queue-order"});
 const fair=evaluateFairHostCapacityV1(policy,sample,[],queue,1000);
 expect(verdict(fair,"big")).toMatchObject({verdict:"wait",reason:"memory"});
 expect(verdict(fair,"small")).toMatchObject({verdict:"admit"});
 expect(hostRequestDrainsV1(policy,sample,[],queue[0]!,1000)).toBe(false);
});

test("when finishing our own grants would admit the head, it is overtaken a bounded number of times and by time, then it is a barrier",()=>{
 const sample=host(10000),running=[grant("run",4000,0)],big=waiter("big",5200,5);
 expect(hostRequestDrainsV1(policy,sample,running,big,1000)).toBe(true);
 const smalls=Array.from({length:HOST_FAIRNESS_V1.maximumBypasses+3},(_,index)=>waiter("s"+index,100,10+index));
 const early=evaluateFairHostCapacityV1(policy,sample,running,[big,...smalls],1000);
 const admitted=smalls.filter(row=>verdict(early,row.request.id.replace(/x+$/u,"")).verdict==="admit");
 expect(admitted.length).toBe(HOST_FAIRNESS_V1.maximumBypasses);
 expect(early.decisions.filter(row=>row.reason==="queue-order").length).toBe(3);
 const late=evaluateFairHostCapacityV1(policy,sample,running,[big,waiter("small",100,HOST_FAIRNESS_V1.bypassMilliseconds)],HOST_FAIRNESS_V1.bypassMilliseconds+10);
 expect(verdict(late,"small")).toMatchObject({verdict:"wait",reason:"queue-order"});
});

test("a batch request that has waited long enough ranks as interactive",()=>{
 const sample=host(10000),old=waiter("old",3000,0,"batch"),fresh=waiter("fresh",3000,HOST_FAIRNESS_V1.agingMilliseconds,"interactive");
 const evaluation=evaluateFairHostCapacityV1(policy,sample,[],[fresh,old],HOST_FAIRNESS_V1.agingMilliseconds);
 expect(evaluation.decisions.map(row=>row.id)).toEqual(["oldxxxxx","freshxxx"]);
 expect(evaluation.decisions[0]).toMatchObject({verdict:"admit"});expect(evaluation.decisions[1]).toMatchObject({verdict:"wait"});
});

test("starvation bound: in a simulated stream of small requests a drainable large request is admitted within the bypass bound plus one small duration",()=>{
 const smallMs=20000,sample=(running:HostGrantV1[])=>host(17000-running.reduce((sum,row)=>sum+(row.observedMegabytes??0),0));
 let running:HostGrantV1[]=[],queue:HostWaiterV1[]=[waiter("big",9000,0)],admittedAt:number|undefined,serial=0;
 for(let now=0;now<=300000&&admittedAt===undefined;now+=1000){
  running=running.filter(row=>now-row.admittedAtMs<smallMs);
  queue.push(waiter("s"+serial++,2500,now));
  const evaluation=evaluateFairHostCapacityV1(policy,sample(running),running,queue,now);
  for(const decision of evaluation.decisions)if(decision.verdict==="admit"){
   const entry=queue.find(row=>row.request.id===decision.id)!;
   if(entry.request.fixedMegabytes===9000)admittedAt=now;else running.push(grant(entry.request.id,entry.request.fixedMegabytes,now));
   queue=queue.filter(row=>row!==entry);
  }
 }
 expect(admittedAt).toBeDefined();expect(admittedAt!).toBeLessThanOrEqual(HOST_FAIRNESS_V1.bypassMilliseconds+smallMs+2000);
});

test("negative control: the strict barrier alone also admits the large request, but smaller requests behind a head blocked by foreign pressure starve",()=>{
 const sample=host(10000),queue=[waiter("big",5200,0),...Array.from({length:5},(_,index)=>waiter("s"+index,300,10+index))];
 const strict=evaluateHostCapacityV1(policy,sample,[],queue,5000);
 expect(strict.decisions.filter(row=>row.verdict==="admit").length).toBe(0);
 expect(evaluateFairHostCapacityV1(policy,sample,[],queue,5000).decisions.filter(row=>row.verdict==="admit").length).toBe(5);
});
