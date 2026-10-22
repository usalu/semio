import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const file=process.argv[2];const idx=JSON.parse(brotliDecompressSync(readFileSync(file)).toString());
const by=new Map();const kinds=new Map();
for(const r of idx.rows){kinds.set(r.kind,(kinds.get(r.kind)??0)+1);if(r.kind==="analysis"){const k=r.fact.provider+" "+r.fact.revision.slice(0,12);by.set(k,(by.get(k)??0)+1);}}
console.log(file.slice(-30),"rows",idx.rows.length,Object.fromEntries(kinds));for(const[k,v]of by)console.log(" ",k,v);
