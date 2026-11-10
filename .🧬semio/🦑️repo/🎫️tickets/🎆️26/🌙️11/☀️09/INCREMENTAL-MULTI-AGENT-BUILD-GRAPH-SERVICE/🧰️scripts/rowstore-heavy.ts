import{readFileSync}from"node:fs";
import{join,resolve}from"node:path";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const memo=await import(join(B,"🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"));
const pseudo=JSON.parse(readFileSync(process.argv[2],"utf8")) as [string,any][];
const heavy=pseudo.filter(([,r])=>{const q=JSON.parse(r.path);return q.program===1||q.derive===1;}).slice(0,Number(process.argv[3]??2));
console.log("heavy questions",heavy.length);
const derive:any=await memo.openPlannerDeriverV1(process.cwd());
for(const[name,row]of heavy){const t=performance.now(),a=await derive(row);console.log("[HEAVY]",name.slice(0,30),"ms",Math.round(performance.now()-t),"rssMB",Math.round(process.memoryUsage().rss/1048576),"fact",String(a.fact).slice(0,16),"rows",a.rows.length,"digest",Bun.hash(JSON.stringify(a.rows)).toString(16));}
process.exit(0);
