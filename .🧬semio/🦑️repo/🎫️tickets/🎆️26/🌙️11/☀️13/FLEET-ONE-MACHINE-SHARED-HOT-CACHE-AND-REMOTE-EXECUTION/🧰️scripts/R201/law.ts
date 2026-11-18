import {expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {evaluateHostCapacityV1,hostFloorMegabytesV1,keptHostMegabytesV1,readHostCapacityPolicyV1,type HostGrantV1,type HostSampleV1} from "../../📏️decision/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";

const base=readHostCapacityPolicyV1({...fixture.policy,memory:{...fixture.policy.memory,rampMilliseconds:60000,rampIdleAfterMilliseconds:30000,rampIdleObservedBasisPoints:2500,rampIdleKeepMegabytes:3072,rampIdleKeepBasisPoints:15000,childStartMegabytes:512}} as never);
const settled=readHostCapacityPolicyV1({...base,memory:{...base.memory,settledAfterMilliseconds:900000,settledKeepBasisPoints:15000}} as never);
const now=1794999000000;
const host=(available:number):HostSampleV1=>({version:1,atMs:now,platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:available,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:5,openFiles:1000,openFilesMaximum:100000,attachedImages:0} as never);
const read=(observed:number|null,ageMs:number):HostGrantV1=>({id:"MLEDy07wz07HmSYW6Dacyg",operation:"show",jobs:1,cpu:2,reservedMegabytes:14336,admittedAtMs:now-ageMs-100000,resumedAtMs:now-ageMs,observedMegabytes:observed,pid:36171,bootAtMs:0,label:"projects",rampMilliseconds:21900000,phase:"rebuild"} as never);
const child=(id:string,jobs:number):HostGrantV1=>({id,operation:"exec",jobs,cpu:jobs,reservedMegabytes:1536*jobs,admittedAtMs:now-9000000,observedMegabytes:0,pid:36237,bootAtMs:0,label:"graph-farm"} as never);
const farm=[child("DMn1BMGUdQn0P-AjsrruoA",1),child("qb6k3EwqjezH_rDjbrND7w",6),child("XrRReDZCbtcddBuYcMYjPw",1)];
const ask=(megabytes:number)=>[{request:{id:"WaiterWaiterWaiter1",operation:"command",priority:"interactive",minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1},arrivedAtMs:now-1000,pid:4242}] as never;
const headroom=(policy:typeof base,available:number,observed:number,age=9000000)=>evaluateHostCapacityV1(policy,host(available),[read(observed,age),...farm],[],now);

test("a read observed at 4.3 GB holds a booking near that, not its estimate of 14.3 GB, once it is settled; the estimate counts in full before",()=>{
 const grant=read(4301,9000000);
 expect(keptHostMegabytesV1(base,grant,now),"without the rule the whole estimate stays booked (the live state)").toBe(14336);
 expect(keptHostMegabytesV1(settled,grant,now)).toBe(Math.ceil(4301*1.5));
 expect(keptHostMegabytesV1(settled,read(4301,899000),now),"inside its settle time the estimate counts in full").toBe(14336);
 expect(keptHostMegabytesV1(settled,read(null,9000000),now),"an unobserved grant is not settled").toBe(14336);
 expect(keptHostMegabytesV1(settled,read(13000,9000000),now),"never more than it reserved").toBe(14336);
});

test("the spare the farm and the pools see is what the machine has, not the estimate: 6.0 GB observed, 15.1 GB available, 32 GB host",()=>{
 const floor=hostFloorMegabytesV1(base,host(15134));
 const before=headroom(base,15134,6039).headroomMegabytes,after=headroom(settled,15134,6039).headroomMegabytes;
 expect(before).toBe(15134-floor-(14336-6039));
 expect(after).toBe(15134-floor-(Math.ceil(6039*1.5)-6039));
 expect(after).toBeGreaterThan(before+5000);
});

test("no oversubscription when the footprint grows: the booking follows the measurement, a request larger than the room waits, and the room never goes negative into admissions",()=>{
 const floor=hostFloorMegabytesV1(base,host(15134));
 let available=15134;
 for(const observed of [4301,6039,9000,12000,14000,16000]){
  const grew=observed-4301;available=15134-grew;
  const evaluation=headroom(settled,available,observed);
  const kept=keptHostMegabytesV1(settled,read(observed,9000000),now);
  expect(Math.max(kept,observed),`booking covers the measurement at ${observed}`).toBeGreaterThanOrEqual(observed);expect(kept,`booking never exceeds its reservation`).toBeLessThanOrEqual(14336);
  expect(evaluation.headroomMegabytes,`room at ${observed}`).toBe(Math.min(available-floor-Math.max(0,kept-observed),(32768-floor)-Math.max(kept,observed)));
  const wanted=Math.max(1,evaluation.headroomMegabytes+1000);
  const verdict=evaluateHostCapacityV1(settled,host(available),[read(observed,9000000),...farm],ask(wanted),now).decisions[0]!;
  expect(verdict.verdict==="admit"&&verdict.reservedMegabytes>Math.max(0,evaluation.headroomMegabytes),`a request of ${wanted} MB over a room of ${evaluation.headroomMegabytes} MB is not admitted in full at ${observed}`).toBe(false);
 }
});

test("the farm children are charged by what the tree measures: unmeasured grants of the farm count nothing after their start ramp and 512 MB per job inside it",()=>{
 expect(keptHostMegabytesV1(settled,child("qb6k3EwqjezH_rDjbrND7w",6),now-9000000+1000)).toBe(Math.min(9216,6*512));
});

test("the live policy carries the settle rule",()=>{
 const file=join(import.meta.dir,"../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json");
 if(!existsSync(file))return;
 const memory=JSON.parse(readFileSync(file,"utf8")).hostCapacity.memory;
 expect(memory.settledAfterMilliseconds).toBe(900000);
 expect(memory.settledKeepBasisPoints).toBe(15000);
});
