import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const j=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2])).toString());
const t={};for(const r of j.rows)if(["exists","fileExists","directoryExists"].includes(r.kind)){const k=r.kind+":"+r.fact;t[k]=(t[k]||0)+1}console.log(t);
