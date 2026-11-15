import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const L="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const {openFactIndexV1}=await import(L+"🧮️fact-codec/🟨️.mjs");
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const raw=brotliDecompressSync(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),{maximumOutputLength:2**31} as never);
const index:any=openFactIndexV1(raw,{maximumBytes:raw.length});
const min=Number(process.argv[2]??1000);const owners=new Map<number,Set<number>>();
for(const cls of index.classes)for(const id of cls.r){if(index.rows[id].kind!=="file")continue;let s=owners.get(id);if(!s)owners.set(id,s=new Set());for(const o of cls.o)if(!index.owners[o].startsWith("@derived:"))s.add(o);}
const out=[...owners].filter(([,s])=>s.size>=min).sort((a,b)=>b[1].size-a[1].size);
console.log("generation",pointer.generation,"file rows held by >=",min,"owners:",out.length);
for(const [id,s] of out.slice(0,40))console.log(s.size,index.rows[id].path.replace(root+"/","").slice(0,150));
