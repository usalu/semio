import {readFileSync} from "node:fs";
import {pathToFileURL} from "node:url";
const root="C:\\semio\\🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\📚️library\\🏗️build\\";
const observation=await import(pathToFileURL(root+"👁️observation\\🟨️.mjs").href);
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const cap=Number(process.argv[3]??5000),limit=Number(process.argv[4]??64);
const kinds=[...new Set(memo.observations.map((r:any)=>r.kind))] as string[];
for(const kind of kinds){
 const rows=memo.observations.filter((r:any)=>r.kind===kind).slice(0,cap);
 let stale=0;const t=performance.now();
 const ok=await observation.observationsHoldWithinRoundV1(rows,limit,undefined,()=>{stale++;});
 const ms=performance.now()-t;
 console.log(kind.padEnd(20),String(rows.length).padStart(7),"ms",String(Math.round(ms)).padStart(7),"us/row",Math.round(ms*1000/rows.length),ok?"holds":"STALE");
}
