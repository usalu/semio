#!/usr/bin/env python3
"""Applies the hot-path batch to the live tree: `apply.py [--write]` (dry run by default; every anchor must occur exactly once, every docstring emoji of a touched file must stay unique)."""
import os, re, subprocess, sys

sys.path.insert(0, os.path.dirname(__file__))
import patch_authorizers

C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/noop-g-tools/staged/"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B = L + "🏗️build/"
A = L + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/"
write = "--write" in sys.argv
edits: dict[str, str] = {}
created: dict[str, str] = {}


def read(path: str) -> str:
    return open(path, encoding="utf8").read()


def replace_once(path: str, pairs: list[tuple[str, str]]) -> None:
    text = edits.get(path) or read(path)
    for old, new in pairs:
        if text.count(old) != 1:
            raise SystemExit("anchor count %d in %s: %s" % (text.count(old), path, old[:100]))
        text = text.replace(old, new)
    edits[path] = text


edits[A + "🟦️.ts"] = patch_authorizers.patch(read(A + "🟦️.ts"))
created[A + "🧊️memo/🟦️.ts"] = read(C + "memo.ts")
created[A + "🧪️tests/🧊️memo/🟦️.ts"] = read(C + "memo-test.ts")
created[B + "⌨️command/🧪️tests/🌱️plan-child/🟦️.ts"] = read(C + "plan-child-test.ts")

replace_once(B + "📥️source/🔏️closure/🏗️compiler/🟦️.ts", [
    ("export async function captureCompilerBuildSourceClosureV1(workspace:string,entry:string,original:ScriptInvocation,control:ResourceLeaseControl):Promise<BuildSourceClosureV1>{",
     "export async function captureCompilerBuildSourceClosureV1(workspace:string,entry:string,original:ScriptInvocation,control:ResourceLeaseControl,memo?:Readonly<{file:string;served?:(served:\"memo\"|\"capture\")=>void}>):Promise<BuildSourceClosureV1>{"),
    ("captured=await captureNxAuthorizerSourcesV1(root,entry,control.signal,{installed:true});",
     "captured=await captureNxAuthorizerSourcesV1(root,entry,control.signal,{installed:true,...(memo?{memo}:{})});"),
])

replace_once(B + "📥️source/🔏️closure/🚪️command/🟦️.ts", [
    ('import schema from"../🧬️schema/🔣️.json"with{type:"json"};',
     'import schema from"../🧬️schema/🔣️.json"with{type:"json"};\n'
     'import{profileMarkV1,profilePhaseV1}from"../../../⏱️profile/🟨️.mjs";'),
    ("/** 🏃️ Streams an admitted command with unchanged caller stdin",
     "/** 🧊️ Names the source memo of one capture in the per-user cache, or none when the host switched it off; the profile records whether the capture was served from it. */\n"
     "async function sourceMemoV1(workspace:string,entry:string,environment:Readonly<Record<string,string|undefined>>){const{nxAuthorizerMemoBaseV1,nxAuthorizerMemoFileV1,readNxAuthorizerMemoSwitchV1}=await import(\"../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧊️memo/🟦️.ts\");return readNxAuthorizerMemoSwitchV1(environment)?{file:nxAuthorizerMemoFileV1(nxAuthorizerMemoBaseV1(environment),resolve(workspace),entry,true),served:(served:\"memo\"|\"capture\")=>profileMarkV1(\"source.served\",served)}:undefined;}\n"
     "/** 🏃️ Streams an admitted command with unchanged caller stdin"),
    ("snapshot=await captureCompilerBuildSourceClosureV1(workspace,entry,original,control),text=JSON.stringify(snapshot)",
     "snapshot=await profilePhaseV1(\"source.capture\",async()=>captureCompilerBuildSourceClosureV1(workspace,entry,original,control,await sourceMemoV1(workspace,entry,environment))),text=JSON.stringify(snapshot)"),
])

replace_once(L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts", [
    ("await retainSnapshotBudgetV1(store,directory,path,additionalBytes,leaseDirectory,actualControl,new Set(encoded.records.keys()));",
     "if(additionalBytes>0)await retainSnapshotBudgetV1(store,directory,path,additionalBytes,leaseDirectory,actualControl,new Set(encoded.records.keys()));"),
])

replace_once(B + "⌨️command/⚙️runtime/🟦️.ts", [
    ('if(buildOperationRequiresV1(command)==="dependencies"){const {ensureBuildEnvironmentV1',
     'if(buildOperationRequiresV1(command)==="dependencies"&&process.env[BUILD_GRAPH_PLAN_OUT_V1]===undefined){const {ensureBuildEnvironmentV1'),
])

BZ = B + "🔧️tools/🏗️bazel/📜️script.ts"
replace_once(BZ, [
    ('import {createHash,randomUUID} from "node:crypto";',
     'import {createHash,randomUUID} from "node:crypto";\nimport {profileMarkV1,profilePhaseV1} from "../../⏱️profile/🟨️.mjs";'),
    ('bazel=await prepareBazel(root,storage.tools,phaseControl,event=>{if(event.phase!=="downloading")console.log(`[bazel] ${event.phase} ${event.bytes}/${event.total}`);},original.CARGO_NET_OFFLINE==="true"),modules=await prepareBazelModulesV1(root,bazel,invocation,storage,phaseControl);',
     'bazel=await profilePhaseV1("bazel.prepare",()=>prepareBazel(root,storage.tools,phaseControl,event=>{if(event.phase!=="downloading")console.log(`[bazel] ${event.phase} ${event.bytes}/${event.total}`);},original.CARGO_NET_OFFLINE==="true")),modules=await profilePhaseV1("bazel.modules",()=>prepareBazelModulesV1(root,bazel,invocation,storage,phaseControl));'),
    ('const {workspace,plan,implementation,adapter,scopes}=await publishBazelGraphV1(root,graph,request,invocation,storage,phaseControl,session.slot),emitted=',
     'const {workspace,plan,implementation,adapter,scopes}=await profilePhaseV1("bazel.publish",()=>publishBazelGraphV1(root,graph,request,invocation,storage,phaseControl,session.slot)),emitted='),
    ('const {adapter,implementation,scopes}=await publishBazelImplementationV1(root,workspace,invocation,[',
     'const {adapter,implementation,scopes}=await profilePhaseV1("bazel.implementation",()=>publishBazelImplementationV1(root,workspace,invocation,['),
    ('process.env,true).values())]),input=join(workspace,"planner-input.json")',
     'process.env,true).values())])),input=join(workspace,"planner-input.json")'),
    ('await runOwnedCommand(process.execPath,[adapter.runner,adapter.snapshot,adapter.sha256,"planner"',
     'await profilePhaseV1("bazel.planner",()=>runOwnedCommand(process.execPath,[adapter.runner,adapter.snapshot,adapter.sha256,"planner"'),
    ('onProgress:()=>advanceScriptInvocation(invocation,"bazel:captured-planner","running")});',
     'onProgress:()=>advanceScriptInvocation(invocation,"bazel:captured-planner","running")}));'),
    ('exactClosures=await loadBazelClosuresV1(markers,closureFetch??managedBazelClosureFetchV1(root,phaseControl,invocation)),resolutions=await bazelCargoResolutionsV1(root,markers,exactClosures,phaseControl),',
     'exactClosures=await profilePhaseV1("bazel.closures",()=>loadBazelClosuresV1(markers,closureFetch??managedBazelClosureFetchV1(root,phaseControl,invocation))),resolutions=await profilePhaseV1("bazel.resolutions",()=>bazelCargoResolutionsV1(root,markers,exactClosures,phaseControl)),'),
    ('const files=await sourceFiles(root,invocation,[',
     'const files=await profilePhaseV1("bazel.source-files",()=>sourceFiles(root,invocation,['),
    ('...bundled(values)]))]);\n if(hasHermetic&&!execution)',
     '...bundled(values)]))]));\n if(hasHermetic&&!execution)'),
    ('const classified=await bazelSourceClassesV1(root,[',
     'const classified=await profilePhaseV1("bazel.source-classes",()=>bazelSourceClassesV1(root,['),
    ('classes,boundaries,invocation),snapshot=await captureBazelSourceSnapshotV1({root,directory:workspace,',
     'classes,boundaries,invocation)),snapshot=await profilePhaseV1("bazel.source-snapshot",()=>captureBazelSourceSnapshotV1({root,directory:workspace,'),
    ('await invocation.control.yieldContinuation();checkScriptInvocation(invocation);}}),declarable=',
     'await invocation.control.yieldContinuation();checkScriptInvocation(invocation);}})),declarable='),
    ('const fingerprint=await taskFingerprint(root,graph,task,hashed.get(action.id)??[],memo,invocation,phaseControl,action.cache.cache,chosen.sha256+chosen.loaderSha256,exactClosures,resolutions);',
     'const fingerprint=await profilePhaseV1("bazel.fingerprint",()=>taskFingerprint(root,graph,task,hashed.get(action.id)??[],memo,invocation,phaseControl,action.cache.cache,chosen.sha256+chosen.loaderSha256,exactClosures,resolutions));'),
    ('recheck();checkScriptInvocation(invocation);await writeChanged(join(workspace,"BUILD.bazel"),renderBazelBuildV1({',
     'profileMarkV1("bazel.render");recheck();checkScriptInvocation(invocation);await writeChanged(join(workspace,"BUILD.bazel"),renderBazelBuildV1({'),
    ('await writeBazelRuntimeV1(workspace,root,scriptInvocationEnvironment(invocation,process.env));return {workspace,plan,implementation,',
     'profileMarkV1("bazel.rendered");await writeBazelRuntimeV1(workspace,root,scriptInvocationEnvironment(invocation,process.env));return {workspace,plan,implementation,'),
    ('(await captureOwnedCommandV1(bazel,[...startup,"info","bazel-bin",',
     '(await profilePhaseV1("bazel.info",()=>captureOwnedCommandV1(bazel,[...startup,"info","bazel-bin",'),
    ('{signal,control:phaseControl,env:client()})).trim().split("\\n").at(-1)!);\n  await attachBazelServerV1(cache);',
     '{signal,control:phaseControl,env:client()}))).trim().split("\\n").at(-1)!);\n  await attachBazelServerV1(cache);'),
    ('()=>runOwnedCommand(bazel,[...startup,batch.verb,',
     '()=>profilePhaseV1("bazel.invoke",()=>runOwnedCommand(bazel,[...startup,batch.verb,'),
    ('{...rendering,signal:control.commandSignal,env:client()}),{signal:control.commandSignal}).catch(async error=>{await reportBazelFailureTailsV1(',
     '{...rendering,signal:control.commandSignal,env:client()})),{signal:control.commandSignal}).catch(async error=>{await reportBazelFailureTailsV1('),
])

graph_test = L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🧪️tests/🟦️.ts"
extra_test = '''
test("an already retained graph is pinned again without a retention sweep, a graph that adds bytes sweeps",async()=>{
 await withBuildTestInvocationV1(async original=>{
  const graph=fixture.graph as BuildGraphV1,name="other-"+crypto.randomUUID().slice(0,8),grown={...graph,projects:{...graph.projects,[name]:{...graph.projects.fixture,name,root:name}}} as BuildGraphV1,phase=(label:string)=>scriptCompilerAdmissionControlV1(original,label,0);
  await withRepositoryBuildGraphArtifactV1(process.cwd(),graph,original,phase("graph:artifact-first"),async artifact=>{
   const decoy=join(artifact.path.replace(/[^/]+$/u,""),"f".repeat(64)+"~deadbeef");
   try{
    await writeFile(decoy,"decoy");
    await withRepositoryBuildGraphArtifactV1(process.cwd(),graph,original,phase("graph:artifact-again"),async again=>{expect(again).toEqual(artifact);expect(await readFile(decoy,"utf8")).toBe("decoy");});
    await withRepositoryBuildGraphArtifactV1(process.cwd(),grown,original,phase("graph:artifact-grown"),async bigger=>{expect(bigger.digest).not.toBe(artifact.digest);await expect(readFile(decoy,"utf8")).rejects.toThrow();});
   }finally{await rm(decoy,{force:true});}
  });
 });
},120000);
'''
text = edits.get(graph_test) or read(graph_test)
if text.count('import {readFile} from "node:fs/promises";') != 1 or text.count('import {rm} from "node:fs/promises";') != 1:
    raise SystemExit("graph test imports changed")
text = text.replace('import {readFile} from "node:fs/promises";', 'import {readFile,writeFile} from "node:fs/promises";')
edits[graph_test] = text.rstrip("\n") + "\n" + extra_test

for path, text in {**edits, **created}.items():
    docs = re.findall(r"/\*\* (\S+)", text)
    dup = {d for d in docs if docs.count(d) > 1}
    if dup:
        raise SystemExit("docstring emoji not unique in %s: %s" % (path, dup))
    if path.endswith(".ts") and len(path) + 60 > 256:
        raise SystemExit("path too long: " + path)

if "--check" in sys.argv:
    os.makedirs("/private/tmp/noopg/preview", exist_ok=True)
    for index, (path, text) in enumerate({**edits, **created}.items()):
        target = "/private/tmp/noopg/preview/%d.ts" % index
        open(target, "w", encoding="utf8").write(text)
        done = subprocess.run(["bun", "build", target, "--no-bundle", "--target=bun"], capture_output=True, text=True)
        print(("SYNTAX OK  " if done.returncode == 0 else "SYNTAX BAD ") + path[-60:], done.stderr[:300] if done.returncode else "")
        if done.returncode:
            sys.exit(1)
for path, text in {**edits, **created}.items():
    print(("WRITE " if write else "DRY   ") + path)
    if write:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf8").write(text)
print("files:", len(edits) + len(created), "edited:", len(edits), "created:", len(created))
