/** 🔬 Read-only anatomy of the live fact index: decode/pack costs and the rows and owners an edited file touches. */
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{brotliCompressSync,brotliDecompressSync,constants as Z}from"node:zlib";
const [root,file]=process.argv.slice(2);
const G=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations");
const codec=await import(join(G,"🧮️fact-codec/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations");
const cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8"));
const man=JSON.parse(readFileSync(join(store,"m",cur.manifest+".json"),"utf8"));
const t=()=>performance.now(),rss=()=>Math.round(process.memoryUsage().rss/2**20);
let a=t();const packed=readFileSync(join(store,"x",man.factIndex+".facts.br"));const raw=brotliDecompressSync(packed,{maxOutputLength:536870912});
console.log(`gen ${cur.generation} packed ${packed.length} raw ${raw.length} decompress ${(t()-a).toFixed(0)}ms rss ${rss()}`);
a=t();const idx=codec.openFactIndexV1(raw,{maximumBytes:536870912});console.log(`open ${(t()-a).toFixed(0)}ms rows ${idx.rows.length} owners ${idx.owners.length} classes ${idx.classes.length} rss ${rss()}`);
for(const q of [1,3,5]){a=t();const b=brotliCompressSync(raw,{params:{[Z.BROTLI_PARAM_QUALITY]:q,[Z.BROTLI_PARAM_LGWIN]:22,[Z.BROTLI_PARAM_SIZE_HINT]:raw.length}});console.log(`brotli q${q} ${(t()-a).toFixed(0)}ms -> ${b.length}`);}
const kinds=new Map();for(let i=0;i<idx.rows.length;i++){const k=idx.rowKindOf(i);kinds.set(k,(kinds.get(k)??0)+1);}
console.log("kinds",JSON.stringify([...kinds].sort((x,y)=>y[1]-x[1])));
const hits=[];for(let i=0;i<idx.rows.length;i++){const k=idx.rowKindOf(i);if(k==="derived")continue;const r=idx.rows[i];if(r.path===file||r.path.endsWith("/"+file))hits.push(i);}
const holders=new Map(hits.map(i=>[i,new Set()]));
idx.classes.forEach(c=>{for(const r of c.r)if(holders.has(r))for(const o of c.o)holders.get(r).add(idx.owners[o]);});
for(const i of hits){const r=idx.rows[i];console.log(`row ${i} ${r.kind} ${JSON.stringify(r.fact).slice(0,200)} owners ${holders.get(i).size}: ${[...holders.get(i)].slice(0,8).join(" ")}`);}
const der=[];for(let i=0;i<idx.rows.length;i++)if(idx.rowKindOf(i)==="derived"){const r=idx.rows[i];if(JSON.stringify(r).includes(file))der.push(i);}
console.log(`derived rows mentioning the file: ${der.length}`);
const own=new Map();idx.classes.forEach(c=>{for(const o of c.o)own.set(o,(own.get(o)??0)+c.r.length);});
const course=idx.owners.indexOf("@teaching/architecture-course");console.log(`course rows ${own.get(course)} top owners`,JSON.stringify([...own].sort((x,y)=>y[1]-x[1]).slice(0,5).map(([o,n])=>[idx.owners[o],n])));
console.log(`rss ${rss()}`);
