import{readFileSync,readdirSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const observation=await import(B+"👁️observation/🟨️.mjs");
const si=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
console.log("exports",Object.keys(si).filter(k=>/prepare|Analys/.test(k)));
const ce=process.argv[2],prefix=process.argv[3];
const n=readdirSync(join(ce,"e")).find(x=>x.startsWith(prefix));
const layer=d=>JSON.parse(brotliDecompressSync(readFileSync(join(ce,"l",d+".json.br"))).toString("utf8"));
const e=JSON.parse(brotliDecompressSync(readFileSync(join(ce,"e",n))).toString("utf8"));
const rows=[...e.rows];for(const d of new Set(e.layers))rows.push(...layer(d));
const why={};
for(const r of rows){let fresh=true;try{fresh=observation.observationRowFreshV1(r);}catch{}
 if(fresh)continue;
 if(r.kind==="analysis"){const f=await observation.firstFailingRowV1([r],{});const w=(f?.why??"?").replace(/[0-9a-f]{12,}/g,"<h>").slice(0,80);why[w]=(why[w]??0)+1;}else why[r.kind]=(why[r.kind]??0)+1;}
console.log(why);
