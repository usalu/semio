import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const root=process.env.SEMIO_ROOT;const{decodeFactIndexV1}=await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs");
for(const f of process.argv.slice(2)){const e=brotliDecompressSync(readFileSync(f));const s=decodeFactIndexV1(e,{maximumBytes:e.length,build:false}).strings;console.log(f.slice(-40),e.length,"strings",s.length,"nodes",2*s.length+1,"OVER",2*s.length+1>4194304);}
