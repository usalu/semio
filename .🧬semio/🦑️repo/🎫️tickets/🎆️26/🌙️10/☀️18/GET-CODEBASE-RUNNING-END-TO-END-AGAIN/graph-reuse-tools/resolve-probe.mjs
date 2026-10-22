import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const inference=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;inference.prepareSourceAnalysesV1();
const obs=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
const idx=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2])).toString());let shown=0;
for(const r of idx.rows){if(r.kind!=="resolve")continue;if(!r.path.includes("@iarna/toml")&&!r.path.includes("next/package.json"))continue;
 if(await obs.observationsHold([r],1,new Set(),new Set()))continue;console.log(JSON.stringify(r).slice(0,500));if(++shown>=3)break;}
