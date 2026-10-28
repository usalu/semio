/** 🧫️ Planner child of the planfp bench: plans one synthetic project with the real Bazel planner functions (`defer`, `capture`) or runs only the publisher's refresh (`refresh`); under the witness preload it leaves its witness behind. usage: bun bench-child.ts <repo> <checkout> <workspace> <mode> */
import {mkdirSync,realpathSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const[repo,root,workspace,mode]=process.argv.slice(2) as [string,string,string,"defer"|"capture"|"refresh"];
const library=join(repo,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),routing=join(repo,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const api=await import(join(library,"🏗️build/🔧️tools/🏗️bazel/📜️script.ts")),{createNativeBuildCallerCapabilitiesV1}=await import(join(library,"🏗️build/📥️caller/🟦️.ts")),{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1,scriptInvocationEnvironment}=await import(routing),{worldWitnessV1}=await import(join(library,"🏗️build/🔭️witness/🟨️.mjs"));
const corpus=(await import(join(library,"🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🧱️workspace/🔣️.json"),{with:{type:"json"}})).default,graph={...corpus.graph,projects:{gen:corpus.graph.projects.gen},dependencies:[]} as never,request={tasks:["gen:generate"],args:[]} as never,plan=api.planBazelTasksV1(graph,request);
const implementation={version:1 as const,snapshot:join(workspace,"implementation.json"),runner:join(workspace,"📜️script.ts"),sha256:"a".repeat(64),loaderSha256:"b".repeat(64)};
if(mode!=="refresh"){mkdirSync(workspace,{recursive:true});writeFileSync(implementation.snapshot,"{}");writeFileSync(implementation.runner,"//");}
const started=performance.now();
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:planfp-bench",maximumElapsedMilliseconds:600000},createNativeBuildCallerCapabilitiesV1(realpathSync(root),["run","gen:generate"],"bazel",{}),Date.now()),async(original:never)=>{
 process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=scriptInvocationEnvironment(original,{}).SEMIO_SCRIPT_PROCESS_INVOCATION;
 if(mode==="refresh")await api.refreshBazelWorkspaceSourcesV1(realpathSync(root),workspace,Object.values(plan.tasks as Record<string,{label:string}>).map(task=>task.label),original);
 else await api.generateBazelWorkspaceV1(realpathSync(root),graph,request,plan,workspace,original,createIssuedResourceControlV1(original,"bazel:planfp-bench",0),implementation,()=>{},undefined,undefined,undefined,undefined,{},mode);
});
console.log(JSON.stringify({ms:Math.round(performance.now()-started),rss:Math.round(process.memoryUsage().rss/1048576)}));
worldWitnessV1()?.finish();
