import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readdirSync,rmSync} from "node:fs";
import {availableParallelism,tmpdir,totalmem} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {type HostCapacityPolicyV1,estimateHostPhasesV1,hostClassBoundV1,evaluateHostCapacityV1,hostFloorMegabytesV1,scaleHostRequestV1,sizeHostRequestV1,newHostCapacityIdV1,readHostCapacityPolicyV1,readHostEstimateStoreV1,recordHostForecastV1,updateHostEstimateStoreV1,withHostCapacityV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {planTestWorkersV1,readTestWorkerPolicyV1,testWorkerFactsV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/👷️workers/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {currentWorkerHostV1} from "../../../👷️workers/🟦️.ts";
import {buildCapacityAskV1,upgradeBuildCapacityV1,withBuildCapacityV1} from "../../🟦️.ts";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hf-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool")&&!argument.startsWith("--workers"));
const args=["run","@x/y:build","--parallel=2"],leaf=buildCapacityAskV1(strip(args),{},policy)!.lineage.at(-1)!;
const sample=async()=>({host:host(),swappedOutMegabytes:0});
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const holdMachine=(directory:string)=>{
 let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const done=withHostCapacityV1({directory,policy,request:{id:newHostCapacityIdV1(),operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:13500,perJobMegabytes:0,cpuPerJob:1},control:control(),environment:{},write:()=>undefined,sample},async()=>hold);
 return{release,done};
};
const ports=(directory:string,over:object={})=>({directory,policy,strip,tool:"bazel",sample,observe:async()=>1100,write:()=>undefined,exitCode:()=>0,...over});
const finished=(directory:string,ago:number,executed:boolean)=>updateHostEstimateStoreV1(directory,policy,store=>recordHostForecastV1(policy,store,leaf,{executed,peakMegabytes:executed?2800:1200,activeMilliseconds:9000,peakAtMilliseconds:4000,endedAtMs:Date.now()-ago}));

test("The run that follows a finished run of its class answers beside a full machine with the small grant, with the follow-up line in English and German",async()=>{
 for(const [language,pattern] of [["en_US.UTF-8",/^\[capacity\] run: forecast quiet because the earlier run of its class finished \d+s ago and left its results cached: \d+ MB reserved now, the full \d+ MB only if work has to run/u],["de_DE.UTF-8",/^\[capacity\] run: ruhig prognostiziert, weil der frühere Lauf seiner Klasse vor \d+ s endete und seine Ergebnisse zwischengespeichert hinterließ: jetzt \d+ MB reserviert/u]] as const){
  const directory=fresh(),machine=holdMachine(directory),lines:string[]=[];
  await finished(directory,90_000,true);
  await sleep(40);
  const started=Date.now();
  await withBuildCapacityV1(args,{SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:language},async granted=>{
   expect(Date.now()-started,"no capacity queue").toBeLessThan(2500);
   expect(granted.handle!.reservedMegabytes).toBeLessThan(4096);
   expect(granted.args.at(-1),"the tool is told the full jobs").toBe("--parallel=2");
   await sleep(120);
  },ports(directory,{write:(line:string)=>lines.push(line)}));
  expect(lines.some(line=>pattern.test(line)),lines.join("\n")).toBe(true);
  const store=readHostEstimateStoreV1(directory);
  expect(store.classes[`${leaf}~quiet`]!.samples.length,"the follow-up run executed nothing and is learned as quiet").toBe(1);
  machine.release();await machine.done;
  expect(readdirSync(join(directory,"grants"))).toEqual([]);
 }
},30000);

test("A class without a finished run in the follow-up window asks for the full grant at the door",async()=>{
 const directory=fresh(),machine=holdMachine(directory);
 await finished(directory,policy.estimates.quietFollowUpMilliseconds+60_000,false);
 await sleep(40);
 let started=false;
 const waiting=withBuildCapacityV1(args,{SEMIO_HOST_CAPACITY_DIRECTORY:directory},async()=>{started=true;},ports(directory));
 await sleep(300);expect(started,"one old quiet run is neither history nor a follow-up").toBe(false);
 machine.release();await waiting;await machine.done;expect(started).toBe(true);
},30000);

test("A quiet grant is admitted under raised memory pressure while a full request of the same class would wait",async()=>{
 const directory=fresh();
 await finished(directory,60_000,false);await finished(directory,30_000,false);
 const warned=async()=>({host:host({pressure:"warn",availableMegabytes:7000}),swappedOutMegabytes:0});
 const started=Date.now();let granted=0;
 await withBuildCapacityV1(args,{SEMIO_HOST_CAPACITY_DIRECTORY:directory},async given=>{granted=given.handle!.reservedMegabytes;},ports(directory,{sample:warned}));
 expect(Date.now()-started,"admitted at once under raised pressure").toBeLessThan(2500);expect(granted).toBeLessThan(4096);
 const cold=fresh();let waited=true;
 const full=withBuildCapacityV1(args,{SEMIO_HOST_CAPACITY_DIRECTORY:cold},async()=>{waited=false;},ports(cold,{sample:warned}));
 await sleep(300);expect(waited,"without a forecast the soft stop holds the full request").toBe(true);
 await Promise.race([full,sleep(1)]);
},30000);

test("A multi-worker target declares its envelope: the grant carries its cores and megabytes and the end-to-end planner counts on them",async()=>{
 expect(buildCapacityAskV1(["run","@teaching/architecture-course:test-e2e"],{},policy)!.targets).toEqual(["test-e2e"]);
 expect(buildCapacityAskV1(["run-many","--targets=build,test-e2e"],{},policy)!.targets).toEqual(["build","test-e2e"]);
 const workerPolicy=readTestWorkerPolicyV1(),plans:Record<string,{cpu:number;memory:number;stacks:number}>={};
 for(const target of ["test-e2e","test-unit"]){
  const directory=fresh();
  await withBuildCapacityV1(["run",`@x/y:${target}`,"--parallel=1"],{SEMIO_HOST_CAPACITY_DIRECTORY:directory},async granted=>{
   const facts=testWorkerFactsV1(granted.handle!.environment,workerPolicy);
   plans[target]={cpu:granted.handle!.plan.cpu,memory:granted.handle!.plan.memoryMegabytes,stacks:planTestWorkersV1(workerPolicy,"e2e-stacks",facts).workers};
  },ports(directory,{tool:"nx"}));
 }
 expect(plans["test-e2e"]).toMatchObject({cpu:4,memory:6144});expect(plans["test-e2e"]!.stacks,"the sharding is no longer inert on an admitted run").toBeGreaterThan(1);
 expect(plans["test-unit"]).toMatchObject({cpu:1});expect(plans["test-unit"]!.stacks).toBe(1);
},30000);

test("A declared machine is what the door scales against and what the worker policy plans for",()=>{
 const real={cores:availableParallelism(),memoryBytes:totalmem()};
 const declared=currentWorkerHostV1({SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"2048",SEMIO_HOST_CAPACITY_CORES:"1"});
 expect(declared.cores).toBe(1);expect(declared.memoryBytes).toBe(Math.min(Math.floor(real.memoryBytes/1048576),2048)*1048576);
 expect(currentWorkerHostV1({})).toEqual(real);
 expect(currentWorkerHostV1({SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"2G",SEMIO_HOST_CAPACITY_CORES:"x"}),"a malformed declaration means the real host").toEqual(real);
});

test("The profile shrinks the reservation of a large envelope to the share a small machine may give",async()=>{
 const reserved:Record<string,number>={};
 for(const [name,environment] of [["host",{}],["8g",{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"8192"}]] as const){
  const directory=fresh();
  await withBuildCapacityV1(["run","@x/y:test-e2e","--parallel=1"],{SEMIO_HOST_CAPACITY_DIRECTORY:directory,...environment},async granted=>{reserved[name]=granted.handle!.reservedMegabytes;},ports(directory,{tool:"nx"}));
 }
 expect(reserved["8g"]!,"the 8 GB profile reserves at most half of what is above its floor").toBeLessThan(reserved["host"]!);
});

import g5 from "./🧫️fixtures/📼️g5.json";
const sizedFull=(over:HostCapacityPolicyV1,store:object)=>{
 const ask=buildCapacityAskV1(["run","@teaching/architecture-course:build"],{},over)!,base=sizeHostRequestV1(over,{id:"aaaaaaaa1",operation:ask.operation,preferredJobs:ask.preferredJobs,kinds:ask.kinds,targets:ask.targets})!;
 const estimate=estimateHostPhasesV1(over,store as never,ask.lineage,base,g5.nowMs),total=61440,scaled=scaleHostRequestV1(over,base,estimate.ratioBasisPoints,total-hostFloorMegabytesV1(over,{totalMegabytes:total}),estimate.floorMegabytes);
 return scaled.fixedMegabytes+scaled.preferredJobs*scaled.perJobMegabytes;
};
const unbounded=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,estimates:{...buildPolicy.hostCapacity.estimates,fallbackCeilingMegabytes:1048576},classCeilings:[]});

test("g5 replay: the full request a course build grows into is bounded by its declared ceiling (it was 35840 MB, five times what the class measures)",()=>{
 expect(sizedFull(unbounded,g5.store),"without a ceiling the poisoned ratios reproduce the red of gate-g").toBe(35840);
 expect(sizedFull(policy,g5.store),"the declared ceiling of the class").toBeLessThanOrEqual(12288);
});

test("g5 replay: once the class kept enough absolute peaks the full request follows what it measured, never the learned ratio",()=>{
 const classes=JSON.parse(JSON.stringify(g5.store.classes)) as Record<string,{samples:object[]}>;
 for(let index=0;index<5;index++)classes[g5.leaf]!.samples.unshift({atMs:g5.nowMs-1000*index,ratioBasisPoints:20000,activeMilliseconds:600000,peakAtMilliseconds:300000,peakMegabytes:4400+475*index});
 const measured=sizedFull(unbounded,{version:1,classes});
 expect(measured).toBeGreaterThan(5000);expect(measured,"the 90th percentile of 4.4-6.3 GiB plus the margin").toBeLessThanOrEqual(8000);
});

import g13 from "./🧫️fixtures/📼️g13.json";
const classCopy=(store:object,shiftMs=0)=>{const classes=JSON.parse(JSON.stringify((store as {classes:object}).classes)) as Record<string,{samples:{atMs:number;peakMegabytes?:number}[]}>;for(const row of Object.values(classes))for(const item of row.samples)item.atMs+=shiftMs;return {version:1 as const,classes};};
const sizedAs=(over:HostCapacityPolicyV1,store:object,asked:readonly string[],total:number,which:"ratioBasisPoints"|"growRatioBasisPoints"="growRatioBasisPoints",nowMs:number=g13.nowMs)=>{
 const ask=buildCapacityAskV1(strip(asked),{},over)!,base=sizeHostRequestV1(over,{id:"aaaaaaaa1",operation:ask.operation,preferredJobs:ask.preferredJobs,kinds:ask.kinds,targets:ask.targets})!;
 const estimate=estimateHostPhasesV1(over,store as never,ask.lineage,base,nowMs),scaled=scaleHostRequestV1(over,base,estimate[which],total-hostFloorMegabytesV1(over,{totalMegabytes:total}),estimate.floorMegabytes);
 return scaled.fixedMegabytes+scaled.preferredJobs*scaled.perJobMegabytes;
};

test("g13 replay: a class that is neither declared nor measured is bounded by the generic fallback, never by 200% of its default (35840 MB for a dashboard that peaks at 4.8 GB)",()=>{
 expect(sizedAs(unbounded,g13.store,g13.args,g13.totalMegabytes),"without any bound the learned ratios reproduce the red of gate-g").toBe(35840);
 expect(sizedAs(policy,g13.store,g13.args,g13.totalMegabytes),"the generic fallback").toBeLessThanOrEqual(policy.estimates.fallbackCeilingMegabytes);
 expect(sizedAs(policy,g13.store,g13.args,g13.totalMegabytes,"ratioBasisPoints"),"the first admission of a class that learned something is bounded as well").toBeLessThanOrEqual(policy.estimates.fallbackCeilingMegabytes);
 expect(policy.estimates.fallbackCeilingMegabytes).toBeLessThan(g13.spareMegabytes);
});

test("g13 replay: a class without any history keeps its default for the first admission (the gate shrinks its jobs) but never grows into more than the fallback",()=>{
 const empty={version:1,classes:{}};
 expect(sizedAs(policy,empty,g13.args,g13.totalMegabytes,"ratioBasisPoints"),"the conservative default of 8 jobs").toBeGreaterThan(policy.estimates.fallbackCeilingMegabytes);
 expect(sizedAs(policy,empty,g13.args,g13.totalMegabytes)).toBeLessThanOrEqual(policy.estimates.fallbackCeilingMegabytes);
});

test("g13 replay: once the class kept enough absolute peaks the full request follows what it measured (4.6-4.8 GB plus the margin), above or below the fallback",()=>{
 const classes=classCopy(g13.store).classes;
 for(let index=0;index<4;index++)classes[g13.leaf]!.samples.unshift({atMs:g13.nowMs-1000*index,ratioBasisPoints:20000,activeMilliseconds:200000,peakAtMilliseconds:120000,peakMegabytes:g13.peakMegabytes[index%2]!} as never);
 const measured=sizedAs(policy,{version:1,classes},g13.args,g13.totalMegabytes);
 expect(measured).toBeGreaterThanOrEqual(4800);expect(measured,"the 90th percentile of the kept peaks plus the margin").toBeLessThanOrEqual(6200);
 for(const row of classes[g13.leaf]!.samples)if(row.peakMegabytes!==undefined)row.peakMegabytes=14000;
 expect(sizedAs(policy,{version:1,classes},g13.args,g13.totalMegabytes),"a class that measured more than the fallback may ask for it: only declared ceilings cap a measured class").toBeGreaterThan(policy.estimates.fallbackCeilingMegabytes);
});

test("g13 replay: the dashboard build whose quiet reservation has to grow is admitted beside 16 GB of free memory instead of waiting for 28672 MB",async()=>{
 for(const [name,total,available] of [["a large host with 16 GB free",g13.totalMegabytes,16384],["a 16 GB machine",16384,12800]] as const){
  const directory=fresh();
  await updateHostEstimateStoreV1(directory,policy,()=>classCopy(g13.store,Date.now()-g13.nowMs));
  await updateHostEstimateStoreV1(directory,policy,store=>recordHostForecastV1(policy,store,g13.leaf,{executed:true,peakMegabytes:4800,activeMilliseconds:200000,peakAtMilliseconds:120000,endedAtMs:Date.now()-442_000}));
  const lines:string[]=[],environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:String(total)};
  const given=async()=>({host:host({totalMegabytes:total,availableMegabytes:available}),swappedOutMegabytes:0});
  let outcome="";
  await withBuildCapacityV1(g13.args,environment,async granted=>{
   expect(granted.handle!.reservedMegabytes,`${name}: the follow-up is admitted small`).toBeLessThan(4096);
   const controller=new AbortController(),timer=setTimeout(()=>controller.abort(),3000);
   try{outcome=await upgradeBuildCapacityV1(controller.signal,environment);}catch{outcome="starved";}finally{clearTimeout(timer);}
   await sleep(60);
  },ports(directory,{sample:given,write:(line:string)=>lines.push(line)}));
  expect(outcome,`${name}: ${lines.filter(line=>/waiting|grow/u.test(line)).slice(0,2).join(" | ")}`).toBe("upgraded");
  const asked=lines.map(line=>/to (\d+) MB/u.exec(line)?.[1]).find(value=>value!==undefined);
  if(asked!==undefined)expect(Number(asked)).toBeLessThanOrEqual(policy.estimates.fallbackCeilingMegabytes);
 }
},40000);

import g29 from "./🧫️fixtures/📼️g29.json";
const g29Latest=Math.max(...Object.entries(g29.store.classes).filter(([key])=>key.startsWith(g29.leaf)).flatMap(([,row])=>row.samples.map(item=>item.atMs)));
const g29Store=(own?:number)=>{const store=classCopy(g29.store,Date.now()-712_000-g29Latest);if(own!==undefined)for(const row of store.classes[g29.leaf]!.samples)row.peakMegabytes=own;return store;};
const growOn=async(recording:{leaf:string;args:readonly string[]},store:object,total:number,available:number,language:string)=>{
 const directory=fresh();
 await updateHostEstimateStoreV1(directory,policy,()=>store as never);
 const lines:string[]=[],environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:String(total),LANG:language};
 const given=async()=>({host:host({totalMegabytes:total,availableMegabytes:available}),swappedOutMegabytes:0});
 let outcome="",inside:{plan:{jobs:number;cpu:number;memoryMegabytes:number}|null;parallel:string|undefined}={plan:null,parallel:undefined};
 await withBuildCapacityV1(recording.args,environment,async granted=>{
  expect(granted.handle!.reservedMegabytes,"the follow-up is admitted small").toBeLessThan(4097);
  const controller=new AbortController(),timer=setTimeout(()=>controller.abort(),3000);
  try{outcome=await upgradeBuildCapacityV1(controller.signal,environment);}catch{outcome="starved";}finally{clearTimeout(timer);}
  inside={plan:JSON.parse(environment.SEMIO_HOST_CAPACITY_PLAN??"null") as {jobs:number;cpu:number;memoryMegabytes:number}|null,parallel:environment.SEMIO_BUILD_PARALLEL};
  await sleep(60);
 },ports(directory,{sample:given,write:(line:string)=>lines.push(line)}));
 return{outcome,lines,...inside};
};

test("g29 replay: the print class kept no peak of its own, so the pooled peaks of other products (14866 MB in `run`) no longer bound it: it grows into the fallback, not into 18585 MB",()=>{
 const store=g29Store(),now=Date.now(),lineage=["run","run:test",g29.leaf];
 expect(Math.max(...store.classes["run"]!.samples.map(item=>item.peakMegabytes??0)),"the pool holds the dashboard peak that bounded print at 18585 MB").toBeGreaterThan(14000);
 expect(hostClassBoundV1(policy,store as never,lineage,now)).toEqual({megabytes:policy.estimates.fallbackCeilingMegabytes,source:"fallback"});
 expect(sizedAs(policy,store,g29.args,g29.totalMegabytes,"growRatioBasisPoints",now)).toBeLessThanOrEqual(policy.estimates.fallbackCeilingMegabytes);
});

test("g29 replay: the print build whose quiet reservation has to grow is admitted beside 10 GB of spare memory on a 32 GB host instead of waiting for 18585 MB",async()=>{
 const {outcome,lines}=await growOn(g29,g29Store(),g29.totalMegabytes,g29.spareMegabytes+4096,"en_US.UTF-8");
 expect(outcome,lines.filter(line=>/waiting|grow/u.test(line)).slice(0,2).join(" | ")).toBe("upgraded");
 const grown=lines.map(line=>/reservation grown from (\d+) MB to (\d+) MB/u.exec(line)).find(row=>row!==null);
 expect(grown,lines.join("\n")).toBeTruthy();expect(Number(grown![2])).toBeLessThanOrEqual(policy.estimates.fallbackCeilingMegabytes);
});

test("A run that is the only one running never waits on itself: when even the bounded size does not fit, its reservation grows to the largest size that fits and says so in English and German",async()=>{
 for(const [language,pattern] of [["en_US.UTF-8",/reservation grown from (\d+) MB to (\d+) MB of the (\d+) MB it asked for, after \d+s: nothing else of ours is running/u],["de_DE.UTF-8",/Reservierung nach \d+ s von (\d+) MB auf (\d+) MB der angefragten (\d+) MB vergroessert: nichts anderes von uns laeuft/u]] as const){
  const {outcome,lines}=await growOn(g29,g29Store(14000),g29.totalMegabytes,g29.spareMegabytes+4096,language);
  expect(outcome,lines.join("\n")).toBe("upgraded");
  const row=lines.map(line=>pattern.exec(line)).find(item=>item!==null);
  expect(row,lines.join("\n")).toBeTruthy();
  const [from,to,wanted]=[Number(row![1]),Number(row![2]),Number(row![3])];
  expect(wanted,"the measured bound of its own class").toBeGreaterThan(g29.spareMegabytes);expect(to).toBeGreaterThanOrEqual(from);expect(to).toBeLessThan(wanted);expect(to-from,"only what is free").toBeLessThanOrEqual(g29.spareMegabytes+1);
  expect(lines.some(line=>/waiting \d+s to grow|wartet seit/u.test(line)),"it never waited").toBe(false);
 }
},40000);

import g34 from "./🧫️fixtures/📼️g34.json";
const g34Latest=Math.max(...Object.entries(g34.store.classes).filter(([key])=>key.startsWith(g34.leaf)).flatMap(([,row])=>row.samples.map(item=>item.atMs)));
const g34Store=()=>classCopy(g34.store,Date.now()-1_059_000-g34Latest);

test("g34 replay: print asks for 18515 MB because that is its own measured bound (the 90th percentile of the peaks its cold runs kept, 14810 MB, plus the margin), not because of the 200% clamp or a borrowed pool",()=>{
 const store=g34Store(),peaks=store.classes[g34.leaf]!.samples.map(item=>item.peakMegabytes).filter((value):value is number=>value!==undefined).sort((left,right)=>left-right);
 const bound=hostClassBoundV1(policy,store as never,["run","run:test",g34.leaf],Date.now());
 expect(bound.source).toBe("measured");expect(peaks.length).toBeGreaterThanOrEqual(policy.estimates.minimumSamples);
 expect(bound.megabytes,"the weighted 90th percentile of its own peaks, plus 25%").toBeGreaterThanOrEqual(Math.round(peaks[Math.floor(peaks.length/2)]!*1.25));expect(bound.megabytes).toBeLessThanOrEqual(Math.round(peaks.at(-1)!*1.25));
});

test("g34 replay: a partial growth keeps only the jobs the free memory carries at the learned footprint of one job and the plan of the process follows it, in English and German (it kept all 8 jobs on 10239 MB and could swap a 16 GB host)",async()=>{
 for(const [language,pattern] of [["en_US.UTF-8",/reservation grown from (\d+) MB to (\d+) MB of the (\d+) MB it asked for, after \d+s: .*; the run goes on with (\d+) of (\d+) job\(s\)/u],["de_DE.UTF-8",/Reservierung nach \d+ s von (\d+) MB auf (\d+) MB der angefragten (\d+) MB vergroessert: .*; der Lauf geht mit (\d+) von (\d+) Job\(s\) weiter/u]] as const){
  const {outcome,lines,plan,parallel}=await growOn(g34,g34Store(),g34.totalMegabytes,g34.availableMegabytes,language);
  expect(outcome,lines.join("\n")).toBe("upgraded");
  const row=lines.map(line=>pattern.exec(line)).find(item=>item!==null);
  expect(row,lines.join("\n")).toBeTruthy();
  const [from,to,wanted,jobs,preferred]=row!.slice(1).map(Number) as [number,number,number,number,number];
  expect(preferred).toBe(8);expect(jobs,"fewer jobs than asked for, at least one").toBeLessThan(8);expect(jobs).toBeGreaterThanOrEqual(1);
  const ratio=wanted/(1536+8*2048),fixed=Math.round(1536*ratio),perJob=Math.round(2048*ratio);
  expect(to,"the reservation is what these jobs hold at the learned footprint of one job").toBe(fixed+jobs*perJob);expect(to).toBeLessThanOrEqual(g34.availableMegabytes-4096+from);
  expect(to,"the reservation is what these jobs hold, never less than before").toBeGreaterThanOrEqual(from);expect(to).toBeLessThanOrEqual(Math.max(from,fixed+jobs*perJob));
  expect(plan!.jobs,"the plan of the process follows the grant").toBe(jobs);expect(plan!.cpu).toBe(jobs);expect(plan!.memoryMegabytes).toBe(jobs*2048);expect(parallel).toBe(String(jobs));
 }
},40000);

test("A growth that fits entirely keeps the plan and all jobs of the process untouched",async()=>{
 const {outcome,plan,parallel,lines}=await growOn(g34,g34Store(),g34.totalMegabytes,60000,"en_US.UTF-8");
 expect(outcome,lines.join("\n")).toBe("upgraded");expect(plan!.jobs).toBe(8);expect(parallel).toBe("8");expect(lines.some(line=>/of the \d+ MB it asked for/u.test(line))).toBe(false);
},40000);
