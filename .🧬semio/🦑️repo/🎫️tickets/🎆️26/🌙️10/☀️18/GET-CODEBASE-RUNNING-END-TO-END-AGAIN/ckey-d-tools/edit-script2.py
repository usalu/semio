"""Second anchored edit: the narrow action implementation keeps the names `implementation` and `implementation.json`, the wide one becomes `adapter` and `adapter.json`."""
import json, os, re

BZ = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
P = BZ + "/📜️script.ts"
t = open(P, encoding="utf-8").read()
orig = t

def sub(old, new, count=1):
    global t
    assert t.count(old) == count, (t.count(old), old[:100])
    t = t.replace(old, new)

s = t.index("/** 🧊️ Both captured executable closures of a workspace")
e = t.index("\n}\n", t.index("export async function publishBazelImplementationV1")) + 3
new_pub = '''/** 🧊️ Both captured executable closures of a workspace: the narrow implementation that every ordinary action runs, and the adapter that plans and runs the orchestrated tasks. */
export interface BazelImplementationsV1{implementation:BazelImplementationV1;adapter:BazelImplementationV1}

/** 🧊️ The implementation of a generated workspace with its planned tasks; the adapter is named whenever the planner generated it. */
export type BazelPublicationV1={workspace:string;plan:BazelTaskPlanV1;implementation:BazelImplementationV1;adapter?:BazelImplementationV1};

/** 🎬️ Names the entry module of the narrow implementation, relative to the checkout. */
export function bazelImplementationEntryV1(root:string):string{return slash(relative(root,join(dirname(import.meta.filename),"🎬️executor","🟦️.ts")));}

/** 🗺️ Tells whether a task orchestrates children and therefore runs in the adapter, which alone carries the planner. */
export function bazelTaskOrchestratedV1(graph:BuildGraphV1,task:BazelTaskV1):boolean{
 const delegation=(graph.projects[task.project]?.targets[task.target]?.metadata as {semio?:{execution?:{kind?:string;children?:unknown}}}|undefined)?.semio?.execution;
 return delegation?.kind==="orchestration"&&delegation.children!==undefined;
}

/** 🗺️ Refuses an orchestrated task when its workspace was generated without the adapter. */
function requireBazelAdapterV1(adapter:BazelImplementationV1|undefined):BazelImplementationV1{
 if(!adapter)throw new Error("Orchestrated Bazel tasks require the adapter implementation\\nOrchestrierte Bazel-Tasks benötigen die Adapter-Implementierung");
 return adapter;
}

/** 📦️ Publishes both finite immutable executable input sets while the workspace writer is held. */
export async function publishBazelImplementationV1(root:string,workspace:string,original:ScriptInvocation):Promise<BazelImplementationsV1>{
 const publish=async(entry:string,file:string)=>{const captured=await captureBuildSourceClosureV1(root,entry,original),value:BazelImplementationV1={version:1,snapshot:join(workspace,file),runner:join(workspace,"📜️script.ts"),sha256:buildSourceClosureIdentityV1(captured),loaderSha256:hash(buildSourceClosureLoaderV1)};requireBazelImplementationV1(value,value);await writeBazelImmutableInputV1(value.snapshot,JSON.stringify(captured),25165824,original);return value;};
 const adapter=await publish(slash(relative(root,import.meta.filename)),"adapter.json"),implementation=await publish(bazelImplementationEntryV1(root),"implementation.json");
 await writeBazelImmutableInputV1(adapter.runner,buildSourceClosureLoaderV1,25165824,original);
 return {implementation,adapter};
}
'''
t = t[:s] + new_pub + t[e:]

sub("Promise<BazelPublicationV1>{\n requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);await requireBazelStorageV1(root,storage);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const namespace=", "Promise<BazelPublicationV1&Pick<BazelImplementationsV1,\"adapter\">>{\n requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);await requireBazelStorageV1(root,storage);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const namespace=")
sub("const {implementation,executor}=await publishBazelImplementationV1(root,workspace,invocation),input=", "const {adapter,implementation}=await publishBazelImplementationV1(root,workspace,invocation),input=")
sub("JSON.parse(JSON.stringify({version:1,root,workspace,graph,request,implementation,executor}))", "JSON.parse(JSON.stringify({version:1,root,workspace,graph,request,adapter,implementation}))")
sub("await runOwnedCommand(process.execPath,[implementation.runner,implementation.snapshot,implementation.sha256,\"planner\",", "await runOwnedCommand(process.execPath,[adapter.runner,adapter.snapshot,adapter.sha256,\"planner\",")
sub("JSON.stringify(result.executor)!==JSON.stringify(executor)||", "JSON.stringify(result.adapter)!==JSON.stringify(adapter)||")
sub("phaseControl:ResourceLeaseControl,implementation:BazelImplementationV1,executor:BazelImplementationV1,recheck:()=>void,workspace:string)", "phaseControl:ResourceLeaseControl,adapter:BazelImplementationV1,implementation:BazelImplementationV1,recheck:()=>void,workspace:string)")
sub("phaseControl,implementation,executor,recheck,undefined,bazelExecutionPlanV1(", "phaseControl,implementation,recheck,undefined,bazelExecutionPlanV1(")
sub("process.env),storage.paths.pool);\n}", "process.env),storage.paths.pool,adapter);\n}")
sub("implementation:BazelImplementationV1,executor:BazelImplementationV1,recheck:()=>void,closureFetch?:BazelClosureFetchV1,execution?:BazelExecutionPlanV1,pool?:string)", "implementation:BazelImplementationV1,recheck:()=>void,closureFetch?:BazelClosureFetchV1,execution?:BazelExecutionPlanV1,pool?:string,adapter?:BazelImplementationV1)")
sub("if(dirname(implementation.snapshot)!==workspace||dirname(executor.snapshot)!==workspace)", "if(dirname(implementation.snapshot)!==workspace||adapter!==undefined&&dirname(adapter.snapshot)!==workspace)")
sub("const chosen=orchestration?implementation:executor;", "const chosen=orchestration?requireBazelAdapterV1(adapter):implementation;")
sub("sourceIdentity:implementation.sha256,executorIdentity:executor.sha256,extension:", "sourceIdentity:implementation.sha256,...(adapter?{adapterIdentity:adapter.sha256}:{}),extension:")
sub("return {workspace,plan,implementation,executor};\n}", "return {workspace,plan,implementation,...(adapter?{adapter}:{})};\n}")
sub("bun:string;sourceIdentity:string;executorIdentity:string;", "bun:string;sourceIdentity:string;adapterIdentity?:string;")
sub("trees?:readonly string[]}):string{\n", "trees?:readonly string[]}):string{\n if(input.actions.some(action=>action.orchestrated)&&!input.adapterIdentity)throw new Error(\"Orchestrated Bazel actions require the adapter identity\\nOrchestrierte Bazel-Aktionen benötigen die Adapter-Identität\");\n")
sub('const file=action.orchestrated?"implementation.json":"executor.json",identity=action.orchestrated?input.sourceIdentity:input.executorIdentity,launch=', 'const file=action.orchestrated?"adapter.json":"implementation.json",identity=action.orchestrated?input.adapterIdentity!:input.sourceIdentity,launch=')
sub("const {workspace,plan,implementation,executor}=await publishBazelGraphV1(", "const {workspace,plan,implementation,adapter}=await publishBazelGraphV1(")
sub("const chosen=bazelTaskOrchestratedV1(graph,task)?implementation:executor;", "const chosen=bazelTaskOrchestratedV1(graph,task)?requireBazelAdapterV1(adapter):implementation;")

old_old = "export async function runCapturedBuildSourceV1"
rs = t.index(old_old)
re_ = t.index("\n}\n", rs) + 3
fn = t[rs:re_]
fn2 = fn
def fsub(a, b, c=1):
    global fn2
    assert fn2.count(a) == c, (fn2.count(a), a[:80])
    fn2 = fn2.replace(a, b)
fsub("const implementation=bazelCapturedImplementationV1(proof),recheck=proof!.recheck;", "const adapter=bazelCapturedImplementationV1(proof),recheck=proof!.recheck;")
fsub("dirname(input)!==dirname(implementation.snapshot)", "dirname(input)!==dirname(adapter.snapshot)")
a = fn2.index("requireBazelImplementationV1(value.implementation,implementation);")
b = fn2.index('throw new Error("Exact captured Bazel executor implementation required");') + len('throw new Error("Exact captured Bazel executor implementation required");')
fn2 = fn2[:a] + 'requireBazelImplementationV1(value.adapter,adapter);const implementation=value.implementation as BazelImplementationV1;requireBazelImplementationV1(implementation,implementation);if(implementation.snapshot!==join(dirname(adapter.snapshot),"implementation.json")||implementation.runner!==adapter.runner||implementation.loaderSha256!==adapter.loaderSha256||buildSourceClosureIdentityV1(await captureBuildSourceClosureV1(value.root,bazelImplementationEntryV1(value.root),original))!==implementation.sha256)throw new Error("Exact captured Bazel implementation required");' + fn2[b:]
fsub("value.workspace!==dirname(implementation.snapshot)", "value.workspace!==dirname(adapter.snapshot)")
fsub("actualPhase,implementation,executor,recheck,value.workspace)", "actualPhase,adapter,implementation,recheck,value.workspace)")
fsub("await runBazelActionEntryV1(argv,implementation,recheck);", "await runBazelActionEntryV1(argv,adapter,recheck);")
t = t[:rs] + fn2 + t[re_:]
t = re.sub(r"\n{3,}", "\n\n", t)
t = t.rstrip("\n") + "\n"

cur = open(P, encoding="utf-8").read()
assert cur == orig, "script.ts changed while editing; rerun"
open(P + ".tmp", "w", encoding="utf-8").write(t)
os.replace(P + ".tmp", P)

S = BZ + "/🧬️schema/🔣️.json"
j = json.loads(open(S, encoding="utf-8").read())
pl = j["definitions"]["Planner"]
def ren(node):
    node["properties"] = {("adapter" if k == "executor" else k): v for k, v in node["properties"].items()}
    node["required"] = ["adapter" if k == "executor" else k for k in node["required"]]
ren(pl)
ren(pl["definitions"]["Result"])
open(S, "w", encoding="utf-8").write(json.dumps(j, indent=2, ensure_ascii=False) + "\n")
print("edited")
