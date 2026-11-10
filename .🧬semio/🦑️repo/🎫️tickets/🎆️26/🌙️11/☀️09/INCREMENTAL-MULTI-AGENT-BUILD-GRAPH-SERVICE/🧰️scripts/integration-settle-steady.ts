import{readFileSync,readdirSync,statSync,mkdtempSync,rmSync}from"node:fs";
import{join,resolve}from"node:path";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),cut=await import(join(G,"🕰️generations/✂️cutoff/🟨️.mjs")),memo=await import(join(B,"🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"));
const cl=await import(join(G,"🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs")),cas=await import(join(B,"🗄️cas/🗃️store/🟦️.ts"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
let index:any=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
const limit=Number(process.argv[2]??1200),derived=index.rows.filter((r:any)=>r.kind==="derived").slice(0,limit);
const pseudoRows=new Map<string,any>();for(const row of derived)pseudoRows.set(cut.derivedOwnerNameV1(row),row);index=undefined;
const deriver:any=await memo.openPlannerDeriverV1(process.cwd()),hold=async()=>true;
const mem=()=>Math.round(process.memoryUsage().rss/1048576);
for(const label of["first","second","third"]){
 const cacheRoot=mkdtempSync(join(resolve(".🧬semio/🦑️repo/⚡️cache/tests"),"steady-"));
 const settler=cl.createClosureSettlerV1({workspace:{workspaceRoot:process.cwd(),cacheRoot,blobs:cas.createCasStoreV1()}});
 const t=performance.now();await settler.settle({pseudoRows,derive:deriver,hold});
 console.log(label,Math.round(performance.now()-t),"ms",Math.round((performance.now()-t)/pseudoRows.size*10)/10,"ms per closure rss",mem());
 rmSync(cacheRoot,{recursive:true,force:true});
}
process.exit(0);
