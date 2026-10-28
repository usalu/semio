/** 🧷 Read-only: the `file` rows of one owner that carry no analysis of the same path, grouped by suffix, with samples. */
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{brotliDecompressSync}from"node:zlib";
const [root,owner]=process.argv.slice(2);
const codec=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8")),man=JSON.parse(readFileSync(join(store,"m",cur.manifest+".json"),"utf8"));
const idx=codec.openFactIndexV1(brotliDecompressSync(readFileSync(join(store,"x",man.factIndex+".facts.br")),{maxOutputLength:536870912}),{maximumBytes:536870912});
const o=idx.owners.indexOf(owner),ids=new Set();idx.classes.forEach(c=>{if(c.o.includes(o))for(const r of c.r)ids.add(r);});
const files=new Set(),ana=new Set(),res=new Map();for(const i of ids){const k=idx.rowKindOf(i);if(k==="file")files.add(idx.rows[i].path);else if(k==="analysis")ana.add(idx.rows[i].path);else if(k==="resolve"){const r=idx.rows[i];res.set(r.path,JSON.stringify(r.fact).slice(0,80));}}
const groups=new Map();for(const p of files)if(!ana.has(p)){const m=p.match(/(\.d)?\.[a-z]+$/)?.[0]??"none";const g=groups.get(m)??[];g.push(p);groups.set(m,g);}
for(const[m,g]of groups)console.log(m,g.length,"\n  "+g.slice(0,5).map(p=>p.replace(root+"/","")).join("\n  "));
console.log("resolve rows",res.size,[...res].slice(0,3));
