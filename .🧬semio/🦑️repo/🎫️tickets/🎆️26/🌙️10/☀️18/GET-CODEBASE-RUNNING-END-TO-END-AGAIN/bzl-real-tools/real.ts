// 🏃 Runs the staged scratch-host Bazel owner for selected tasks of the static graph under an issued envelope.
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {receiveScriptProcessInvocation,createIssuedResourceControlV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const api=await import("./staged/adapter.ts"),{issueRepositoryScriptProvidersV1}=await import("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts");
issueRepositoryScriptProvidersV1(process.env);
const graph=JSON.parse(readFileSync(process.env.BZL_REAL_GRAPH??join(import.meta.dir,"..","🗑️generated","bzl-breadth","graph.json"),"utf8"));
const tasks=(process.env.BZL_REAL_TASKS??"").split(",").filter(Boolean),base=process.env.BZL_REAL_BASE??"/private/tmp/bzlreal";
if(!tasks.length)throw Error("BZL_REAL_TASKS required");
for(const edge of (process.env.BZL_REAL_EDGES??"").split(",").filter(Boolean)){const[from,to]=edge.split(">"),cut=from!.lastIndexOf(":"),project=Object.values<any>(graph.projects).find((row:any)=>row.name===from!.slice(0,cut)),target=project.targets[from!.slice(cut+1)];target.dependsOn=[...(target.dependsOn??[]),to];}
const root=process.cwd(),started=performance.now(),options=(process.env.BZL_REAL_OPTIONS??"").split(" ").filter(Boolean);
let code=1;
await receiveScriptProcessInvocation(process.env,async original=>{
 const phase=createIssuedResourceControlV1(original,"bazel:real",0);
 code=await api.runBazelTasksScratchV1(root,graph,{tasks,args:[],parallel:Number(process.env.BZL_REAL_JOBS??4),skipCache:process.env.BZL_REAL_SKIP_CACHE==="1"},original,phase,{bazel:join(root,".cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"),workspace:join(base,"ws"),outputBase:join(base,"ob"),installBase:join(base,"install"),diskCache:join(base,"disk"),environment:{...process.env},options});
});
console.log(JSON.stringify({tasks,code,seconds:Math.round((performance.now()-started)/100)/10}));
process.exitCode=code;
