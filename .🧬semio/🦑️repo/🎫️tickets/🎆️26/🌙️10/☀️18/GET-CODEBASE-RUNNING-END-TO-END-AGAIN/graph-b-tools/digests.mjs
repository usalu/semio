import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const j=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2])).toString());
const own=new Map();j.classes.forEach(c=>{for(const r of c.r)own.set(r,(own.get(r)||0)+c.o.length)});
const rows=j.rows.map((r,i)=>[r,i]).filter(([r])=>r.kind==="digest");
for(const[r,i]of rows.sort((a,b)=>own.get(b[1])-own.get(a[1])).slice(0,40))console.log(String(own.get(i)).padStart(5),r.path.slice(-110));
