/** 🧲 Read-only probe of the live generation: which owners hold fact rows of files under the build library / process module, by row kind, and how many owners a single library file edit would make stale (rows without cutoff). Usage: bun holders.mjs <root> [prefix...] */
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{brotliDecompressSync}from"node:zlib";
const [root,...prefixArgs]=process.argv.slice(2);
const prefixes=(prefixArgs.length?prefixArgs:["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/","🧰️framework/🔨️modules/🏃️process/"]).map(p=>join(root,p));
const codec=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8")),man=JSON.parse(readFileSync(join(store,"m",cur.manifest+".json"),"utf8"));
const idx=codec.openFactIndexV1(brotliDecompressSync(readFileSync(join(store,"x",man.factIndex+".facts.br")),{maxOutputLength:536870912}),{maximumBytes:536870912});
const hit=new Map();
for(let i=0;i<idx.rows.length;i++){const r=idx.rows[i];if(prefixes.some(p=>r.path.startsWith(p)))hit.set(i,{kind:idx.rowKindOf(i),path:r.path,fact:r.fact});}
const holders=new Map();
idx.classes.forEach(c=>{for(const r of c.r)if(hit.has(r)){const s=holders.get(r)??new Set();for(const o of c.o)s.add(idx.owners[o]);holders.set(r,s);}});
const byKind={},ownersByKind={},ownerKinds=new Map();
for(const[i,row]of hit){const hs=holders.get(i)??new Set();byKind[row.kind]=(byKind[row.kind]??0)+1;const ok=ownersByKind[row.kind]??=new Set();for(const o of hs){ok.add(o);const m=ownerKinds.get(o)??{};m[row.kind]=(m[row.kind]??0)+1;ownerKinds.set(o,m);}}
const derived=o=>o.startsWith("@derived:");
const out={generation:cur.generation,rows:idx.rows.length,owners:idx.owners.length,libraryRows:hit.size,rowsByKind:byKind,ownersByKind:Object.fromEntries(Object.entries(ownersByKind).map(([k,s])=>[k,{all:s.size,nonDerived:[...s].filter(o=>!derived(o)).length}]))};
console.log(JSON.stringify(out));
const nd=[...ownerKinds].filter(([o])=>!derived(o)).sort((a,b)=>Object.values(b[1]).reduce((x,y)=>x+y,0)-Object.values(a[1]).reduce((x,y)=>x+y,0));
console.log("non-derived owners holding library rows:",nd.length);
for(const[o,m]of nd.slice(0,40))console.log(" ",o,JSON.stringify(m));
const dv=[...ownerKinds].filter(([o])=>derived(o));
console.log("derived owners holding library rows:",dv.length);
for(const[o,m]of dv.slice(0,15))console.log(" ",o.slice(0,200),JSON.stringify(m));
const samples={};for(const[i,row]of hit){const s=samples[row.kind]??=[];if(s.length<3)s.push({path:row.path.slice(root.length+1),fact:JSON.stringify(row.fact).slice(0,200),holders:[...(holders.get(i)??[])].length});}
console.log(JSON.stringify(samples,null,1));
const fanout=[...hit].map(([i,row])=>({kind:row.kind,path:row.path.slice(root.length+1),n:[...(holders.get(i)??[])].filter(o=>!derived(o)).length})).sort((a,b)=>b.n-a.n);
console.log("top fan-out rows:");for(const f of fanout.slice(0,25))console.log(" ",f.n,f.kind,f.path);
console.log("rss MB",Math.round(process.memoryUsage().rss/2**20));
