import {readFileSync} from "node:fs";
import {pathToFileURL} from "node:url";
const root="C:\\semio\\🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\📚️library\\🏗️build\\";
const observation=await import(pathToFileURL(root+"👁️observation\\🟨️.mjs").href);
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const skip=new Set(["analysis","field","file","readdir"]);
const rows=memo.observations as {kind:string;path:string}[];
const keep=rows.map(row=>!skip.has(row.kind));
const ownerRows=memo.owners.map((o:any)=>o.rows.filter((i:number)=>keep[i]).map((i:number)=>rows[i]));
const total=ownerRows.reduce((a:number,r:any[])=>a+r.length,0),distinct=keep.filter(Boolean).length;
console.log("owners",ownerRows.length,"owner row refs",total,"distinct kept rows",distinct);
const mode=process.argv[3]??"shared";
let t=performance.now();
if(mode==="shared"){
 await observation.withObservationRoundV1(async()=>{
  for(const r of ownerRows)await observation.observationsHoldWithinRoundV1(r,64);
 });
}else{
 for(const r of ownerRows)await observation.observationsHoldWithinRoundV1(r,64);
}
console.log(mode,"per-owner holds ms",Math.round(performance.now()-t));
t=performance.now();
await observation.observationsHoldWithinRoundV1(rows.filter((_,i)=>keep[i]),64);
console.log("one hold over distinct rows ms",Math.round(performance.now()-t));
