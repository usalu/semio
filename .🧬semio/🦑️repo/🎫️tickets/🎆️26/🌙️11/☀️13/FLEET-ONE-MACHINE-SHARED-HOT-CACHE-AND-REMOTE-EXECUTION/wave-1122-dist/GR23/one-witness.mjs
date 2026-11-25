import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const dir=process.argv[2],head=JSON.parse(readFileSync(join(dir,"head.json"),"utf8"));
const d=head.chunks.find(Boolean);const c=JSON.parse(brotliDecompressSync(readFileSync(join(dir,"c",d+".json.br"))).toString());
const r=c.records.find(r=>r[2]==="module");console.log(JSON.stringify(r).slice(0,4000));
const w=r[5].witness;for(const k of Object.keys(w))console.log(k,JSON.stringify(w[k]).length,Array.isArray(w[k])?w[k].length:typeof w[k]);
