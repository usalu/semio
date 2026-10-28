import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🧫️fixtures/🔣️.json";
import {simulateLeaseQueueV1,type LeasePriorityV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts";
import {observeLeaseHolderV1,leaseReclaimPortsV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts";
import {leaseRegistryFolderV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts";

const mean=(rows:number[])=>rows.reduce((a,b)=>a+b,0)/rows.length,p95=(rows:number[])=>[...rows].sort((a,b)=>a-b)[Math.min(rows.length-1,Math.floor(rows.length*0.95))]!;
for(const id of ["tonight-0","tonight-0-with-dev"]){
 const sim=fixture.simulations.find(row=>row.id===id)!,jobs=sim.jobs.map(job=>({...job,class:job.class as LeasePriorityV1})),klass=new Map(jobs.map(job=>[job.id,job.class]));
 for(const mode of ["fifo","priority"] as const){
  const rows=simulateLeaseQueueV1(jobs,mode==="fifo"),line:string[]=[];
  for(const c of ["interactive","agent","background"] as const){const waits=rows.filter(row=>klass.get(row.id)===c).map(row=>row.wait/60);if(waits.length)line.push(`${c} n=${waits.length} mean ${mean(waits).toFixed(0)} p95 ${p95(waits).toFixed(0)} max ${Math.max(...waits).toFixed(0)} min`);}
  console.log(id.padEnd(20),mode.padEnd(9),line.join(" | "));
 }
}
const root=mkdtempSync(join(tmpdir(),"lgm-")),directory="/w/lease",now=Date.now(),folder=leaseRegistryFolderV1(root,directory);mkdirSync(folder,{recursive:true});
for(let pid=1;pid<=40;pid++){
 const holds=Array.from({length:130},(_,i)=>({resource:"native-fixed-image:/w/.cache/b/"+i,mode:"shared",sinceMs:now-1000,task:1}));
 writeFileSync(join(folder,pid+"-"+now+".json"),JSON.stringify({version:1,pid:process.pid+(pid===1?0:1000000+pid),startedAtMs:now,updatedAtMs:now,command:"c",directory,standing:{version:1,priority:"agent",implementation:{digest:"0123456789abcdef",loadedAtMs:now,components:{}},progressAtMs:now},holds,waits:[]}));
}
const input={directory,resource:"repository-graph-facts:/w:shard:7",headPid:process.pid,headMode:"exclusive" as const,headArrivalMs:now,root,environment:{SEMIO_LEASE_RECLAIM:"request"},ports:{...leaseReclaimPortsV1,write:()=>{}}};
const start=performance.now();for(let i=0;i<200;i++)await observeLeaseHolderV1(input);
console.log("observeLeaseHolderV1 with 40 registry entries of 130 holds:",((performance.now()-start)/200).toFixed(2),"ms per call (every 15 s per waiter)");
rmSync(root,{recursive:true,force:true});
