// 🏃 Runs the overlay's scratch-host Bazel owner for selected fixture tasks under a genuinely issued process envelope.
// env: BZL_COV_TASKS (comma list), BZL_COV_OPTIONS (extra bazel options), BZL_COV_INTERACTIVE=1, BZL_COV_SKIP_CACHE=1, BZL_COV_JOBS
import {readFileSync} from "node:fs";
const OV="/private/tmp/bzlcov-ov",BASE="/private/tmp/bzlcov",L=OV+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",B=L+"/🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
const routing=await import(OV+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{createNativeBuildCallerCapabilitiesV1}=await import(L+"/🏗️build/📥️caller/🟦️.ts"),{issueRepositoryScriptProvidersV1}=await import(L+"/🏃️process/🔌️providers/🟦️.ts"),api=await import(B);
issueRepositoryScriptProvidersV1(process.env);
const root=OV,graph=JSON.parse(readFileSync(BASE+"/graph.json","utf8")),tasks=(process.env.BZL_COV_TASKS??"").split(",").filter(Boolean);
if(!tasks.length)throw Error("BZL_COV_TASKS required");
const first=tasks[0]!,capabilities=createNativeBuildCallerCapabilitiesV1(root,["run",first],"bazel",{});
let code=1;const started=performance.now();
await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"bzl-coverage:harness",maximumElapsedMilliseconds:3300000},capabilities,Date.now()),async original=>{
 const run=async(invocation:any)=>{
  const phase=routing.createIssuedResourceControlV1(invocation,"bazel:coverage",0);
  code=await api.runBazelTasksScratchV1(root,graph,{tasks,args:[],parallel:Number(process.env.BZL_COV_JOBS??4),skipCache:process.env.BZL_COV_SKIP_CACHE==="1"},invocation,phase,{bazel:"/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel",workspace:BASE+"/ws",outputBase:BASE+"/ob",installBase:BASE+"/install",diskCache:BASE+"/disk",environment:{...process.env},interactive:process.env.BZL_COV_INTERACTIVE==="1",options:(process.env.BZL_COV_OPTIONS??"").split(" ").filter(Boolean)});
 };
 await run(original);
});
console.log(JSON.stringify({tasks,code,seconds:Math.round((performance.now()-started)/100)/10}));
process.exitCode=code;
