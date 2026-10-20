import {createWarmPoolV1,writeWarmPreloadV1} from "/private/tmp/hot-cache/ov/resident/delegate/index.mjs";
import {spawn} from "node:child_process";
import {join} from "node:path";
const root="/Users/ueli/Documents/semio",L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const mods=JSON.parse(await Bun.file("/private/tmp/hot-cache/real/mods.json").text()).map(m=>join(root,L,m));
const n=Number(process.argv[2]??"3");
const preload=writeWarmPreloadV1("/private/tmp/hot-cache/real");
const cold=[];
for(let i=0;i<n;i++){
 const t=performance.now();
 const out=await new Promise(resolve=>{const c=spawn("bun",["/private/tmp/hot-cache/real/warm-main.ts","run","x"],{cwd:root,stdio:["ignore","pipe","inherit"]});let s="";c.stdout.on("data",d=>s+=d);c.on("exit",()=>resolve(s));});
 cold.push({wallMs:Math.round(performance.now()-t),child:JSON.parse(out)});
}
const warm=[];
const pool=createWarmPoolV1({command:["bun","--preload",preload,"/private/tmp/hot-cache/real/warm-main.ts"],environment:{...process.env,SEMIO_WARM_MODULES:JSON.stringify(mods)},cwd:root,size:1,idleMilliseconds:120000});
pool.top();
for(let i=0;i<n;i++){
 const start=Date.now();while(pool.idleCount()<1&&Date.now()-start<120000)await new Promise(r=>setTimeout(r,50));
 const worker=pool.take();const t=performance.now();let out="";
 await pool.run(worker,{argv:["run","x"],cwd:root,env:{}},{onOutput:async(s,d)=>{if(s==="stdout")out+=d;}});
 warm.push({requestToExitMs:Math.round(performance.now()-t),child:JSON.parse(out)});
}
await pool.close();
console.log(JSON.stringify({load:require("os").loadavg()[0],cold,warm},null,1));
