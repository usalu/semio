import {readFileSync} from "node:fs";
const obs=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs");
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const rows=memo.observations as {kind:string;path:string;fact:unknown}[];
const kinds=[...new Set(rows.map(r=>r.kind))];
console.log("rows",rows.length);
for(const kind of kinds){
 const subset=rows.filter(r=>r.kind===kind);const t=performance.now();
 let stale=0;
 const ok=await obs.observationsHold(subset,64,new Set(),new Set(),undefined,()=>{stale++;});
 console.log(kind.padEnd(20),String(subset.length).padStart(7),Math.round(performance.now()-t)+"ms",ok?"holds":"stale");
}
const t=performance.now();const ok=await obs.observationsHold(rows,64,new Set(),new Set());console.log("all",rows.length,Math.round(performance.now()-t)+"ms",ok);
