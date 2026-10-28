/** 🧾 Read-only: the rows of one owner by kind, and its `file` rows, from the live fact index. */
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{brotliDecompressSync}from"node:zlib";
const [root,owner,kindFilter="file"]=process.argv.slice(2);
const codec=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8")),man=JSON.parse(readFileSync(join(store,"m",cur.manifest+".json"),"utf8"));
const idx=codec.openFactIndexV1(brotliDecompressSync(readFileSync(join(store,"x",man.factIndex+".facts.br")),{maxOutputLength:536870912}),{maximumBytes:536870912});
const o=idx.owners.indexOf(owner),ids=new Set();idx.classes.forEach(c=>{if(c.o.includes(o))for(const r of c.r)ids.add(r);});
const kinds=new Map(),sel=[];for(const i of ids){const k=idx.rowKindOf(i);kinds.set(k,(kinds.get(k)??0)+1);if(k===kindFilter)sel.push(idx.rows[i]);}
console.log(owner,ids.size,JSON.stringify([...kinds].sort((a,b)=>b[1]-a[1])));
const exclusive=new Map();idx.classes.forEach(c=>{for(const r of c.r)if(ids.has(r))exclusive.set(r,(exclusive.get(r)??0)+c.o.length);});
for(const r of sel.slice(0,60))console.log(r.kind,r.path,JSON.stringify(r.fact).slice(0,90));
console.log("selected",sel.length);
