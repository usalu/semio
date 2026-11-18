import{readdirSync,readFileSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const store=process.argv[2],cas=join(store,"v1/main/cas");
const docs=[];
for(const d of readdirSync(cas))for(const f of readdirSync(join(cas,d))){const raw=readFileSync(join(cas,d,f));let b;try{b=brotliDecompressSync(raw)}catch{continue}docs.push({hash:f.slice(0,8),packed:raw.length,bytes:b});}
const rootRe=/\$\{?[A-Z_]*ROOT\}?|«[^»]*»/;
let total=0,totalPacked=0;
for(const d of docs.sort((a,b)=>b.bytes.length-a.bytes.length)){
 total+=d.bytes.length;totalPacked+=d.packed;
 let j;try{j=JSON.parse(Buffer.from(d.bytes).toString())}catch{console.log(d.hash,d.bytes.length,"nonjson");continue}
 const kind=j.observations?"record":j.recipe?"closure":"other";
 console.log(d.hash,kind,"raw",d.bytes.length,"packed",d.packed,Object.keys(j).join(","));
 if(kind==="record"){
  const by={};for(const sect of["authority","observations"])for(const r of j[sect]){const k=sect+":"+r.kind;const s=JSON.stringify(r).length;by[k]=by[k]||{n:0,bytes:0,path:0};by[k].n++;by[k].bytes+=s;by[k].path+=r.path.length}
  for(const[k,v]of Object.entries(by).sort((a,b)=>b[1].bytes-a[1].bytes))console.log("   ",k,JSON.stringify(v));
  console.log("    image",j.image.length);
 } else if(kind==="closure"){
  const by={};for(const r of [...j.rows,...j.layers.flat()]){const k=r.kind;by[k]=by[k]||{n:0,bytes:0};by[k].n++;by[k].bytes+=JSON.stringify(r).length}
  console.log("    rows",j.rows.length,"layers",j.layers.length,JSON.stringify(by));
 }
}
console.log("TOTAL raw",total,"packed",totalPacked,"docs",docs.length);
