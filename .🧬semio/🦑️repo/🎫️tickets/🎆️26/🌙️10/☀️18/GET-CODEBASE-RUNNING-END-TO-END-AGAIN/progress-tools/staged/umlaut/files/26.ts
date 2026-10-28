import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {readHostCapacityPolicyV1,readHostEstimateStoreV1,recordHostForecastV1,updateHostEstimateStoreV1,withHostCapacityV1,newHostCapacityIdV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {runCapacityCommandV1} from "../../⌨️command/🟦️.ts";
import {buildCapacityAskV1,upgradeBuildCapacityV1,withBuildCapacityV1} from "../../🟦️.ts";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hq-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool")&&!argument.startsWith("--workers"));
const args=["run","@x/y:build","--parallel=2"],leaf=buildCapacityAskV1(strip(args),{},policy)!.lineage.at(-1)!;
const sample=async()=>({host:host(),swappedOutMegabytes:0});
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const seed=async(directory:string,runs:readonly boolean[])=>{
 const base=Date.now()-3_600_000;
 for(const [index,executed] of runs.entries())await updateHostEstimateStoreV1(directory,policy,store=>recordHostForecastV1(policy,store,leaf,{executed,peakMegabytes:executed?9000:1200,activeMilliseconds:8000,peakAtMilliseconds:4000,endedAtMs:base+index*60_000}));
};
const holdMachine=(directory:string)=>{
 let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const done=withHostCapacityV1({directory,policy,request:{id:newHostCapacityIdV1(),operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:13500,perJobMegabytes:0,cpuPerJob:1},control:control(),environment:{},write:()=>undefined,sample},async()=>hold);
 return{release,done};
};
const ports=(directory:string,over:object={})=>({directory,policy,strip,tool:"bazel",sample,observe:async()=>1100,write:()=>undefined,exitCode:()=>0,...over});
const run=(directory:string,body:Parameters<typeof withBuildCapacityV1>[2],over:object={},environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory})=>withBuildCapacityV1(args,environment,body,ports(directory,over));

test("A run forecast to execute nothing answers beside a full machine with a small grant and is learned as quiet without teaching its class",async()=>{
 const directory=fresh(),machine=holdMachine(directory),lines:string[]=[];
 await seed(directory,[false,false,false,false,false]);
 await sleep(40);
 const started=Date.now();
 await run(directory,async granted=>{
  expect(Date.now()-started,"no capacity queue").toBeLessThan(2500);
  expect(granted.handle!.reservedMegabytes).toBeLessThan(4096);
  expect(granted.args.at(-1),"the tool is told the full jobs").toBe("--parallel=2");
  await sleep(120);
 },{write:(line:string)=>lines.push(line)});
 expect(lines.some(line=>/^\[capacity\] run: forecast quiet from \d+ earlier run\(s\) of its class \(100% of them executed nothing\): \d+ MB reserved now, the full \d+ MB only if work has to run/u.test(line))).toBe(true);
 const store=readHostEstimateStoreV1(directory);
 expect(store.classes[`${leaf}~quiet`]!.samples.length).toBe(6);expect(store.classes[`${leaf}~busy`]).toBeUndefined();
 expect(store.classes[leaf],"a quiet run never teaches the class of real builds").toBeUndefined();
 machine.release();await machine.done;
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
},30000);

test("When work has to run the run waits in the queue only then, is learned as busy and the forecast flips at once",async()=>{
 const directory=fresh(),machine=holdMachine(directory),lines:string[]=[],events:string[]=[];let observed=1100;
 await seed(directory,[false,false,false,false,false]);
 await sleep(40);
 const environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"};
 const finished=run(directory,async granted=>{
  events.push("quiet answer");
  const waiting=upgradeBuildCapacityV1(undefined,environment);
  await sleep(500);events.push("still waiting");
  machine.release();
  expect(await waiting).toBe("upgraded");events.push("work may start");observed=9000;
  expect(granted.handle!.reservedMegabytes,"the grant still names the quiet reservation it was admitted with").toBeLessThan(4096);
  await sleep(150);
 },{write:(line:string)=>lines.push(line),observe:async()=>observed},environment);
 await finished;await machine.done;
 expect(events).toEqual(["quiet answer","still waiting","work may start"]);
 expect(lines.some(line=>/^\[capacity\] run: work has to run, waiting \d+s to grow the reservation from \d+ MB to \d+ MB/u.test(line))).toBe(true);
 expect(lines.some(line=>/^\[capacity\] run: reservation grown from \d+ MB to \d+ MB after \d+s/u.test(line))).toBe(true);
 const store=readHostEstimateStoreV1(directory);
 expect(store.classes[`${leaf}~busy`]!.samples.length).toBe(1);expect(store.classes[`${leaf}~quiet`]!.samples.length).toBe(5);
 expect(store.classes[leaf]!.samples.length,"the real run teaches the class of real builds").toBe(1);
 const second=holdMachine(directory);await sleep(40);
 let started=false;
 const waiting=run(directory,async granted=>{started=true;expect(granted.handle!.reservedMegabytes,"after a busy run the next run of the class follows it: the small grant answers at once").toBeLessThan(4096);await sleep(120);});
 await sleep(300);expect(started,"the follow-up forecast does not queue behind the full machine").toBe(true);
 second.release();await waiting;await second.done;
},30000);

test("Nothing is forecast for another tool, an operation without a forecast or without history, and a run under a full grant records only that it executed",async()=>{
 const directory=fresh(),machine=holdMachine(directory);
 await seed(directory,[false,false,false,false,false]);
 await sleep(40);
 let started=false;
 const other=run(directory,async()=>{started=true;},{tool:"nx"});
 await sleep(250);expect(started,"another tool has no action gate and asks for the full grant").toBe(false);
 machine.release();await other;await machine.done;expect(started).toBe(true);
 const cold=fresh();
 await run(cold,async granted=>{expect(granted.handle!.reservedMegabytes,"no history, the full request").toBeGreaterThan(4096);expect(await upgradeBuildCapacityV1(undefined,{SEMIO_HOST_CAPACITY_DIRECTORY:cold,SEMIO_HOST_CAPACITY_GRANT:granted.handle!.id})).toBe("held");await sleep(80);},{},{SEMIO_HOST_CAPACITY_DIRECTORY:cold});
 const store=readHostEstimateStoreV1(cold);
 expect(store.classes[`${leaf}~busy`]!.samples.length,"a run under a full grant that executed is busy").toBe(1);
 expect(await upgradeBuildCapacityV1(undefined,{})).toBe("absent");
});

test("repo capacity status names a quiet grant and the upgrade that waits, in English and German",async()=>{
 const directory=fresh(),machine=holdMachine(directory),english:string[]=[],german:string[]=[];
 await seed(directory,[false,false,false,false,false]);
 await sleep(40);
 const environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory};
 const context=(language:string,write:(line:string)=>void)=>({environment:{LANG:language},write,directory,policy,sample});
 await run(directory,async()=>{
  const waiting=upgradeBuildCapacityV1(undefined,environment);
  await sleep(450);
  expect(await runCapacityCommandV1(["status"],context("en_US.UTF-8",line=>english.push(line)))).toBe(0);
  expect(await runCapacityCommandV1(["status"],context("de_DE.UTF-8",line=>german.push(line)))).toBe(0);
  machine.release();await waiting;
 },{},environment);
 await machine.done;
 expect(english.some(line=>/^\[capacity\]   quiet \S+ run: forecast to execute nothing, \d+ MB reserved instead of its full request, \S+ MB observed, pid \d+, since \d+s$/u.test(line)),english.join("\n")).toBe(true);
 expect(english.some(line=>/^\[capacity\]   growing \S+ run: wants 2 job\(s\), \d+ MB, since \d+s, pid \d+$/u.test(line)),english.join("\n")).toBe(true);
 expect(german.some(line=>/^\[capacity\]   ruhig \S+ run: soll nichts ausführen, \d+ MB statt der vollen Anfrage reserviert/u.test(line))).toBe(true);
 expect(german.some(line=>/^\[capacity\]   wächst \S+ run: möchte 2 Job\(s\), \d+ MB, seit \d+ s, pid \d+$/u.test(line))).toBe(true);
});
