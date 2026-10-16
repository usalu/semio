/** 🧮️ Aggregates the imported symbols per framework target from a scan output. Read-only. */
import { readFileSync, writeFileSync } from "node:fs";
const ROOT="/Users/ueli/Documents/semio";
const [scanPath,outPath]=process.argv.slice(2);
const edges=JSON.parse(readFileSync(scanPath,"utf8"));
const sdk=JSON.parse(readFileSync(ROOT+"/🧰️framework/📦️packages/🟦️typescript/package.json","utf8"));
const allowed=new Set(Object.values(sdk.exports).map((t:any)=>"🧰️framework/📦️packages/🟦️typescript/"+t.slice(2)));
const cache=new Map<string,string>();
const agg=new Map<string,{files:Set<string>,names:Map<string,number>,star:number,def:number,sideEffect:number,other:number}>();
for(const e of edges){
  if(allowed.has(e.target))continue;
  let src=cache.get(e.file);if(src===undefined){src=readFileSync(ROOT+"/"+e.file,"utf8");cache.set(e.file,src)}
  const a=agg.get(e.target)??{files:new Set(),names:new Map(),star:0,def:0,sideEffect:0,other:0};agg.set(e.target,a);
  a.files.add(e.file);
  const esc=e.spec.replace(/[.*+?^${}()|[\]\\]/g,"\\$&");
  const re=new RegExp("(import|export)\\s+(type\\s+)?([^;]*?)\\s*from\\s*[\"']"+esc+"[\"']","gs");
  let m,found=false;
  const lineStart=src.split("\n").slice(0,e.line-1).join("\n").length;
  while((m=re.exec(src))){
    const clause=m[3];
    const at=src.slice(0,m.index).split("\n").length;
    if(Math.abs(at-e.line)>40)continue;
    found=true;
    const inb=clause.match(/\{([^}]*)\}/s);
    if(clause.includes("*"))a.star++;
    if(inb)for(const raw of inb[1].split(",")){const n=raw.trim().replace(/^type\s+/,"").split(/\s+as\s+/)[0].trim();if(n)a.names.set(n,(a.names.get(n)||0)+1)}
    const dm=clause.replace(/\{[^}]*\}/s,"").replace(/,/g,"").trim();
    if(dm&&!dm.startsWith("*"))a.def++;
    break;
  }
  if(!found){const ctx=src.split("\n")[e.line-1]??"";if(/^\s*import\s*["']/.test(ctx))a.sideEffect++;else a.other++}
}
const out:any[]=[];
for(const [t,a] of agg)out.push({target:t,files:a.files.size,star:a.star,def:a.def,side:a.sideEffect,other:a.other,names:[...a.names].sort((x,y)=>y[1]-x[1])});
out.sort((x,y)=>y.files-x.files);
writeFileSync(outPath,JSON.stringify(out,null,1));
for(const o of out)console.log(String(o.files).padStart(4),o.target.replace("🧰️framework/",""),"| star",o.star,"def",o.def,"other",o.other+o.side,"|",o.names.slice(0,30).map((n:any)=>n[0]).join(","));
