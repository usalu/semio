import {readFileSync,writeFileSync} from "node:fs";
import {evaluateHostCapacityV1,readHostCapacityPolicyV1} from "/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts";
const file=process.argv[2]!,raw=readFileSync(file,"utf8"),fixture=JSON.parse(raw),policy=readHostCapacityPolicyV1(fixture.policy);
const host=(availableMegabytes:number)=>({...fixture.cases[0].host,platform:"linux",cores:12,totalMegabytes:32768,availableMegabytes,load:1});
const req=(id:string,fixedMegabytes:number,perJobMegabytes:number,preferredJobs:number,arrivedAtMs:number,pid:number)=>({request:{id,operation:"cargo",priority:"batch",minimumJobs:1,preferredJobs,fixedMegabytes,perJobMegabytes,cpuPerJob:1},arrivedAtMs,pid});
const cases=[
 {name:"cargo-oversized-request-runs-with-the-jobs-that-fit",note:"room = 18000-6553 = 11447; (11447-1536)/2048 = 4 jobs of 8, never a wait",host:host(18000),ledger:[],queue:[req("cargo0001",1536,2048,8,0,1)],nowMs:1000000},
 {name:"cargo-waits-only-when-one-job-does-not-fit",note:"room = 8000-6553 = 1447 < 1536+2048: even one job does not fit",host:host(8000),ledger:[],queue:[req("cargo0001",1536,2048,8,0,1)],nowMs:1000000},
 {name:"cargo-waiters-are-admitted-side-by-side-while-they-fit",note:"room = 24000-6553 = 17447; two of 4 jobs need 5632 each, the third of 8 gets (17447-11264-1536)/1024 = 4 jobs",host:host(24000),ledger:[],queue:[req("cargo0001",1536,1024,4,0,1),req("cargo0002",1536,1024,4,1,2),req("cargo0003",1536,1024,8,2,3)],nowMs:1000000},
].map(row=>({...row,expect:evaluateHostCapacityV1(policy,row.host as never,[],row.queue as never,row.nowMs)}));
const marker='\n ],\n "corpus": [';
const at=raw.indexOf(marker);
writeFileSync(file,raw.slice(0,at)+cases.map(row=>",\n  "+JSON.stringify(row)).join("")+raw.slice(at));
for(const row of cases)console.log(row.name,JSON.stringify(row.expect.decisions));
