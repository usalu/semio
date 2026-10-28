/** 🧮 Read-only: for one owner, how its `file` rows overlap its `analysis` rows (path, provider) and how many other owners hold the same file rows. */
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{brotliDecompressSync}from"node:zlib";
const [root,owner]=process.argv.slice(2);
const codec=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8")),man=JSON.parse(readFileSync(join(store,"m",cur.manifest+".json"),"utf8"));
const idx=codec.openFactIndexV1(brotliDecompressSync(readFileSync(join(store,"x",man.factIndex+".facts.br")),{maxOutputLength:536870912}),{maximumBytes:536870912});
const o=idx.owners.indexOf(owner),ids=new Set();idx.classes.forEach(c=>{if(c.o.includes(o))for(const r of c.r)ids.add(r);});
const holders=new Map();idx.classes.forEach(c=>{for(const r of c.r)if(ids.has(r))holders.set(r,(holders.get(r)??0)+c.o.length);});
const fileP=new Map(),anaP=new Map();
for(const i of ids){const k=idx.rowKindOf(i);if(k==="file")fileP.set(idx.rows[i].path,i);else if(k==="analysis"){const r=idx.rows[i];const s=anaP.get(r.path)??[];s.push(r.fact.provider.split(":").slice(0,2).join(":"));anaP.set(r.path,s);}}
let both=0,onlyFile=0;const ext=new Map(),prov=new Map();for(const[p]of fileP){if(anaP.has(p)){both++;for(const v of anaP.get(p))prov.set(v,(prov.get(v)??0)+1);}else{onlyFile++;const e=p.split(".").pop();ext.set(e,(ext.get(e)??0)+1);}}
console.log(`file rows ${fileP.size}: with analysis ${both}, file only ${onlyFile}`,JSON.stringify([...ext]),"providers on paired",JSON.stringify([...prov]));
const h=[...fileP.values()].map(i=>holders.get(i));h.sort((a,b)=>a-b);console.log("holders per file row: min",h[0],"median",h[h.length>>1],"max",h.at(-1));
const anaOnly=[...anaP.keys()].filter(p=>!fileP.has(p)).length;console.log("analysis paths without file row",anaOnly);
