import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {openFactIndexV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs";
import {CUTOFF_POLICY_V1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs";
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const raw=brotliDecompressSync(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),{maximumOutputLength:2**31} as never);
const index:any=openFactIndexV1(raw,{maximumBytes:raw.length});
const needle=process.argv[2]!;
const prefix=CUTOFF_POLICY_V1.ownerPrefix;
const kinds=new Map<string,number>();const real=new Set<number>(),pseudo=new Set<number>();
const rowIds=new Set<number>();
for(let id=0;id<index.rows.length;id++){const r=index.rows[id];if(r.path.includes(needle)){rowIds.add(id);kinds.set(r.kind+" "+(r.fact?.provider??"").replace(/:[0-9a-f]{16,}$/,""),(kinds.get(r.kind+" "+(r.fact?.provider??"").replace(/:[0-9a-f]{16,}$/,""))??0)+1);}}
for(const cls of index.classes){if(!cls.r.some((id:number)=>rowIds.has(id)))continue;for(const o of cls.o)(index.owners[o].startsWith(prefix)?pseudo:real).add(o);}
console.log("needle",needle,"rows",rowIds.size,"real owners",real.size,"pseudo owners",pseudo.size,"prefix",prefix);
console.log([...kinds]);
console.log([...real].slice(0,5).map(o=>index.owners[o]));
