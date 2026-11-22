import fs from "node:fs";
import {readFileSync} from "node:fs";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const obs=await import(G+"/👁️observation/🟨️.mjs");
const producer=await import(G+"/🕸️graph/📥️inference/🟨️.mjs");
await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const all=memo.observations as {kind:string;path:string}[];
const kinds:Record<string,number>={};for(const r of all)kinds[r.kind]=(kinds[r.kind]??0)+1;
console.log("rows by kind",kinds);
const counts=new Map<string,{n:number;paths:Set<string>;ms:number}>();
const port=new Proxy(fs,{get(target,key:string){const v=(target as any)[key];if(typeof v!=="function"||!/Sync$/.test(key))return v;return(...a:unknown[])=>{const t=performance.now();try{return v.apply(target,a);}finally{const c=counts.get(key)??{n:0,paths:new Set(),ms:0};c.n++;c.ms+=performance.now()-t;if(typeof a[0]==="string")c.paths.add(a[0]);counts.set(key,c);}};}});
const limit=Number(process.argv[3]??633);const t0=performance.now();let rowsDone=0;
await obs.withObservedWorldV1(port,()=>obs.settleObservedComputationV1(async()=>{
 for(const owner of memo.owners.slice(0,limit)){const rows=owner.rows.map((i:number)=>all[i]);await obs.observationsHoldWithinRoundV1(rows,64);rowsDone+=rows.length;}
 return 1;
}));
console.log("owners",limit,"rows",rowsDone,Math.round(performance.now()-t0)+"ms");
for(const [k,c] of [...counts].sort((a,b)=>b[1].ms-a[1].ms))console.log(k.padEnd(14),String(c.n).padStart(8)+" calls",String(c.paths.size).padStart(7)+" distinct",Math.round(c.ms)+"ms");
