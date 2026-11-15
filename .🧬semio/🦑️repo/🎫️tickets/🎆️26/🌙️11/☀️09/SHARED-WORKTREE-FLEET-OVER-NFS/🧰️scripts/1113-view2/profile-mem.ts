import {heapStats} from "bun:jsc";
const [moduleRoot,mirror,worktree,steps]=process.argv.slice(2) as [string,string,string,string];
const {openReplicaV1,closeReplicaV1}=await import(`${moduleRoot}/🔁️replica/🟦️.ts`);
const git=await import(`${moduleRoot}/🌿️git/🟦️.ts`);
const mb=(n:number)=>(n/1048576).toFixed(0);
const show=(label:string)=>{(globalThis as any).Bun.gc(true);const m=process.memoryUsage(),h=heapStats();console.log(label.padEnd(24),"rss",mb(m.rss),"heapUsed",mb(m.heapUsed),"heapCap",mb(h.heapCapacity),"ab",mb(m.arrayBuffers),"maxRSS",mb(process.resourceUsage().maxRSS*1024));};
show("start");
const machine=JSON.parse(await Bun.file(`${mirror}/.cache/f/machine.json`).text());
const replica=await openReplicaV1({worktree:mirror,storeRoot:machine.storeRoot,originId:machine.originId,groupId:machine.groupId,lock:false});
show("opened");
for(const step of steps.split(",")){
 const t=performance.now();
 if(step==="view"){const v=await git.gitViewV1(worktree);console.log("view",v?.lines.length);}
 if(step==="tracked"){const v=await git.trackedEntriesV1(worktree);console.log("tracked",v?.length);}
 if(step==="guards"){const v=await git.gitGuardsV1(worktree,{});console.log("guards",v?.trackedIgnored);}
 if(step==="state"){await git.gitStateV1(worktree);}
 console.log(step,"ms",(performance.now()-t).toFixed(0));
 show("after "+step);
}
await closeReplicaV1(replica);
show("closed");
