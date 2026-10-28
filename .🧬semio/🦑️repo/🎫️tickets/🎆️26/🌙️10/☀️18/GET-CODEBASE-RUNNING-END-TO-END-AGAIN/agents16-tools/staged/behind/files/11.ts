/** 🧾️ Contract: the door labels its capacity request with what it builds (the target of `repo run`, control characters dropped) and the project farm labels its request `graph-farm`, so the progress lines of other agents can name whom they stand behind.
 * @see ../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🧾️blockers/🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {readHostCapacityPolicyV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {requestBuildFarmCapacityV1,withBuildCapacityV1} from "../../🟦️.ts";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"ag16l-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const host=(availableMegabytes:number):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:8,totalMegabytes:16384,availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const rows=(directory:string,child:"grants"|"queue")=>readdirSync(join(directory,child)).filter(name=>name.endsWith(".json")).map(name=>JSON.parse(readFileSync(join(directory,child,name),"utf8")));

test("The door labels its request with the target it builds, and the farm labels its request graph-farm",async()=>{
 const directory=fresh();
 const label=await withBuildCapacityV1(["run","@teaching/architecture-course:build","--parallel=2"],{SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"},async granted=>rows(directory,"grants").find(grant=>grant.id===granted.handle!.id)?.label,{directory,policy,strip:(args:readonly string[])=>args,tool:"bazel",sample:async()=>({host:host(14000),swappedOutMegabytes:0}),observe:async()=>500,write:()=>undefined,exitCode:()=>0});
 expect(label).toBe("@teaching/architecture-course:build");
 const farm=requestBuildFarmCapacityV1({preferredJobs:2,environment:{SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"},directory,policy,write:()=>undefined,sample:async()=>({host:host(3000),swappedOutMegabytes:0})});
 await sleep(80);
 expect(rows(directory,"queue").map(waiter=>waiter.request.label)).toEqual(["graph-farm"]);
 farm.release();await farm.granted;
});

test("The door drops control characters from the label it derives from the command line",async()=>{
 const directory=fresh();
 const label=await withBuildCapacityV1(["run","@x/y:bu\u001b[2Jild","--parallel=2"],{SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"},async granted=>rows(directory,"grants").find(grant=>grant.id===granted.handle!.id)?.label,{directory,policy,strip:(args:readonly string[])=>args,tool:"bazel",sample:async()=>({host:host(14000),swappedOutMegabytes:0}),observe:async()=>500,write:()=>undefined,exitCode:()=>0});
 expect(label).toBe("@x/y:bu[2Jild");
});
