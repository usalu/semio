import{brotliDecompressSync}from"node:zlib";import{readFileSync}from"node:fs";
const j=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2])).toString());
console.log(JSON.stringify(j.rows.slice(0,3)),JSON.stringify(j.owners.slice(0,3)));
const kinds={};for(const r of j.rows){const k=Array.isArray(r)?r[0]:r.kind;kinds[k]=(kinds[k]||0)+1}console.log(kinds);
for(const r of j.rows){const s=JSON.stringify(r);if(s.includes("stamp")||s.includes("inferrer:"))console.log(s.slice(0,300))}
