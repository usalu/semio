const fs=require("node:fs");
const MOD=process.env.MOD||"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs",ROOT=process.env.TREE||"/Users/ueli/Documents/semio",SEL=JSON.parse(process.env.SELECT||'{"kind":"all"}');
let jsc;try{jsc=require("bun:jsc");}catch{}
const calls={};["lstatSync","opendirSync","readdirSync","openSync","fstatSync"].forEach(n=>{const o=fs[n];fs[n]=function(...a){calls[n]=(calls[n]||0)+1;return o.apply(this,a);};});
const mem=l=>{if(jsc)jsc.gcAndSweep();else if(global.gc)global.gc();const m=process.memoryUsage();console.log("MEM",l,"rss",(m.rss/1048576).toFixed(0),"heapUsed",(m.heapUsed/1048576).toFixed(0),"ext",(m.external/1048576).toFixed(0),jsc?("objs "+jsc.heapStats().objectCount):"");};
const {createBunWorkspaceOwnershipV1}=require(MOD);
const own=createBunWorkspaceOwnershipV1({ownershipPath:MOD,ownershipSource:fs.readFileSync(MOD,"utf8")});
(async()=>{
 mem("start");
 for(const label of ["first","repeat"]){
  const t0=performance.now();let mid;
  await own.withCurrentBunWorkspaceOwnershipV1(ROOT,SEL,{signal:new AbortController().signal,frontierBatchPaths:64,advance:async()=>{}},async c=>{mid=performance.now();console.log(label,"derive",(mid-t0).toFixed(0),"ms paths",c.observed().paths.length,JSON.stringify(calls));mem(label+" in custody");const t=performance.now();await c.recheck();console.log(label,"recheck",(performance.now()-t).toFixed(0),"ms");const o=c.observed();mem(label+" after observed()");});
  mem(label+" closed");
 }
})().catch(e=>{console.log("ERR",e.stack.slice(0,500));});
