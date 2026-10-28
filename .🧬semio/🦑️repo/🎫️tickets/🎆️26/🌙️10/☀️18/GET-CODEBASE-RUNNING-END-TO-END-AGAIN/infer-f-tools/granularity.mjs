import{readFileSync}from"node:fs";import{createHash}from"node:crypto";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const idx=G.unpackFactIndexV1(readFileSync(process.argv[2]),1<<29);
const ownersOfRow=new Map();
for(const c of idx.classes)for(const o of c.o)for(const r of c.r){let s=ownersOfRow.get(r);if(!s)ownersOfRow.set(r,s=new Set());s.add(o);}
const changed=new Map();let analysisRows=0;
for(let i=0;i<idx.rows.length;i++){const row=idx.rows[i];if(row.kind!=="analysis")continue;analysisRows++;
  let sha;try{sha=createHash("sha256").update(readFileSync(row.path)).digest("hex");}catch{sha="!missing";}
  if(sha!==row.fact.source.sha256){let e=changed.get(row.path);if(!e)changed.set(row.path,e={rows:[],owners:new Set()});e.rows.push(i);for(const o of ownersOfRow.get(i)??[])e.owners.add(o);}}
const all=new Set();for(const e of changed.values())for(const o of e.owners)all.add(o);
console.log(JSON.stringify({analysisRows,changedFiles:changed.size,ownersAffectedByChangedAnalysis:all.size,totalOwners:idx.owners.length}));
const top=[...changed].sort((a,b)=>b[1].owners.size-a[1].owners.size).slice(0,15);
for(const[p,e]of top)console.log(e.owners.size,p.replace("/Users/ueli/Documents/semio/","").slice(-110));
const hist={};for(const e of changed.values()){const k=e.owners.size>1000?">1000":e.owners.size>100?">100":e.owners.size>10?">10":"<=10";hist[k]=(hist[k]??0)+1;}console.log(JSON.stringify(hist));
