import {expect,test}from"bun:test";
import Ajv from"ajv";
import {availableParallelism,tmpdir}from"node:os";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import {join}from"node:path";
import {spawnSync}from"node:child_process";
import {fileURLToPath}from"node:url";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";
declareBuildTestEnvelopeV1({owner:"test:compiler-units",maximumElapsedMilliseconds:300000,capabilities:{}});
import {testPythonV1}from"../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import type{ScriptInvocation}from"../../../../🧭️routing/📥️invocation/🟦️.ts";
import {acquireCompilerCapacityLeaseV1,readCompilerCapacityRequestV1,scriptCompilerAdmissionControlV1}from"../../🟦️.ts";
import orderPolicy from"../../../../🔒️leases/🧭️order/🔣️policy.json";
import {fixedCompilerCapacityRequestV1,compileUnitMegabytesV1,compileUnitsForHostV1,compileUnitsRequestV1,describeCompileUnitsV1,planCompileUnitsV1,readCompileUnitsHostV1,requireGrantedCompileUnitsV1,type CompileUnitsHostV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import leaseSchema from"../../🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
const artifactOutput=process.env.SEMIO_TEST_ARTIFACT_DIR??mkdtempSync(join(tmpdir(),"semio-capacity-"));if(process.env.SEMIO_TEST_ARTIFACT_DIR===undefined)process.once("exit",()=>{rmSync(artifactOutput,{recursive:true,force:true});});
const output=artifactOutput;
mkdirSync(output,{recursive:true});
const distinctClaimants=join(output,"lease-order-policy.json");writeFileSync(distinctClaimants,JSON.stringify({...orderPolicy,fairness:{...orderPolicy.fairness,reentrantShared:false}}));process.env.SEMIO_LEASE_ORDER_POLICY=distinctClaimants;process.env.SEMIO_LEASE_ORDER="off";
const storage=mkdtempSync(join(output,"units-")),oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url)),schemaPath=fileURLToPath(new URL("../🧬️schema/🔣️.json",import.meta.url));
let invocation!:ScriptInvocation;
const control=(additional?:AbortSignal)=>scriptCompilerAdmissionControlV1(invocation,"test:compile-units",120_000,additional);
const python=(cases:unknown):unknown[]=>{const result=spawnSync(testPythonV1()??"python3",["-I",oracle,schemaPath],{input:JSON.stringify(cases),encoding:"utf8",maxBuffer:1<<26});expect(result.status,result.stderr).toBe(0);return JSON.parse(result.stdout);};
test("compile units values agree with an independent schema oracle",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema,"units");const host=ajv.getSchema("units#/definitions/Host")!,decision=ajv.getSchema("units#/definitions/Decision")!;
 for(const row of fixture.cases){expect(host(row.host)).toBe(true);expect(decision(row.expect)).toBe(true);}
 for(const row of fixture.invalid){expect(host(row.host),row.name).toBe(false);expect(()=>planCompileUnitsV1(row.host as never),row.name).toThrow();}
});
test("the fixture cases are planned exactly as written down",()=>{
 for(const row of fixture.cases)expect(planCompileUnitsV1(row.host as CompileUnitsHostV1,row.intent),row.name).toEqual(row.expect as never);
});
test("every state of a generated grid is planned like the Python model",()=>{
 const grid:{host:CompileUnitsHostV1;intent:{needed:number|null}}[]=[];
 for(const cores of[1,2,3,4,6,8,10,12,16,24,64])for(const available of[null,0,1024,2048,4096,6000,8192,12288,16384,65536])for(const pressure of[null,"normal","warn","critical"] as const)for(const granted of[null,1,2,5,12])for(const memory of granted===null?[null]:[granted*2048,2908,12000])for(const unit of[700,2048])for(const needed of[null,1,3,9])grid.push({host:{cores,availableMegabytes:available,pressure,grantedJobs:granted,grantedMegabytes:memory,unitMegabytes:unit},intent:{needed}});
 const expected=python(grid);expect(grid.length).toBeGreaterThan(40000);
 grid.forEach((row,index)=>expect(planCompileUnitsV1(row.host,row.intent),JSON.stringify(row)).toEqual(expected[index] as never));
});
test("the plan never exceeds half of the host, never starts below one core and keeps minimum within preferred",()=>{
 for(const cores of[1,2,3,4,5,7,10,33]){const plan=planCompileUnitsV1({cores,availableMegabytes:null,pressure:null,grantedJobs:null,grantedMegabytes:null,unitMegabytes:2048});expect(plan.preferred).toBeLessThanOrEqual(Math.max(1,Math.floor(cores/2)));expect(plan.minimum).toBeGreaterThanOrEqual(1);expect(plan.minimum).toBeLessThanOrEqual(plan.preferred);if(cores>=2)expect(plan.preferred*2).toBeLessThanOrEqual(cores);}
});
test("compile requests admit minimum within maximum like the schema oracle",()=>{
 const oracleRequest=new Ajv({strict:true}).compile(leaseSchema.definitions.Request);
 for(const row of fixture.requests){expect(oracleRequest(row.value),row.name).toBe(row.schemaAccepts);if(row.accepted)expect(readCompilerCapacityRequestV1(row.value as never),row.name).toMatchObject(row.value);else expect(()=>readCompilerCapacityRequestV1(row.value as never),row.name).toThrow();}
 expect(compileUnitsRequestV1(planCompileUnitsV1({cores:10,availableMegabytes:null,pressure:null,grantedJobs:null,grantedMegabytes:null,unitMegabytes:2048}))).toEqual({kind:"available",maximumParallelism:5,minimumParallelism:3});
});
test("the live host is read with free memory, pressure and the cores of the machine",async()=>{
 const live=await readCompileUnitsHostV1({environment:{}}),planned=await compileUnitsForHostV1({environment:{}});
 expect(live.cores).toBe(availableParallelism());expect(live.grantedJobs).toBeNull();expect(planned.request.kind).toBe("available");expect(planned.decision.preferred).toBeLessThanOrEqual(Math.max(1,Math.floor(live.cores/2)));
 const granted=await readCompileUnitsHostV1({environment:{SEMIO_HOST_CAPACITY_PLAN:JSON.stringify({version:1,id:"abcdefgh",jobs:3,cpu:3,memoryMegabytes:8192,fixedMegabytes:1536,floorMegabytes:4096})}});
 expect([granted.grantedJobs,granted.grantedMegabytes,granted.unitMegabytes]).toEqual([3,8192,2048]);
 const aborted=AbortSignal.abort(Error("cancelled"));await expect(readCompileUnitsHostV1({environment:{},signal:aborted})).rejects.toThrow("cancelled");
});
test("the footprint of one compile unit is the measured quantile of the newest unit peaks, equal to the Python model and numpy",()=>{
 const history=(peaks:readonly number[])=>new Map(peaks.map((peak,index)=>[`unit-${index}`,[{peakRssBytes:peak*3},{peakRssBytes:peak}]] as const));
 const cases=[[],[700],Array.from({length:7},(_,index)=>(index+1)*300*1048576),Array.from({length:8},(_,index)=>(index+1)*300*1048576),Array.from({length:40},(_,index)=>((index*7919)%97+1)*31*1048576+index),Array.from({length:12},()=>10*1048576),Array.from({length:200},(_,index)=>(index%13+1)*512*1048576)];
 const answers=python(cases.map(peaks=>({history:Object.fromEntries([...history(peaks)].map(([key,rows])=>[key,rows]))})));
 cases.forEach((peaks,index)=>expect(compileUnitMegabytesV1(history(peaks)),String(index)).toBe(answers[index] as number));
 expect(compileUnitMegabytesV1(),"no history: the authored footprint").toBe(schema.definitions.Policy.const.megabytesPerUnit);
 expect(compileUnitMegabytesV1(history(cases[2]!)),"fewer than the minimum samples: the authored footprint").toBe(2048);
 expect(compileUnitMegabytesV1(history(cases[3]!)),"the 75th percentile by nearest rank of 8 units is the 6th").toBe(1800);
 expect(compileUnitMegabytesV1(history(cases[5]!)),"never below the minimum").toBe(256);
});
test("a compile under a grant that grew after the process started plans with the grant as it is now, and its memory carries more measured units than its jobs",async()=>{
 const directory=mkdtempSync(join(output,"grant-")),grants=join(directory,"grants");mkdirSync(grants,{recursive:true});
 const plan={version:1,id:"abcdefgh",jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536,floorMegabytes:4096},environment={SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:"abcdefgh",SEMIO_HOST_CAPACITY_PLAN:JSON.stringify(plan)};
 const before=await readCompileUnitsHostV1({environment,cores:10});expect([before.grantedJobs,before.grantedMegabytes],"without a readable row the carried plan holds").toEqual([1,2048]);
 writeFileSync(join(grants,"abcdefgh.json"),JSON.stringify({id:"abcdefgh",operation:"run",jobs:4,cpu:4,reservedMegabytes:9728,admittedAtMs:1,observedMegabytes:3000,pid:process.pid,bootAtMs:1,grownAtMs:2}));
 const after=await readCompileUnitsHostV1({environment,cores:10});expect([after.grantedJobs,after.grantedMegabytes],"the grown row is followed").toEqual([4,8192]);
 const measured=new Map(Array.from({length:16},(_,index)=>[`u${index}`,[{peakRssBytes:900*1048576}]] as const)),one=await readCompileUnitsHostV1({environment:{SEMIO_HOST_CAPACITY_PLAN:JSON.stringify(plan)},cores:10,history:measured});
 const decision=planCompileUnitsV1({...one,availableMegabytes:20000,pressure:"normal"});
 expect([one.unitMegabytes,decision.preferred,decision.reason,decision.budgetMegabytes],"one granted job whose 2048 MB carry two units of 900 MB plans two cores within that memory").toEqual([900,2,"granted",2048]);
 expect(planCompileUnitsV1({...one,unitMegabytes:2048,availableMegabytes:20000,pressure:"normal"}).preferred,"at the authored footprint it is one core, as before").toBe(1);
});
test("granted units stay within the plan",()=>{
 const plan=planCompileUnitsV1({cores:10,availableMegabytes:null,pressure:null,grantedJobs:null,grantedMegabytes:null,unitMegabytes:2048});
 expect(requireGrantedCompileUnitsV1(plan,5)).toBe(5);expect(requireGrantedCompileUnitsV1(plan,3)).toBe(3);expect(()=>requireGrantedCompileUnitsV1(plan,6)).toThrow();expect(()=>requireGrantedCompileUnitsV1(plan,2)).toThrow();
});
test("plans and grants are described in English and German",()=>{
 const values={operation:"cargo",preferred:5,minimum:3,cores:10,reason:"memory",granted:4,seconds:12};
 expect(describeCompileUnitsV1("en","planned",values)).toBe("[compile-units] cargo: 5 of 10 cores planned (at least 3 to start; the free memory carries fewer compile units)");
 expect(describeCompileUnitsV1("de","granted",values)).toContain("4 von 10 Kernen nach 12 s zugeteilt");
 expect(describeCompileUnitsV1("de","waiting",values)).toContain("mindestens 3 freie Kerne von 10");
});
test("real core leases admit the planned compiles side by side in first come first served order like the Python queue model",()=>withBuildTestInvocationV1(async original=>{invocation=original;
 const cores=availableParallelism();if(cores<2)return;
 const held:Awaited<ReturnType<typeof acquireCompilerCapacityLeaseV1>>[]=[],plan=planCompileUnitsV1({cores,availableMegabytes:null,pressure:null,grantedJobs:null,grantedMegabytes:null,unitMegabytes:2048}),request=compileUnitsRequestV1(plan);
 try{
  const first=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request});held.push(first);
  const second=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request});held.push(second);
  const model=python([{cores,requests:[[plan.minimum,plan.preferred],[plan.minimum,plan.preferred]]}])[0] as number[];
  expect([first.parallelism,second.parallelism]).toEqual(model);expect(first.mode).toBe("shared");expect(first.parallelism+second.parallelism).toBeLessThanOrEqual(cores);
  const free=cores-first.parallelism-second.parallelism;
  let entered=false;const waiting=acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"available",minimumParallelism:free+1,maximumParallelism:free+1}}).then(claim=>{entered=true;return claim;});
  await Bun.sleep(150);expect(entered).toBe(false);
  held.shift()!.release();const third=await waiting;held.push(third);expect(third.parallelism).toBe(free+1);
 }finally{for(const claim of held)claim.release();}
}),60_000);
test("a compile waiting for its minimum cores is cancelled and gives nothing back",()=>withBuildTestInvocationV1(async original=>{invocation=original;
 const cores=availableParallelism(),all=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:cores}});
 try{
  const controller=new AbortController(),waiting=acquireCompilerCapacityLeaseV1({directory:storage,control:control(controller.signal),environment:{},request:{kind:"available",minimumParallelism:1,maximumParallelism:cores}});
  await Bun.sleep(100);controller.abort();await expect(waiting).rejects.toThrow();
 }finally{all.release();}
 const again=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:cores}});again.release();
}),60_000);
test("a fixed-class tool (the Bun bundler) gets a short counted slot equal to the independent Python derivation, never the whole host",()=>{
 const answers=python(fixture.fixed.map(row=>({fixed:row.fixed,cores:row.cores})));
 fixture.fixed.forEach((row,index)=>{
  const actual=fixedCompilerCapacityRequestV1(row.fixed as "bun-bundle",row.cores);
  expect(actual,String(row.cores)).toEqual(row.expect as never);expect(answers[index]).toEqual(row.expect);expect(actual.kind).toBe("available");
  expect((actual as {maximumParallelism:number}).maximumParallelism).toBeLessThanOrEqual(Math.max(1,Math.floor(row.cores/2)));
 });
 expect(()=>fixedCompilerCapacityRequestV1("nope" as never,4)).toThrow();
});
test("Bun bundlers are admitted side by side exactly as the independent queue model grants them, and the old whole-host request is the control that never overlaps",()=>withBuildTestInvocationV1(async original=>{invocation=original;
 const cores=availableParallelism();if(cores<4)return;
 const slot=fixedCompilerCapacityRequestV1("bun-bundle") as {kind:"available";minimumParallelism:number;maximumParallelism:number},count=40,model=python([{cores,requests:Array.from({length:count},()=>[slot.minimumParallelism,slot.maximumParallelism])}])[0] as number[];
 expect(model.length,"at least two bundlers run at once").toBeGreaterThanOrEqual(2);
 const held:Awaited<ReturnType<typeof acquireCompilerCapacityLeaseV1>>[]=[],ask=(request:Parameters<typeof acquireCompilerCapacityLeaseV1>[0]["request"])=>acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request});
 try{
  for(let index=0;index<model.length;index++)held.push(await ask(slot));
  expect(held.map(claim=>claim.parallelism)).toEqual(model);expect(held.reduce((sum,claim)=>sum+claim.parallelism,0)).toBeLessThanOrEqual(cores);
  if(model.length<count){let entered=false;const waiting=ask(slot).then(claim=>{entered=true;return claim;});await Bun.sleep(150);expect(entered,"a bundler beyond the model waits").toBe(false);held.shift()!.release();held.push(await waiting);}
  for(const claim of held.splice(0))claim.release();
  const whole=await ask({kind:"whole-host"});held.push(whole);let beside=false;const blocked=ask(slot).then(claim=>{beside=true;return claim;});await Bun.sleep(150);expect(beside,"a whole-host holder admits no bundler beside it").toBe(false);whole.release();held.length=0;held.push(await blocked);
 }finally{for(const claim of held)claim.release();}
}),60_000);
