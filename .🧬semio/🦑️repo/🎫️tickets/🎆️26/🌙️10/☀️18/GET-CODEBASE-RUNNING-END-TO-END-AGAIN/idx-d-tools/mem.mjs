import{brotliDecompressSync,brotliCompressSync}from"node:zlib";import{readFileSync}from"node:fs";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");const M=await import(L+"🧠️memory/🟨️.mjs");
const mode=process.argv[3],path=process.argv[2];
const gc=()=>{Bun.gc(true);};
if(mode==="new"){ // build the new packed file from the old once
 const v=JSON.parse(brotliDecompressSync(readFileSync(path),{maxOutputLength:1<<30}).toString("utf8"));
 const{bytes}=G.packFactIndexV1(v,1<<29);require("node:fs").writeFileSync("/private/tmp/claude-501/-Users-ueli-Documents-semio/3d0a4ac3-5d70-4c6b-91df-262fe6307e4a/scratchpad/new.facts.br",bytes);process.exit(0);}
gc();const base=process.memoryUsage();const t=performance.now();let idx;
if(mode==="old"){idx=JSON.parse(brotliDecompressSync(readFileSync(path),{maxOutputLength:1<<30}).toString("utf8"));M.internJsonStringsV1(idx,M.graphStringInternerV1);}
else idx=G.unpackFactIndexV1(readFileSync(path),1<<29);
const ms=performance.now()-t;const peak=process.memoryUsage();gc();const after=process.memoryUsage();
console.log(mode,"ms",ms|0,"rows",idx.rows.length,"peakRSS MB",(peak.rss/1048576)|0,"heap after gc MB",((after.heapUsed-base.heapUsed)/1048576)|0,"rss after MB",(after.rss/1048576)|0);
