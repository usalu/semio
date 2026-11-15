const [module,worktree,store,countText="400"]=process.argv.slice(2);
const count=+countText;
const replica=await import(`${module}/🔁️replica/🟦️.ts`),attest=await import(`${module}/🩺️attest/🟦️.ts`),git=await import(`${module}/🌿️git/🟦️.ts`),journal=await import(`${module}/📒️journal/🟦️.ts`);
const r=await replica.openReplicaV1({worktree,storeRoot:store,originId:"b1",groupId:"g1",lock:false,concurrency:8},()=>{});
const rows:Record<string,{cpuUs:number;wallUs:number}>={};
const measure=async(name:string,fn:()=>Promise<unknown>)=>{
 for(let i=0;i<20;i++)await fn();
 const c0=process.cpuUsage(),t0=performance.now();
 for(let i=0;i<count;i++)await fn();
 const c=process.cpuUsage(c0),w=performance.now()-t0;
 rows[name]={cpuUs:+(((c.user+c.system))/count).toFixed(1),wallUs:+(w*1000/count).toFixed(1)};
};
await measure("syncEnabledV1 (sync switch read)",()=>replica.syncEnabledV1(worktree));
await measure("storeReachableV1 (stat store root)",()=>replica.storeReachableV1(store));
await measure("readRetirementV1 (retirement record)",()=>attest.readRetirementV1(store,"b1"));
await measure("repairRequestedV1 (lstat)",()=>replica.repairRequestedV1(worktree));
await measure("rebaseRequestedV1 (lstat)",()=>git.rebaseRequestedV1(worktree));
await measure("listOriginsV1 (readdir origins)",()=>journal.listOriginsV1(store));
await measure("readHeadV1 x1 origin",()=>journal.readHeadV1(store,"b1"));
await measure("followOnceV1 idle (reuse=false)",()=>replica.followOnceV1(r));
await measure("followOnceV1 idle (reuse=true)",()=>replica.followOnceV1(r,undefined,true));
await measure("gitStateV1 (heartbeat)",()=>git.gitStateV1(worktree));
import {mkdirSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
const gw=process.env.GITWT!;
const hasGit=gw!==undefined;
if(hasGit){
 await measure("gitStateV1 on a git worktree (heartbeat)",()=>git.gitStateV1(gw));
 const attester=attest.createAttesterV1({storeRoot:store,originId:"zz",groupId:"g1",now:Date.now,codeRoot:module,git:s=>git.gitStateV1(gw,s),reachable:async()=>true,view:async()=>null});
 await attester.start();
 const snap={sync:true,files:1,manifest:"0".repeat(64),cursors:{},incoming:0,hlc:"0000000000001-000000-zz",activity:0};
 await measure("attester.tick (idle round, git worktree)",()=>attester.tick(snap));
}
console.log(JSON.stringify(rows,null,1));
await replica.closeReplicaV1(r);
