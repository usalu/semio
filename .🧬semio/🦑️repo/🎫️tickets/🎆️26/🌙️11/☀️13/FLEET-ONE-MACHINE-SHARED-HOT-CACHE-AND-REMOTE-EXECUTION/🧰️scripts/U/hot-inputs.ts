import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {openFactIndexV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs";
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const raw=brotliDecompressSync(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),{maximumOutputLength:2**31} as never);
const index:any=openFactIndexV1(raw,{maximumBytes:raw.length});
const min=Number(process.argv[2]??1000);
const ownersOf=new Map<number,Set<number>>();
for(const cls of index.classes)for(const id of cls.r){let s=ownersOf.get(id);if(!s)ownersOf.set(id,s=new Set());for(const o of cls.o)s.add(o);}
const byKey=new Map<string,{owners:Set<number>;n:number}>();
for(let id=0;id<index.rows.length;id++){
 const row=index.rows[id];if(row.kind!=="analysis")continue;
 const p=row.path.startsWith(root+"/")?row.path.slice(root.length+1):row.path;
 const key=p+"\t"+(row.fact?.provider??"").replace(/:[0-9a-f]{16,}$/,"");
 let e=byKey.get(key);if(!e)byKey.set(key,e={owners:new Set(),n:0});e.n++;for(const o of ownersOf.get(id)??[])e.owners.add(o);
}
console.log("generation",pointer.generation,"owners",index.owners.length,"rows",index.rows.length);
for(const [k,e] of [...byKey].filter(([,e])=>e.owners.size>=min).sort((a,b)=>b[1].owners.size-a[1].owners.size).slice(0,Number(process.argv[3]??60)))console.log(e.owners.size,e.n,k);
