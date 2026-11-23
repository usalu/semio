import {readFileSync} from "node:fs";
import {pathToFileURL} from "node:url";
const root="C:\\semio\\🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\📚️library\\🏗️build\\";
const observation=await import(pathToFileURL(root+"👁️observation\\🟨️.mjs").href);
const path=process.argv[2],mode=process.argv[3]??"round",limit=Number(process.argv[4]??64),cap=Number(process.argv[5]??0);
const memo=JSON.parse(readFileSync(path,"utf8"));
let rows=memo.observations as unknown[];
if(cap>0)rows=rows.slice(0,cap);
const byKind=new Map<string,number>();
const t=performance.now();
let verified=0;
const ok=mode==="round"
 ?await observation.observationsHoldWithinRoundV1(rows,limit,undefined,(row:any)=>{byKind.set("stale:"+row.kind+":"+row.path.slice(0,120),1);},n=>{verified=n;})
 :await observation.observationsHold(rows,limit,new Set(),new Set(),undefined,(row:any)=>{byKind.set("stale:"+row.kind+":"+row.path.slice(0,120),1);},n=>{verified=n;});
console.log(JSON.stringify({ok,mode,limit,rows:rows.length,verified,ms:Math.round(performance.now()-t),stale:[...byKind.keys()].slice(0,10)}));
