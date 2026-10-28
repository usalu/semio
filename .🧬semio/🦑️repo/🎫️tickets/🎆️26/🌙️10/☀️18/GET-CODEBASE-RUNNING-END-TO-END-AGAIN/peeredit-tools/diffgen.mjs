/** 🔬 Read-only: attributes every dirty owner of a recorded delta generation to the facts that changed against its parent. For each owner of `built.dirty` it lists the parent rows it held whose fact differs in (or vanished from) the child, grouped by kind, plus whether a membership change under its root explains it. Usage: bun diffgen.mjs <root> <child-manifest-digest> */
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{brotliDecompressSync}from"node:zlib";
const [root,childDigest]=process.argv.slice(2);
const codec=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations"),man=d=>JSON.parse(readFileSync(join(store,"m",d+".json"),"utf8"));
const open=m=>codec.openFactIndexV1(brotliDecompressSync(readFileSync(join(store,"x",m.factIndex+".facts.br")),{maxOutputLength:536870912}),{maximumBytes:536870912});
const child=man(childDigest),parent=man(child.parent),dirty=new Set(child.built?.dirty??[]);
const key=(kind,r)=>kind+"\0"+r.path+"\0"+(r.fact&&typeof r.fact==="object"&&!Array.isArray(r.fact)&&typeof r.fact.provider==="string"?r.fact.provider.split(":").slice(0,2).join(":"):"");
const h=v=>createHash("sha1").update(codec.canonicalJsonV1(v)).digest("base64");
let idx=open(child);const now=new Map();
for(let i=0;i<idx.rows.length;i++){const r=idx.rows[i];{const k=key(idx.rowKindOf(i),r),v=h(r.fact),was=now.get(k);now.set(k,was===undefined?v:was+"|"+v);}}
idx=null;Bun.gc(true);
const old=open(parent),changed=new Map();
for(let i=0;i<old.rows.length;i++){const r=old.rows[i],k=key(old.rowKindOf(i),r),n=now.get(k);if(n===undefined||!n.split("|").includes(h(r.fact)))changed.set(i,{kind:old.rowKindOf(i),path:r.path.startsWith(root+"/")?r.path.slice(root.length+1):r.path,gone:n===undefined});}
now.clear();
const perOwner=new Map(),holders=new Map();
old.classes.forEach(c=>{const rs=c.r.filter(r=>changed.has(r));if(!rs.length)return;for(const o of c.o){const name=old.owners[o];const s=perOwner.get(name)??new Set();for(const r of rs){s.add(r);const hs=holders.get(r)??new Set();hs.add(name);holders.set(r,hs);}perOwner.set(name,s);}});
const derived=n=>n.startsWith("@derived:");
console.log(JSON.stringify({child:child.generation,parent:parent.generation,dirty:dirty.size,changedRows:changed.size}));
const roots=Object.entries(parent.owners).map(([n,r])=>[n,r.root]);
const explained=new Set();
console.log("== changed rows (kind, path, non-derived holders, dirty holders, derived holders)");
for(const[i,row]of[...changed].sort((a,b)=>(holders.get(b[0])?.size??0)-(holders.get(a[0])?.size??0))){const hs=[...(holders.get(i)??[])];const nd=hs.filter(n=>!derived(n));const dd=nd.filter(n=>dirty.has(n));dd.forEach(n=>explained.add(n));if(!hs.length)continue;console.log(" ",row.kind,row.gone?"(gone)":"",row.path.slice(0,150),"nd",nd.length,"dirty",dd.length,"derived",hs.length-nd.length);}
const unexplained=[...dirty].filter(n=>!explained.has(n));
console.log("== dirty owners not explained by a changed non-derived row they held:",unexplained.length,unexplained.map(n=>n+" root="+(parent.owners[n]?.root??"?")).join("\n   "));
console.log("rss MB",Math.round(process.memoryUsage().rss/2**20));
