import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const ld=f=>JSON.parse(brotliDecompressSync(readFileSync(f)).toString());
const a=ld(process.argv[2]),b=ld(process.argv[3]);
const key=r=>r.kind+"\0"+r.path;
const ma=new Map(),mb=new Map();for(const r of a.rows)ma.set(key(r),r);for(const r of b.rows)mb.set(key(r),r);
let n=0;for(const[k,r]of ma){const w=mb.get(k);if(w&&JSON.stringify(w.fact)!==JSON.stringify(r.fact)&&r.kind==="analysis"&&n++<3){console.log(k.slice(0,120));const f=r.fact,g=w.fact;for(const x of Object.keys(f))if(JSON.stringify(f[x])!==JSON.stringify(g[x]))console.log(" ",x,JSON.stringify(f[x]).slice(0,200),"=>",JSON.stringify(g[x]).slice(0,200));}}
const added=[...mb.keys()].filter(k=>!ma.has(k)&&k.startsWith("analysis"));
const byProv={};for(const k of added){const p=mb.get(k).fact.provider.split(":").slice(0,3).join(":");byProv[p]=(byProv[p]||0)+1}console.log(byProv);
const dirs={};for(const k of added){const p=k.split("\0")[1].replace(/\/Users\/ueli\/Documents\/semio\//,"").split("/").slice(0,4).join("/");dirs[p]=(dirs[p]||0)+1}
console.log(Object.entries(dirs).sort((x,y)=>y[1]-x[1]).slice(0,10));
console.log(added.slice(0,5).map(k=>k.slice(0,150)));
