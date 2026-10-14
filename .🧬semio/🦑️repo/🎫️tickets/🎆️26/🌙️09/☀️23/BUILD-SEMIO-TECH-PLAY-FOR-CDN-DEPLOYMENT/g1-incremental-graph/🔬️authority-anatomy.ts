/** 🧭️ Splits a persisted project record's rows into load-time authority facts and the project's own facts, by kind and path family. */
import {readFileSync} from "node:fs";
const path=process.argv[2]!,text=readFileSync(path,"utf8"),header=JSON.parse(text.slice(0,text.indexOf("\n")));
const key=(row:any)=>JSON.stringify([row.kind,row.path,row.fact]),authority=new Set((header.authorityObservations as any[]).map(key));
const own=(header.observations as any[]).filter(row=>!authority.has(key(row))),load=header.authorityObservations as any[];
const summarize=(rows:any[])=>{const kinds:Record<string,number>={};const fam=new Map<string,number>();for(const row of rows){kinds[row.kind]=(kinds[row.kind]??0)+1;const p=String(row.kind==="resolve"?JSON.parse(row.path)[1]:row.path).replace("/Users/ueli/Documents/semio/","");const k=row.kind+" "+p.split("/").slice(0,6).join("/");fam.set(k,(fam.get(k)??0)+1);}return {count:rows.length,kinds,families:[...fam].sort((a,b)=>b[1]-a[1]).slice(0,25)};};
console.log(JSON.stringify({load:summarize(load),own:summarize(own),ownFiles:own.filter(r=>r.kind==="file").map(r=>r.path.replace("/Users/ueli/Documents/semio/","")).slice(0,40),ownAnalysis:own.filter(r=>r.kind==="analysis").length},null,1));
