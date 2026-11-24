import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const ce=process.argv[2],since=Number(process.argv[3]);
const names=readdirSync(join(ce,"e")).map(n=>[n,statSync(join(ce,"e",n)).mtimeMs]).filter(([,t])=>t>=since).sort((a,b)=>a[1]-b[1]);
const seen=new Map();let entries=0,noAnalysis=0;
const dec=p=>JSON.parse(brotliDecompressSync(readFileSync(p)).toString("utf8"));
for(const[n,t]of names){let e;try{e=dec(join(ce,"e",n));}catch{continue;}entries++;
 const found=new Set();
 const scan=rows=>{for(const r of rows)if(r.kind==="analysis"){const role=String(r.fact.provider).replace(/:v1:.*/,"");found.add(role+" "+r.fact.revision.slice(0,10));if(found.size>=3)return;}};
 scan(e.rows);for(const d of new Set(e.layers)){if(found.size>=3)break;try{scan(dec(join(ce,"l",d+".json.br")));}catch{}}
 if(found.size===0)noAnalysis++;
 for(const k of found){const s=seen.get(k)??{first:t,last:t,n:0};s.last=t;s.n++;seen.set(k,s);}}
console.log({entries,noAnalysis});
for(const[k,s]of [...seen].sort((a,b)=>a[1].first-b[1].first))console.log(k.padEnd(48),new Date(s.first).toISOString().slice(5,16),"->",new Date(s.last).toISOString().slice(5,16),"entries",s.n);
