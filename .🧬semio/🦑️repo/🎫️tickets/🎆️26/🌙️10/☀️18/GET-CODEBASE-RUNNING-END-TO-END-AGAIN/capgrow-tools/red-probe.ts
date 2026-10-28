/** 🔴️ capgrow: behavioural red/green probe on one root (base or batch): the stored grow worlds against the live decision, and a holder admitted with 1 of 8 jobs whose host frees memory; prints `worlds <mismatches>/<total>` and `holder jobs <n> parallel <p>`. Usage: bun red-probe.ts <root> */
import {mkdtempSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
const root=process.argv[2]!,hc=`${root}/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity`;
const capacity=await import(`${hc}/🟦️.ts`),decisionFixture=(await import(`${hc}/🧪️tests/🧫️fixtures/🔣️.json`)).default,grow=(await import(`${hc}/🧪️tests/🔼️upgrade/🌱️grow/🧫️fixtures/🔣️.json`)).default;
const policy=capacity.readHostCapacityPolicyV1({...decisionFixture.policy,memory:{...decisionFixture.policy.memory,maximumQuiet:4}});
let mismatches=0;for(const world of grow.worlds)if(JSON.stringify(capacity.evaluateHostCapacityV1(policy,world.host,world.ledger,world.queue,world.nowMs))!==JSON.stringify(world.expect))mismatches++;
console.log(`worlds ${mismatches}/${grow.worlds.length} differ from the oracle`);
const live=capacity.readHostCapacityPolicyV1({...decisionFixture.policy,memory:{...decisionFixture.policy.memory,rampMilliseconds:120},load:{...decisionFixture.policy.load,overrideAfterMilliseconds:600000},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:40,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000,lendAfterMilliseconds:600000}});
const floor=capacity.hostFloorMegabytesV1(live,{totalMegabytes:32768}),directory=mkdtempSync(join(tmpdir(),"cgp-")),request={id:capacity.newHostCapacityIdV1(),operation:"run",priority:"batch",minimumJobs:1,preferredJobs:8,fixedMegabytes:1536,perJobMegabytes:2048,cpuPerJob:1};let free=4000;
const host=()=>({host:{version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:floor+free,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10},swappedOutMegabytes:0});
await capacity.withHostCapacityV1({directory,policy:live,request,upgrade:request,control:{signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined},environment:{},write:(line:string)=>console.log(line),sample:async()=>host(),observe:async()=>3000,waited:()=>0,children:()=>[1]},async(handle:{jobs:number;id:string})=>{
 free=30000;await new Promise(resolve=>setTimeout(resolve,1500));
 const status=await capacity.readHostCapacityStatusV1({directory,policy:live,control:{signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined},sample:async()=>host()});
 console.log(`holder jobs ${status.grants.find((grant:{id:string})=>grant.id===handle.id).jobs} parallel ${handle.jobs}`);
});
