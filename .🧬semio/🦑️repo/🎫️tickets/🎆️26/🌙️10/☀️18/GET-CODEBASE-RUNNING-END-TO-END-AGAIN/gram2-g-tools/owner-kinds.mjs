// [gram2-g] row kinds and shared/private split of named owners of a sealed generation index (read only). usage: bun owner-kinds.mjs <gen-dir> <owner>...
import{readFileSync}from"node:fs";import{join}from"node:path";
const D=process.argv[2],WANT=process.argv.slice(3);
const g=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
for(const name of WANT){const at=index.owners.indexOf(name);if(at<0){console.log(name,"not found");continue;}
 const kinds={};let rows=0,priv=0;
 for(const c of index.classes){if(!c.o.includes(at))continue;for(const id of c.r){const r=index.rows[id];const k=kinds[r.kind]??={n:0,priv:0};k.n++;rows++;if(c.o.length===1){k.priv++;priv++;}}}
 console.log(name,"rows",rows,"private",priv,Object.entries(kinds).sort((a,b)=>b[1].n-a[1].n).map(([k,v])=>k+":"+v.n+"("+v.priv+")").join(" "));}
console.log("rss MB",Math.round(process.memoryUsage().rss/1048576));
