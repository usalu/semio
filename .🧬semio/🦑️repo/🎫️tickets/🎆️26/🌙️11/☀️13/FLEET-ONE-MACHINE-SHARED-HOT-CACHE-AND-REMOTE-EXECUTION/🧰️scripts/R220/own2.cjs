const fs=require("node:fs"),cp=require("node:child_process");
const MOD=process.env.MOD||"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs",ROOT=process.env.TREE||"/Users/ueli/Documents/semio",SEL=JSON.parse(process.env.SELECT||'{"kind":"all"}');
const jsc=require("bun:jsc");
const fp=()=>{const o=cp.spawnSync("/usr/bin/footprint",[String(process.pid)],{encoding:"utf8"}).stdout.match(/Footprint:\s*([\d.]+)\s*(KB|MB|GB)/);return o?(+o[1])*({KB:1/1024,MB:1,GB:1024}[o[2]]):NaN;};
const t0=performance.now();let peak=0;const log=[];
const sample=(l)=>{const f=fp();peak=Math.max(peak,f);const m=process.memoryUsage();log.push(`${((performance.now()-t0)/1000).toFixed(1)}s ${l} footprint=${f.toFixed(0)}MB heapUsed=${(m.heapUsed/1048576).toFixed(0)} ext=${(m.external/1048576).toFixed(0)} rss=${(m.rss/1048576).toFixed(0)}`);console.log(log[log.length-1]);};
const Module=require("node:module"),{dirname}=require("node:path");
let src=fs.readFileSync(process.env.SRC||MOD,"utf8");
for(const pair of JSON.parse(process.env.PATCH||"[]")){ if(!src.includes(pair[0])) throw Error("patch miss "+pair[0]); src=src.split(pair[0]).join(pair[1]); }
const m=new Module(MOD,null);m.filename=MOD;m.paths=Module._nodeModulePaths(dirname(MOD));m._compile(src,MOD);
const {createBunWorkspaceOwnershipV1}=m.exports;
const own=createBunWorkspaceOwnershipV1({ownershipPath:MOD,ownershipSource:fs.readFileSync(MOD,"utf8")});
let steps=0;const EVERY=+process.env.EVERY||40000;
(async()=>{
 sample("start");
 await own.withPublishedBunWorkspaceOwnershipV1(ROOT,SEL,{signal:new AbortController().signal,frontierBatchPaths:64,advance:async p=>{if(++steps%EVERY===0)sample("step "+steps+" "+p.operation);}},async c=>{
  sample("custody");
  const o=c.observed();sample("observed paths="+o.paths.length);
  jsc.gcAndSweep();sample("after gc");
  const h=jsc.heapStats();console.log("heapSize",(h.heapSize/1048576).toFixed(0),"objs",h.objectCount,JSON.stringify(Object.entries(h.objectTypeCounts).sort((a,b)=>b[1]-a[1]).slice(0,12)));
  await c.recheck();sample("after recheck");
 });
 sample("closed");jsc.gcAndSweep();sample("closed gc");console.log(cp.spawnSync("/usr/bin/footprint",[String(process.pid)],{encoding:"utf8"}).stdout.split("\n").slice(0,30).join("\n"));console.log("PEAK",peak.toFixed(0),"MB total",((performance.now()-t0)/1000).toFixed(0),"s steps",steps);
})().catch(e=>console.log("ERR",e.stack.slice(0,800)));
