import{readFileSync,readdirSync,statSync}from"node:fs";
import{join,resolve}from"node:path";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),memo=await import(join(B,"🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
const index=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
const count=Number(process.argv[2]??40),derived=index.rows.filter((r:any)=>r.kind==="derived"),step=Math.max(1,Math.floor(derived.length/count)),sample=derived.filter((_:any,i:number)=>i%step===0).slice(0,count);
const derive:any=await memo.openPlannerDeriverV1(process.cwd());let shown=0;
for(const row of sample){
 const plain=await derive(row,undefined,{}),fast=await derive(row,undefined,{transient:true}),fast2=await derive(row,undefined,{transient:true});if(fast2.fact!==fast.fact)console.log("SECOND PASS DIFFERS");
 if(plain.fact!==fast.fact&&shown++<3){const a=new Set<string>(plain.files),b=new Set<string>(fast.files);const q=JSON.parse(row.path);console.log("DIFF",JSON.stringify({roots:q.roots.map((x:string)=>x.slice(-50)),selectors:q.selectors,resources:q.resources.length,view:q.view}),"onlyPlain",[...a].filter(x=>!b.has(x)).slice(0,4),"onlyFast",[...b].filter(x=>!a.has(x)).slice(0,6),a.size,b.size);}
}
console.log("done");process.exit(0);
