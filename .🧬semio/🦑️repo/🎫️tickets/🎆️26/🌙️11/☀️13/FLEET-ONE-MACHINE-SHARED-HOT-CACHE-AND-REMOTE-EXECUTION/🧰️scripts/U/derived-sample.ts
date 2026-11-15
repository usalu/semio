import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {openFactIndexV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs";
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const raw=brotliDecompressSync(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),{maximumOutputLength:2**31} as never);
const index:any=openFactIndexV1(raw,{maximumBytes:raw.length});
let n=0;const kinds=new Map<string,number>();
for(let id=0;id<index.rows.length;id++){const r=index.rows[id];kinds.set(r.kind,(kinds.get(r.kind)??0)+1);if(r.kind==="derived"&&n<3&&JSON.stringify(r.fact).length<3000){n++;console.log(JSON.stringify({path:r.path,fact:r.fact}).slice(0,1400));}}
console.log([...kinds].sort((a,b)=>b[1]-a[1]).map(([k,v])=>k+"="+v).join(" "));
