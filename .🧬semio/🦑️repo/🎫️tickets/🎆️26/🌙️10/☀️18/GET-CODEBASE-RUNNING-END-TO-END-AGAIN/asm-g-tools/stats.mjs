// [asm-g] manifest + fact-index census of a scratch generation. usage: bun stats.mjs <dir>
import{readFileSync}from"node:fs";import{join}from"node:path";import{brotliDecompressSync}from"node:zlib";
const D=process.argv[2],E="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const g=await import(E);const mem=()=>Math.round(process.memoryUsage().rss/1048576);
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const by={};let bytes={};for(const[name,row]of Object.entries(manifest.owners)){const k=(row.tag??"primary")+"/"+row.kind;by[k]=(by[k]??0)+1;bytes[k]=(bytes[k]??0)+row.bytes;}
console.log("generation",pointer.generation,"owners",Object.keys(manifest.owners).length,"built",JSON.stringify(manifest.built).slice(0,120));
console.log("by tag/kind",JSON.stringify(by),"fragment bytes",JSON.stringify(bytes));
let t=performance.now();const raw=readFileSync(join(D,"x",manifest.factIndex+".facts.br"));console.log("index packed MB",(raw.length/1048576).toFixed(1),"rss",mem());
const index=g.unpackFactIndexV1(raw,g.GENERATION_POLICY_V1.maximumIndexBytes);console.log("index unpack ms",Math.round(performance.now()-t),"rows",index.rows.length,"owners",index.owners.length,"classes",index.classes.length,"rss",mem());
const kinds={};for(const r of index.rows)kinds[r.kind]=(kinds[r.kind]??0)+1;console.log("row kinds",JSON.stringify(kinds));
const per=new Map();for(const c of index.classes)for(const o of c.o)per.set(o,(per.get(o)??0)+c.r.length);
const rowsOf=name=>per.get(index.owners.indexOf(name))??0;
const tags={};for(const[name,row]of Object.entries(manifest.owners)){const k=row.tag??"primary";(tags[k]??=[]).push(rowsOf(name));}
for(const[k,l]of Object.entries(tags)){l.sort((a,b)=>a-b);console.log(k,"owners",l.length,"rows sum",l.reduce((a,b)=>a+b,0),"median",l[l.length>>1],"max",l.at(-1));}
