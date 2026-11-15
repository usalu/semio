import{readdirSync,readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const D=process.argv[2]+"/e",names=readdirSync(D).filter(n=>n.endsWith(".json.br"));
let n=0,packed=0,raw=0,rows=0,maxRaw=0;const t=performance.now();
const kept=[];
for(const name of names){const b=readFileSync(join(D,name));packed+=b.length;const text=brotliDecompressSync(b,{maxOutputLength:268435456}).toString("utf8");raw+=text.length;maxRaw=Math.max(maxRaw,text.length);const rec=JSON.parse(text);rows+=rec.rows?.length??0;kept.push(rec);n++;
 if(n%5000===0){console.log(n,"packedMB",Math.round(packed/1048576),"rawMB",Math.round(raw/1048576),"rows",rows,"rssMB",Math.round(process.memoryUsage().rss/1048576),"heapMB",Math.round(process.memoryUsage().heapUsed/1048576),"s",Math.round((performance.now()-t)/1000));}
 if(process.memoryUsage().rss>2.8e9){console.log("ABORT at",n,"rss>2.8GB");break;}}
console.log("final",n,"of",names.length,"packedMB",Math.round(packed/1048576),"rawMB",Math.round(raw/1048576),"rows",rows,"maxRawKB",Math.round(maxRaw/1024),"rssMB",Math.round(process.memoryUsage().rss/1048576),"heapMB",Math.round(process.memoryUsage().heapUsed/1048576));
