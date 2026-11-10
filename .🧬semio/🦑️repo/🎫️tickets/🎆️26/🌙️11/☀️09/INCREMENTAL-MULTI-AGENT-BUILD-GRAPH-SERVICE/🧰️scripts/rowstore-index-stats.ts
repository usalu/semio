import{readFileSync,readdirSync,statSync}from"node:fs";
import{join,resolve}from"node:path";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
const index:any=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
const kinds=new Map<string,number>(),factKinds=new Map<string,number>();const facts=new Set<any>();let sumPath=0;
for(const r of index.rows){kinds.set(r.kind,(kinds.get(r.kind)??0)+1);facts.add(r.fact);sumPath+=r.path.length;}
const flat=[...facts].filter(f=>f&&typeof f==="object"&&typeof f.sha256==="string");
console.log("kinds",JSON.stringify([...kinds]));console.log("distinct fact objects",facts.size,"flat",flat.length,"avg path",Math.round(sumPath/index.rows.length));
const ty=new Map<string,number>();for(const f of facts){const t=f===null?"null":typeof f==="object"?(Array.isArray(f)?"array":Object.keys(f).sort().join(",").slice(0,50)):typeof f;ty.set(t,(ty.get(t)??0)+1);}
console.log([...ty].sort((a,b)=>b[1]-a[1]).slice(0,10));
console.log("distinct paths",new Set(index.rows.map((r:any)=>r.path)).size);
process.exit(0);
