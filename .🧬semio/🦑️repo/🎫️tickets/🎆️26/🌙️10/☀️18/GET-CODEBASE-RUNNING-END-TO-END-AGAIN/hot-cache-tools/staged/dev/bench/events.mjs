#!/usr/bin/env bun
/** 📡 Watcher backends compared: edit-to-delivery latency and the cost of a build storm in excluded directories. */
import {mkdirSync,rmSync,writeFileSync,appendFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {createWatchSupervisorV1} from "../resident/watch/index.mjs";
import {POLICY_V1} from "../resident/protocol/policy.mjs";
import {createFilterV1} from "../resident/state/index.mjs";
import {percentiles,load} from "./lib.mjs";

const backend=process.argv[2]??"native",root="/private/tmp/hot-cache/ev-"+backend,n=Number(process.argv[3]??"200"),storm=Number(process.argv[4]??"20000");
rmSync(root,{recursive:true,force:true});mkdirSync(root+"/src",{recursive:true});mkdirSync(root+"/pkgs/a/node_modules/x",{recursive:true});
for(let i=0;i<50;i++)writeFileSync(root+"/src/f"+i+".ts","x");
const filter=createFilterV1({rooted:[".git"],names:["node_modules"]});
let received=new Map(),rowsSeen=0,wake;
const supervisor=createWatchSupervisorV1({root:realpath(root),policy:POLICY_V1,filter,rooted:[".git"],names:["node_modules"],binary:backend==="native"?"/private/tmp/hot-cache/bin/semio-repo-watch":undefined,backend:backend==="native"?"native":"portable",fenceDirectory:".semio-fence",instance:"bench",
 onBatch:async rows=>{const at=performance.now();for(const row of rows){rowsSeen++;if(!received.has(row[2]))received.set(row[2],at);}wake?.();},onLost:async()=>{}});
function realpath(p){return spawnSync("realpath",[p]).stdout.toString().trim();}
const started=await supervisor.start();
const latencies=[];
for(let i=0;i<n;i++){
 const path="src/f"+(i%50)+".ts";received.delete(path);
 const t0=performance.now();appendFileSync(root+"/"+path,"y");
 while(!received.has(path)){await new Promise(r=>{wake=r;setTimeout(r,2);});if(performance.now()-t0>2000)break;}
 latencies.push((received.get(path)??performance.now())-t0);
 await new Promise(r=>setTimeout(r,5));
}
const watcherPid=spawnSync("pgrep",["-f","semio-repo-watch watch --root "+realpath(root)]).stdout.toString().trim().split("\n")[0];
const cpu=pid=>pid?Number(spawnSync("ps",["-o","cputime=","-p",pid]).stdout.toString().trim().split(":").reduce((a,b)=>a*60+Number(b),0)):NaN;
const cpuBefore=cpu(watcherPid),selfBefore=process.cpuUsage(),rowsBefore=rowsSeen;
const t1=performance.now();
for(let i=0;i<storm;i++)writeFileSync(root+"/pkgs/a/node_modules/x/s"+i+".js","x");
const stormMs=performance.now()-t1;
await new Promise(r=>setTimeout(r,500));
const selfAfter=process.cpuUsage(selfBefore);
console.log(JSON.stringify({backend,state:started,load:load(),editToDeliveryMs:percentiles(latencies),storm:{files:storm,writeMs:Math.round(stormMs),rowsDeliveredToJs:rowsSeen-rowsBefore,watcherProcessCpuSeconds:Math.round((cpu(watcherPid)-cpuBefore)*100)/100,serviceProcessCpuMs:Math.round((selfAfter.user+selfAfter.system)/1000)}}));
await supervisor.stop();rmSync(root,{recursive:true,force:true});
process.exit(0);
