import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const {openFactIndexV1}=await import(B+"🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs");
const {rowDependenciesV1}=await import(B+"🕸️graph/🕰️generations/🔎️doubt/🟨️.mjs");
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const raw=brotliDecompressSync(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),{maxOutputLength:2**31});
const index:any=openFactIndexV1(raw,{maximumBytes:raw.length});
let t=performance.now();let n=0;
const exclude=new Set<string>();
for(let id=0;id<index.rows.length;id++){const row=index.rows[id];const own=new Set<string>(),paths=new Set<string>();rowDependenciesV1(row,own,paths);if(own.size)n++;}
console.log("settled-like loop ms",Math.round(performance.now()-t),"rows with dirs",n);
t=performance.now();for(let id=0;id<index.rows.length;id++){index.rowKindOf(id);index.rowPathIdOf(id);}console.log("ids-only loop ms",Math.round(performance.now()-t));
