import{readFileSync,readdirSync,statSync,writeFileSync}from"node:fs";
import{join,resolve}from"node:path";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),cut=await import(join(G,"🕰️generations/✂️cutoff/🟨️.mjs"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
const index:any=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
const derived=index.rows.filter((r:any)=>r.kind==="derived");
writeFileSync(process.argv[2],JSON.stringify(derived.map((r:any)=>[cut.derivedOwnerNameV1(r),r])));
console.log("wrote",derived.length);process.exit(0);
