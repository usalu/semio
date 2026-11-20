import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {spawn} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync,appendFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {readySource,readySchema} from "./ready-terminal-evidence-stage.ts";
import cases from "./ready-terminal-evidence.cases.json";
import caseSchema from "./ready-terminal-evidence.cases.schema.json";

const {READY_POLICY_V1:P,createReadyPortsV1,readyPlatformV1,runReadyStageV1,runReadyV1}=await import(readySource);
const {default:schema}=await import(readySchema);
const {READY_TREE_POLICY_V1:TREE}=await import(resolve(dirname(readySource),"🌲️process-tree","🟦️.ts"));
const childSource=`
 const fs=require("node:fs"),[log,end,counter,inflight,observedStop,step]=process.argv.slice(1);
 const activity=(resource,extra={})=>"[script] "+JSON.stringify({stage:"running",activity:{resource,mode:"exclusive",elapsedMs:0},...extra})+"\\n";
 fs.writeFileSync(log,activity("native-artifact-input-read:"+counter)+step);
 process.on("SIGTERM",()=>{
  fs.copyFileSync(inflight,observedStop);
  const receipt=JSON.stringify({exit:143,signal:null,spawnFailed:false,endedMs:Date.now()});
  fs.writeFileSync(observedStop+".exit",receipt);
  fs.writeFileSync(end,receipt);
  process.exit(143);
 });
 setInterval(()=>{},100);
`;
const wait=async(check:()=>boolean)=>{
 const deadline=performance.now()+6000;
 while(!check()&&performance.now()<deadline)await Bun.sleep(15);
 expect(check()).toBe(true);
};
const bench=test.skipIf(process.platform==="win32");

test("the neutral terminal evidence cases satisfy their closed schema",()=>{
 expect(new Ajv({strict:true,strictRequired:false}).validate(caseSchema,cases)).toBe(true);
});

for(const fixture of cases)bench(fixture.name,async()=>{
 const scratch=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!scratch)throw Error("Test artifact directory required");
 const root=mkdtempSync(join(scratch,"rt-")),directory=join(root,...P.directory),store=join(root,"s");
 mkdirSync(directory,{recursive:true});
 const running=join(directory,P.inflightName),log=join(directory,"tools.log"),exitFile=join(directory,"tools.exit"),observedStop=join(directory,"at-stop.json"),startedMs=Date.now();
 const step=fixture.step,line=step===undefined?"":`Reading the project graph slice (4000 s): waiting for owners / Warte auf Projekte ${step.seconds} of ${step.bound}.\n`;
 const child=spawn(process.execPath,["-e",childSource,log,exitFile,String(fixture.complete),running,observedStop,line],{cwd:root,stdio:"ignore",detached:true});
 const reaped=new Promise<void>(done=>child.once("exit",()=>done()));
 const stage={name:"tools",kind:"repo" as const,arguments:["tools"],timeoutMilliseconds:fixture.softMs,inputs:[],bootstrapInput:false};
 const tree={...TREE,stallMilliseconds:fixture.idleMs,stallStopMilliseconds:step?.quietMs??60000,speedFactor:step?.speed??TREE.speedFactor,stageHardCapMultipliers:{tools:fixture.hardMultiplier},semanticsSources:[]};
 const environment={SEMIO_FLEET_STORE_ROOT:store};
 const first=new AbortController(),loop=new AbortController();
 using oracle=new Database(":memory:");
 try{
  await wait(()=>existsSync(log));
  const launch={version:1,platform:readyPlatformV1(),digest:"f".repeat(64),recipe:"d".repeat(64),prerequisites:[]};
  writeFileSync(running,JSON.stringify({stage:stage.name,pid:child.pid,startedMs,exitFile,log,launch}));
  const detached=runReadyStageV1({root,environment,stage,tree,speed:1,signal:first.signal,jobs:1,onLine:()=>undefined});
  await wait(()=>JSON.parse(readFileSync(running,"utf8")).watchdog!==undefined);
  first.abort();
  expect(await detached).toMatchObject({detached:true,startedMs});
  const original=JSON.parse(readFileSync(running,"utf8"));
  const preflood=original.watchdog.lastActivity??null;
  if(step!==undefined)await Bun.sleep(step.quietMs+50);
  writeFileSync(running,JSON.stringify({...original,watchdog:{...original.watchdog,armedAt:startedMs+1,...step===undefined?{}:{stallAt:Date.now()}}}));
  const row=(resource:string,stage="running")=>"[script] "+JSON.stringify({stage,activity:{resource,mode:"exclusive",elapsedMs:12345}})+"\n";
  if(step===undefined)appendFileSync(log,row("native-artifact-input-read:"+(fixture.complete-1))+row("secret/path:1234")+row("native-artifact-input-read:9223372036854775808")+row("native-artifact-input-read:99999","waiting")+"[repo] still working after 126 s; last phase started: boot.loaded\n".repeat(fixture.floodRows));
  const ports=createReadyPortsV1({root,environment,origin:"private",group:undefined,stages:[stage],say:()=>undefined});
  const refusals:string[]=[];
  const timeout=setTimeout(()=>loop.abort(),12000);
  try{
   await runReadyV1({...ports,repoPresent:()=>true,disk:()=>null,diskUnknown:()=>[],memory:()=>({ok:true,availableMegabytes:8192}),
    mirror:()=>undefined,
    status:{update(value){if(value.refusal!==undefined&&value.refusal!==null)refusals.push(value.refusal.code);ports.status.update(value);}},
    run(selected,options){return runReadyStageV1({root,environment,stage:selected,tree,speed:1,...options});},
    sleep:async()=>{loop.abort();}},loop.signal);
  }finally{clearTimeout(timeout);}
  expect(existsSync(running)).toBe(false);
  const doc=JSON.parse(readFileSync(join(directory,P.timingsName),"utf8"));
  const mirrored=JSON.parse(readFileSync(join(store,"v1","origins","private",P.timingsName),"utf8"));
  const valid=new Ajv({strict:false}).compile({...schema, $ref:"#/definitions/Timings"});
  expect(valid(doc)).toBe(true);
  expect(mirrored).toEqual(doc);
  expect(doc.runs).toHaveLength(1);
  const [timing]=doc.runs;
  if(step!==undefined){
   const expected=oracle.query<{measurement:string;minQuietMs:number;maxQuietMs:number},[number,number,number,number,number,number]>(`SELECT
    json_object('seconds',?1,'bound',?2,'limit',CAST((?2*?3*1000+?4+999)/1000 AS INTEGER)) AS measurement,
    ?4 AS minQuietMs,?5-?6 AS maxQuietMs`).get(step.seconds,step.bound,step.speed,step.quietMs,timing.endedMs,original.watchdog.step.movedAt)!;
   const atStop=JSON.parse(readFileSync(observedStop,"utf8"));
   expect(atStop).toMatchObject({stop:"overdue"});
   expect(refusals).toContain("stage-step-overdue");
   expect(new Ajv({strict:false}).validate({...schema,$ref:"#/definitions/Inflight"},atStop)).toBe(true);
   const lastActivity={resource:"native-artifact-input-read",counter:fixture.complete};
   const measurements=timing.watchdog?.stepOverdue;
   expect(measurements).toMatchObject(JSON.parse(expected.measurement));
   expect(measurements.quietMs).toBeGreaterThanOrEqual(expected.minQuietMs);
   expect(measurements.quietMs).toBeLessThanOrEqual(expected.maxQuietMs);
   expect(timing.watchdog).toEqual({lastActivity,stepOverdue:atStop.stepOverdue});
   expect(timing).toMatchObject({outcome:"stalled",exit:143,signal:null,endEvidence:"exit-receipt",startedMs,digest:launch.digest.slice(0,12)});
   await reaped;
   for(const record of [atStop,{...atStop,stop:"stall"},{...atStop,stepOverdue:{...atStop.stepOverdue,quietMs:-1}},{...atStop,stepOverdue:{...atStop.stepOverdue,seconds:0}}]){
    writeFileSync(running,JSON.stringify(record));
    writeFileSync(exitFile,readFileSync(observedStop+".exit"));
    const adopted=await runReadyStageV1({root,environment,stage,tree,speed:1,signal:new AbortController().signal,jobs:1,onLine:()=>undefined});
    expect(adopted).toMatchObject({stalled:true,startedMs,exit:143,physicalEnd:{endedMs:timing.endedMs,endEvidence:"exit-receipt"}});
    expect(adopted.watchdog).toEqual(record===atStop?timing.watchdog:{lastActivity});
   }
   writeFileSync(running,JSON.stringify(atStop));
   const vanished=await runReadyStageV1({root,environment,stage,tree,speed:1,signal:new AbortController().signal,jobs:1,onLine:()=>undefined});
   expect(vanished).toMatchObject({stalled:true,startedMs,exit:null,physicalEnd:{endEvidence:"observed"},watchdog:timing.watchdog});
   return;
  }
  const expected=oracle.query(`SELECT CASE
    WHEN ?1>?2*?3 THEN 'hard-cap' WHEN ?1>?2 AND ?1>?4 THEN 'soft-cap-idle' END AS deadline,
    json_object('resource','native-artifact-input-read','counter',max(?5,?6)) AS lastActivity`).get(
      timing.endedMs-startedMs,fixture.softMs,fixture.hardMultiplier,fixture.idleMs,fixture.complete,fixture.complete-1) as {deadline:string;lastActivity:string};
  expect(expected.deadline).toBe(fixture.deadline);
  expect(preflood).toEqual(JSON.parse(expected.lastActivity));
  expect(timing.watchdog).toEqual({deadline:expected.deadline,lastActivity:JSON.parse(expected.lastActivity)});
  expect(JSON.parse(readFileSync(observedStop,"utf8"))).toMatchObject({stop:"timeout",deadline:expected.deadline,watchdog:{lastActivity:JSON.parse(expected.lastActivity)}});
  expect(timing).toMatchObject({outcome:"stage-timeout",exit:143,signal:null,endEvidence:"exit-receipt",startedMs,digest:launch.digest.slice(0,12)});
  expect(ports.state.read().failure?.code).toBe("stage-timeout");
 }finally{
  first.abort();loop.abort();
  if(child.exitCode===null&&child.signalCode===null)child.kill("SIGKILL");
  await reaped;
  rmSync(root,{recursive:true,force:true});
 }
},18000);
