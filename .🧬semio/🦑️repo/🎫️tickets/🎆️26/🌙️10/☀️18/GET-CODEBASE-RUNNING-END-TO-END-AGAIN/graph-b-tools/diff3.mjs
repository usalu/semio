import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const ld=f=>JSON.parse(brotliDecompressSync(readFileSync(f)).toString());
const a=ld(process.argv[2]),b=ld(process.argv[3]);
const key=r=>r.kind+"\0"+r.path+(r.kind==="analysis"?"\0"+r.fact.provider.replace(/:[0-9a-f]{64}$/,""):"");
const ma=new Map(),mb=new Map();for(const r of a.rows)ma.set(key(r),r);for(const r of b.rows)mb.set(key(r),r);
const short=p=>p.replace(/\/Users\/ueli\/Documents\/semio\//g,"");
const changed=[],gone=[],added=[];
for(const[k,r]of ma){const w=mb.get(k);if(!w)gone.push(k);else if(JSON.stringify(w.fact)!==JSON.stringify(r.fact)){changed.push([k,r,w]);}}
for(const k of mb.keys())if(!ma.has(k))added.push(k);
const kinds=l=>{const t={};for(const k of l){const kind=(Array.isArray(k)?k[0]:k).split("\0")[0];t[kind]=(t[kind]||0)+1}return t};
console.log("changed",changed.length,kinds(changed),"gone",gone.length,kinds(gone),"added",added.length,kinds(added));
const sameSrc=changed.filter(([k,r,w])=>r.kind==="analysis"&&r.fact.source.sha256===w.fact.source.sha256);console.log("analysis changed with same source sha",sameSrc.length,"differing source sha",changed.filter(([k,r,w])=>r.kind==="analysis"&&r.fact.source.sha256!==w.fact.source.sha256).length);
for(const [k,r,w] of changed.filter(([k,r,w])=>r.kind==="analysis"&&r.fact.source.sha256!==w.fact.source.sha256).slice(0,40))console.log("SRC",short(k).replace("\0"," ").slice(0,170),"result", r.fact.resultSha256===w.fact.resultSha256?"same":"DIFF");
const ra=changed.filter(([k,r,w])=>r.kind==="analysis"&&r.fact.source.sha256!==w.fact.source.sha256);
console.log("source changed but resultSha same:",ra.filter(([k,r,w])=>r.fact.resultSha256===w.fact.resultSha256).length,"of",ra.length);
for(const [k,r,w] of changed.filter(([k,r])=>r.kind!=="analysis").slice(0,30))console.log("OTH",short(k).replace("\0"," ").slice(0,150),JSON.stringify(r.fact).slice(0,80),"=>",JSON.stringify(w.fact).slice(0,80));
console.log("added sample",added.filter(k=>!k.startsWith("analysis")).slice(0,10).map(k=>short(k).slice(0,140)));
