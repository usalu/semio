import {open} from "node:fs/promises";
import {streamJsonObjectV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🧠️admission-memo/🌊️json-stream/🟦️.ts";
import * as observation from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
const file=await open(process.argv[2]!,"r"),rows:any[]=[],owners:{suffix:string;rows:number[]}[]=[];
for await(const e of streamJsonObjectV1(file,new Set(["observations","owners"]))){
 if("element" in e&&e.key==="observations")rows.push(e.element);
 if("element" in e&&e.key==="owners")owners.push({suffix:(e.element as any).suffix,rows:(e.element as any).rows});
}
await file.close();
const staleRows:any[]=[],t0=performance.now();
const hold=await observation.observationsHold(rows,64,new Set(),new Set(),undefined,(row:any)=>{if(row.kind!=="global"&&row.kind!=="registry")staleRows.push(row);},undefined,true);
console.log("rows",rows.length,"owners",owners.length,"hold",hold,"ms",Math.round(performance.now()-t0),"stale rows",staleRows.length);
const staleKey=new Set(staleRows.map(r=>JSON.stringify([r.kind,r.path])));
const byRow=new Map<string,number>(),staleOwners=new Set<string>();
for(const o of owners)for(const i of o.rows){const k=JSON.stringify([rows[i].kind,rows[i].path]);if(staleKey.has(k)){staleOwners.add(o.suffix);byRow.set(k,(byRow.get(k)??0)+1);}}
console.log("owners with a stale row",staleOwners.size,"of",owners.length);
console.log([...byRow].sort((a,b)=>b[1]-a[1]).slice(0,25).map(([p,n])=>n+" "+p.replace(process.cwd()+"/","").slice(0,230)).join("\n"));
