import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const inference=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;inference.prepareSourceAnalysesV1();
const obs=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
const idx=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2])).toString());
const ids=process.argv.slice(3).map(Number);
for(const id of ids){const r=idx.rows[id];console.log(id,r.kind,r.path.slice(-60),"alone:",await obs.observationsHold([r],64,new Set(),true));}
console.log("together",await obs.observationsHold(ids.map(i=>idx.rows[i]),64,new Set(),true));
