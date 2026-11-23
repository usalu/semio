import {readFileSync} from "node:fs";
import {pathToFileURL} from "node:url";
const root="C:\\semio\\🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\📚️library\\🏗️build\\👁️observation\\";
const variant=process.argv[3]??"patched",kinds=new Set((process.argv[4]??"lstat,exists,fileExists,directoryExists,readdir").split(","));
const observation=await import(pathToFileURL(root+(variant==="baseline"?"🟨️.baseline-perf.mjs":"🟨️.mjs")).href);
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const rows=(memo.observations as {kind:string;path:string}[]).filter(r=>kinds.has(r.kind));
const dropped=new Set<any>();let ok=false,attempts=0;
const t=performance.now();
for(;attempts<10;attempts++){
 ok=await observation.observationsHoldWithinRoundV1(rows.filter(r=>!dropped.has(r)),64,undefined,(row:any)=>{dropped.add(rows.find(r=>r.kind===row.kind&&r.path===row.path));});
 if(ok)break;
}
const cpu=process.cpuUsage();
console.log(JSON.stringify({variant,kinds:[...kinds].join(","),rows:rows.length,ok,attempts,wallMs:Math.round(performance.now()-t),cpuUserMs:Math.round(cpu.user/1000),cpuSystemMs:Math.round(cpu.system/1000)}));
