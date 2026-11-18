import{readdirSync,readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const cas=join(process.argv[2],"v1/main/cas");let raw=0,packed=0,n=0,rec=0,clo=0,big=0;
for(const d of readdirSync(cas))for(const f of readdirSync(join(cas,d))){const b=readFileSync(join(cas,d,f));let x;try{x=brotliDecompressSync(b)}catch{continue}n++;raw+=x.length;packed+=b.length;let j;try{j=JSON.parse(x.toString())}catch{continue}if(j.version===2&&j.digest)rec=x.length;else if(j.version===2&&j.recipe){clo+=x.length;big=Math.max(big,x.length)}}
console.log(JSON.stringify({docs:n,rawBytes:raw,packedBytes:packed,recordRaw:rec,closuresRaw:clo,largestClosureRaw:big}));
