/** 🧮 Read-only: per row kind and file extension, how many library rows and how many non-derived holders (max/median), plus the file rows of one sample owner. Usage: bun kinds.mjs <root> <owner> */
import{readFileSync}from"node:fs";
import{join,extname}from"node:path";
import{brotliDecompressSync}from"node:zlib";
const [root,sample]=process.argv.slice(2);
const prefixes=["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/","🧰️framework/🔨️modules/🏃️process/"].map(p=>join(root,p));
const codec=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8")),man=JSON.parse(readFileSync(join(store,"m",cur.manifest+".json"),"utf8"));
const idx=codec.openFactIndexV1(brotliDecompressSync(readFileSync(join(store,"x",man.factIndex+".facts.br")),{maxOutputLength:536870912}),{maximumBytes:536870912});
const hit=new Map();
for(let i=0;i<idx.rows.length;i++){const k=idx.rowKindOf(i);if(k!=="file"&&k!=="analysis"&&k!=="typescriptText"&&k!=="digest")continue;const r=idx.rows[i];if(prefixes.some(p=>r.path.startsWith(p)))hit.set(i,{kind:k,path:r.path,fact:r.fact});}
const n=new Map(),sampleRows=[];const si=idx.owners.indexOf(sample);
idx.classes.forEach(c=>{const nd=c.o.filter(o=>!idx.owners[o].startsWith("@derived:")).length;const has=c.o.includes(si);for(const r of c.r)if(hit.has(r)){n.set(r,(n.get(r)??0)+nd);if(has)sampleRows.push(r);}});
const g={};for(const[i,row]of hit){const key=row.kind+" "+(row.kind==="analysis"?String(row.fact?.provider??"").split(":").slice(0,2).join(":"):extname(row.path));const a=g[key]??=[];a.push(n.get(i)??0);}
for(const[k,a]of Object.entries(g).sort()){a.sort((x,y)=>x-y);console.log(k.padEnd(48),"rows",String(a.length).padStart(6),"holders max",a.at(-1),"median",a[a.length>>1],"rows held by >100",a.filter(x=>x>100).length);}
const sk={};for(const r of sampleRows){const row=hit.get(r);const key=row.kind+" "+extname(row.path);sk[key]=(sk[key]??0)+1;}
console.log("sample",sample,JSON.stringify(sk));
const ts=sampleRows.filter(r=>hit.get(r).kind==="file").map(r=>hit.get(r).path.slice(root.length+1));console.log(ts.filter(p=>p.endsWith(".ts")).slice(0,8).join("\n"));console.log(ts.filter(p=>!p.endsWith(".ts")).slice(0,8).join("\n"));
console.log("rss MB",Math.round(process.memoryUsage().rss/2**20));
