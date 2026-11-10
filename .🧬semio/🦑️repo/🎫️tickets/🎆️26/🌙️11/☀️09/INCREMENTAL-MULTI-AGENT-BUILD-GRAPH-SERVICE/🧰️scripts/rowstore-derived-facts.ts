import{readFileSync,readdirSync,statSync}from"node:fs";
import{join,resolve}from"node:path";
import{heapStats}from"bun:jsc";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
let index:any=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
let derived:any[]=index.rows.filter((r:any)=>r.kind==="derived");
index=undefined;
const lens=derived.map(r=>JSON.stringify(r.fact).length+r.path.length).sort((a,b)=>b-a);
console.log("derived",derived.length,"sum chars",lens.reduce((a,b)=>a+b,0),"max",lens[0],"median",lens[lens.length>>1]);
console.log("sample path",derived[0].path.slice(0,200),"fact",JSON.stringify(derived[0].fact).slice(0,200));
for(let i=0;i<3;i++){await new Promise(r=>setTimeout(r,30));Bun.gc(true);}
const h=heapStats();console.log("heapMB",Math.round(h.heapSize/1048576),"extra",Math.round(h.extraMemorySize/1048576),"objects",h.objectCount);
process.exit(0);
