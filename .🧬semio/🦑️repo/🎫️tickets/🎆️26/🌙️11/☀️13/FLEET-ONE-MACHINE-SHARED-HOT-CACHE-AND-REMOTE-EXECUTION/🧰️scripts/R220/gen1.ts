import fs from "node:fs";import cp from "node:child_process";import {createRequire} from "node:module";
const require=createRequire(import.meta.url);
const jsc=require("bun:jsc");
const MOD=process.env.MOD||"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs",ROOT="/Users/ueli/Documents/semio";
const GEN=(process.env.GEN||"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🟦️.ts");
const fp=()=>{const o=cp.spawnSync("/usr/bin/footprint",[String(process.pid)],{encoding:"utf8"}).stdout.match(/Footprint:\s*([\d.]+)\s*(KB|MB|GB)/);return o?(+o[1])*({KB:1/1024,MB:1,GB:1024}as any)[o[2]]:NaN;};
const t0=performance.now();let peak=0;
const sample=(l:string)=>{const f=fp();peak=Math.max(peak,f);const m=process.memoryUsage();console.log(`${((performance.now()-t0)/1000).toFixed(1)}s ${l} footprint=${f.toFixed(0)}MB heapUsed=${(m.heapUsed/1048576).toFixed(0)} ext=${(m.external/1048576).toFixed(0)}`);};
const {createBunWorkspaceOwnershipV1}=require(MOD);
const own=createBunWorkspaceOwnershipV1({ownershipPath:MOD,ownershipSource:fs.readFileSync(MOD,"utf8")});
const gen=await import(GEN);
const z="0".repeat(64);
let ticks=0;
await own.withCurrentBunWorkspaceOwnershipV1(ROOT,{kind:"all"},{signal:new AbortController().signal,frontierBatchPaths:64,advance:async()=>{}},async(c:any)=>{
  const observed=c.observed();jsc.gcAndSweep();sample("before capture (ownership held)");
  const control={signal:new AbortController().signal,check(){},async tick(){if(++ticks%2===0)sample("capture tick "+ticks);}};
  for(let i=0;i<2;i++){
  const input:any={workspace:ROOT,storeRoot:"/tmp/store",scope:".",startedAtMs:Date.now(),runtime:{manager:"bun",revision:"1",node:"1",platform:"p",arch:"a",sha256:z},producer:{files:[],environment:[]},recipeSha256:z,ownershipSha256:z,required:[],inputs:[],membership:observed,bindings:[],ready:[],execution:{scopes:[],runtimeSha256:z,observations:[]},scopeReadSha256:z,control};
  let r:any;try{r=await gen.captureGenerationReceiptV1(input);}catch(e:any){console.log("capture error",String(e.message).slice(0,200));}
  console.log("capture result",r?.status,r?.reason,r?.files?.map((f:any)=>f.name.slice(0,12)+":"+f.bytes.length));
  r=undefined;sample("after capture "+i);
  }
  jsc.gcAndSweep();sample("after capture gc");
});
console.log("PEAK",peak.toFixed(0),"total",((performance.now()-t0)/1000).toFixed(0),"s");
