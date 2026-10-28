import {expect,test} from "bun:test";
import {createHostPhaseResizerV1,estimateHostPhasesV1,estimateHostPhaseV1,estimateHostRatioV1,emptyHostEstimateStoreV1,hostPhaseSegmentsV1,hostRebuildSlotsV1,readHostCapacityPolicyV1,recordHostOutcomeV1,recordHostPhasesV1,scaleHostRequestV1,type HostEstimateStoreV1,type HostPhaseEstimateV1,type HostPhaseReadingV1,type HostPhaseSegmentV1,type HostPhaseTransitionV1,type HostPhaseV1,type HostRequestV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import host from "../🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1({...host.policy,memory:{...host.policy.memory,rampMilliseconds:60000},estimates:fixture.policy});
const run=(over:object={})=>({jobs:1,peakMegabytes:1000,activeMilliseconds:60000,peakAtMilliseconds:30000,endedAtMs:1000,...over});
const segment=(phase:HostPhaseV1,start:number,active:number,peak:number,peakAt:number):HostPhaseSegmentV1=>({phase,startMilliseconds:start,activeMilliseconds:active,peakMegabytes:peak,peakAtMilliseconds:peakAt});
const request:HostRequestV1={id:"aaaaaaaa1",operation:"run",priority:"batch",minimumJobs:1,preferredJobs:4,fixedMegabytes:1536,perJobMegabytes:2048,cpuPerJob:1};

test("The cut of the readings into phase segments equals the Python oracle on every stored vector",()=>{
 for(const item of fixture.segmentCases)expect(hostPhaseSegmentsV1(item.readings as unknown as HostPhaseReadingV1[],item.transitions as HostPhaseTransitionV1[],item.end),item.name).toEqual(item.expect as HostPhaseSegmentV1[]);
 expect(fixture.segmentCases.some(item=>item.expect.length===0)&&fixture.segmentCases.some(item=>item.expect.some(row=>row.phase==="rebuild")),"vectors reach unmarked and rebuilt runs").toBe(true);
});

test("Recording by phase, the estimate by phase, the re-sizing at a boundary and the rebuild slots equal the oracle on every stored vector",()=>{
 const stores:HostEstimateStoreV1[]=[];
 for(const item of fixture.recordCases){
  let store=emptyHostEstimateStoreV1();
  for(const step of item.history)store=recordHostPhasesV1(policy,store,step.lineage,step.operation,step.defaultMegabytes,step.run as never);
  expect(store,item.name).toEqual(item.expect as HostEstimateStoreV1);stores.push(store);
 }
 fixture.estimateCases.forEach((item,index)=>expect(estimateHostPhasesV1(policy,stores[index]!,item.lineage,item.request as HostRequestV1,item.nowMs),item.name).toEqual(item.expect as HostPhaseEstimateV1));
 for(const item of fixture.resizeCases)expect(createHostPhaseResizerV1(policy,item.request as HostRequestV1,item.estimate as HostPhaseEstimateV1,item.capacityMegabytes)(item.phase as HostPhaseV1,item.context),JSON.stringify(item.context)+item.phase).toEqual((item.expect??undefined) as never);
 for(const item of fixture.floorCases)expect(scaleHostRequestV1(policy,item.request as HostRequestV1,item.ratioBasisPoints,item.capacityMegabytes,item.floorMegabytes),JSON.stringify(item)).toEqual(item.expect as HostRequestV1);
 expect(fixture.floorCases.some(item=>{const plain=scaleHostRequestV1(policy,item.request as HostRequestV1,item.ratioBasisPoints,item.capacityMegabytes);return item.expect.fixedMegabytes>plain.fixedMegabytes;}),"vectors reach a lifted floor").toBe(true);
 fixture.slotCases.forEach((item,index)=>expect(hostRebuildSlotsV1(policy,stores[Math.floor(index/2)]!,item.totalMegabytes,item.floorMegabytes,item.nowMs)).toBe(item.expect));
 const reached=fixture.estimateCases.map(item=>item.expect);
 expect(reached.some(row=>row.floorMegabytes>0)&&reached.some(row=>row.floorMegabytes===0)&&reached.some(row=>row.phases.rebuild!==undefined),"vectors reach learned and unlearned global phases").toBe(true);
 expect(new Set(fixture.slotCases.map(item=>item.expect)).size,"vectors reach several slot counts").toBeGreaterThan(1);
 expect(fixture.resizeCases.filter(item=>item.expect).length,"vectors reach resizes").toBeGreaterThan(40);
});

test("Law: the recorded night replayed as a stream (a quarter of the runs build the graph): the TypeScript asks exactly what the Python model asks, a run served by coverage asks at least 30 % less, and the memory used above reservations, build included, does not grow",()=>{
 let old=emptyHostEstimateStoreV1(),split=emptyHostEstimateStoreV1();
 const asks={old:[] as number[],split:[] as number[]},estimates:HostPhaseEstimateV1[]=[],capacity=14336,need=(value:HostRequestV1,jobs:number)=>value.fixedMegabytes+jobs*value.perJobMegabytes;
 for(const step of fixture.replay){
  const request=step.request as HostRequestV1,lineage=step.lineage;
  const before=estimateHostRatioV1(policy,old,lineage,step.nowMs),after=estimateHostPhasesV1(policy,split,lineage,request,step.nowMs);
  estimates.push(after);asks.old.push(need(scaleHostRequestV1(policy,request,before.ratioBasisPoints,capacity),step.jobs));asks.split.push(need(scaleHostRequestV1(policy,request,after.ratioBasisPoints,capacity,after.floorMegabytes),step.jobs));
  expect(asks.old.at(-1)).toBe(step.expect.askOld);expect(asks.split.at(-1)).toBe(step.expect.askNew);
  const whole=step.phased;
  old=recordHostOutcomeV1(policy,old,lineage,{operation:"run",defaultMegabytes:step.defaultMegabytes,jobs:step.jobs,peakMegabytes:whole.peakMegabytes,activeMilliseconds:whole.activeMilliseconds,peakAtMilliseconds:whole.peakAtMilliseconds,endedAtMs:whole.endedAtMs});
  split=recordHostPhasesV1(policy,split,lineage,"run",step.defaultMegabytes,step.phased as never);
 }
 const covered=fixture.replay.map((step,index)=>[step,index] as const).filter(([step,index])=>!step.rebuild&&index>=100),mean=(rows:number[])=>rows.reduce((sum,row)=>sum+row,0)/rows.length;
 expect(mean(covered.map(([,index])=>asks.split[index]!)),"covered runs ask less than 0.7 of the single class").toBeLessThan(0.7*mean(covered.map(([,index])=>asks.old[index]!)));
 const above=(index:number,split:boolean):number=>{
  const step=fixture.replay[index]!,resize=createHostPhaseResizerV1(policy,step.request as HostRequestV1,estimates[index]!,capacity);
  let held=split?asks.split[index]!:asks.old[index]!,total=0,previous="";
  for(const row of (step.phased.phases as HostPhaseSegmentV1[])){
   if(split){const change=resize(row.phase,{jobs:step.jobs,reservedMegabytes:held,activeMilliseconds:row.startMilliseconds});if(change)held=change.reservedMegabytes;}
   if(row.peakMegabytes>held&&!(row.phase==="plan"&&previous==="rebuild"))total+=(row.peakMegabytes-held)*row.activeMilliseconds/1000;
   previous=row.phase;
  }
  return total;
 };
 expect(fixture.replay.some(step=>step.rebuild)&&fixture.replay.some(step=>!step.rebuild)).toBe(true);
 const sum=(split:boolean)=>fixture.replay.reduce((total,_,index)=>index>=100?total+above(index,split):total,0);
 expect(sum(true),"memory used above the reservations, build included, does not grow").toBeLessThanOrEqual(sum(false));
});

test("Law: a run without phases is recorded and estimated exactly as before, so the phase split equals the single-class bound when the phases coincide",()=>{
 let old=emptyHostEstimateStoreV1(),split=emptyHostEstimateStoreV1();
 for(let index=0;index<9;index++){
  const outcome=run({peakMegabytes:900+index*131,endedAtMs:1000+index,activeMilliseconds:20000+index,peakAtMilliseconds:9000+index});
  old=recordHostOutcomeV1(policy,old,["run","run:test"],{operation:"run",defaultMegabytes:3584,...outcome});
  split=recordHostPhasesV1(policy,split,["run","run:test"],"run",3584,outcome);
 }
 expect(split).toEqual(old);
 const estimate=estimateHostPhasesV1(policy,split,["run","run:test"],request,5000),before=estimateHostRatioV1(policy,old,["run","run:test"],5000);
 expect(estimate).toMatchObject({...before,floorMegabytes:0});expect(Object.keys(estimate.phases)).toEqual(["execution"]);
 const phased=recordHostPhasesV1(policy,emptyHostEstimateStoreV1(),["run"],"run",3584,run({phases:[segment("execution",0,60000,1000,30000)],peakAtMilliseconds:30000}));
 expect(phased.classes["run"]!.samples).toEqual(recordHostOutcomeV1(policy,emptyHostEstimateStoreV1(),["run"],{operation:"run",defaultMegabytes:3584,...run()}).classes["run"]!.samples);
});

test("Law: when the execution phase decides, the phased request equals the single-class request; a heavy graph phase of the same total peak no longer inflates the class of the task",()=>{
 const covered=(index:number,peak=1100)=>run({peakMegabytes:peak,endedAtMs:2000+index,activeMilliseconds:50000,peakAtMilliseconds:40000,phases:[segment("assembly",0,4000,380,2000),segment("plan",4000,6000,900,8000),segment("execution",10000,40000,peak,40000)]});
 const rebuilt=(index:number)=>run({peakMegabytes:6700,endedAtMs:3000+index,activeMilliseconds:290000,peakAtMilliseconds:20000,phases:[segment("assembly",0,4000,380,2000),segment("plan",4000,6000,900,8000),segment("rebuild",10000,240000,6700,60000),segment("execution",250000,40000,1100,290000)]});
 let split=emptyHostEstimateStoreV1(),single=emptyHostEstimateStoreV1(),coveredOnly=emptyHostEstimateStoreV1();
 for(let index=0;index<12;index++){
  const next=index%3===0?rebuilt(index):covered(index);
  split=recordHostPhasesV1(policy,split,["run","run:test"],"run",3584,next);
  single=recordHostOutcomeV1(policy,single,["run","run:test"],{operation:"run",defaultMegabytes:3584,jobs:1,peakMegabytes:next.peakMegabytes,activeMilliseconds:next.activeMilliseconds,peakAtMilliseconds:next.peakAtMilliseconds,endedAtMs:next.endedAtMs});
  if(index%3!==0)coveredOnly=recordHostPhasesV1(policy,coveredOnly,["run","run:test"],"run",3584,next);
 }
 const phased=estimateHostPhasesV1(policy,split,["run","run:test"],request,9000),whole=estimateHostRatioV1(policy,single,["run","run:test"],9000),clean=estimateHostPhasesV1(policy,coveredOnly,["run","run:test"],request,9000);
 expect(whole.ratioBasisPoints,"the whole-tree peak of a rebuilding run carries the graph builder into the class").toBeGreaterThan(phased.ratioBasisPoints*2);
 expect(phased.ratioBasisPoints,"a rebuild never reaches the class of the task").toBe(clean.ratioBasisPoints);
 expect(phased.phases.rebuild!.megabytes,"the build teaches the global rebuild class").toBeGreaterThan(5000);
 expect(phased.phases.execution!.megabytes).toBeLessThan(phased.phases.rebuild!.megabytes/2);
 const scaled=scaleHostRequestV1(policy,request,phased.ratioBasisPoints,28000,phased.floorMegabytes),plain=scaleHostRequestV1(policy,request,phased.ratioBasisPoints,28000);
 expect(phased.floorMegabytes).toBeLessThan(plain.fixedMegabytes+plain.perJobMegabytes);expect(scaled).toEqual(plain);
 const heavy=recordHostPhasesV1(policy,emptyHostEstimateStoreV1(),["run"],"run",3584,run({phases:[segment("execution",0,60000,1000,30000)]}));
 expect(estimateHostPhasesV1(policy,heavy,["run"],request,2000).floorMegabytes).toBe(0);
});

test("Law: the global phases decide the floor of the reservation when they are larger than the execution, and a run that never starts a tool is learned by its plan phase",()=>{
 let store=emptyHostEstimateStoreV1();
 for(let index=0;index<8;index++)store=recordHostPhasesV1(policy,store,["show","show:projects"],"show",5120,run({endedAtMs:4000+index,peakMegabytes:2100,activeMilliseconds:30000,peakAtMilliseconds:20000,phases:[segment("assembly",0,3000,400,2000),segment("plan",3000,27000,2100,20000)]}));
 const show:HostRequestV1={...request,operation:"show",preferredJobs:1,fixedMegabytes:5120,perJobMegabytes:0,cpuPerJob:2};
 const estimate=estimateHostPhasesV1(policy,store,["show","show:projects"],show,9000);
 expect(store.classes["show:projects"]!.samples.length,"its plan phase is its execution").toBe(8);
 expect(estimate.floorMegabytes).toBeGreaterThan(2000);expect(estimate.floorMegabytes).toBeLessThan(2800);
 const scaled=scaleHostRequestV1(policy,show,estimate.ratioBasisPoints,28000,estimate.floorMegabytes);
 expect(scaled.fixedMegabytes+scaled.minimumJobs*scaled.perJobMegabytes,"far below the 5120 MB default").toBeLessThan(3500);
 expect(scaled.fixedMegabytes+scaled.minimumJobs*scaled.perJobMegabytes).toBeGreaterThanOrEqual(estimate.floorMegabytes);
});

test("Law: entering a later phase only ever shrinks the reservation, the graph build raises it once to the learned rebuild size and the next boundary gives it back, and without learned phases nothing is resized",()=>{
 const covered=(index:number)=>run({endedAtMs:5000+index,peakMegabytes:1500,activeMilliseconds:90000,peakAtMilliseconds:80000,phases:[segment("assembly",0,4000,380,2000),segment("plan",4000,6000,2600,8000),segment("execution",10000,80000,1500,80000)]});
 const rebuilt=(index:number)=>run({endedAtMs:6000+index,peakMegabytes:9000,activeMilliseconds:300000,peakAtMilliseconds:100000,phases:[segment("assembly",0,4000,380,2000),segment("plan",4000,6000,2600,8000),segment("rebuild",10000,200000,9000,100000),segment("execution",210000,90000,1500,300000)]});
 let store=emptyHostEstimateStoreV1();
 for(let index=0;index<9;index++)store=recordHostPhasesV1(policy,store,["run","run:build"],"run",3584,index%4===0?rebuilt(index):covered(index));
 const small={...request,preferredJobs:2},estimate=estimateHostPhasesV1(policy,store,["run","run:build"],small,9000),resize=createHostPhaseResizerV1(policy,small,estimate,28000);
 const admitted=scaleHostRequestV1(policy,small,estimate.ratioBasisPoints,28000,estimate.floorMegabytes),held=admitted.fixedMegabytes+2*admitted.perJobMegabytes;
 const afterPlan=resize("plan",{jobs:2,reservedMegabytes:held,activeMilliseconds:4000}),afterBuild=resize("rebuild",{jobs:2,reservedMegabytes:afterPlan?.reservedMegabytes??held,activeMilliseconds:10000});
 expect(estimate.floorMegabytes,"the plan phase decides the floor").toBeGreaterThan(estimate.phases.execution!.megabytes);
 expect(afterBuild!.reservedMegabytes,"the graph build is booked at its learned size").toBeGreaterThan(8000);
 const afterExecution=resize("execution",{jobs:2,reservedMegabytes:afterBuild!.reservedMegabytes,activeMilliseconds:210000});
 expect(afterExecution!.reservedMegabytes,"the execution gets back to its own need").toBeLessThan(afterBuild!.reservedMegabytes/2);
 expect(afterExecution!.reservedMegabytes).toBeLessThanOrEqual(held);
 for(const phase of ["assembly","plan","execution"] as const)expect(resize(phase,{jobs:2,reservedMegabytes:1,activeMilliseconds:0}),`${phase} never grows a reservation`).toBeUndefined();
 const untouched=createHostPhaseResizerV1(policy,request,estimateHostPhasesV1(policy,emptyHostEstimateStoreV1(),["run"],request,1),28000);
 for(const phase of ["assembly","plan","rebuild","execution"] as const)expect(untouched(phase,{jobs:4,reservedMegabytes:scaleHostRequestV1(policy,request,10000,28000).fixedMegabytes+4*scaleHostRequestV1(policy,request,10000,28000).perJobMegabytes,activeMilliseconds:0}),phase).toBeUndefined();
});

test("Law: the plan stretch that directly follows a graph build is not learned, because the builder still holds its memory until it exits",()=>{
 const rebuilt=(index:number)=>run({endedAtMs:8000+index,peakMegabytes:6700,activeMilliseconds:300000,peakAtMilliseconds:60000,phases:[segment("assembly",0,4000,380,2000),segment("plan",4000,6000,900,8000),segment("rebuild",10000,240000,6700,60000),segment("plan",250000,5000,6600,251000),segment("execution",255000,45000,1100,290000)]});
 let store=emptyHostEstimateStoreV1();
 for(let index=0;index<6;index++)store=recordHostPhasesV1(policy,store,["run"],"run",3584,rebuilt(index));
 expect(estimateHostPhaseV1(policy,store,"plan",3584,9000)!.megabytes,"the clean plan stretch of 900 MB, not the 6600 MB residual").toBeLessThan(2000);
 expect(estimateHostPhaseV1(policy,store,"rebuild",3584,9000)!.megabytes).toBeGreaterThan(5000);
});

test("Law: the compute slots of a rebuild follow what the rebuild measurably takes, between one and the maximum, and an unlearned rebuild is booked at its prior",()=>{
 expect(hostRebuildSlotsV1(policy,emptyHostEstimateStoreV1(),32768,4096,1)).toBe(3);
 expect(hostRebuildSlotsV1(policy,emptyHostEstimateStoreV1(),16384,2048,1)).toBe(1);
 let store=emptyHostEstimateStoreV1();
 for(let index=0;index<6;index++)store=recordHostPhasesV1(policy,store,["run"],"run",3584,run({endedAtMs:7000+index,phases:[segment("rebuild",0,60000,1200,30000)]}));
 const learned=estimateHostPhaseV1(policy,store,"rebuild",policy.estimates.rebuildPriorMegabytes,8000)!;
 expect(learned.megabytes).toBeLessThan(2000);
 expect(hostRebuildSlotsV1(policy,store,32768,4096,8000),"a cheap rebuild earns more computers").toBe(policy.estimates.maximumRebuildSlots);
 expect(hostRebuildSlotsV1(policy,store,4096,4096,8000)).toBe(1);
});

test("The phase vocabulary of the schema is the vocabulary of the code, and the policy carries the phase bounds",()=>{
 expect(schema.definitions.Phase.enum).toEqual(["assembly","plan","rebuild","execution"]);
 expect(schema.definitions.Estimates.required).toEqual(expect.arrayContaining(["phaseReferenceMegabytes","rebuildShareBasisPoints","maximumRebuildSlots"]));
 expect(schema.definitions.Messages.const.en.phased).toContain("{phase}");expect(schema.definitions.Messages.const.de.phased).toContain("{phase}");
});
