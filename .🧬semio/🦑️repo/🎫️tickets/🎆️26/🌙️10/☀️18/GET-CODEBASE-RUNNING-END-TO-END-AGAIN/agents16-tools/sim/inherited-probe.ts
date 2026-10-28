/** 🔬️ Probe: what jobs a nested invocation that inherits a quiet grant hands its body (real `withHostCapacityV1`, temp gate directory, the holder's upgrade simulated by rewriting the row after 300 ms). Usage: bun inherited-probe.ts <root> <gate-dir> */
import {mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const [root,dir]=process.argv.slice(2) as [string,string];
const hc=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts"));
const policy=hc.readHostCapacityPolicyV1(JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json"),"utf8")).hostCapacity);
mkdirSync(join(dir,"grants"),{recursive:true});mkdirSync(join(dir,"queue"),{recursive:true});
const id="probeQuietGrant0001",now=Date.now(),boot=hc.hostBootAtV1(dir,now);
const full={id,operation:"run",priority:"batch",minimumJobs:1,preferredJobs:8,fixedMegabytes:300,perJobMegabytes:400,cpuPerJob:1};
const plan=hc.planHostCapacityV1(policy,{totalMegabytes:16384},full,8);
const row={id,operation:"run",jobs:1,cpu:1,reservedMegabytes:2048,admittedAtMs:now,observedMegabytes:null,pid:process.pid,bootAtMs:boot,quiet:true,arrivedAtMs:now};
writeFileSync(join(dir,"grants",id+".json"),JSON.stringify(row));
setTimeout(()=>{const{quiet:_,...rest}=row;writeFileSync(join(dir,"grants",id+".json"),JSON.stringify({...rest,jobs:8,cpu:8,reservedMegabytes:3500,upgradedAtMs:Date.now(),resumedAtMs:Date.now()}));},300);
const environment={SEMIO_HOST_CAPACITY_DIRECTORY:dir,SEMIO_HOST_CAPACITY_GRANT:id,SEMIO_HOST_CAPACITY_PLAN:JSON.stringify(plan),SEMIO_LANGUAGE:"en"};
const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))};
const quiet=hc.quietHostRequestV1({...full,id:hc.newHostCapacityIdV1()},{quiet:true,megabytes:2048,rampMilliseconds:60000,samples:1,shareBasisPoints:10000});
await hc.withHostCapacityV1({directory:dir,policy,request:{...quiet,id:full.id},upgrade:full,control,environment,write:(line:string)=>console.log(line)},async(handle:any)=>{console.log(JSON.stringify({inherited:handle.inherited,handleJobs:handle.jobs,planJobs:handle.plan.jobs,rowJobsNow:JSON.parse(readFileSync(join(dir,"grants",id+".json"),"utf8")).jobs}));});
