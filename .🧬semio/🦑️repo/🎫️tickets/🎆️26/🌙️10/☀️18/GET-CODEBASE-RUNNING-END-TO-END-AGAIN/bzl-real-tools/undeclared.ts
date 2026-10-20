// 🔬 Lists the TypeScript modules a task's entry script imports (bun build metafile) that no declared input pattern covers.
import {mkdirSync} from "node:fs";
import {join,relative,resolve} from "node:path";
const root=process.cwd(),[entryRel,...ids]=process.argv.slice(2);
const B="../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
const api=await import(B);
const graph=JSON.parse(await Bun.file(join(import.meta.dir,"..","🗑️generated","bzl-breadth","graph.json")).text());
const patterns:string[]=[];
for(const id of ids){const plan=api.planBazelTasksV1(graph,{tasks:[id],args:[]});patterns.push(...api.bazelTaskInputsV1(graph,plan.tasks[id]).filter((v:unknown)=>typeof v==="string"));}
const meta=join(root,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/bzl-real/meta-"+Math.abs([...entryRel].reduce((h,c)=>h*31+c.charCodeAt(0)|0,7))+".json");
const result=Bun.spawnSync(["bun","build","--target=bun",entryRel,"--outfile=/dev/null","--metafile="+meta],{cwd:root});
if(result.exitCode)throw Error(new TextDecoder().decode(result.stderr));
const inputs=Object.keys(JSON.parse(await Bun.file(meta).text()).inputs).filter(p=>!p.includes("node_modules")&&!p.startsWith("..")&&!p.startsWith("/")),positive=patterns.filter(p=>!p.startsWith("!")),negative=patterns.filter(p=>p.startsWith("!")).map(p=>p.slice(1));
const covered=(file:string)=>positive.some(p=>new Bun.Glob(p.replace(/^\{workspaceRoot\}\//,"")).match(file))&&!negative.some(p=>new Bun.Glob(p).match(file));
const impl=new Set(JSON.parse(await Bun.file("/private/tmp/bzlreal/ws/implementation.json").text()).files.map((f:any)=>typeof f==="string"?f:f.path));
const undeclared=inputs.filter(f=>!covered(f));
console.log(JSON.stringify({entry:entryRel,imported:inputs.length,undeclared:undeclared.length,outsideImplementation:undeclared.filter(f=>!impl.has(f)).length,sample:undeclared.map(f=>[require("node:fs").statSync(f).mtimeMs,f]).sort((a,b)=>a[0]-b[0]).filter(r=>!impl.has(r[1])).slice(0,25).map(r=>new Date(r[0]).toISOString().slice(0,16)+" "+r[1])},null,1));
