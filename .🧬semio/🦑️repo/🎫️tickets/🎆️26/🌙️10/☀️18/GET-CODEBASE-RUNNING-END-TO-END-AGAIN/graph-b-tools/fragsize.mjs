import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const g="/Users/ueli/Documents/semio/.cache/b/g/m/generations";
const ms=readdirSync(join(g,"m")).map(f=>[f,statSync(join(g,"m",f)).mtimeMs]).sort((a,b)=>b[1]-a[1]);const m=JSON.parse(readFileSync(join(g,"m",ms[0][0])));
const rows=[];for(const[name,o]of Object.entries(m.owners)){if(o.kind!=="project")continue;try{const t=brotliDecompressSync(readFileSync(join(g,"o",o.fragment+".json.br")));rows.push([t.length,name]);}catch{}}
rows.sort((a,b)=>b[0]-a[0]);console.log("generation",m.generation,"projects",rows.length,"sum MB",rows.reduce((a,r)=>a+r[0],0)/1e6);
for(const r of rows.slice(0,12))console.log(r.join(" "));
