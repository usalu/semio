/** 🧾️ Contract: every waiting and growing progress line names what the waiter stands behind, in English and German: the runs of ours that hold the resource its reason names or the requests served before it, by the label of their request (the target of `repo run`), state, jobs and megabytes; the host itself when nothing of ours holds it back. The selection equals the independent Python model on every stored world of the upgrade oracle.
 * @see ./🐍️oracle/🐍️.py
 * @see ./🥒️.feature
 * @see ../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/result-agents16.md */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {describeHostBlockersV1,evaluateHostCapacityV1,hostBlockersV1,hostBootAtV1,isHostCapacityV1,readHostCapacityPolicyV1,upgradeHostGrantV1,withHostCapacityV1,type HostCapacityPolicyV1,type HostGrantV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from "../../🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import upgradeFixture from "../🔼️upgrade/🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1(decisionFixture.policy),fast=readHostCapacityPolicyV1({...decisionFixture.policy,wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:10}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"ag16b-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
type World={name:string;host:HostSampleV1;ledger:HostGrantV1[];queue:HostWaiterV1[];nowMs:number;policy?:{maximumQuiet:number}};
const machine=(availableMegabytes:number):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:8,totalMegabytes:16384,availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:2,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const running=(id:string,label:string,reservedMegabytes:number,pid:number,bootAtMs:number,admittedAtMs:number):HostGrantV1=>({id,operation:"run",label,jobs:8,cpu:8,reservedMegabytes,admittedAtMs,observedMegabytes:400,pid,bootAtMs,rampMilliseconds:600_000});
const control=(signal:AbortSignal)=>({signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});

test("What a waiting request stands behind equals the independent Python model on every stored world, and the vectors reach every reason that names runs of ours and the host's own",()=>{
 const stored=upgradeFixture as unknown as {decisions:World[];agents:World[]},worlds=new Map([...stored.decisions,...stored.agents].map(world=>[world.name,world]));
 for(const vector of fixture.vectors){
  const world=worlds.get(vector.world)!,used:HostCapacityPolicyV1=world.policy===undefined?policy:{...policy,memory:{...policy.memory,maximumQuiet:world.policy.maximumQuiet}};
  const evaluation=evaluateHostCapacityV1(used,world.host,world.ledger,world.queue,world.nowMs);
  expect(hostBlockersV1(used,world.ledger,world.queue,evaluation,vector.id,world.nowMs),`${vector.world} ${vector.id}`).toEqual(vector.expect as never);
 }
 for(const key of ["memory:named","memory:host","load:named","ramp:named","quiet-limit:named","queue-order:named","pressure-critical:host"])expect((fixture.coverage as Record<string,number>)[key]??0,key).toBeGreaterThan(0);
});

test("A request waiting at the door names the runs of ours that hold the memory it needs, in English and German",async()=>{
 for(const [language,expected] of [["en_US.UTF-8","; behind @semio-tech/print-rs:test (running, 8 job(s), 3649 MB), @teaching/architecture-course:build (running, 8 job(s), 2861 MB)"],["de_DE.UTF-8","; hinter @semio-tech/print-rs:test (läuft, 8 Job(s), 3649 MB), @teaching/architecture-course:build (läuft, 8 Job(s), 2861 MB)"]] as const){
  const directory=fresh(),now=Date.now(),boot=hostBootAtV1(directory,now);mkdirSync(join(directory,"grants"),{recursive:true});mkdirSync(join(directory,"queue"),{recursive:true});
  for(const grant of [running("courseaaaa000001","@teaching/architecture-course:build",2861,process.pid,boot,now),running("printaaaaa000001","@semio-tech/print-rs:test",3649,process.pid,boot,now)])writeFileSync(join(directory,"grants",grant.id+".json"),JSON.stringify(grant));
  const abort=new AbortController(),lines:string[]=[],request:HostRequestV1={id:"dashaaaaaa000001",operation:"run",label:"@semio-tech/repo-dashboard-rs:build",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:3684,perJobMegabytes:0,cpuPerJob:1};
  const waited=withHostCapacityV1({directory,policy:fast,request,control:control(abort.signal),environment:{LANG:language},write:line=>{lines.push(line);abort.abort();},sample:async()=>({host:machine(9000),swappedOutMegabytes:0})},async()=>"admitted").catch(()=>"cancelled");
  expect(await waited).toBe("cancelled");
  expect(lines.length,language).toBe(1);
  expect(lines[0]!.endsWith(expected),lines[0]).toBe(true);
 }
});

test("A growing run names what it stands behind and does not count its own quiet grant as running, in English and German",async()=>{
 for(const [language,expected] of [["en","; 1 running, 0 ahead); behind @teaching/architecture-course:build (running, 8 job(s), 2861 MB)"],["de","; 1 laufen, 0 davor); hinter @teaching/architecture-course:build (läuft, 8 Job(s), 2861 MB)"]] as const){
  const now=Date.now(),a=running("courseaaaa000001","@teaching/architecture-course:build",2861,11,1,now),c:HostGrantV1={id:"dashaaaaaa000001",operation:"run",label:"@semio-tech/repo-dashboard-rs:build",jobs:1,cpu:1,reservedMegabytes:1502,admittedAtMs:now,observedMegabytes:200,pid:13,bootAtMs:1,quiet:true,arrivedAtMs:now,rampMilliseconds:60_000};
  const request:HostRequestV1={id:c.id,operation:"run",label:c.label,priority:"interactive",minimumJobs:8,preferredJobs:8,fixedMegabytes:404,perJobMegabytes:1000,cpuPerJob:1},lines:string[]=[];let state=c,polls=0;
  const ledger={takeLock:()=>true,releaseLock:()=>undefined,holdsLock:()=>true,writeJson:()=>undefined,remove:()=>undefined,queuePath:()=>join(fresh(),"waiter.json"),readGrants:()=>[a,state],readWaiters:()=>[] as HostWaiterV1[],sample:async()=>({...machine(9000),atMs:Date.now()})};
  const outcome=await upgradeHostGrantV1({ledger:ledger as never,directory:fresh(),policy:fast,request,control:control(new AbortController().signal),state:()=>state,publish:next=>{state=next;},requested:()=>++polls<3,stopped:()=>false,now:Date.now,pid:13,language,write:line=>lines.push(line)});
  expect(outcome).toBe("withdrawn");
  expect(lines[0]!.endsWith(expected),lines[0]).toBe(true);
 }
});

test("A request held back by the host itself names nothing of ours",()=>{
 expect([describeHostBlockersV1("en",[]),describeHostBlockersV1("de",[])]).toEqual(["the host itself (nothing of ours)","dem Rechner selbst (nichts von uns)"]);
});

test("A label is a line of printable text: control characters (escape sequences, tabs, line breaks) are refused, any other Unicode is kept",()=>{
 const request:HostRequestV1={id:"dashaaaaaa000001",operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:900,perJobMegabytes:0,cpuPerJob:1};
 const valid=(label:string)=>isHostCapacityV1("Request",{...request,label});
 expect(["@x/y:build","Kurs ü 🧪 @teaching/architecture-course:build","a".repeat(120)].map(valid)).toEqual([true,true,true]);
 expect(["a\u001b[31mb","a\tb","a\nb","a\u007fb","","a".repeat(121)].map(valid)).toEqual([false,false,false,false,false,false]);
});
