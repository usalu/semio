import{brotliDecompressSync,brotliCompressSync,constants as z}from"node:zlib";import{readFileSync}from"node:fs";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const{encodeFactIndexV1,decodeFactIndexV1,canonicalJsonV1}=await import(L+"🧮️fact-codec/🟨️.mjs");
const v=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2]),{maxOutputLength:1<<30}).toString("utf8"));
let t=performance.now();const raw=encodeFactIndexV1(v,{maximumBytes:1<<29});console.log("encode ms",(performance.now()-t)|0,"raw",raw.length);
for(const [q,w] of [[5,22],[5,24],[9,24]]){t=performance.now();const p=brotliCompressSync(raw,{params:{[z.BROTLI_PARAM_QUALITY]:q,[z.BROTLI_PARAM_LGWIN]:w,[z.BROTLI_PARAM_SIZE_HINT]:raw.length}});console.log("brotli q",q,"w",w,p.length,"ms",(performance.now()-t)|0);}
t=performance.now();const d=decodeFactIndexV1(raw);console.log("decode ms",(performance.now()-t)|0);
t=performance.now();const s=decodeFactIndexV1(raw,{build:false});console.log("scan ms",(performance.now()-t)|0,"strings",s.strings.length);
const a=canonicalJsonV1(v),b=canonicalJsonV1(d);console.log("canonical equal",a===b,a.length);
const raw2=encodeFactIndexV1(d,{maximumBytes:1<<29});console.log("re-encode identical",Buffer.compare(raw,raw2)===0);
