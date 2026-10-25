import{readFileSync,readdirSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const np=process.argv[2];let best;
for(const f of readdirSync(np).filter(n=>/^[a-f0-9]{24}\.json$/.test(n))){const text=brotliDecompressSync(readFileSync(join(np,f))).toString();const h=JSON.parse(text.slice(0,text.indexOf("\n")));const rows=[...h.observations];for(const b of h.blocks)rows.push(...JSON.parse(brotliDecompressSync(readFileSync(join(np,"ob",b+".br"))).toString()));if(!best||rows.length>best.rows.length)best={f,rows};}
const ws="/Users/ueli/Documents/semio/";const by={};
for(const r of best.rows){const p=r.path.replace(ws,"").replace(/^\["[^\]]*"/,"");const k=r.kind+" "+p.split("/").slice(0,4).join("/");by[k]=(by[k]||0)+1;}
console.log(best.f,best.rows.length);console.log(Object.entries(by).sort((a,b)=>b[1]-a[1]).slice(0,14).map(([k,v])=>v+" "+k).join("\n"));
