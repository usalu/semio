/** 🧫️ The child of the excluded-names oracle: plans the project-rooted fixture once with the real Bazel planner functions; started under the witness preload the finished run leaves its witness behind. */
import {mkdirSync,realpathSync,writeFileSync}from"node:fs";
import {join}from"node:path";
import corpus from"./🔣️.json";
import {worldWitnessV1}from"../../../../../../🔭️witness/🟨️.mjs";

const[root,workspace,repo]=process.argv.slice(2) as [string,string,string];
const library=join(repo,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),routing=join(repo,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const api=await import(join(import.meta.dirname,"../../../../📜️script.ts")),{createNativeBuildCallerCapabilitiesV1}=await import(join(library,"🏗️build/📥️caller/🟦️.ts")),{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1,scriptInvocationEnvironment}=await import(routing);
mkdirSync(workspace,{recursive:true});
const snapshot=join(workspace,"implementation.json"),runner=join(workspace,"📜️script.ts");writeFileSync(snapshot,"{}");writeFileSync(runner,"//");
const graph=corpus.graph as never,request=corpus.request as {tasks:string[]},plan=api.planBazelTasksV1(graph,request);
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:excluded-names-oracle",maximumElapsedMilliseconds:120000},createNativeBuildCallerCapabilitiesV1(realpathSync(root),["run",request.tasks[0]!],"bazel",{}),Date.now()),async(original:never)=>{
 process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=scriptInvocationEnvironment(original,{}).SEMIO_SCRIPT_PROCESS_INVOCATION;
 await api.generateBazelWorkspaceV1(realpathSync(root),graph,request,plan,workspace,original,createIssuedResourceControlV1(original,"bazel:excluded-names-oracle",0),{version:1,snapshot,runner,sha256:"a".repeat(64),loaderSha256:"b".repeat(64)},()=>{});
});
worldWitnessV1()?.finish();
