import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const ld=f=>JSON.parse(brotliDecompressSync(readFileSync(f)).toString());
const a=ld(process.argv[2]),b=ld(process.argv[3]);
const key=r=>r.kind+"\0"+r.path;
const ma=new Map(),mb=new Map();for(const r of a.rows)ma.set(key(r),JSON.stringify(r.fact));for(const r of b.rows)mb.set(key(r),JSON.stringify(r.fact));
const changed=[],gone=[],added=[];
for(const[k,v]of ma){const w=mb.get(k);if(w===undefined)gone.push(k);else if(w!==v)changed.push(k);}
for(const k of mb.keys())if(!ma.has(k))added.push(k);
const kinds=l=>{const t={};for(const k of l){const kind=k.split("\0")[0];t[kind]=(t[kind]||0)+1}return t};
console.log("rows",ma.size,mb.size,"changed",changed.length,kinds(changed),"gone",gone.length,kinds(gone),"added",added.length,kinds(added));
for(const k of changed.slice(0,40))console.log("CHG",k.replace("\0"," ").replace(/\/Users\/ueli\/Documents\/semio\//g,"").slice(0,200));
