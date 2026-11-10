import{readFileSync,readdirSync,statSync}from"node:fs";
import{join,resolve}from"node:path";
const G=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),cut=await import(join(G,"🕰️generations/✂️cutoff/🟨️.mjs"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
const index=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
const sel=gen.selectFactClassesV1(index,undefined);
for(const needle of process.argv.slice(2)){
 const ids=index.rows.map((r,i)=>[r,i]).filter(([r])=>r.path.endsWith(needle)||r.path.includes(needle)).map(([,i])=>i);
 console.log("==",needle,"rows",ids.map(i=>index.rows[i].kind).join(","));
 const idset=new Set(ids);let shown=0;
 for(const at of sel.classes){const c=index.classes[at];if(!c.r.some(id=>idset.has(id)))continue;const owners=c.o.filter(o=>cut.isDerivedOwnerV1(index.owners[o])).length;if(!owners)continue;const kinds={};for(const id of c.r)kinds[index.rows[id].kind]=(kinds[index.rows[id].kind]??0)+1;console.log(" class owners",owners,"rows",c.r.length,JSON.stringify(kinds));if(++shown>=4)break;}
}
