// [walk-g] readdir rows of named owners of a sealed generation, grouped by path prefix (read only). usage: bun readdir-census.mjs <gen-dir> <owner>...
import{readFileSync}from"node:fs";import{join}from"node:path";
const D=process.argv[2],WANT=process.argv.slice(3),L="/Users/ueli/Documents/semio/";
const g=await import(L+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
for(const name of WANT){const at=index.owners.indexOf(name);if(at<0){console.log(name,"not found");continue;}
 const by={},kinds={};let n=0;
 for(const c of index.classes){if(!c.o.includes(at))continue;for(const id of c.r){const r=index.rows[id];kinds[r.kind]=(kinds[r.kind]??0)+1;if(r.kind!=="readdir")continue;n++;const rel=r.path.startsWith(L)?r.path.slice(L.length):r.path;const key=rel.split("/").slice(0,2).join("/");by[key]=(by[key]??0)+1;}}
 console.log(name,"kinds",JSON.stringify(kinds));console.log("readdir",n);console.log(Object.entries(by).sort((a,b)=>b[1]-a[1]).slice(0,25).map(([k,v])=>v+"\t"+k).join("\n"));}
console.log("rss MB",Math.round(process.memoryUsage().rss/1048576));
