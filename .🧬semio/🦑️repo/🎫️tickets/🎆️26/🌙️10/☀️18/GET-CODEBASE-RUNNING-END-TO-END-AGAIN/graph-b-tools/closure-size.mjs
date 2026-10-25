import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync,brotliCompressSync}from"node:zlib";import{join}from"node:path";
const np=process.argv[2],n=Number(process.argv[3]??60);
const files=readdirSync(np).filter(f=>/^[a-f0-9]{24}\.json$/.test(f)).map(f=>[f,statSync(join(np,f)).mtimeMs]).sort((a,b)=>b[1]-a[1]).slice(0,n);
const all=new Map();let total=0,count=0;
for(const[f]of files){const text=brotliDecompressSync(readFileSync(join(np,f))).toString();const h=JSON.parse(text.slice(0,text.indexOf("\n")));let rows=[...h.observations];
 for(const b of h.blocks)rows.push(...JSON.parse(brotliDecompressSync(readFileSync(join(np,"ob",b+".br"))).toString()));
 for(const r of rows)if(r.kind==="closure"){count++;total+=r.path.length;all.set(r.path+r.fact,r.path.length);}}
const distinct=[...all.values()].reduce((a,b)=>a+b,0);
console.log("records",files.length,"closure rows",count,"avg path",Math.round(total/count),"distinct rows",all.size,"distinct bytes",distinct,"brotli of distinct",brotliCompressSync(Buffer.from([...all.keys()].join("\n"))).length);
