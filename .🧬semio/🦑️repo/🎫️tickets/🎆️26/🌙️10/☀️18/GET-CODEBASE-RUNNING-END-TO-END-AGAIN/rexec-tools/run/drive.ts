// 🏃 Runs the scratch-host Bazel owner of the staged adapter for selected tasks of the static graph under an issued envelope; every knob is an environment variable.
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {receiveScriptProcessInvocation,createIssuedResourceControlV1} from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const api=await import(resolve(process.env.REXEC_ADAPTER!)),{issueRepositoryScriptProvidersV1}=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts");
issueRepositoryScriptProvidersV1(process.env);
const graph=JSON.parse(readFileSync(process.env.REXEC_GRAPH!,"utf8")),tasks=(process.env.REXEC_TASKS??"").split(",").filter(Boolean),base=process.env.REXEC_BASE!;
if(!tasks.length||!base)throw Error("REXEC_TASKS and REXEC_BASE required");
const root=process.cwd(),started=performance.now(),options=(process.env.REXEC_OPTIONS??"").split(" ").filter(Boolean);
let code=1;
await receiveScriptProcessInvocation(process.env,async original=>{
 const phase=createIssuedResourceControlV1(original,"bazel:rexec",0);
 code=await api.runBazelTasksScratchV1(root,graph,{tasks,args:[],parallel:Number(process.env.REXEC_JOBS??4)},original,phase,{bazel:join(root,".cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"),workspace:join(base,"ws"),outputBase:join(base,"ob"),installBase:process.env.REXEC_INSTALL??"/private/tmp/rexec/install",diskCache:process.env.REXEC_DISK||undefined,environment:{...process.env},options,bun:process.env.REXEC_BUN||undefined});
});
console.log(JSON.stringify({tasks,code,seconds:Math.round((performance.now()-started)/100)/10}));
process.exitCode=code;
