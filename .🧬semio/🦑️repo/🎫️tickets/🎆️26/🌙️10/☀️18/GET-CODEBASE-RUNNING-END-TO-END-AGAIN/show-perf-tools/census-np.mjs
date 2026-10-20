import {readFileSync,readdirSync,statSync} from "node:fs";import {brotliDecompressSync} from "node:zlib";import {join} from "node:path";
const dir="/Users/ueli/Documents/semio/.cache/b/g/m/inference/np",files=readdirSync(dir).filter(f=>f.endsWith(".json"));
const rows=[];
for(const f of files){const s=statSync(join(dir,f));let text;try{text=brotliDecompressSync(readFileSync(join(dir,f))).toString("utf8");}catch{continue;}const nl=text.indexOf("\n"),h=JSON.parse(text.slice(0,nl));const kinds={};for(const r of h.observations)kinds[r.kind]=(kinds[r.kind]??0)+1;rows.push({f,mtime:s.mtimeMs,size:s.size,rows:h.observations.length,kinds,result:text.length-nl});}
const recent=rows.filter(r=>r.mtime>=Date.parse("2026-10-20T06:30:00+02:00")),old=rows.filter(r=>r.mtime<Date.parse("2026-10-19T00:00:00+02:00"));
const med=a=>[...a].sort((x,y)=>x-y)[a.length>>1];
const agg=(set,name)=>{const k={};for(const r of set)for(const [a,b] of Object.entries(r.kinds))k[a]=(k[a]??0)+b;console.log(name,"result chars median",med(set.map(r=>r.result)),"p90",[...set.map(r=>r.result)].sort((x,y)=>x-y)[Math.floor(set.length*.9)],"sum MB",Math.round(set.reduce((a,r)=>a+r.result,0)/1e6),set.length,"median rows",med(set.map(r=>r.rows)),"median bytes",med(set.map(r=>r.size)),"p90 rows",[...set.map(r=>r.rows)].sort((x,y)=>x-y)[Math.floor(set.length*.9)],"kinds/avg",JSON.stringify(Object.fromEntries(Object.entries(k).map(([a,b])=>[a,Math.round(b/set.length)]))));};
agg(old,"old(10-18)");agg(recent,"recent(10-20)");
