import{readFileSync,readdirSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const observation=await import(B+"👁️observation/🟨️.mjs");
const ce=process.argv[2],prefix=process.argv[3];
const n=readdirSync(join(ce,"e")).find(x=>x.startsWith(prefix));
const layer=d=>JSON.parse(brotliDecompressSync(readFileSync(join(ce,"l",d+".json.br"))).toString("utf8"));
const e=JSON.parse(brotliDecompressSync(readFileSync(join(ce,"e",n))).toString("utf8"));
const rows=[...e.rows];for(const d of new Set(e.layers))rows.push(...layer(d));
const groups={};let k=0;
for(const r of rows){let fresh=true;try{fresh=observation.observationRowFreshV1(r);}catch{}
 if(fresh)continue;const key=r.kind+(r.kind==="analysis"?" "+String(r.fact?.provider).slice(0,40)+" rev "+String(r.fact?.revision).slice(0,16):"");
 const g=groups[key]??={n:0,ex:[]};g.n++;if(g.ex.length<2)g.ex.push(JSON.stringify(r).slice(0,420));}
for(const[key,g]of Object.entries(groups))console.log(g.n,key,"\n  ",g.ex.join("\n   "));
