import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readHostCapacityPolicyV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {acquireBuildFarmCapacityV1,withBuildCapacityV1} from "../../🟦️.ts";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"farm-capacity-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const fast=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0}});

test("with the gate off the farm gets every job it asks for and nothing is reserved",async()=>{
 const directory=fresh(),granted=await acquireBuildFarmCapacityV1({preferredJobs:6,waitMilliseconds:100,environment:{SEMIO_HOST_CAPACITY:"off"},directory,policy:fast,write:()=>{}});
 expect(granted!.jobs).toBe(6);granted!.release();
 expect(existsSync(join(directory,"grants"))?readdirSync(join(directory,"grants")):[]).toEqual([]);
},60000);

test("a fresh grant is requested beside the grant the invocation holds, held until released and given back",async()=>{
 const directory=fresh(),lines:string[]=[],environment:NodeJS.ProcessEnv={},sample=async()=>({host:host(),swappedOutMegabytes:0});
 await withBuildCapacityV1(["show","projects"],environment,async outer=>{
  expect(outer.handle).toBeDefined();
  expect(environment.SEMIO_HOST_CAPACITY_GRANT).toBe(outer.handle!.id);
  expect(readdirSync(join(directory,"grants")).length).toBe(1);
  const granted=await acquireBuildFarmCapacityV1({preferredJobs:6,waitMilliseconds:2000,environment,directory,policy:fast,write:line=>lines.push(line),sample});
  expect(granted).toBeDefined();
  expect(granted!.jobs).toBeGreaterThanOrEqual(1);expect(granted!.jobs).toBeLessThanOrEqual(6);
  expect(readdirSync(join(directory,"grants")).length).toBe(2);
  expect(environment.SEMIO_HOST_CAPACITY_GRANT).toBe(outer.handle!.id);
  granted!.release();
  const started=Date.now();
  while(readdirSync(join(directory,"grants")).length>1&&Date.now()-started<2000)await new Promise(done=>setTimeout(done,10));
  expect(readdirSync(join(directory,"grants")).length).toBe(1);
 },{directory,policy:fast,strip:args=>args.filter(argument=>!argument.startsWith("--tool")),write:()=>{},sample});
 expect(readdirSync(join(directory,"grants")).length).toBe(0);
},60000);

test("a host that cannot admit a worker in time leaves the farm without a grant instead of waiting for ever",async()=>{
 const directory=fresh(),started=Date.now();
 const granted=await acquireBuildFarmCapacityV1({preferredJobs:6,waitMilliseconds:300,environment:{},directory,policy:fast,write:()=>{},sample:async()=>({host:host({availableMegabytes:300,pressure:"critical"}),swappedOutMegabytes:0})});
 expect(granted).toBeUndefined();
 expect(Date.now()-started).toBeLessThan(5000);
 expect(existsSync(join(directory,"queue"))?readdirSync(join(directory,"queue")):[]).toEqual([]);
},60000);
