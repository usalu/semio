import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const b=readFileSync(process.argv[2]);const t=performance.now();const raw=brotliDecompressSync(b);const j=JSON.parse(raw.toString());console.log("br",b.length,"raw",raw.length,"rows",j.rows.length,"parse ms",Math.round(performance.now()-t));
