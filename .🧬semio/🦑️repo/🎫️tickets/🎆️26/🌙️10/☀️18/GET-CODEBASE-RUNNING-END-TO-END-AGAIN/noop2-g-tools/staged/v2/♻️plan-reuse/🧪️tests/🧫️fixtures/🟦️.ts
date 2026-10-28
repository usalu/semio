/** 🧫️ The child of the oracle: plans the scope fixture once with the real Bazel planner functions in the workspace it is given; started under the witness preload the finished run leaves its witness behind. */
import {mkdirSync,realpathSync,writeFileSync}from"node:fs";
import {join}from"node:path";
import corpus from"../../../🎬️scopes/🧪️tests/🧱️workspace/🔣️.json";
import {worldWitnessV1}from"../../../../../🔭️witness/🟨️.mjs";

const[root,workspace,extra,repo]=process.argv.slice(2) as [string,string,string|undefined,string];
const library=join(repo,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),routing=join(repo,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const api=await import(join(import.meta.dirname,"../../../📜️script.ts")),{createNativeBuildCallerCapabilitiesV1}=await import(join(library,"🏗️build/📥️caller/🟦️.ts")),{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1,scriptInvocationEnvironment}=await import(routing);
const implementation=(file:string,sha:string)=>{const value={version:1 as const,snapshot:join(workspace,file),runner:join(workspace,"📜️script.ts"),sha256:sha,loaderSha256:"b".repeat(64)};writeFileSync(value.snapshot,"{}");writeFileSync(value.runner,"//");return value;};
mkdirSync(workspace,{recursive:true});
const graph=corpus.graph as never,request={...(corpus.request as object),...(extra?JSON.parse(extra):{})} as never,plan=api.planBazelTasksV1(graph,request),full=implementation("implementation.json","a".repeat(64));
const capabilities=createNativeBuildCallerCapabilitiesV1(realpathSync(root),["run",(corpus.request as {tasks:string[]}).tasks[0]],"bazel",{});
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:plan-reuse-oracle",maximumElapsedMilliseconds:120000},capabilities,Date.now()),async(original:never)=>{
 process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=scriptInvocationEnvironment(original,{}).SEMIO_SCRIPT_PROCESS_INVOCATION;
 await api.generateBazelWorkspaceV1(realpathSync(root),graph,request,plan,workspace,original,createIssuedResourceControlV1(original,"bazel:plan-reuse-oracle",0),full,()=>{});
});
worldWitnessV1()?.finish();
