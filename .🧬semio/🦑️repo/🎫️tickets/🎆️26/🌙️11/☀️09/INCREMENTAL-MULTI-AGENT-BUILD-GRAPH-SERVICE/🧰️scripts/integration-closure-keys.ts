import{readFileSync,readdirSync,statSync}from"node:fs";
import{join,resolve}from"node:path";
const G=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
const index=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
const sets:Record<string,Set<string>>={};const add=(k:string,v:unknown)=>(sets[k]??=new Set()).add(JSON.stringify(v));
const kinds:Record<string,number>={};
for(const r of index.rows){if(r.kind!=="derived")continue;const q=JSON.parse(r.path);
 const k=q.program===1?"program":q.derive===1?"derive:"+q.recipe?.fn:"walk";kinds[k]=(kinds[k]??0)+1;
 if(k!=="walk")continue;
 add("all",[q.roots,q.requiredSources,q.resources,q.selectors,q.provider,q.view]);
 add("roots",q.roots);add("entry(first root)",q.roots[0]);add("roots+selectors",[q.roots,q.selectors]);add("noView",[q.roots,q.requiredSources,q.resources,q.selectors,q.provider]);
 add("resources",q.resources);add("selectors",q.selectors);add("view",q.view);add("requiredSources",q.requiredSources);add("provider",q.provider);
}
console.log(JSON.stringify(kinds));for(const[k,v]of Object.entries(sets))console.log(k,v.size);
const roots=new Map<string,number>();for(const r of index.rows){if(r.kind!=="derived")continue;const q=JSON.parse(r.path);if(q.walk===1)for(const x of q.roots)roots.set(x,(roots.get(x)??0)+1);}
console.log("distinct root entries",roots.size,"top",[...roots].sort((a,b)=>b[1]-a[1]).slice(0,3).map(([k,v])=>k.slice(-60)+" x"+v));
