import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const inference=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;inference.prepareSourceAnalysesV1();
const obs=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
const idx=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2])).toString());const step=Number(process.argv[3]??5);
const tally={};const bad=[];
for(let i=0;i<idx.rows.length;i+=step){const r=idx.rows[i];if(["field","digest","stamp"].includes(r.kind))continue;
 const ok=await obs.observationsHold([r],1,new Set(),new Set());const t=tally[r.kind]??={n:0,stale:0};t.n++;if(!ok){t.stale++;if(bad.length<25)bad.push(r.kind+" "+r.path.replace("/Users/ueli/Documents/semio/",""));}}
console.log(JSON.stringify(tally));console.log(bad.join("\n"));
