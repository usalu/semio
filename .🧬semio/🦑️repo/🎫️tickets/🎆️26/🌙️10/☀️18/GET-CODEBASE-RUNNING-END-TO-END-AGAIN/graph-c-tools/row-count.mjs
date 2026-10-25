import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const idx=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2])).toString()),needle=process.argv[3];
const ownersOf=new Map();idx.classes.forEach(c=>{for(const r of c.r){let s=ownersOf.get(r);if(!s)ownersOf.set(r,s=new Set());for(const o of c.o)s.add(o);}});
idx.rows.forEach((r,i)=>{if(r.path.includes(needle))console.log(i,r.kind,"owners",ownersOf.get(i)?.size??0, r.path.slice(-80));});
