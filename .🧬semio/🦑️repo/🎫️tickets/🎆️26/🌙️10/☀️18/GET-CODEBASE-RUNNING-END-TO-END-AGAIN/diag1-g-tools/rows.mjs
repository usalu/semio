import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const SRC=process.env.GEN??"/Users/ueli/Documents/semio/.cache/b/g/m/generations";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const c=await import(B+"🧮️fact-codec/🟨️.mjs");
const p=JSON.parse(readFileSync(SRC+"/current.json","utf8")),m=JSON.parse(readFileSync(SRC+"/m/"+p.manifest+".json","utf8"));
const raw=brotliDecompressSync(readFileSync(SRC+"/x/"+m.factIndex+".facts.br"));
const view=c.openFactIndexV1(raw);
console.log("rows",view.rows.length,"classes",view.classes.length,"owners",view.owners.length, Object.keys(view).join(","));
const want=process.argv[2],pat=process.argv[3]?new RegExp(process.argv[3]):null;
let oi=view.owners.indexOf(want);console.log("owner idx",oi);
let n=0;
for(const k of view.classes){ if(!k.o.includes(oi))continue; for(const id of k.r){const r=view.rows[id]; if(pat&&!pat.test(JSON.stringify(r)))continue; if(n++<(+process.env.MAX||40))console.log(JSON.stringify(r).slice(0,300));}}
console.log("matched",n);
