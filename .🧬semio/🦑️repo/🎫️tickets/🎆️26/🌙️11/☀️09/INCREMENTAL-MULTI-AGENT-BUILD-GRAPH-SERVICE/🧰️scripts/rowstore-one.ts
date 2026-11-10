import{readFileSync}from"node:fs";import{join,resolve}from"node:path";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const memo=await import(join(B,"🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"));
const pseudo=JSON.parse(readFileSync(process.argv[2],"utf8")) as [string,any][];
const derive:any=await memo.openPlannerDeriverV1(process.cwd());
for(const[,row]of pseudo.slice(Number(process.argv[3]??0),Number(process.argv[3]??0)+3)){try{const a=await derive(row,undefined,{transient:true});console.log("ok",String(a.fact).slice(0,12));}catch(e:any){console.log("ERR",String(e?.stack??e).slice(0,600));}}
process.exit(0);
