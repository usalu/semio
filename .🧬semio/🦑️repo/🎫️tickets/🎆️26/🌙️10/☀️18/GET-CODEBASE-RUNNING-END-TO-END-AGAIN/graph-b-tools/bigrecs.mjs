import{readFileSync,readdirSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join,dirname}from"node:path";import{createHash}from"node:crypto";
const np=process.argv[2],files=JSON.parse(readFileSync("/private/tmp/gr1022/files.json","utf8"));
const roots=new Map();for(const f of files){const r=dirname(f)||".";roots.set(createHash("sha256").update(r).digest("hex").slice(0,24),r);}
const out=[];
for(const f of readdirSync(np).filter(n=>/^[a-f0-9]{24}\.json$/.test(n))){
 const text=brotliDecompressSync(readFileSync(join(np,f))).toString();const h=JSON.parse(text.slice(0,text.indexOf("\n")));
 const rows=[...h.observations];for(const b of h.blocks)rows.push(...JSON.parse(brotliDecompressSync(readFileSync(join(np,"ob",b+".br"))).toString()));
 const kinds={};for(const r of rows)kinds[r.kind]=(kinds[r.kind]||0)+1;
 out.push([rows.length,roots.get(f.slice(0,24))??f,kinds]);
}
out.sort((a,b)=>b[0]-a[0]);for(const r of out.slice(0,8))console.log(r[0],r[1].slice(-70),JSON.stringify(r[2]));
console.log("over 3000:",out.filter(r=>r[0]>3000).length,"of",out.length);
