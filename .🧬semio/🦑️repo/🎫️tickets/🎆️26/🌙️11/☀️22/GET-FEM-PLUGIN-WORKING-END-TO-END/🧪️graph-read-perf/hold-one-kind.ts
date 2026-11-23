import {readFileSync} from "node:fs";
import {pathToFileURL} from "node:url";
const root="C:\\semio\\🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\📚️library\\🏗️build\\";
const observation=await import(pathToFileURL(root+"👁️observation\\🟨️.mjs").href);
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const kind=process.argv[3],cap=Number(process.argv[4]??5000),limit=Number(process.argv[5]??64);
const rows=memo.observations.filter((r:any)=>r.kind===kind).slice(0,cap);
let stale=0;const t=performance.now();
const ok=await observation.observationsHoldWithinRoundV1(rows,limit,undefined,()=>{stale++;});
const ms=performance.now()-t;
console.log(kind,rows.length,"ms",Math.round(ms),"us/row",Math.round(ms*1000/rows.length),ok?"holds":"STALE");
const {lstatSync}=await import("node:fs");
const t2=performance.now();for(const r of rows){try{lstatSync(r.path);}catch{}}
console.log("raw lstatSync us/row",Math.round((performance.now()-t2)*1000/rows.length));
