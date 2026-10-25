import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const j=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2])).toString());
console.log(Object.keys(j),j.capturedAt,j.directories.length,j.members.length,JSON.stringify(j.directories.slice(0,4)),JSON.stringify(j.directories.filter(r=>r[0].split("/").length<2).slice(0,6)));
