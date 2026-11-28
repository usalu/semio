/** 🪂️ Contract: a small request that fits runs ahead of a large request that holds the line, without delaying it beyond the end of what already ran when it arrived and without overcommitting the room. A 5000 MB request waits beside ten running 500 MB requests while a stream of 500 MB requests arrives; once the large request holds the line (thirty seconds) the stream still runs ahead of it while the request has waited less than the backfill bound (twice the measured median wait of the running requests, 120 s for 60 s measured) and only while it fits beside the large request once everything running has ended; afterwards the stream is drained behind the large request. With measured waits of zero the bound is `wait.bypassMilliseconds` and nothing runs ahead (the strict reservation of the starvation ticket).
 * @see ./🐍️.py
 * @see ./🧫️fixtures/🔣️.json */
import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {evaluateHostCapacityV1,hostBackfillBoundMillisecondsV1,readHostCapacityPolicyV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../🟦️.ts";
import base from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

type Row={admittedAtSecond:number|null;ranAhead:number;mostUsed:number};
const python=testPythonV1()??Bun.which("python3")??"python3",oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const expected=(()=>{const done=Bun.spawnSync([python,"-I",oracle]);if(done.exitCode!==0)throw Error(new TextDecoder().decode(done.stderr));return JSON.parse(new TextDecoder().decode(done.stdout)) as {backfill:Row;strict:Row};})();
const policy=readHostCapacityPolicyV1({...base.policy,memory:{...base.policy.memory,...fixture.memory}} as never),origin=1000000;
const request=(id:string,megabytes:number)=>({id,operation:"command",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1});
const grant=(id:string,megabytes:number,admittedAtMs:number,waitedSeconds:number):HostGrantV1=>({id,operation:"command",jobs:1,cpu:1,reservedMegabytes:megabytes,admittedAtMs,arrivedAtMs:admittedAtMs-waitedSeconds*1000,observedMegabytes:megabytes,pid:1,bootAtMs:0}) as never;

const scenario=(waitedSeconds:number):Row=>{
 let ledger:HostGrantV1[]=fixture.initialEndSeconds.map((_,at)=>grant(`seed${at}`,fixture.smallMegabytes,origin-3600000,waitedSeconds));
 const ends=new Map<string,number>(fixture.initialEndSeconds.map((end,at)=>[`seed${at}`,end]));
 let queue:HostWaiterV1[]=[{request:request("big",fixture.bigMegabytes),arrivedAtMs:origin,pid:1}];
 let arrivals=0,ranAhead=0,mostUsed=0,admittedAtSecond:number|null=null;
 const patience=policy.wait.bypassMilliseconds;
 for(let second=0;second<fixture.horizonSeconds&&admittedAtSecond===null;second+=fixture.stepSeconds){
  const now=origin+second*1000;
  ledger=ledger.filter(row=>!(ends.has(row.id)&&ends.get(row.id)!<=second));
  if(second%fixture.arrivalEverySeconds===0)queue.push({request:request(`small${arrivals++}`,fixture.smallMegabytes),arrivedAtMs:now,pid:1});
  const used=ledger.reduce((sum,row)=>sum+row.reservedMegabytes,0),host:HostSampleV1={version:1,atMs:now,platform:"darwin",cores:16,totalMegabytes:fixture.totalMegabytes,availableMegabytes:fixture.totalMegabytes-fixture.foreignMegabytes-used,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:1000,openFilesMaximum:245760,attachedImages:0} as never;
  const evaluation=evaluateHostCapacityV1(policy,host,ledger,queue,now);
  let usedNow=used;
  for(const row of evaluation.decisions.filter(decision=>decision.verdict==="admit")){
   if(admittedAtSecond!==null)break;
   if(row.id==="big"){admittedAtSecond=second;continue;}
   if(second*1000>=patience)ranAhead++;
   usedNow+=row.reservedMegabytes;
   ledger=[...ledger,grant(row.id,row.reservedMegabytes,now,waitedSeconds)];ends.set(row.id,second+fixture.runSeconds);
  }
  mostUsed=Math.max(mostUsed,usedNow);
  queue=queue.filter(waiter=>!evaluation.decisions.some(decision=>decision.id===waiter.request.id&&decision.verdict==="admit"));
 }
 return {admittedAtSecond,ranAhead,mostUsed};
};

test("requests run ahead of the large request that holds the line while it has waited less than the backfill bound, and the whole run equals the oracle",()=>{
 const run=scenario(fixture.measuredWaitSeconds);
 expect(expected.backfill.ranAhead).toBeGreaterThan(0);
 expect(run).toEqual(expected.backfill);
});

test("no starvation: the large request is admitted no later than the end of what already ran when it arrived, however many requests ran ahead",()=>{
 const run=scenario(fixture.measuredWaitSeconds),drained=Math.max(...fixture.initialEndSeconds)+fixture.stepSeconds;
 expect(run.admittedAtSecond).not.toBeNull();
 expect(run.admittedAtSecond!).toBeLessThanOrEqual(drained);
});

test("no overcommit: what runs never exceeds the room the host gives out above its floor and the interactive reserve",()=>{
 const run=scenario(fixture.measuredWaitSeconds),room=expected.backfill.mostUsed;
 expect(run.mostUsed).toBeLessThanOrEqual(fixture.totalMegabytes-fixture.foreignMegabytes-4096);
 expect(run.mostUsed).toBeLessThanOrEqual(8192);
 expect(room).toBe(run.mostUsed);
});

test("without measured waits the bound is bypassMilliseconds and the strict reservation of the starvation ticket holds: nothing runs ahead, admission equals the oracle",()=>{
 const run=scenario(0);
 expect(expected.strict.ranAhead).toBe(0);
 expect(run).toEqual(expected.strict);
});

test("the bound is twice the median measured wait, within bypassMilliseconds and agingMilliseconds, and bypassMilliseconds below three measurements",()=>{
 const rows=(waits:number[])=>waits.map(seconds=>({admittedAtMs:origin,arrivedAtMs:origin-seconds*1000}));
 expect(hostBackfillBoundMillisecondsV1(policy,rows([100,100]))).toBe(policy.wait.bypassMilliseconds);
 expect(hostBackfillBoundMillisecondsV1(policy,rows([60,60,60]))).toBe(120000);
 expect(hostBackfillBoundMillisecondsV1(policy,rows([1,1,1]))).toBe(policy.wait.bypassMilliseconds);
 expect(hostBackfillBoundMillisecondsV1(policy,rows([900,900,900]))).toBe(policy.wait.agingMilliseconds);
 expect(hostBackfillBoundMillisecondsV1(policy,[{admittedAtMs:origin},{admittedAtMs:origin},{admittedAtMs:origin}])).toBe(policy.wait.bypassMilliseconds);
});
