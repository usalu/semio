import {readFileSync,statSync} from "node:fs";
import {createHash} from "node:crypto";
import {brotliDecompressSync} from "node:zlib";
import {openFactIndexV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs";
import {CUTOFF_POLICY_V1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs";
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const raw=brotliDecompressSync(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),{maximumOutputLength:2**31} as never);
const index:any=openFactIndexV1(raw,{maximumBytes:raw.length});
const prefix=CUTOFF_POLICY_V1.ownerPrefix;
const rowOwners=new Map<number,Set<number>>();
for(const cls of index.classes)for(const id of cls.r){let s=rowOwners.get(id);if(!s)rowOwners.set(id,s=new Set());for(const o of cls.o)s.add(o);}
const changed=new Map<string,Set<number>>();let rows=0,checked=0;
for(let id=0;id<index.rows.length;id++){
 const r=index.rows[id];if(r.kind!=="analysis")continue;rows++;
 let st;try{st=statSync(r.path);}catch{st=undefined;}
 const src=r.fact?.source;
 if(st&&src&&st.size===src.size&&st.mtimeMs===src.mtimeMs)continue;
 checked++;
 let now:string|undefined;try{now=createHash("sha256").update(readFileSync(r.path)).digest("hex");}catch{now=undefined;}
 if(now===src?.sha256)continue;
 const p=r.path.startsWith(root+"/")?r.path.slice(root.length+1):r.path;
 let e=changed.get(p);if(!e)changed.set(p,e=new Set());
 for(const o of rowOwners.get(id)??[])if(!index.owners[o].startsWith(prefix))e.add(o);
}
console.log("generation",pointer.generation,"analysis rows",rows,"stamp-different",checked,"changed files",changed.size);
const all=new Set<number>();for(const e of changed.values())for(const o of e)all.add(o);
console.log("distinct real owners holding a changed analysis directly",all.size);
for(const [p,e] of [...changed].sort((a,b)=>b[1].size-a[1].size).slice(0,25))console.log(e.size,p);
