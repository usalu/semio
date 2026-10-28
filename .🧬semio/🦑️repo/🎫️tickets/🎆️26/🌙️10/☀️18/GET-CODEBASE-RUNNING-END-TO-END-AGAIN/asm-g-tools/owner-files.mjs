// [asm-g] lists the file/analysis rows an owner holds (to pick a file to "touch"). usage: bun owner-files.mjs <gen-dir> <owner> [limit]
import{readFileSync}from"node:fs";import{join}from"node:path";
const D=process.argv[2],NAME=process.argv[3],LIM=Number(process.argv[4]??"15"),E="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const g=await import(E),pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
const id=index.owners.indexOf(NAME),out=[];let consumersOf=new Map();
for(const c of index.classes)if(c.o.includes(id))for(const r of c.r){const row=index.rows[r];if(row.kind==="file"||row.kind==="analysis")out.push([row.kind,row.path.replace("/Users/ueli/Documents/semio/",""),c.o.length]);}
console.log(NAME,"file/analysis rows",out.length);for(const r of out.sort((a,b)=>a[2]-b[2]).slice(0,LIM))console.log(r.join("  "));
