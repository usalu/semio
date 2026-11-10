import{readFileSync,readdirSync,statSync}from"node:fs";
import{join,resolve}from"node:path";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),memo=await import(join(B,"🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
const index=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
const derived=index.rows.filter(r=>r.kind==="derived");const derive=await memo.openPlannerDeriverV1(process.cwd());
const kindsOfQuestion={};
for(const row of derived){const q=JSON.parse(row.path);const k=q.program===1?"program":q.derive===1?"derive:"+q.recipe?.fn:"walk";kindsOfQuestion[k]=(kindsOfQuestion[k]??0)+1;}
console.log(JSON.stringify(kindsOfQuestion));
for(const want of ["derive:crate-index","derive:source-roots","walk","program"]){
 const row=derived.find(r=>{const q=JSON.parse(r.path);return(q.program===1?"program":q.derive===1?"derive:"+q.recipe?.fn:"walk")===want;});if(!row)continue;
 const out=await derive(row);const kinds={};const files=[];for(const r of out.rows){kinds[r.kind]=(kinds[r.kind]??0)+1;if(r.kind==="file")files.push(r.path);}
 console.log(want,JSON.stringify(kinds),"file sample",files.slice(0,3).map(f=>f.slice(-60)),"names",new Set(files.map(f=>f.split("/").pop())).size);
 const names={};for(const f of files){const n=f.split("/").pop();names[n]=(names[n]??0)+1;}console.log(JSON.stringify(Object.entries(names).sort((a,b)=>b[1]-a[1]).slice(0,5)));
}
