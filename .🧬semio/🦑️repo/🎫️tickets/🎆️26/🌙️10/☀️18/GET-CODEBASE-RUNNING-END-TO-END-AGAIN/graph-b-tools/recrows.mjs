import{readFileSync,readdirSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const np=process.argv[2];const kinds={};let recs=0,rows=0,perRec=[];
for(const f of readdirSync(np).filter(n=>/^[a-f0-9]{24}\.json$/.test(n))){
 const text=brotliDecompressSync(readFileSync(join(np,f))).toString();const h=JSON.parse(text.slice(0,text.indexOf("\n")));
 let n=h.observations.length;const ks=[...h.observations];
 for(const b of h.blocks){const rs=JSON.parse(brotliDecompressSync(readFileSync(join(np,"ob",b+".br"))).toString());n+=rs.length;ks.push(...rs);}
 recs++;rows+=n;perRec.push(n);for(const r of ks)kinds[r.kind]=(kinds[r.kind]||0)+1;
}
perRec.sort((a,b)=>a-b);console.log("records",recs,"rows total",rows,"median",perRec[perRec.length>>1],"max",perRec.at(-1));console.log(JSON.stringify(kinds));
