// 🧪 Bun-bundler oracle over project scripts: how many workspace modules Bun resolves from each entry that the shipped closure misses, and that the patched closure misses.
import {readFileSync,rmSync,mkdirSync} from "node:fs";import {join} from "node:path";
import {G,W,loadBoth,scriptEntries} from "./lib.ts";
const [strideArg="17",limitArg="100000"]=process.argv.slice(2);
const {createScriptInputCache,shipped,patched}=await loadBoth(),entries=scriptEntries(Number(strideArg),0,Number(limitArg)),dir=join(G,"oracle-entries");mkdirSync(dir,{recursive:true});
const caches={shipped:createScriptInputCache({workspaceRoot:W}),patched:patched.createScriptInputCache({workspaceRoot:W})};
const rows:any[]=[];
for(const [i,entry] of entries.entries()){
 const meta=join(dir,`m${i}.json`),proc=Bun.spawnSync(["bun","build","--target=bun","--outfile=/dev/null","--metafile="+meta,join(W,entry)],{cwd:W,stdout:"pipe",stderr:"pipe"});
 if(proc.exitCode){rows.push({entry,status:"bun-build-failed"});continue;}
 const oracle=Object.keys(JSON.parse(readFileSync(meta,"utf8")).inputs).filter(p=>!p.includes("node_modules")&&!p.startsWith("..")&&!p.startsWith("/"));
 const run=async(api:any,cache:any)=>{try{return new Set((await api.relativeScriptInputs([join(W,entry)],W,cache,true) as string[]).map(x=>x.replace("{workspaceRoot}/","")));}catch{return undefined;}};
 const old=await run(shipped,caches.shipped),next=await run(patched,caches.patched);
 const missing=(set:Set<string>|undefined)=>set?oracle.filter(p=>!set.has(p)&&!set.has(p.normalize("NFC"))).length:-1;
 rows.push({entry,oracle:oracle.length,shippedMissing:missing(old),patchedMissing:missing(next)});
}
rmSync(dir,{recursive:true,force:true});
const ok=rows.filter(r=>r.oracle!==undefined);
console.log(JSON.stringify({entries:rows.length,oracleFailed:rows.length-ok.length,oracleModules:ok.reduce((s,r)=>s+r.oracle,0),shippedEntriesWithMissing:ok.filter(r=>r.shippedMissing>0).length,shippedMissingModules:ok.reduce((s,r)=>s+Math.max(0,r.shippedMissing),0),patchedEntriesWithMissing:ok.filter(r=>r.patchedMissing>0).length,patchedMissingModules:ok.reduce((s,r)=>s+Math.max(0,r.patchedMissing),0),patchedErrors:ok.filter(r=>r.patchedMissing<0).length,sample:ok.filter(r=>r.patchedMissing!==0).slice(0,5)}));
