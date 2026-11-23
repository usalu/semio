import {readFileSync} from "node:fs";
import {pathToFileURL} from "node:url";
const root="C:\\semio\\🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\📚️library\\🏗️build\\👁️observation\\";
const variant=process.argv[3]??"patched";
const observation=await import(pathToFileURL(root+(variant==="baseline"?"🟨️.baseline-perf.mjs":"🟨️.mjs")).href);
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const skip=new Set(["analysis","field","file"]);
const rows=(memo.observations as {kind:string;path:string}[]).filter(r=>!skip.has(r.kind));
const {lstatSync,statSync,existsSync}=await import("node:fs");
let t=performance.now();
const dropped=new Set<any>();let ok=false,attempts=0;
for(;attempts<10;attempts++){
 ok=await observation.observationsHoldWithinRoundV1(rows.filter(r=>!dropped.has(r)),64,undefined,(row:any)=>{dropped.add(rows.find(r=>r.kind===row.kind&&r.path===row.path));});
 if(ok)break;
}
console.log(JSON.stringify({variant,rows:rows.length,ok,ms:Math.round(performance.now()-t)}));
