const fs=require("node:fs");
const MOD="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs",ROOT="/Users/ueli/Documents/semio";
const calls={};["lstatSync","opendirSync","readdirSync","openSync","fstatSync","readSync","closeSync"].forEach(n=>{const o=fs[n];fs[n]=function(...a){calls[n]=(calls[n]||0)+1;return o.apply(this,a);};});
const {createBunWorkspaceOwnershipV1}=require(MOD);
const own=createBunWorkspaceOwnershipV1({ownershipPath:MOD,ownershipSource:fs.readFileSync(MOD,"utf8")});
const snap=()=>{const s=JSON.stringify(calls);for(const k in calls)calls[k]=0;return s;};
const N=+process.env.RECHECKS||3;
(async()=>{
 const t0=performance.now();let mid;
 await own.withCurrentBunWorkspaceOwnershipV1(ROOT,{kind:"all"},{signal:new AbortController().signal,frontierBatchPaths:64,advance:async()=>{}},async c=>{
  mid=performance.now();console.log("derive+fence",(mid-t0).toFixed(0),"ms paths",c.observed().paths.length,snap());
  for(let i=0;i<N;i++){const t=performance.now();await c.recheck();console.log("recheck",i,(performance.now()-t).toFixed(0),"ms",snap());}
 });
 console.log("close",(performance.now()-mid).toFixed(0),"ms",snap());
})();
