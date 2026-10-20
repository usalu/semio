#!/usr/bin/env bun
/** ⏪ Offline edits are recovered by FSEvents replay from the persisted cursor instead of a rescan. */
import {mkdirSync,rmSync,writeFileSync,realpathSync} from "node:fs";
import {createWatchSupervisorV1} from "../resident/watch/index.mjs";
import {POLICY_V1} from "../resident/protocol/policy.mjs";
import {createFilterV1} from "../resident/state/index.mjs";
const root="/private/tmp/hot-cache/ev-replay";rmSync(root,{recursive:true,force:true});mkdirSync(root+"/src",{recursive:true});
const real=realpathSync(root),filter=createFilterV1({names:["node_modules"]});
const mk=(restore,seen)=>createWatchSupervisorV1({root:real,policy:POLICY_V1,filter,names:["node_modules"],binary:"/private/tmp/hot-cache/bin/semio-repo-watch",backend:"native",fenceDirectory:".semio-fence",instance:"r",restore,onBatch:async rows=>{for(const r of rows)seen.add(r[2]);},onLost:async(s,reason)=>{seen.add("LOST:"+reason);}});
const first=new Set(),a=mk(undefined,first);await a.start();writeFileSync(real+"/src/a.ts","1");await a.fence();
const saved={cursor:a.state().cursor,uuid:a.state().uuid};await a.stop();
for(let i=0;i<20;i++)writeFileSync(real+"/src/offline"+i+".ts","x");
await new Promise(r=>setTimeout(r,300));
const second=new Set(),t=performance.now(),b=mk(saved,second);const st=await b.start();
await new Promise(r=>setTimeout(r,500));
const found=[...second].filter(p=>p.startsWith("src/offline")).length;
console.log(JSON.stringify({saved,replay:st,offlineEditsRecovered:found+"/20",lostFrames:[...second].filter(x=>x.startsWith("LOST")),startMs:Math.round(performance.now()-t)}));
await b.stop();rmSync(root,{recursive:true,force:true});process.exit(0);
