import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {openFactIndexV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs";
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const raw=brotliDecompressSync(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),{maximumOutputLength:2**31} as never);
const index:any=openFactIndexV1(raw,{maximumBytes:raw.length});
const name=process.argv[2]!;const o=index.owners.indexOf(name);
console.log("owner",name,o);
const rowIds=new Set<number>();
for(const cls of index.classes)if(cls.o.includes(o))for(const id of cls.r)rowIds.add(id);
const kinds=new Map<string,number>();let tool=0;
for(const id of rowIds){const r=index.rows[id];const k=r.kind+" "+(r.fact?.provider??"").replace(/:[0-9a-f]{16,}$/,"");kinds.set(k,(kinds.get(k)??0)+1);if(r.path.includes("/🔨️modules/")||r.path.includes("🦑️repo"))tool++;}
console.log("rows",rowIds.size,"toolish",tool);console.log([...kinds].sort((a,b)=>b[1]-a[1]).slice(0,30));
let n=0;for(const id of rowIds){const r=index.rows[id];if(r.kind==="closure"||r.kind==="derived"){if(n++<12)console.log(JSON.stringify(r).slice(0,500));}}
const ex=[...rowIds].map(id=>index.rows[id]).filter(r=>r.path.includes("unit-cache/📜️script.ts"));
for(const r of ex)console.log(JSON.stringify(r).slice(0,400));
