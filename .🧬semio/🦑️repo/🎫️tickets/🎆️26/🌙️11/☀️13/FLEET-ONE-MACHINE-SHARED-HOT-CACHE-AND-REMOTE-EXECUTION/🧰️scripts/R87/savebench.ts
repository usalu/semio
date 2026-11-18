import{readdirSync,readFileSync}from"node:fs";import{brotliDecompressSync,brotliCompressSync,constants}from"node:zlib";import{createHash}from"node:crypto";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",ce=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/ce";
const OB=await import(B+"👁️observation/🟨️.mjs");const rowJson=(OB as any).observationRowJsonV1;
const names=readdirSync(ce+"/e").filter(n=>n.endsWith(".json.br"));
let ents=0,layers=0,rows=0,tDigest=0,tParse=0,tBrotli=0,bytesOut=0;
for(let i=0;i<names.length&&ents<6;i+=Math.floor(names.length/6)){
 const e=JSON.parse(brotliDecompressSync(readFileSync(ce+"/e/"+names[i])).toString("utf8"));ents++;
 for(const d of e.layers.slice(0,229)){
  let t=performance.now();const arr=JSON.parse(brotliDecompressSync(readFileSync(ce+"/l/"+d+".json.br")).toString("utf8"));tParse+=performance.now()-t;
  t=performance.now();const h=createHash("sha256");h.update("[");for(let a=0;a<arr.length;a++){if(a>0)h.update(",");h.update(rowJson(arr[a]));}h.update("]");h.digest("hex");tDigest+=performance.now()-t;
  t=performance.now();const text="["+arr.map(rowJson).join(",")+"]";const out=brotliCompressSync(Buffer.from(text),{params:{[constants.BROTLI_PARAM_QUALITY]:5}});tBrotli+=performance.now()-t;bytesOut+=out.length;
  layers++;rows+=arr.length;
 }}
console.log(JSON.stringify({entries:ents,layers,rowsPerLayer:rows/layers,digestMsPerEntry:tDigest/ents,brotliQ5MsPerEntry:tBrotli/ents,readParseMsPerEntry:tParse/ents}));
process.exit(0);
