import{readFileSync}from"node:fs";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const idx=G.unpackFactIndexV1(readFileSync(process.argv[2]),1<<29);
const kinds=new Map();for(const r of idx.rows)kinds.set(r.kind,(kinds.get(r.kind)??0)+1);
console.log("owners",idx.owners.length,"rows",idx.rows.length,"classes",idx.classes.length,JSON.stringify([...kinds]));
const per=new Array(idx.owners.length).fill(0).map(()=>({})); 
for(const c of idx.classes)for(const o of c.o)for(const r of c.r){const k=idx.rows[r].kind;per[o][k]=(per[o][k]??0)+1;}
const tot={};for(const p of per)for(const[k,v]of Object.entries(p)){tot[k]??=[];tot[k].push(v);}
for(const[k,v]of Object.entries(tot)){v.sort((a,b)=>a-b);console.log(k,"owners",v.length,"median",v[v.length>>1],"p90",v[Math.floor(v.length*.9)],"max",v.at(-1));}
const closure=idx.rows.filter(r=>r.kind==="closure").slice(0,3);console.log(JSON.stringify(closure).slice(0,1500));
