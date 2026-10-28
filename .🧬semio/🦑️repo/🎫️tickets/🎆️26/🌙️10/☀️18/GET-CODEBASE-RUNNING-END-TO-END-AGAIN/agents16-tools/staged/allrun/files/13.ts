/** 📐️ Tests for build host-capacity estimates. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdirSync,mkdtempSync,readdirSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {churnProtocolV1} from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readHostCapacityPolicyV1,isHostCapacityV1,type HostCapacityPolicyV1,type HostRequestV1} from "../../📏️decision/🟦️.ts";
import {emptyHostEstimateStoreV1,estimateHostRatioV1,hostEstimateLineageV1,hostRequestPriorityV1,learnableHostRunV1,rampHostRequestV1,readHostEstimateStoreV1,recordHostOutcomeV1,scaleHostRequestV1,updateHostEstimateStoreV1,type HostEstimateStoreV1,type HostEstimateV1,type HostOutcomeV1,type HostRunStatusV1} from "../../📐️estimates/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import host from "../🧫️fixtures/🔣️.json";

const policy:HostCapacityPolicyV1=readHostCapacityPolicyV1({...host.policy,memory:{...host.policy.memory,rampMilliseconds:60000},estimates:fixture.policy});
const outcome=(over:Partial<HostOutcomeV1>={}):HostOutcomeV1=>({operation:"run",jobs:1,peakMegabytes:1000,defaultMegabytes:3584,activeMilliseconds:60000,peakAtMilliseconds:30000,endedAtMs:1000,...over});

test("The learned size of every stored case equals the Python oracle, whose weighted quantile was checked against numpy",()=>{
 for(const item of fixture.cases)expect(estimateHostRatioV1(policy,item.store as HostEstimateStoreV1,item.lineage,item.nowMs),item.name).toEqual(item.expect);
 const reached=new Set(fixture.cases.map(item=>`${item.expect.samples>0}:${item.expect.quick}`));
 for(const needed of ["false:false","true:false","true:true"])expect(reached.has(needed),`vectors reach ${needed}`).toBe(true);
});

test("The time to peak, the ramp slot and the learn rule equal the oracle on every stored vector",()=>{
 for(const item of fixture.ramps)expect(rampHostRequestV1(item.base as HostRequestV1,item.scaled as HostRequestV1,item.estimate as HostEstimateV1,(item as {measured?:boolean}).measured??false),JSON.stringify(item.estimate)).toEqual(item.expect as HostRequestV1);
 for(const item of fixture.learns)expect(learnableHostRunV1(item.status as HostRunStatusV1,item.peakRatioBasisPoints,{ratioBasisPoints:item.estimateRatioBasisPoints}),JSON.stringify(item)).toBe(item.expect);
 expect(fixture.ramps.some(item=>item.expect.rampSlot)&&fixture.ramps.some(item=>!item.expect.rampSlot),"vectors reach both slot verdicts").toBe(true);
 expect(fixture.ramps.some(item=>(item as {measured?:boolean}).measured===true&&(item.scaled.fixedMegabytes<item.base.fixedMegabytes||item.scaled.perJobMegabytes<item.base.perJobMegabytes)&&!item.expect.rampSlot),"a shrunk reservation backed by the class's own measured peaks takes no ramp slot").toBe(true);
 const ramps=fixture.cases.map(item=>item.expect.rampMilliseconds);
 expect(Math.min(...ramps)).toBeGreaterThanOrEqual(policy.memory.rampMilliseconds);expect(Math.max(...ramps)).toBeLessThanOrEqual(policy.estimates.maximumRampMilliseconds);
 expect(ramps.some(ramp=>ramp<policy.estimates.defaultRampMilliseconds)&&ramps.some(ramp=>ramp>policy.estimates.defaultRampMilliseconds),"vectors learn shorter and longer ramps than the default").toBe(true);
});

test("Law: a class that peaks late keeps its reservation counted for the time it needs, an unknown class counts the conservative default, and old samples without a time to peak count it at their end",()=>{
 expect(estimateHostRatioV1(policy,emptyHostEstimateStoreV1(),["run","run:test"],5).rampMilliseconds).toBe(policy.estimates.defaultRampMilliseconds);
 let late=emptyHostEstimateStoreV1(),early=emptyHostEstimateStoreV1();
 for(let index=0;index<6;index++){
  late=recordHostOutcomeV1(policy,late,["run"],outcome({peakMegabytes:2000,activeMilliseconds:1_400_000,peakAtMilliseconds:1_300_000,endedAtMs:1000+index}));
  early=recordHostOutcomeV1(policy,early,["run"],outcome({peakMegabytes:2000,activeMilliseconds:50_000,peakAtMilliseconds:20_000,endedAtMs:1000+index}));
 }
 const slow=estimateHostRatioV1(policy,late,["run"],2000),quick=estimateHostRatioV1(policy,early,["run"],2000);
 expect(slow.rampMilliseconds,"a run that peaks after 1300 s is protected for at least that long").toBeGreaterThanOrEqual(1_300_000);
 expect(quick.rampMilliseconds,"a run that peaks after 20 s needs no more than the policy minimum").toBe(policy.memory.rampMilliseconds);
 const legacy:HostEstimateStoreV1={version:1,classes:{run:{samples:Array.from({length:6},(_,index)=>({atMs:1000+index,ratioBasisPoints:5000,activeMilliseconds:600_000}))}}};
 expect(estimateHostRatioV1(policy,legacy,["run"],2000).rampMilliseconds,"without a recorded time to peak the peak counts as reached at the end").toBeGreaterThanOrEqual(600_000);
 const clamped=recordHostOutcomeV1(policy,emptyHostEstimateStoreV1(),["run"],outcome({activeMilliseconds:86_400_000,peakAtMilliseconds:86_400_000}));
 let many=clamped;for(let index=0;index<5;index++)many=recordHostOutcomeV1(policy,many,["run"],outcome({activeMilliseconds:86_400_000,peakAtMilliseconds:86_400_000,endedAtMs:2000+index}));
 expect(estimateHostRatioV1(policy,many,["run"],3000).rampMilliseconds).toBe(policy.estimates.maximumRampMilliseconds);
});

test("Law: only completed runs teach a class, a failed or cancelled run only when its partial peak already exceeds the estimate, so early deaths can never lower a reservation",()=>{
 let store=emptyHostEstimateStoreV1();
 for(let index=0;index<6;index++)store=recordHostOutcomeV1(policy,store,["run"],outcome({peakMegabytes:3000,defaultMegabytes:3000,endedAtMs:1000+index}));
 const estimate=estimateHostRatioV1(policy,store,["run"],2000);
 for(const status of ["failed","cancelled"] as const){
  expect(learnableHostRunV1(status,Math.round(400*10000/3000),estimate),`${status} at 13 %`).toBe(false);
  expect(learnableHostRunV1(status,estimate.ratioBasisPoints,estimate),`${status} at the estimate`).toBe(false);
  expect(learnableHostRunV1(status,estimate.ratioBasisPoints+1,estimate),`${status} above the estimate`).toBe(true);
 }
 expect(learnableHostRunV1("ok",1,estimate)).toBe(true);
});

test("Request scaling and priority equal the oracle on every stored vector",()=>{
 for(const item of fixture.scales)expect(scaleHostRequestV1(policy,item.request as HostRequestV1,item.ratioBasisPoints,item.capacityMegabytes)).toEqual(item.expect as HostRequestV1);
 for(const item of fixture.priorities)expect(hostRequestPriorityV1(item.base as never,(item.standing??undefined) as never,{ratioBasisPoints:10000,rampMilliseconds:300000,samples:0,quick:item.quick})).toBe(item.expect as never);
});

test("An unknown class keeps the conservative default and a class borrows its size from the class above it until it earned trust",()=>{
 expect(estimateHostRatioV1(policy,emptyHostEstimateStoreV1(),["run","run:build","run:@a/b:build"],5)).toEqual({ratioBasisPoints:10000,rampMilliseconds:policy.estimates.defaultRampMilliseconds,samples:0,quick:false});
 let store=emptyHostEstimateStoreV1();
 for(let index=0;index<6;index++)store=recordHostOutcomeV1(policy,store,["run","run:build"],outcome({peakMegabytes:700,endedAtMs:1000+index}));
 const parent=estimateHostRatioV1(policy,store,["run","run:build"],2000).ratioBasisPoints,child=estimateHostRatioV1(policy,store,["run","run:build","run:@a/b:build"],2000);
 expect(parent).toBeLessThan(10000);expect(child.ratioBasisPoints).toBe(parent);
expect(estimateHostRatioV1(policy,store,["run","run:build","run:@a/b:build"],2000).samples).toBeGreaterThan(0);
 let heavy=emptyHostEstimateStoreV1();
 for(let index=0;index<6;index++)heavy=recordHostOutcomeV1(policy,heavy,["run","run:build"],outcome({peakMegabytes:9000,endedAtMs:1000+index}));
 expect(estimateHostRatioV1(policy,heavy,["run","run:build"],2000).ratioBasisPoints).toBeGreaterThan(10000);
 expect(estimateHostRatioV1(policy,heavy,["run","run:build","run:@a/b:build"],2000).ratioBasisPoints,"an unknown class never inherits more than the default").toBe(10000);
 store=recordHostOutcomeV1(policy,store,["run","run:build","run:@a/b:build"],outcome({peakMegabytes:7000,endedAtMs:2000}));
 const learned=estimateHostRatioV1(policy,store,["run","run:build","run:@a/b:build"],2000).ratioBasisPoints;
 expect(learned).toBeGreaterThan(parent);
});

test("Estimates decay: the same history weighs less as it ages, and old peaks stop mattering after many half-lives",()=>{
 let store=emptyHostEstimateStoreV1();
 for(let index=0;index<5;index++)store=recordHostOutcomeV1(policy,store,["run"],outcome({peakMegabytes:7000,endedAtMs:1000+index}));
 for(let index=0;index<5;index++)store=recordHostOutcomeV1(policy,store,["run"],outcome({peakMegabytes:400,endedAtMs:policy.estimates.halfLifeMilliseconds*6+index}));
 const recent=estimateHostRatioV1(policy,store,["run"],policy.estimates.halfLifeMilliseconds*6+10);
 expect(recent.ratioBasisPoints).toBeLessThan(5000);
 expect(estimateHostRatioV1(policy,store,["run"],10).ratioBasisPoints).toBeGreaterThan(estimateHostRatioV1(policy,store,["run"],policy.estimates.halfLifeMilliseconds*40).ratioBasisPoints-1);
});

test("Learned sizes are always inside the authored bounds and the reservation never falls under the minimum, while the tool envelope stays at the default",()=>{
 for(const item of fixture.cases){const got=estimateHostRatioV1(policy,item.store as HostEstimateStoreV1,item.lineage,item.nowMs);expect(got.ratioBasisPoints>=policy.estimates.minimumRatioBasisPoints&&got.ratioBasisPoints<=policy.estimates.maximumRatioBasisPoints,item.name).toBe(true);}
 const request:HostRequestV1={id:"aaaaaaaa1",operation:"run",priority:"batch",minimumJobs:1,preferredJobs:8,fixedMegabytes:100,perJobMegabytes:100,cpuPerJob:1};
 const scaled=scaleHostRequestV1(policy,request,3000,28000);
 expect(scaled.fixedMegabytes+scaled.minimumJobs*scaled.perJobMegabytes).toBeGreaterThanOrEqual(policy.estimates.minimumMegabytes);
 expect(scaled.envelopePerJobMegabytes).toBe(100);
});

test("A learned reservation can never exceed what an idle machine could grant: the minimum need stays under the authored share of the memory above the floor, however large the learned peak",()=>{
 for(const item of fixture.scales){
  const got=scaleHostRequestV1(policy,item.request as HostRequestV1,item.ratioBasisPoints,item.capacityMegabytes),ceiling=Math.floor(item.capacityMegabytes*policy.estimates.maximumShareBasisPoints/10000);
  expect(got.fixedMegabytes+got.minimumJobs*got.perJobMegabytes,JSON.stringify(item).slice(0,80)).toBeLessThanOrEqual(Math.max(ceiling,policy.estimates.minimumMegabytes));
 }
 const huge=scaleHostRequestV1(policy,{id:"aaaaaaaa1",operation:"run",priority:"batch",minimumJobs:1,preferredJobs:4,fixedMegabytes:1536,perJobMegabytes:2048,cpuPerJob:1},40000,12000);
 expect(huge.fixedMegabytes+huge.perJobMegabytes).toBeLessThanOrEqual(7200);
});

test("The store stays bounded: newest samples per class and most recently used classes, and recording is pure",()=>{
 const frozen=recordHostOutcomeV1(policy,emptyHostEstimateStoreV1(),["op"],outcome());
 const again=recordHostOutcomeV1(policy,frozen,["op"],outcome({endedAtMs:2000}));
 expect(frozen.classes["op"]!.samples.length).toBe(1);expect(again.classes["op"]!.samples.length).toBe(2);
 let store=emptyHostEstimateStoreV1();
 for(let index=0;index<300;index++)store=recordHostOutcomeV1(policy,store,["op",`op:k${index}`],outcome({operation:"op",peakMegabytes:100+index,defaultMegabytes:1000,activeMilliseconds:10,peakAtMilliseconds:5,endedAtMs:index}));
 expect(Object.keys(store.classes).length).toBe(fixture.bounded.classes);
 expect(Math.min(...Object.values(store.classes).map(row=>Math.max(...row.samples.map(sample=>sample.atMs))))).toBe(fixture.bounded.oldestKept);
 expect(store.classes["op"]!.samples.length).toBe(fixture.bounded.rootSamples);
 expect(isHostCapacityV1("EstimateStore",store)).toBe(true);
});

test("The estimate schema definitions agree with Ajv, an independent validator",()=>{
 const ajv=new Ajv({strict:true,allowUnionTypes:true}),store=recordHostOutcomeV1(policy,emptyHostEstimateStoreV1(),["run"],outcome());
 const probes:Record<string,{valid:unknown[];invalid:unknown[]}>={
  EstimateStore:{valid:[store,emptyHostEstimateStoreV1()],invalid:[{...store,version:2},{version:1,classes:{"has space":{samples:[]}}},{version:1,classes:{run:{samples:[{atMs:-1,ratioBasisPoints:1,activeMilliseconds:0}]}}},{version:1}]},
  Outcome:{valid:[outcome()],invalid:[outcome({jobs:0}),{...outcome(),extra:1},outcome({defaultMegabytes:0}),{...outcome(),peakAtMilliseconds:undefined},outcome({peakAtMilliseconds:-1})]},
  Estimate:{valid:[{ratioBasisPoints:4000,rampMilliseconds:90000,samples:3,quick:true}],invalid:[{ratioBasisPoints:0,rampMilliseconds:90000,samples:3,quick:true},{ratioBasisPoints:4000,samples:3,quick:true},{ratioBasisPoints:4000,rampMilliseconds:999,samples:3,quick:true}]},
  Estimates:{valid:[fixture.policy],invalid:[{...fixture.policy,minimumSamples:0},{...fixture.policy,extra:true}]},
 };
 for(const [name,probe] of Object.entries(probes)){
  const oracle=ajv.compile({definitions:schema.definitions,$ref:`#/definitions/${name}`});
  for(const value of probe.valid){expect(oracle(value),`${name} valid`).toBe(true);expect(isHostCapacityV1(name as never,value),name).toBe(true);}
  for(const value of probe.invalid){expect(oracle(value),`${name} invalid ${JSON.stringify(value).slice(0,60)}`).toBe(false);expect(isHostCapacityV1(name as never,value),name).toBe(false);}
 }
});

test("Lineage names the classes from general to specific and skips empty parts",()=>{
 expect(hostEstimateLineageV1("run","build","@a/b:build")).toEqual(["run","run:build","run:build:@a/b:build"]);
 expect(hostEstimateLineageV1("show","")).toEqual(["show"]);
});

test("The persisted store survives concurrent writers, a corrupt file and a held lock, and a dropped sample never fails the caller",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"ram-est-"));
 try{
  expect(readHostEstimateStoreV1(directory)).toEqual(emptyHostEstimateStoreV1());
  const results=await Promise.all(Array.from({length:8},(_,index)=>updateHostEstimateStoreV1(directory,policy,store=>recordHostOutcomeV1(policy,store,["run"],outcome({endedAtMs:1000+index})),{pid:process.pid+index})));
  expect(results.every(Boolean)).toBe(true);
  expect(readHostEstimateStoreV1(directory).classes["run"]!.samples.length).toBe(8);
  writeFileSync(join(directory,schemaFile()),"{not json");
  expect(readHostEstimateStoreV1(directory)).toEqual(emptyHostEstimateStoreV1());
  mkdirSync(join(directory,"estimates.lock"));
  let clock=0;
  expect(await updateHostEstimateStoreV1(directory,policy,store=>store,{now:()=>clock+=policy.wait.lockStaleMilliseconds/2,sleep:async()=>undefined})).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

const schemaFile=():string=>schema.definitions.Protocol.const.estimatesFile;

test("A store another run holds open (a Windows sharing violation) is replaced after the declared pauses; one that stays busy drops the sample, leaves no temporary file and says so in every language",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"ram-est-busy-")),busy=(code:string)=>Object.assign(new Error(code),{code}),update=(store:HostEstimateStoreV1)=>recordHostOutcomeV1(policy,store,["run"],outcome({endedAtMs:7}));
 try{
  const pauses:number[]=[];let failures=2;
  const done=await updateHostEstimateStoreV1(directory,policy,update,{platform:"win32",sleepSync:ms=>{pauses.push(ms);},rename:(from,to)=>{if(failures-->0)throw busy("EPERM");renameSync(from,to);}});
  expect(done).toBe(true);expect(pauses).toEqual(churnProtocolV1().profiles.read.backoffMs.slice(0,2));expect(readHostEstimateStoreV1(directory).classes["run"]!.samples.length).toBe(1);
  const written:string[]=[],original=process.stderr.write.bind(process.stderr);(process.stderr as {write:unknown}).write=(chunk:string)=>{written.push(String(chunk));return true;};
  let kept:boolean;try{kept=await updateHostEstimateStoreV1(directory,policy,update,{platform:"win32",sleepSync:()=>undefined,rename:()=>{throw busy("EBUSY");}});}finally{(process.stderr as {write:unknown}).write=original;}
  expect(kept).toBe(false);expect(readdirSync(directory).filter(name=>name.endsWith(".tmp")),"no temporary file stays behind").toEqual([]);
  expect(written.join("").split("\n").filter(Boolean).length,"one line per declared language").toBe(churnProtocolV1().languages.length);expect(written.join("")).toContain(schemaFile());
  expect(readHostEstimateStoreV1(directory).classes["run"]!.samples.length,"the stored samples are untouched").toBe(1);
  expect(await updateHostEstimateStoreV1(directory,policy,update,{platform:"linux",sleepSync:()=>{throw Error("no pause expected");},rename:()=>{throw busy("EPERM");}}),"on POSIX a permission refusal is final").toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
