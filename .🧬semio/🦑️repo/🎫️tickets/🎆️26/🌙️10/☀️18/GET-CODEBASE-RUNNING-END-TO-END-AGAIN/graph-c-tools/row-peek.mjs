import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{createHash}from"node:crypto";
const idx=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2])).toString()),needle=process.argv[3];
idx.rows.forEach((r,i)=>{if(r.path.includes(needle)){console.log(i,r.kind,JSON.stringify(r.fact).slice(0,300),r.path.slice(-120));}});
