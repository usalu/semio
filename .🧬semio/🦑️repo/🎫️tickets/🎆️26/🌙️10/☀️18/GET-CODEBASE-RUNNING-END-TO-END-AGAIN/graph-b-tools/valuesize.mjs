import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const np=process.argv[2];const rows=[];
for(const f of readdirSync(np).filter(f=>/^[a-f0-9]{24}\.json$/.test(f))){const text=brotliDecompressSync(readFileSync(join(np,f))).toString();const v=text.slice(text.indexOf("\n")+1);const o=JSON.parse(v);rows.push([v.length,f,o.closureRecipes?JSON.stringify(o.closureRecipes).length:0,Object.keys(o.closureRecipes??{}).length,statSync(join(np,f)).mtimeMs]);}
rows.sort((a,b)=>b[0]-a[0]);console.log(rows.slice(0,6).map(r=>r.slice(0,4).join(" ")).join("\n"));
const late=rows.filter(r=>r[2]>0);console.log("late",late.length,"of",rows.length,"sum recipes MB",late.reduce((a,r)=>a+r[2],0)/1e6,"sum values MB",rows.reduce((a,r)=>a+r[0],0)/1e6);
