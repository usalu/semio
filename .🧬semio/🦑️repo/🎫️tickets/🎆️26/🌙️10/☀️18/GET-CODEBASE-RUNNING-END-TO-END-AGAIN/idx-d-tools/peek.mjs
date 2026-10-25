import{brotliDecompressSync}from"node:zlib";import{readFileSync}from"node:fs";
const v=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2]),{maxOutputLength:1<<30}).toString("utf8"));
for(const k of["analysis","file","readdir","resolve","closure","exists"]){const r=v.rows.filter(r=>r.kind===k);for(const i of[0,Math.floor(r.length/2)])console.log(JSON.stringify(r[i]).slice(0,900));}
