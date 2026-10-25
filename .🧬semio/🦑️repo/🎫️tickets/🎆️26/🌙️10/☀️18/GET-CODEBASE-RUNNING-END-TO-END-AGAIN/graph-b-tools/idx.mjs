import{brotliDecompressSync}from"node:zlib";import{readFileSync}from"node:fs";
const f=process.argv[2];const j=JSON.parse(brotliDecompressSync(readFileSync(f)).toString());
console.log(Object.keys(j));for(const k of Object.keys(j)){const v=j[k];console.log(k,Array.isArray(v)?"array"+v.length:typeof v==="object"?"obj"+Object.keys(v).length:v);}
console.log(JSON.stringify(j).slice(0,1500));
