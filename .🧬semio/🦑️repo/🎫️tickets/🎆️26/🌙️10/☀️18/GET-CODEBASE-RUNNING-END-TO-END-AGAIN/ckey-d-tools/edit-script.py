"""Anchored, atomic edits of bazel/📜️script.ts that split the implementation into adapter and executor. Aborts without writing when an anchor is missing."""
import sys

BZ = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
P = BZ + "/📜️script.ts"
t = open(P, encoding="utf-8").read()
orig = t

def sub(old, new, count=1):
    global t
    assert t.count(old) == count, (t.count(old), old[:90])
    t = t.replace(old, new)

old_pub_start = t.index("/** 📦️ Publishes one finite immutable executable input set while the workspace writer is held. */")
old_pub_end = t.index("\n}\n", old_pub_start) + 3
new_pub = '''/** 🧊️ Both captured executable closures of a workspace: the adapter implementation of the planner and of orchestrated tasks, and the narrow executor implementation every other action runs. */
export interface BazelImplementationsV1{implementation:BazelImplementationV1;executor:BazelImplementationV1}

/** 🧊️ Both closures with the planned workspace and tasks they were generated for. */
export type BazelPublicationV1=BazelImplementationsV1&{workspace:string;plan:BazelTaskPlanV1};

/** 🎬️ Names the entry module of the narrow executor implementation, relative to the checkout. */
export function bazelExecutorEntryV1(root:string):string{return slash(relative(root,join(dirname(import.meta.filename),"🎬️executor","🟦️.ts")));}

/** 🗺️ Tells whether a task orchestrates children and therefore runs in the adapter implementation, which alone carries the planner. */
export function bazelTaskOrchestratedV1(graph:BuildGraphV1,task:BazelTaskV1):boolean{
 const delegation=(graph.projects[task.project]?.targets[task.target]?.metadata as {semio?:{execution?:{kind?:string;children?:unknown}}}|undefined)?.semio?.execution;
 return delegation?.kind==="orchestration"&&delegation.children!==undefined;
}

/** 📦️ Publishes both finite immutable executable input sets while the workspace writer is held. */
export async function publishBazelImplementationV1(root:string,workspace:string,original:ScriptInvocation):Promise<BazelImplementationsV1>{
 const publish=async(entry:string,file:string)=>{const captured=await captureBuildSourceClosureV1(root,entry,original),value:BazelImplementationV1={version:1,snapshot:join(workspace,file),runner:join(workspace,"📜️script.ts"),sha256:buildSourceClosureIdentityV1(captured),loaderSha256:hash(buildSourceClosureLoaderV1)};requireBazelImplementationV1(value,value);await writeBazelImmutableInputV1(value.snapshot,JSON.stringify(captured),25165824,original);return value;};
 const implementation=await publish(slash(relative(root,import.meta.filename)),"implementation.json"),executor=await publish(bazelExecutorEntryV1(root),"executor.json");
 await writeBazelImmutableInputV1(implementation.runner,buildSourceClosureLoaderV1,25165824,original);
 return {implementation,executor};
}
'''
t = t[:old_pub_start] + new_pub + t[old_pub_end:]

sub("Promise<{workspace:string;plan:BazelTaskPlanV1;implementation:BazelImplementationV1}>", "Promise<BazelPublicationV1>", 3)
sub("const implementation=await publishBazelImplementationV1(root,workspace,invocation),input=", "const {implementation,executor}=await publishBazelImplementationV1(root,workspace,invocation),input=")
sub("JSON.parse(JSON.stringify({version:1,root,workspace,graph,request,implementation}))", "JSON.parse(JSON.stringify({version:1,root,workspace,graph,request,implementation,executor}))")
sub("JSON.stringify(result.implementation)!==JSON.stringify(implementation)||", "JSON.stringify(result.implementation)!==JSON.stringify(implementation)||JSON.stringify(result.executor)!==JSON.stringify(executor)||")
sub("phaseControl:ResourceLeaseControl,implementation:BazelImplementationV1,recheck:()=>void,workspace:string)", "phaseControl:ResourceLeaseControl,implementation:BazelImplementationV1,executor:BazelImplementationV1,recheck:()=>void,workspace:string)")
sub("phaseControl,implementation,recheck,undefined,bazelExecutionPlanV1(", "phaseControl,implementation,executor,recheck,undefined,bazelExecutionPlanV1(")
sub("implementation:BazelImplementationV1,recheck:()=>void,closureFetch?:BazelClosureFetchV1", "implementation:BazelImplementationV1,executor:BazelImplementationV1,recheck:()=>void,closureFetch?:BazelClosureFetchV1")
sub('if(dirname(implementation.snapshot)!==workspace)throw new Error("Captured Bazel planner workspace changed");', 'if(dirname(implementation.snapshot)!==workspace||dirname(executor.snapshot)!==workspace)throw new Error("Captured Bazel planner workspace changed");')
sub("const metadata=graph.projects[task.project]?.targets[task.target]?.metadata as {semio?:{execution?:{kind?:string;children?:unknown}}}|undefined,delegation=metadata?.semio?.execution,orchestration=delegation?.kind===\"orchestration\"&&delegation.children!==undefined?buildOrchestrationContextV1(", "const orchestration=bazelTaskOrchestratedV1(graph,task)?buildOrchestrationContextV1(")
sub("if(orchestration){if(!authorized){", "if(orchestration){orchestrated.add(action.id);if(!authorized){")
sub("let authorized:Set<string>|undefined;const trees=", "const orchestrated=new Set<string>();let authorized:Set<string>|undefined;const trees=")
sub('await writeChanged(join(workspace,task.label+".json"),JSON.stringify(bazelManifestV1(task,implementation,orchestration)));', 'const chosen=orchestration?implementation:executor;await writeChanged(join(workspace,task.label+".json"),JSON.stringify(bazelManifestV1(task,chosen,orchestration)));')
sub("action.cache.cache,implementation.sha256+implementation.loaderSha256,exactClosures,resolutions)", "action.cache.cache,chosen.sha256+chosen.loaderSha256,exactClosures,resolutions)")
sub("bun:process.execPath,sourceIdentity:implementation.sha256,extension:", "bun:process.execPath,sourceIdentity:implementation.sha256,executorIdentity:executor.sha256,extension:")
sub("...(sources.has(action.id)?{sources:sources.get(action.id)}:{}),", "...(sources.has(action.id)?{sources:sources.get(action.id)}:{}),...(orchestrated.has(action.id)?{orchestrated:true as const}:{}),")
sub("return {workspace,plan,implementation};\n}", "return {workspace,plan,implementation,executor};\n}")

sub("renderBazelBuildV1(input:{actions:readonly (BazelActionV1&{sources?:string})[];groups:ReadonlyMap<string,BazelSourceGroupV1>;bun:string;sourceIdentity:string;", "renderBazelBuildV1(input:{actions:readonly (BazelActionV1&{sources?:string;orchestrated?:true})[];groups:ReadonlyMap<string,BazelSourceGroupV1>;bun:string;sourceIdentity:string;executorIdentity:string;")
sub('["runner",quote("📜️script.ts")],["source",quote("implementation.json")],["source_identity",quote(input.sourceIdentity)]', '["runner",quote("📜️script.ts")],["source",quote(file)],["source_identity",quote(identity)]')
sub('list(["📜️script.ts","implementation.json",input.sourceIdentity,"birth-test"', 'list(["📜️script.ts",file,identity,"birth-test"')
sub('const launch=action.kind==="launch",reason=', 'const file=action.orchestrated?"implementation.json":"executor.json",identity=action.orchestrated?input.sourceIdentity:input.executorIdentity,launch=action.kind==="launch",reason=')

sub("const {workspace,plan,implementation}=await publishBazelGraphV1(", "const {workspace,plan,implementation,executor}=await publishBazelGraphV1(")
sub("for(const task of Object.values(plan.tasks).filter(task=>task.definition.continuous)){\n", "for(const task of Object.values(plan.tasks).filter(task=>task.definition.continuous)){\n    const chosen=bazelTaskOrchestratedV1(graph,task)?implementation:executor;\n")
sub('"--",implementation.runner,implementation.snapshot,implementation.sha256,"birth-launch"', '"--",chosen.runner,chosen.snapshot,chosen.sha256,"birth-launch"')

sub("requireBazelImplementationV1(value.implementation,implementation);", 'requireBazelImplementationV1(value.implementation,implementation);const executor=value.executor as BazelImplementationV1;requireBazelImplementationV1(executor,executor);if(executor.snapshot!==join(dirname(implementation.snapshot),"executor.json")||executor.runner!==implementation.runner||executor.loaderSha256!==implementation.loaderSha256||buildSourceClosureIdentityV1(await captureBuildSourceClosureV1(value.root,bazelExecutorEntryV1(value.root),original))!==executor.sha256)throw new Error("Exact captured Bazel executor implementation required");')
sub("original,storage,actualPhase,implementation,recheck,value.workspace)", "original,storage,actualPhase,implementation,executor,recheck,value.workspace)")

if t == orig:
    raise SystemExit("nothing changed")
cur = open(P, encoding="utf-8").read()
assert cur == orig, "script.ts changed while editing; rerun"
open(P + ".tmp", "w", encoding="utf-8").write(t)
import os
os.replace(P + ".tmp", P)
print("edited")
