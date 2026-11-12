import{readFileSync}from"node:fs";
import{brotliDecompressSync}from"node:zlib";
const G=process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const gen=await import(G+"🟨️.mjs"),{decodeFactIndexV1}=await import(G+"🧮️fact-codec/🟨️.mjs");
const mem=l=>{Bun.gc(true);const m=process.memoryUsage();console.log(l,"rssMB",Math.round(m.rss/1e6),"heapMB",Math.round(m.heapUsed/1e6),"extMB",Math.round(m.external/1e6),"peakMB",Math.round(process.resourceUsage().maxRSS/1024));};
const file=process.argv[2];const bytes=readFileSync(file);mem("start");
const raw=brotliDecompressSync(bytes,{maxOutputLength:2**31});console.log("expanded",raw.length);mem("raw");
const index=gen.unpackFactIndexV1(bytes,2**31);mem("unpacked");
let inc=0,rr=0,oo=0;for(const c of index.classes){inc+=c.r.length*c.o.length;rr+=c.r.length;oo+=c.o.length;}
const derived=index.owners.filter(n=>n.startsWith("derived:")||n.startsWith("\0")).length;
console.log(JSON.stringify({owners:index.owners.length,rows:index.rows.length,classes:index.classes.length,incidences:inc,classRowRefs:rr,classOwnerRefs:oo,derived,sample:index.owners.slice(0,3)}));
const kinds={};for(const r of index.rows)kinds[r.kind]=(kinds[r.kind]??0)+1;console.log(JSON.stringify(kinds));
