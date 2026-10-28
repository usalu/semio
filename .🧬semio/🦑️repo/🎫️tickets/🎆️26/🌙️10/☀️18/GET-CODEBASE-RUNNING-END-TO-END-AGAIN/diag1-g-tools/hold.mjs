import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const SRC=process.argv[2];
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const c=await import(B+"🧮️fact-codec/🟨️.mjs");
const p=JSON.parse(readFileSync(SRC+"/current.json","utf8")),m=JSON.parse(readFileSync(SRC+"/m/"+p.manifest+".json","utf8"));
const view=c.openFactIndexV1(brotliDecompressSync(readFileSync(SRC+"/x/"+m.factIndex+".facts.br")));
const holders=new Map(),kinds={};
for(const k of view.classes){let hit=false;for(const id of k.r){const r=view.rows[id];if(String(r.path).includes("🪶️extent/🟦️.ts")){hit=true;kinds[r.kind]=(kinds[r.kind]||0)+1;}}if(hit)for(const o of k.o){const n=view.owners[o];holders.set(n,(holders.get(n)||0)+1);}}
const un=Object.entries(m.owners).filter(([n,r])=>r.kind==="unavailable"&&!r.tag).map(([n])=>n);
console.log("gen",p.generation,"unavailable(non-tag)",un.length,"holders of facade rows",holders.size,"row kinds",JSON.stringify(kinds));
console.log("unavailable w/o facade rows:",un.filter(n=>!holders.has(n)).length,"| holders that are not unavailable:",[...holders.keys()].filter(n=>!un.includes(n)&&!n.startsWith("test-")).slice(0,5));
