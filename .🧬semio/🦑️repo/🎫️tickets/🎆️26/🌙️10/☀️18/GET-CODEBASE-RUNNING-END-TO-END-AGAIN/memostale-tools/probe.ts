/** 🔬️ Finds which recheck facts of the newest sealed graph memo no longer hold: identities by the memo's own rule, rows one by one with the engine's observationsHold. Usage: bun probe.ts <memo directory> <root> */
import {readdirSync,readFileSync,statSync} from "node:fs";
import {join} from "node:path";
const [directory,root]=process.argv.slice(2) as [string,string];
const L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {memoIdentityHoldsV1}=await import(L+"/🧭️invocation/🕸️graph/🧠️memo/🟦️.ts");
const {observationsHold}=await import(L+"/👁️observation/🟨️.mjs");
const newest=readdirSync(directory).filter(name=>name.endsWith(".chk")).sort((a,b)=>statSync(join(directory,b)).mtimeMs-statSync(join(directory,a)).mtimeMs)[0]!;
const recheck=JSON.parse(readFileSync(join(directory,newest),"utf8")) as {rows:{kind:string;path:string;fact:unknown}[];identities:{path:string}[]};
for(const identity of recheck.identities)if(!memoIdentityHoldsV1(root,identity as never))console.log("[DEBUG] identity changed",JSON.stringify(identity));
const kinds=new Map<string,number>();let bad=0;
for(let index=0;index<recheck.rows.length;index+=256){const chunk=recheck.rows.slice(index,index+256);if(await observationsHold(chunk,64,new Set(),true))continue;for(const row of chunk)if(!await observationsHold([row],1,new Set(),true)){bad++;kinds.set(row.kind,(kinds.get(row.kind)??0)+1);if(bad<=25)console.log("[DEBUG] stale row",JSON.stringify(row).slice(0,400));}}
console.log("[DEBUG] rows",recheck.rows.length,"stale",bad,JSON.stringify([...kinds]));
