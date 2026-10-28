/** ⏱️ Measures the pruned whole-checkout walk the staged universe adds for print's plan: pruned names from the live schema-index task's negations, then a 16-lane readdir walk like the witness does (read-only). usage: bun walk-cost.ts <mirror-root> <task.json> <repo> */
import {readdir} from "node:fs/promises";
import {join} from "node:path";
const [mirror,task,repo]=process.argv.slice(2) as [string,string,string];
const {bazelPrunedNamesV1,bazelUniversePrunesV1}=await import(join(mirror,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌌️universe/🟦️.ts"));
const inputs=(await Bun.file(task).json()).task.definition.inputs as unknown[];
const negations=inputs.filter((v):v is string=>typeof v==="string"&&v.startsWith("!")).map(v=>v.slice(1).replaceAll("{workspaceRoot}/",""));
const whole=bazelPrunedNamesV1(negations);console.log(JSON.stringify(whole));
const excluded=new Set([".git",".🧬semio",".cache",".nx","node_modules","target","dist","build","coverage",".venv","__pycache__"]);
const started=performance.now(),pending=[""],seen=new Set<string>();let dirs=0;
const lane=async()=>{while(pending.length){const d=pending.pop()!;if(seen.has(d))continue;seen.add(d);dirs++;let entries;try{entries=await readdir(join(repo,d),{withFileTypes:true});}catch{continue;}for(const e of entries)if(e.isDirectory()&&!excluded.has(e.name)&&!bazelUniversePrunesV1(whole,e.name))pending.push(d?d+"/"+e.name:e.name);if(process.memoryUsage().rss>500*2**20)throw Error("rss");}};
await Promise.all(Array.from({length:16},lane));
console.log(JSON.stringify({dirs,ms:Math.round(performance.now()-started),rssMb:Math.round(process.memoryUsage().rss/2**20)}));
