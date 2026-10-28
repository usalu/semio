// [gram2-g] kinds, path-length and fact-size census of the distinct rows of a sealed generation index (read only, one decode ~350 MB). usage: bun index-kinds.mjs <gen-dir>
import{readFileSync}from"node:fs";import{join}from"node:path";
const D=process.argv[2];
const g=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
const kinds={};let pathBytes=0,factBytes=0;for(const r of index.rows){const k=kinds[r.kind]??={n:0,path:0,fact:0};k.n++;k.path+=r.path.length;const f=JSON.stringify(r.fact).length;k.fact+=f;pathBytes+=r.path.length;factBytes+=f;}
console.log("rows",index.rows.length,"owners",index.owners.length,"classes",index.classes.length,"pathBytes",pathBytes,"factJsonBytes",factBytes);
for(const [k,v] of Object.entries(kinds).sort((a,b)=>b[1].n-a[1].n))console.log(k.padEnd(12),String(v.n).padStart(8),"avgPath",(v.path/v.n).toFixed(0),"avgFactJson",(v.fact/v.n).toFixed(0));
const per=new Map();for(const c of index.classes)for(const o of c.o)per.set(o,(per.get(o)??0)+c.r.length);
const top=[...per].sort((a,b)=>b[1]-a[1]).slice(0,8).map(([o,n])=>index.owners[o]+":"+n);console.log("top owners by rows",top.join(" | "));
const dist=[...per.values()].sort((a,b)=>a-b);console.log("rows/owner p50",dist[dist.length>>1],"p90",dist[Math.floor(dist.length*.9)],"p99",dist[Math.floor(dist.length*.99)],"sum",dist.reduce((a,b)=>a+b,0));
console.log("rss MB",Math.round(process.memoryUsage().rss/1048576));
