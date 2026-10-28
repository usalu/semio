/** 🗜 Read-only: pack and unpack cost of the live fact index at several brotli qualities (best of 3). */
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{brotliCompressSync,brotliDecompressSync,constants as Z}from"node:zlib";
const store=join(process.argv[2],".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8")),man=JSON.parse(readFileSync(join(store,"m",cur.manifest+".json"),"utf8"));
const raw=brotliDecompressSync(readFileSync(join(store,"x",man.factIndex+".facts.br")),{maxOutputLength:536870912});
const best=f=>{let m=1e9,v;for(let i=0;i<3;i++){const a=performance.now();v=f();m=Math.min(m,performance.now()-a);}return[m,v];};
for(const q of [1,2,3,4,5]){const[c,b]=best(()=>brotliCompressSync(raw,{params:{[Z.BROTLI_PARAM_QUALITY]:q,[Z.BROTLI_PARAM_LGWIN]:22,[Z.BROTLI_PARAM_SIZE_HINT]:raw.length}}));const[d]=best(()=>brotliDecompressSync(b,{maxOutputLength:536870912}));console.log(JSON.stringify({quality:q,packMs:Math.round(c),unpackMs:Math.round(d),bytes:b.length,raw:raw.length}));}
