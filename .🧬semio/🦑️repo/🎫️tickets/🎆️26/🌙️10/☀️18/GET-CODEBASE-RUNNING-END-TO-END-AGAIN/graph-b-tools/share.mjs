import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const j=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2])).toString());
const own=new Array(j.rows.length).fill(0);j.classes.forEach(c=>{for(const r of c.r)own[r]+=c.o.length});
const buckets=[[1,1],[2,9],[10,99],[100,999],[1000,99999]];
for(const[lo,hi]of buckets){const kinds={};let n=0;own.forEach((o,i)=>{if(o>=lo&&o<=hi){n++;const k=j.rows[i].kind;kinds[k]=(kinds[k]||0)+1}});console.log(`owners ${lo}-${hi}: ${n} rows`,JSON.stringify(kinds))}
