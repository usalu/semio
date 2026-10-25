import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const np=process.argv[2],n=Number(process.argv[3]??30);
const files=readdirSync(np).filter(f=>/^[a-f0-9]{24}\.json$/.test(f)).map(f=>[f,statSync(join(np,f)).mtimeMs]).sort((a,b)=>b[1]-a[1]).slice(0,n);
const counts=[];let closure=0;
for(const[f]of files){const text=brotliDecompressSync(readFileSync(join(np,f))).toString();const h=JSON.parse(text.slice(0,text.indexOf("\n")));let c=h.observations.length;let k=h.observations.filter(r=>r.kind==="closure").length;
 for(const b of h.blocks){const rs=JSON.parse(brotliDecompressSync(readFileSync(join(np,"ob",b+".br"))).toString());c+=rs.length;k+=rs.filter(r=>r.kind==="closure").length;}counts.push(c);closure+=k;}
counts.sort((a,b)=>a-b);console.log("newest",files.length,"rows median",counts[counts.length>>1],"max",counts.at(-1),"closure facts",closure);
