import{readFileSync,readdirSync,statSync}from"node:fs";
import{join,resolve}from"node:path";
const G=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),cut=await import(join(G,"🕰️generations/✂️cutoff/🟨️.mjs")),cl=await import(join(G,"🕰️generations/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs").replace("🕰️generations/🧮️derived-nodes","🧮️derived-nodes"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
let t=performance.now();const index=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);const load=performance.now()-t;
t=performance.now();const sel=gen.selectFactClassesV1(index,undefined),verifiable=cut.verifiableRowIdsV1(index,sel.ids);const select=performance.now()-t;
const derived=[...cut.derivedRowIdsV1(index)];console.log(JSON.stringify({rows:index.rows.length,derived:derived.length,verifiable:verifiable.length,loadMs:Math.round(load),selectMs:Math.round(select)}));
const raw=verifiable.filter(id=>index.rows[id].kind==="file");
for(const n of[0,1,50,1000,10000]){
 const stale=new Set(raw.filter((_,i)=>n&&i%Math.max(1,Math.floor(raw.length/n))===0).slice(0,n));
 t=performance.now();const{suspects}=cut.splitStaleConsumersV1(index,sel.classes,stale);const split=performance.now()-t;
 t=performance.now();const rows=cut.suspectRowsV1(index,suspects);const sr=performance.now()-t;
 const settler=cl.createClosureSettlerV1({});t=performance.now();
 const r=await settler.settle({pseudoRows:rows,derive:async row=>({fact:row,rows:[row]}),hold:async()=>true});const settle=performance.now()-t;
 console.log(JSON.stringify({stale:stale.size,suspects:suspects.size,splitMs:+split.toFixed(1),suspectRowsMs:+sr.toFixed(1),settleMsReplayedDerive:+settle.toFixed(1),held:r.held.size,changed:r.changed.size,failed:r.failed}));
}
