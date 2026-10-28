import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {estimateHostRatioV1,evaluateHostCapacityV1,forecastHostQuietV1,hostFloorMegabytesV1,hostDeclarationKeyV1,hostProfileV1,planHostCapacityV1,quietHostRequestV1,newHostCapacityIdV1,readHostCapacityStatusV1,withHostCapacityV1,readHostCapacityPolicyV1,recordHostForecastV1,scaleHostRequestV1,sizeHostRequestV1,type HostCapacityPolicyV1,type HostEstimateStoreV1,type HostForecastV1,type HostGrantV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from "../../🟦️.ts";
import {planTestWorkersV1,readTestWorkerPolicyV1,testWorkerFactsV1} from "../../../../🧪️testing/👷️workers/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import recorded from "./🧫️fixtures/📼️recorded.json";

const plain=(value:unknown)=>JSON.parse(JSON.stringify(value));
const make=(base:object,over:{quietSoftStopGrants:number;quietFollowUpMilliseconds:number;envelopes:readonly object[]}):HostCapacityPolicyV1=>readHostCapacityPolicyV1({...base,memory:{...(base as {memory:object}).memory,quietSoftStopGrants:over.quietSoftStopGrants},estimates:{...(base as {estimates:object}).estimates,quietFollowUpMilliseconds:over.quietFollowUpMilliseconds},envelopes:over.envelopes,classCeilings:(base as {classCeilings?:readonly object[]}).classCeilings??[]});
const off=make(decisionFixture.policy,fixture.policy.off),on=make(decisionFixture.policy,fixture.policy.on);
const ramped=(policy:HostCapacityPolicyV1)=>readHostCapacityPolicyV1({...policy,memory:{...policy.memory,rampMilliseconds:60000}}),forecastOn=ramped(on),forecastOff=ramped(off);
const oldPolicy=make(recorded.policy,fixture.policy.replay.old),newPolicy=make(recorded.policy,fixture.policy.replay.new);
const replay=fixture.replay;

test("Gate-g row 4c replays: the hot no-op that learned 200% of its default and waited 165 s for 7168 MB is forecast quiet and asks for the small grant",()=>{
 const store=replay.store as HostEstimateStoreV1,[sized,followed]=replay.steps,total=replay.host.totalMegabytes;
 for(const policy of [oldPolicy,newPolicy]){
  const estimate=estimateHostRatioV1(policy,store,replay.lineage,sized!.nowMs),floor=hostFloorMegabytesV1(policy,{totalMegabytes:total}),base=sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"run",preferredJobs:1,kinds:["build"]})!;
  const scaled=scaleHostRequestV1(policy,base,estimate.ratioBasisPoints,total-floor);
  expect({ratioBasisPoints:estimate.ratioBasisPoints,samples:estimate.samples,fixedMegabytes:scaled.fixedMegabytes,perJobMegabytes:scaled.perJobMegabytes,needMegabytes:scaled.fixedMegabytes+scaled.perJobMegabytes,defaultMegabytes:base.fixedMegabytes+base.perJobMegabytes}).toEqual(sized!.ask);
 }
 expect(sized!.ask.ratioBasisPoints,"the class learned the 200% bound").toBe(20000);expect(sized!.ask.samples,"from 12 earlier runs").toBe(12);expect(sized!.ask.needMegabytes,"and the run needs 7168 MB").toBe(7168);
 expect(forecastHostQuietV1(oldPolicy,store,replay.leaf,sized!.nowMs),"before: the class has no quiet run, so the door asks for the full grant").toEqual(sized!.forecast as HostForecastV1);
 expect(sized!.forecast.quiet).toBe(false);
 const now=forecastHostQuietV1(newPolicy,store,replay.leaf,followed!.nowMs);
 expect(now).toEqual(followed!.forecast as HostForecastV1);expect(now.quiet,"after: the run follows a finished run of its class").toBe(true);expect(now.followUpMilliseconds).toBe(90000);
 const full=sizeHostRequestV1(newPolicy,{id:"aaaaaaaa1",operation:"run",preferredJobs:1,kinds:["build"]})!;
 expect(quietHostRequestV1(full,now).fixedMegabytes,"it reserves a quarter of the 7168 MB").toBeLessThan(sized!.ask.needMegabytes/3);
});

test("On the 16 GB profile of the bench2-g log the quiet grant is admitted beside raised pressure, the full request keeps waiting, and the hard stops still hold",()=>{
 for(const world of replay.worlds){
  const policy=world.policy==="new"?newPolicy:oldPolicy;
  expect(plain(evaluateHostCapacityV1(policy,world.host as HostSampleV1,world.ledger as HostGrantV1[],world.queue as HostWaiterV1[],world.nowMs)),world.name).toEqual(world.expect);
 }
 const verdict=(name:string)=>replay.worlds.find(world=>world.name===name)!.expect.decisions[0]!;
 expect(verdict("full request, raised pressure (the bench2-g log)").reason).toBe("memory");
 expect(verdict("quiet request, the soft stop holds it").reason).toBe("pressure-warn");
 expect(verdict("quiet request, the soft stop is ignored").verdict).toBe("admit");
 expect(verdict("quiet request, attached images at the maximum, the soft stop holds it").reason).toBe("attaches");
 expect(verdict("quiet request, attached images at the maximum, ignored").verdict).toBe("admit");
 expect(verdict("quiet request, critical pressure is a hard stop").reason).toBe("pressure-critical");
});

test("The chain goes on: every run after a finished run is forecast quiet for the follow-up window, the window ends, the forecast equals the oracle on every step",()=>{
 let store=plain(replay.store) as HostEstimateStoreV1;
 for(const step of replay.sequence){
  store=recordHostForecastV1(newPolicy,store,replay.leaf,step.run);
  expect(forecastHostQuietV1(newPolicy,store,replay.leaf,step.nextNowMs),step.name).toEqual(step.expect as HostForecastV1);
  expect(step.expect.quiet,step.name).toBe(true);
 }
 expect(plain(store),"the recorded store ends as the oracle's").toEqual(replay.finalStore);
 expect(forecastHostQuietV1(newPolicy,store,replay.leaf,replay.afterWindow.nowMs)).toEqual(replay.afterWindow.expect as HostForecastV1);
 expect(replay.afterWindow.expect.quiet,"without a run for longer than the window and without enough quiet history the door asks for the full grant again").toBe(false);
});

test("The forecast of every stored class equals the oracle under the follow-up rule and replays its recorded runs",()=>{
 for(const item of fixture.forecasts){
  let store:HostEstimateStoreV1={version:1,classes:{}};
  for(const run of item.runs)store=recordHostForecastV1(forecastOn,store,item.leaf,run);
  expect(plain(store),`${item.name} store`).toEqual(item.store);
  expect(forecastHostQuietV1(forecastOn,item.store as HostEstimateStoreV1,item.leaf,item.nowMs),item.name).toEqual(item.expect as HostForecastV1);
  expect(forecastHostQuietV1(forecastOff,item.store as HostEstimateStoreV1,item.leaf,item.nowMs).followUpMilliseconds,"the knob at 0 switches the rule off").toBeUndefined();
 }
 expect(fixture.forecastCoverage.followUp).toBeGreaterThan(0);expect(fixture.forecastCoverage.busy).toBeGreaterThan(0);
});

test("Quiet requests and the full requests around them decide as the oracle on every stored world, and the flag changes only quiet requests",()=>{
 for(const world of fixture.decisions){
  const got=evaluateHostCapacityV1(on,world.host as HostSampleV1,world.ledger as HostGrantV1[],world.queue as HostWaiterV1[],world.nowMs);
  expect(plain(got),world.name).toEqual(world.expect);
  const plainGot=evaluateHostCapacityV1(off,world.host as HostSampleV1,world.ledger as HostGrantV1[],world.queue as HostWaiterV1[],world.nowMs);
  const quietIds=new Set((world.queue as HostWaiterV1[]).filter(row=>row.request.quiet===true).map(row=>row.request.id));
  if(quietIds.size===0)expect(plain(got.decisions),`${world.name}: without quiet requests the flag changes nothing`).toEqual(plain(plainGot.decisions));
 }
 expect(fixture.decisionsChangedByTheFlag,"the vectors reach worlds the flag changes").toBeGreaterThan(0);
});

test("A declared envelope sizes an end-to-end target at what its workers hold, and the test-worker policy counts on it",()=>{
 const workerPolicy=readTestWorkerPolicyV1(fixture.workerPolicy);
 for(const item of fixture.envelopes){
  const policy=item.policy==="on"?on:off,request=sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:item.operation,preferredJobs:item.jobs,kinds:item.kinds as never,targets:item.targets})!;
  expect(plain(request),item.name).toEqual(item.expect.request);
  const plan=planHostCapacityV1(policy,{totalMegabytes:32768},request as HostRequestV1,item.grantedJobs);
  expect({jobs:plan.jobs,cpu:plan.cpu,memoryMegabytes:plan.memoryMegabytes,fixedMegabytes:plan.fixedMegabytes},item.name).toEqual(item.expect.plan);
  const facts=testWorkerFactsV1({SEMIO_HOST_CAPACITY_PLAN:JSON.stringify(plan)},workerPolicy,{cores:10,totalMegabytes:32768});
  expect({cores:facts.cores,memoryMegabytes:facts.memoryMegabytes},item.name).toEqual(item.expect.facts);
  expect(item.expect.workers.map(row=>{const got=planTestWorkersV1(workerPolicy,row.runner as never,facts);return{runner:row.runner,workers:got.workers,bound:got.bound};}),item.name).toEqual(item.expect.workers);
 }
 const byName=(name:string)=>fixture.envelopes.find(item=>item.name===name)!.expect.workers.find(row=>row.runner==="e2e-stacks")!;
 expect(byName("e2e run, no envelope declared"),"the control: one stack bounded by the 2048 MB of the admitted job").toEqual({runner:"e2e-stacks",workers:1,bound:"memory"});
 expect(byName("e2e run, envelope declared").workers,"with the declared envelope the policy plans its stacks").toBeGreaterThan(1);
 expect(byName("unit tests are not end-to-end").workers).toBe(1);
});

test("A declared machine shrinks what every planner sees, never grows it, and a malformed value means the real host",()=>{
 for(const item of fixture.profiles){
  const real={...item.real,declared:false};
  expect(plain(hostProfileV1(item.environment,real)),item.name).toEqual(item.expect);
  expect(hostDeclarationKeyV1(item.environment),item.name).toBe(item.key);
 }
});

const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const shared=readHostCapacityPolicyV1({...plain(on),wait:{...on.wait,sampleMaximumAgeMilliseconds:600000}});
const controls=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const machine=(total:number,calls:string[],label:string)=>async()=>{calls.push(label);return{host:{version:1 as const,atMs:Date.now(),platform:"darwin" as const,cores:10,totalMegabytes:total,availableMegabytes:total-4000,pressure:"normal" as const,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10},swappedOutMegabytes:0};};

test("The sample file is shared by every process of the machine, so a process reuses it only when it declares the same machine and never inherits another process's declaration",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"hs-"));roots.push(directory);const calls:string[]=[];
 const status=(environment:Record<string,string>,label:string,total:number)=>readHostCapacityStatusV1({directory,policy:shared,control:controls(),environment,sample:machine(total,calls,label)});
 const plainHost=(await status({},"plain",32768)).host;expect(plainHost.totalMegabytes).toBe(32768);
 await status({},"plain again",32768);expect(calls,"an undeclared process reuses the fresh sample of an undeclared process").toEqual(["plain"]);
 const small=(await status({SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"},"small",16384)).host;
 expect(calls,"a process that declares a machine takes its own sample").toEqual(["plain","small"]);expect(small.totalMegabytes).toBe(16384);
 await status({SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"},"small again",16384);expect(calls,"and reuses the sample of the same declaration").toEqual(["plain","small"]);
 const back=(await status({},"plain after",32768)).host;expect(calls,"a process without a declaration does not inherit the declared sample").toEqual(["plain","small","plain after"]);expect(back.totalMegabytes).toBe(32768);
 expect(JSON.parse(readFileSync(join(directory,"sample.json"),"utf8")).declared,"the file names the machine it describes").toBe("");
 await status({SEMIO_HOST_CAPACITY_CORES:"4"},"cores",32768);expect(JSON.parse(readFileSync(join(directory,"sample.json"),"utf8")).declared).toBe("-/4");
 await withHostCapacityV1({directory,policy:shared,request:{id:newHostCapacityIdV1(),operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:1024,perJobMegabytes:0,cpuPerJob:1},control:controls(),environment:{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"},write:()=>undefined,sample:machine(16384,calls,"gate")},async()=>undefined);
 expect(calls.at(-1),"the gate itself samples per declaration").toBe("gate");
},30000);

test("The closed schema accepts the declared envelope and the new knobs and refuses malformed ones, equal to Ajv",()=>{
 const ajv=new Ajv({strict:false,allErrors:false});ajv.addSchema({$id:"hc",definitions:schema.definitions});
 const agree=(definition:"Policy"|"Envelope",value:unknown)=>{const mine=(()=>{try{readHostCapacityPolicyV1(value);return true;}catch{return false;}})(),theirs=ajv.validate({$ref:`hc#/definitions/${definition}`},value) as boolean;if(definition==="Policy")expect(mine,JSON.stringify(value).slice(0,80)).toBe(theirs);return theirs;};
 const envelope={name:"end-to-end",pattern:"^test-e2e(?:-|$)",cpu:4,megabytes:6144};
 expect(agree("Envelope",envelope)).toBe(true);expect(agree("Envelope",{...envelope,cpu:0})).toBe(false);expect(agree("Envelope",{...envelope,extra:1})).toBe(false);expect(agree("Envelope",{...envelope,name:"End"})).toBe(false);
 const policy=plain(on);expect(agree("Policy",policy)).toBe(true);
 expect(agree("Policy",{...policy,envelopes:[{...envelope,megabytes:0}]})).toBe(false);
 const {envelopes:_,...without}=policy;expect(agree("Policy",without),"the envelopes are declared, never implied").toBe(false);
 expect(agree("Policy",{...policy,memory:{...policy.memory,quietAvoidsSoftStops:"yes"}})).toBe(false);
 expect(agree("Policy",{...policy,estimates:{...policy.estimates,quietFollowUpMilliseconds:-1}})).toBe(false);
});

test("Three agents on a 16 GB host: at most maximumQuiet quiet grants are outstanding, at most quietSoftStopGrants of them under a soft stop, and the upgrade honours the soft stop",()=>{
 const small=make(decisionFixture.policy,fixture.policy.on),limited=readHostCapacityPolicyV1({...plain(small),memory:{...small.memory,maximumQuiet:2,quietSoftStopGrants:1}});
 const at=1_800_000_000_000,host=(pressure:"normal"|"warn"):HostSampleV1=>({version:1,atMs:at,platform:"darwin",cores:8,totalMegabytes:16384,availableMegabytes:9000,pressure,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
 const full=(id:string)=>({id,operation:"run",priority:"batch" as const,minimumJobs:1,preferredJobs:8,fixedMegabytes:3072,perJobMegabytes:512,cpuPerJob:1}),quiet=(id:string)=>quietHostRequestV1(full(id),{quiet:true,megabytes:2048,rampMilliseconds:60000,samples:3,shareBasisPoints:9000});
 const agents=(pressure:"normal"|"warn")=>evaluateHostCapacityV1(limited,host(pressure),[],["aaaaaaaa1","bbbbbbbb2","cccccccc3"].map((id,index)=>({request:quiet(id),arrivedAtMs:at+index,pid:100+index})),at+10).decisions.map(row=>`${row.id[0]}:${row.verdict}:${row.reason}`);
 expect(agents("normal"),"three quiet agents, cap two").toEqual(["a:admit:fits","b:admit:fits","c:wait:quiet-limit"]);
 expect(agents("warn"),"under a raised pressure only quietSoftStopGrants of them start, the rest wait for the soft stop").toEqual(["a:admit:fits","b:wait:pressure-warn","c:wait:pressure-warn"]);
 const held:HostGrantV1={id:"aaaaaaaa1",operation:"run",jobs:1,cpu:1,reservedMegabytes:2048,admittedAtMs:at-1000,observedMegabytes:500,pid:100,bootAtMs:1,quiet:true,arrivedAtMs:at-1000};
 const upgrade=evaluateHostCapacityV1(limited,host("warn"),[held],[{request:full("aaaaaaaa1"),arrivedAtMs:at-1000,pid:100,upgrade:true}],at+10).decisions[0]!;
 expect(`${upgrade.verdict}:${upgrade.reason}`,"once the run needs real work it honours the soft stop like any request").toBe("wait:pressure-warn");
});
