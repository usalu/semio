import{readFileSync,readdirSync,statSync}from"node:fs";
import{join,resolve}from"node:path";
import{createHash}from"node:crypto";
import{heapStats}from"bun:jsc";
import{writeFileSync}from"node:fs";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),memo=await import(join(B,"🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
const index=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
const derived=index.rows.filter(r=>r.kind==="derived"),count=Number(process.argv[2]??20),step=Math.max(1,Math.floor(derived.length/count)),sample=derived.filter((_,i)=>i%step===0).slice(0,count);
const derive=await memo.openPlannerDeriverV1(process.cwd());const PASS:any=process.env.FAST?{transient:true}:{};
const hot=["🏗️build/🔣️policy.json","🏃️process/🔌️providers/🔣️.json","🏃️process/🔌️providers/🧬️schema/🔣️.json","typescript/lib/typescript.js"];
const fieldRows=new Map();let counter=0;const T0=performance.now();const dump:any[]=[];let same=0,differ=0;const tally={};let ms=0;const kinds={},fileRows=new Map();
const roundApi=await import(join(B,"👁️observation/🟨️.mjs"));const run=process.env.ROUND?roundApi.withObservationRoundV1:(f:any)=>f();
await run(async()=>{for(const row of (process.env.TWICE?[...sample,...sample]:sample)){const t=performance.now();let out;try{out=await derive(row,undefined,PASS);}catch(e){console.log("derive failed",String(e).slice(0,100));continue;}const dt=performance.now()-t;if(process.env.MEM&&(++counter)%50===0)if(counter%100===0){Bun.gc(true);const h=heapStats();console.log("heap",counter,Math.round(h.heapSize/1048576),"MB objects",h.objectCount,"arrays",h.objectTypeCounts.Array,"strings",h.objectTypeCounts.string,"Sets",h.objectTypeCounts.Set,"Maps",h.objectTypeCounts.Map);}console.log("after",counter,"closures rssMB",Math.round(process.memoryUsage().rss/1048576),"elapsed s",Math.round((performance.now()-T0)/1000));if(process.env.PER)console.log("closure",Math.round(dt),"ms rows",out.rows.length);ms+=dt;if(out.fact===row.fact)same++;else differ++;if(process.env.DUMP){const keys=out.rows.map((r:any)=>r.kind+"\0"+r.path+"\0"+JSON.stringify(r.fact)).sort();dump.push({path:row.path.slice(0,60),fact:out.fact,rows:createHash("sha256").update(keys.join("\n")).digest("hex"),count:keys.length,keys:process.env.KEYS?keys:undefined});}
 for(const r of out.rows){if(r.kind==="field")fieldRows.set(r.path+"\0"+r.fact.format+"\0"+r.fact.pointer,r);kinds[r.kind]=(kinds[r.kind]??0)+1;if(r.kind==="file"){const set=fileRows.get(r.path)??new Set();set.add(row.path);fileRows.set(r.path,set);}for(const h of hot)if(r.path.includes(h))(tally[h+"|"+r.kind]??=new Set()).add(row.path);}}
});console.log("answers equal to the recorded fact",same,"differ",differ);console.log(JSON.stringify({sample:sample.length,msTotal:Math.round(ms),kinds}));
for(const[k,v]of Object.entries(tally))console.log(k,v.size);

const rank=[...fileRows].map(([path,set])=>[set.size,path.slice(-80)]).sort((a,b)=>b[0]-a[0]).slice(0,12);console.log("file rows per path (closures of the sample, top):");for(const r of rank)console.log(...r);

const obs=await import(join(B,"👁️observation/🟨️.mjs"));const rows=[...fieldRows.values()];console.log("distinct field rows",rows.length);
for(const label of["cold","again"]){const t=performance.now();const ok=await obs.observationsHold(rows,64);console.log(label,ok,Math.round(performance.now()-t),"ms");}

if(process.env.DUMP)writeFileSync(process.env.DUMP,JSON.stringify(dump));
