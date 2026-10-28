/** 🧳️ Read-only: replays the batch's carry step on the live generation store for every sealed memo record (`bun carrylive.ts <module-root> <live-root> <memo-dir>`): from the record's generation to the one the pointer names now, with wall/CPU/RSS. The carry module is loaded from `<module-root>` (a mirror with batch `carry`); nothing is written. */
import {readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
const [moduleRoot,root,directory]=process.argv.slice(2) as [string,string,string];
const {carryGraphMemoPinV1}=await import(join(moduleRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧳️carry/🟦️.ts"));
const generations=join(root,".cache/b/g/m/generations"),pointer=JSON.parse(readFileSync(join(generations,"current.json"),"utf8"));
for(const name of readdirSync(directory).filter(name=>name.endsWith(".json"))){
 const record=JSON.parse(readFileSync(join(directory,name),"utf8"));if(record.state!=="sealed")continue;
 const t=performance.now(),c=process.cpuUsage();
 const answer=await carryGraphMemoPinV1({generations,from:{generation:record.pointer.generation,manifest:record.pointer.manifest},to:{generation:pointer.generation,manifest:pointer.manifest},pin:record.pin});
 const cpu=process.cpuUsage(c);
 console.log(JSON.stringify({record:name.slice(0,12),projects:record.projects,from:record.pointer.generation,to:pointer.generation,answer:answer.carried?{carried:true,covered:JSON.parse(answer.pin).covered.count}:answer,wallMs:Math.round(performance.now()-t),cpuMs:Math.round((cpu.user+cpu.system)/1000),rssMb:Math.round(process.memoryUsage().rss/1048576)}));
}
