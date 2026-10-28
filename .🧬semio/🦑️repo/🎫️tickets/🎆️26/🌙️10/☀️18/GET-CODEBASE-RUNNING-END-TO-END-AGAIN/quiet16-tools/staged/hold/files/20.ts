/** 🤫️ Contract of the door: a quiet forecast holds until work really runs. The door that re-enters under the quiet grant it holds (as `repo run` does) starts at once beside a full machine with the tool told all its jobs and never grows the grant; a run whose tree outgrows the quiet reservation grows without an executor asking and is learned as one that executed work; every learned outcome and the forecast that follows it equal the independent Python model.
 * @see ./🐍️oracle/🐍️.py
 * @see ./🥒️.feature
 * @see ../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🤫️quiet/🟦️.ts
 * @see ../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/result-quiet16.md */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {forecastHostQuietV1,newHostCapacityIdV1,readHostCapacityPolicyV1,readHostCapacityStatusV1,readHostEstimateStoreV1,recordHostForecastV1,updateHostEstimateStoreV1,withHostCapacityV1,type HostEstimateStoreV1,type HostForecastV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {buildCapacityAskV1,upgradeBuildCapacityV1,withBuildCapacityV1} from "../../🟦️.ts";
import oracle from "./🧫️fixtures/🔣️.json" with {type:"json"};

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"q16d-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool")&&!argument.startsWith("--workers"));
const args=["run","@x/y:build","--parallel=2"],leaf=buildCapacityAskV1(strip(args),{},policy)!.lineage.at(-1)!;
const sample=async()=>({host:host(),swappedOutMegabytes:0});
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const seed=async(directory:string,runs:readonly boolean[])=>{const base=Date.now()-3_600_000;for(const [index,executed] of runs.entries())await updateHostEstimateStoreV1(directory,policy,store=>recordHostForecastV1(policy,store,leaf,{executed,peakMegabytes:executed?9000:1200,activeMilliseconds:8000,peakAtMilliseconds:4000,endedAtMs:base+index*60_000}));};
const holdMachine=(directory:string)=>{let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});const done=withHostCapacityV1({directory,policy,request:{id:newHostCapacityIdV1(),operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:13500,perJobMegabytes:0,cpuPerJob:1},control:control(),environment:{},write:()=>undefined,sample},async()=>hold);return{release,done};};
const ports=(directory:string,over:object={})=>({directory,policy,strip,tool:"bazel",sample,observe:async()=>1100,write:()=>undefined,exitCode:()=>0,...over});
const counts=(directory:string)=>{const store=readHostEstimateStoreV1(directory);return{quiet:store.classes[`${leaf}~quiet`]?.samples.length??0,busy:store.classes[`${leaf}~busy`]?.samples.length??0};};

test("The door that re-enters under the quiet grant it holds starts at once beside a full machine, tells the tool all its jobs and never grows the grant",async()=>{
 const directory=fresh(),machine=holdMachine(directory),lines:string[]=[];
 await seed(directory,[false,false,false,false,false]);
 await sleep(40);
 const environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"},started=Date.now();let inner:{inherited:boolean;args:readonly string[];ms:number;quiet:boolean|undefined}|undefined;
 await withBuildCapacityV1(args,environment,async outer=>{
  await withBuildCapacityV1(args,environment,async granted=>{
   const row=(await readHostCapacityStatusV1({directory,policy,control:control(),sample})).grants.find(item=>item.id===outer.handle!.id)!;
   inner={inherited:granted.handle!.inherited,args:granted.args,ms:Date.now()-started,quiet:row.quiet};
   await sleep(80);
  },ports(directory,{write:(line:string)=>lines.push(line)}));
 },ports(directory,{write:(line:string)=>lines.push(line)}));
 machine.release();await machine.done;
 expect(inner).toMatchObject({inherited:true,quiet:true});expect(inner!.args.at(-1),"the tool is told the full jobs").toBe("--parallel=2");
 expect(inner!.ms,"no capacity queue for the re-entered door").toBeLessThan(2500);
 expect(lines.filter(line=>/grow|grown/u.test(line))).toEqual([]);
 expect(counts(directory)).toEqual({quiet:6,busy:0});
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
},30000);

test("A run whose tree outgrows the quiet reservation grows although no executor asked, and is learned as one that executed work",async()=>{
 const directory=fresh(),lines:string[]=[];let observed=600;
 await seed(directory,[false,false,false,false,false]);
 await withBuildCapacityV1(args,{SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"de_DE.UTF-8"},async granted=>{
  expect(granted.handle!.reservedMegabytes,"quiet").toBeLessThan(4096);
  await sleep(60);observed=6000;await sleep(400);
 },ports(directory,{write:(line:string)=>lines.push(line),observe:async()=>observed}));
 expect(lines.some(line=>/^\[capacity\] run: Reservierung nach \d+ s von \d+ MB auf \d+ MB vergrößert/u.test(line)),lines.join("\n")).toBe(true);
 expect(counts(directory)).toEqual({quiet:5,busy:1});
});

test("Every learned outcome and the forecast that follows it equal the independent Python model, the live store of 14:50 included",async()=>{
 expect(oracle.leaf).toBe(leaf);
 for(const key of ["askedBusy","outgrownBusy","quiet","forecastQuiet","forecastFull","followUp"])expect((oracle.coverage as Record<string,number>)[key]??0,key).toBeGreaterThan(0);
 for(const sequence of oracle.sequences){
  const directory=fresh();
  await updateHostEstimateStoreV1(directory,policy,()=>sequence.store as unknown as HostEstimateStoreV1);
  for(const [index,step] of sequence.steps.entries()){
   const environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory};
   expect(forecastHostQuietV1(policy,readHostEstimateStoreV1(directory),leaf,step.nowMs),`${sequence.name} #${index}`).toEqual(step.expect.forecast as HostForecastV1);
   await withBuildCapacityV1(args,environment,async()=>{if(step.asked)await upgradeBuildCapacityV1(undefined,environment);await sleep(70);},ports(directory,{now:()=>step.nowMs,observe:async()=>step.peakMegabytes}));
   const store=readHostEstimateStoreV1(directory),learned=(suffix:string)=>store.classes[`${leaf}~${suffix}`]?.samples.some(row=>row.atMs===step.nowMs)===true;
   expect([learned("quiet"),learned("busy")],`${sequence.name} #${index}: executed ${step.expect.executed}`).toEqual([!step.expect.executed,step.expect.executed]);
  }
  expect(forecastHostQuietV1(policy,readHostEstimateStoreV1(directory),leaf,sequence.after.nowMs),`${sequence.name} after`).toEqual(sequence.after.forecast as HostForecastV1);
 }
},120000);
