const fs=require("node:fs"),Module=require("node:module"),{dirname}=require("node:path");
const MOD="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs",ROOT="/Users/ueli/Documents/semio";
const jsc=require("bun:jsc");
const m=new Module(MOD,null);m.filename=MOD;m.paths=Module._nodeModulePaths(dirname(MOD));
m._compile(fs.readFileSync(MOD,"utf8")+"\nmodule.exports.__dbg={listingCache};",MOD);
const {createBunWorkspaceOwnershipV1,__dbg}=m.exports;
const own=createBunWorkspaceOwnershipV1({ownershipPath:MOD,ownershipSource:fs.readFileSync(MOD,"utf8")});
const mb=()=>{jsc.gcAndSweep();jsc.gcAndSweep();return process.memoryUsage().heapUsed/1048576;};
const rss=()=>process.memoryUsage().rss/1048576;
(async()=>{
 const h0=mb();let hin,hobs;
 await own.withCurrentBunWorkspaceOwnershipV1(ROOT,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async c=>{hin=mb();const o=c.observed();hobs=mb();console.log("observed copy MB",(hobs-hin).toFixed(0),"paths",o.paths.length);});
 const hclosed=mb();
 let entries=0,projected=0;for(const r of __dbg.listingCache.values()){entries+=r.entries.length;if(r.projected)projected+=r.projected.length;}
 console.log("base",h0.toFixed(0),"in custody",hin.toFixed(0),"closed",hclosed.toFixed(0),"listingCache dirs",__dbg.listingCache.size,"entries",entries,"projected",projected);
 __dbg.listingCache.clear();const hc=mb();console.log("after cache clear",hc.toFixed(0),"cache MB",(hclosed-hc).toFixed(0),"rss",rss().toFixed(0));
 console.log(JSON.stringify(jsc.heapStats().objectTypeCounts).slice(0,400));
})().catch(e=>console.log("ERR",e.stack));
