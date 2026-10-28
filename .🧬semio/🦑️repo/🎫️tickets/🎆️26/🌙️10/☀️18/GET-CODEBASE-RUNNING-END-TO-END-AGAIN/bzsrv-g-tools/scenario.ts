import {mkdirSync,rmSync,appendFileSync,existsSync} from "node:fs";
import {join} from "node:path";
import * as sessions from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts";
import {reapBazelServersV1,stopBazelServerV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🔌️port/🟦️.ts";
import {bazel,flagsFor,serverPid,subtreeRss,shutdown,table} from "./probe.ts";
const tools=import.meta.dir,S="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/bz";
const [label,spareArg="2500",memoryArg="16384",holdArg="20",tailArg="170"]=process.argv.slice(2),spare=Number(spareArg),memory=Number(memoryArg),hold=Number(holdArg),tail=Number(tailArg);
const SOURCE="/Users/ueli/Documents/semio/.cache/b/p/m/bazel/workspaces/s1",KEYS=["760c0f403769cd16","7540552c30354eeb","98ca1998fcd5adc1"];
const root=join(S,"scn-"+label);rmSync(root,{recursive:true,force:true});mkdirSync(join(root,"p/bazel/outputs/server"),{recursive:true});
const floor=process.env.SCN_FLOOR==="old"?{floorBasisPoints:2000,floorMinimumMegabytes:4096,floorMaximumMegabytes:8192}:{floorBasisPoints:1250,floorMinimumMegabytes:1536,floorMaximumMegabytes:4096},plan=sessions.planBazelServersV1({cores:10,memoryMegabytes:memory},floor),reaping=process.env.SCN_REAP==="1";
const held=new Set<string>(),states=new Map<string,any>(),queue:number[]=[];let tail0=Promise.resolve(),tickets=0;
const port:any={
 async claim(slot:string){if(held.has(slot))return undefined;held.add(slot);return()=>{held.delete(slot);};},
 async gate(){const previous=tail0;let open!:()=>void;tail0=new Promise<void>(done=>{open=done;});await previous;return()=>open();},
 async spare(){return spare;},async imageFree(){return Infinity;},queued:()=>queue.length,
 enqueue:()=>{const id=++tickets;queue.push(id);return{first:()=>queue[0]===id,leave:()=>{const at=queue.indexOf(id);if(at>=0)queue.splice(at,1);}};},
 async read(slot:string){return states.get(slot);},async write(slot:string,state:any){states.set(slot,state);},
 sleep:(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds)),now:()=>Date.now(),progress:(line:string)=>{console.log(line);},
};
const started=Date.now(),series:{t:number;servers:number;rssMb:number;held:number}[]=[];
const timer=setInterval(()=>{const rows=table();let count=0,rss=0;for(const[pid,row]of rows)if(row.command.startsWith("bazel(")&&row.command.includes(root)){count++;rss+=subtreeRss(pid,rows);}series.push({t:Math.round((Date.now()-started)/1000),servers:count,rssMb:Math.round(rss/1024),held:held.size});},2000);
const results:Record<string,unknown>[]=[];
async function run(index:number){
 const key=KEYS[index]!,begin=Date.now();
 await Bun.sleep(index*1500);
 const session=await sessions.acquireBazelSessionV1(port,plan,{key,signal:new AbortController().signal,language:"en",timeoutMilliseconds:900000});
 const waited=Date.now()-begin;
 const pkg=join(root,"p/bazel/workspaces",session.slot,key);
 if(!existsSync(pkg)){mkdirSync(join(root,"p/bazel/workspaces",session.slot),{recursive:true});Bun.spawnSync(["cp","-cR",key==="760c0f403769cd16"?join(S,"pkg-"+key):join(SOURCE,key),pkg]);}
 if(reaping)await reapBazelServersV1({leases:"",server:join(root,"p/bazel/outputs/server"),workspaces:join(root,"p/bazel/workspaces")},port,plan,new AbortController().signal,base=>stopBazelServerV1(base,new AbortController().signal),{budget:plan.slots,language:"en",progress:line=>console.log(line)});
 const flags=flagsFor(tools,join(root,"p"),session.slot,plan.heapMegabytes),extra:string[]=[];
 flags.startup=flags.startup.filter(value=>!value.startsWith("--host_jvm_args=")&&!value.startsWith("--max_idle_secs="));flags.startup.push(...sessions.bazelSessionStartupV1(plan,Number(process.env.SCN_IDLE??"120")));
 const build=await bazel(flags,extra,pkg,["build","--nobuild",...flags.command,"//:all"]);
 await Bun.sleep(hold*1000);
 await session.release();
 results.push({run:index,key,slot:session.slot,mode:session.mode,waitedMs:waited,buildMs:build.wall,doneAtMs:Date.now()-started,peakRssMb:build.peakRssMb});
}
await Promise.all([0,1,2].map(run));
const doneAt=Math.round((Date.now()-started)/1000);
await Bun.sleep(tail*1000);clearInterval(timer);
const peak=series.reduce((best,row)=>row.servers>best.servers||row.servers===best.servers&&row.rssMb>best.rssMb?row:best,series[0]!),peakRss=Math.max(...series.map(row=>row.rssMb));
const lingering=series.filter(row=>row.t>doneAt&&row.servers>0).length*2;
const row={label,memory,spare,plan:{slots:plan.slots,heap:plan.heapMegabytes},policy:{idle:process.env.SCN_IDLE??"120"},runs:results,allRunsDoneSec:doneAt,peakServers:Math.max(...series.map(value=>value.servers)),peakRssMb:peakRss,peak,lingerSecondsAfterDone:lingering,series:series.filter((_,index)=>index%5===0)};
const out=join(tools,"..","🗑️generated","bzsrv-g");mkdirSync(out,{recursive:true});appendFileSync(join(out,"scenario.jsonl"),JSON.stringify(row)+"\n");
console.log(JSON.stringify({...row,series:undefined},null,1));
for(const slot of ["s0","s1","s2","s3"]){const pkg=join(root,"p/bazel/workspaces",slot);if(existsSync(pkg))for(const key of KEYS)if(existsSync(join(pkg,key))){shutdown(flagsFor(tools,join(root,"p"),slot,1024),join(pkg,key));break;}}
rmSync(root,{recursive:true,force:true});
