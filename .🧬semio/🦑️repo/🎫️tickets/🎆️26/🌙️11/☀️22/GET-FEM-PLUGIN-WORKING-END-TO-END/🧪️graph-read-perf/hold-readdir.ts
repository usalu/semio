import {readFileSync} from "node:fs";
import {pathToFileURL} from "node:url";
const root="C:\\semio\\🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\📚️library\\🏗️build\\";
const observation=await import(pathToFileURL(root+"👁️observation\\🟨️.mjs").href);
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const kind=process.argv[3]??"readdir",mode=process.argv[4]??"batch";
let rows=(memo.observations as {kind:string;path:string}[]).filter(r=>r.kind===kind);
console.log(kind,"rows",rows.length);
for(let attempt=0;attempt<20;attempt++){
 let stale:any;const t=performance.now();
 const ok=await observation.observationsHoldWithinRoundV1(rows,64,undefined,(row:any)=>{stale=row;});
 const ms=Math.round(performance.now()-t);
 console.log("attempt",attempt,"ok",ok,"ms",ms,"rows",rows.length,"us/row",Math.round(ms*1000/rows.length));
 if(ok)break;
 rows=rows.filter(r=>r!==stale);
}
