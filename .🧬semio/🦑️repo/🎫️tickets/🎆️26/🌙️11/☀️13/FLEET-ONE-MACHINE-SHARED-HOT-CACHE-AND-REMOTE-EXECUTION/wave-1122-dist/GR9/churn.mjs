import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{createHash}from"node:crypto";import{join}from"node:path";
const ce=process.argv[2],count=Number(process.argv[3]??60);
const dec=p=>JSON.parse(brotliDecompressSync(readFileSync(p)).toString("utf8"));
const names=readdirSync(join(ce,"e")).map(n=>[n,statSync(join(ce,"e",n)).mtimeMs]).sort((a,b)=>b[1]-a[1]).slice(0,count);
const shaNow=new Map(),layers=new Map();const now=Date.now();
const cur=p=>{let s=shaNow.get(p);if(s===undefined){try{s=createHash("sha256").update(readFileSync(p)).digest("hex");}catch{s="!gone";}shaNow.set(p,s);}return s;};
const rowsOf=d=>{let r=layers.get(d);if(!r){try{r=dec(join(ce,"l",d+".json.br")).filter(x=>x.kind==="analysis").map(x=>[x.path,x.fact.source?.sha256]);}catch{r=[];}layers.set(d,r);}return r;};
const out=[];
for(const[n,t]of names){const e=dec(join(ce,"e",n));const srcs=new Map();for(const r of e.rows)if(r.kind==="analysis")srcs.set(r.path,r.fact.source?.sha256);for(const d of new Set(e.layers))for(const[p,s]of rowsOf(d))srcs.set(p,s);
 const changed=[...srcs].filter(([p,s])=>s!==undefined&&cur(p)!==s).map(([p])=>p);out.push({ageMin:Math.round((now-t)/60000),files:srcs.size,changed:changed.length,ex:changed.slice(0,2).map(p=>p.slice(-70))});}
out.sort((a,b)=>a.ageMin-b.ageMin);
for(const o of out)console.log(String(o.ageMin).padStart(5),"min",String(o.files).padStart(6),"files changed",o.changed,o.ex.join(" | "));
