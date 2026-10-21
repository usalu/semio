#!/usr/bin/env python3
"""Anchored apply of the phase profile hooks to the build CLI script.
Dry run by default; --write applies all-or-nothing; --target <file> writes the hooked copy to another path (same directory keeps the relative imports valid).
Every anchor must occur exactly once in the current file (drift refusal); an edit whose replacement is already present is skipped (idempotent)."""
import sys,os,argparse,hashlib
ROOT=os.getcwd()
SCRIPT="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts"
P='"./🌡️host-capacity/🟦️.ts"'
EDITS=[
("import","import { join, resolve, dirname } from \"node:path\";\n",
 "import { join, resolve, dirname } from \"node:path\";\nimport {profileMarkV1,profilePhaseV1} from \"./⏱️profile/🟨️.mjs\";\n"),
("door","  const{withBuildCapacityV1}=await import(\"./🌡️host-capacity/🟦️.ts\");\n  const{withImplementationPinV1}=await import(\"./📥️inputs/🔏️revision/🟨️.mjs\");\n  await withBuildCapacityV1(args.slice(prefix),environment,granted=>withImplementationPinV1(()=>runAdmittedBuildCliV1(root,[...args.slice(0,prefix),...granted.args],environment)));",
 "  const{withBuildCapacityV1}=await profilePhaseV1(\"door.load\",()=>import(\"./🌡️host-capacity/🟦️.ts\"));\n  const{withImplementationPinV1}=await import(\"./📥️inputs/🔏️revision/🟨️.mjs\");\n  await profilePhaseV1(\"door.capacity\",()=>withBuildCapacityV1(args.slice(prefix),environment,granted=>profilePhaseV1(\"door.admitted\",()=>withImplementationPinV1(()=>runAdmittedBuildCliV1(root,[...args.slice(0,prefix),...granted.args],environment)))));"),
("capabilities","const capabilities=incoming?.capabilities??await createBuildBootstrapCapabilitiesV1(root,args,environment),envelope=",
 "const capabilities=incoming?.capabilities??await profilePhaseV1(\"door.capabilities\",()=>createBuildBootstrapCapabilitiesV1(root,args,environment)),envelope="),
("script","await withScriptProcessEnvelope(envelope,original=>runScriptMain(new ScriptRouter(root,root).register(\"repo\",RepoScript).register(\"build\",BuildScript).register(\"test\",TestScript),{invocation:original,argv:[...args]}));",
 "await withScriptProcessEnvelope(envelope,original=>profilePhaseV1(\"door.script\",()=>runScriptMain(new ScriptRouter(root,root).register(\"repo\",RepoScript).register(\"build\",BuildScript).register(\"test\",TestScript),{invocation:original,argv:[...args]})));"),
("invocation-capacity",",{withBuildCapacityV1}=await import(\"./🌡️host-capacity/🟦️.ts\");\n const{withImplementationPinV1}=await import(\"./📥️inputs/🔏️revision/🟨️.mjs\");\n await withBuildCapacityV1(args,process.env,granted=>withImplementationPinV1(()=>runAdmittedBuildInvocationV1(root,granted.args,parent,phaseControl)),{control:phaseControl});",
 ",{withBuildCapacityV1}=await profilePhaseV1(\"invocation.gate-load\",()=>import(\"./🌡️host-capacity/🟦️.ts\"));\n const{withImplementationPinV1}=await import(\"./📥️inputs/🔏️revision/🟨️.mjs\");\n await profilePhaseV1(\"invocation.gate\",()=>withBuildCapacityV1(args,process.env,granted=>profilePhaseV1(\"invocation.admitted\",()=>withImplementationPinV1(()=>runAdmittedBuildInvocationV1(root,granted.args,parent,phaseControl))),{control:phaseControl}));"),
("caller","const{readOwnedBuildInvocationV1}=await import(\"./📥️caller/🟦️.ts\");readOwnedBuildInvocationV1(root,parent,args);\n const{parseBuildInvocationV1,resolveBuildInvocationV1,buildInvocationRequestsHelpV1}=await import(\"./🧭️invocation/🟦️.ts\"),selected=",
 "const{readOwnedBuildInvocationV1}=await profilePhaseV1(\"invocation.load\",()=>import(\"./📥️caller/🟦️.ts\"));profilePhaseV1(\"invocation.read\",()=>readOwnedBuildInvocationV1(root,parent,args));\n const{parseBuildInvocationV1,resolveBuildInvocationV1,buildInvocationRequestsHelpV1}=await profilePhaseV1(\"invocation.parse-load\",()=>import(\"./🧭️invocation/🟦️.ts\")),selected="),
("recovery-plan","if(readNativeRepoRecoveryPlanV1(root,resolveBuildInvocationV1(root,selected.args,{...process.env,SEMIO_BUILD_MODE:(parent.capabilities as {buildMode:string}).buildMode}).args)){",
 "if(profilePhaseV1(\"invocation.recovery-plan\",()=>readNativeRepoRecoveryPlanV1(root,resolveBuildInvocationV1(root,selected.args,{...process.env,SEMIO_BUILD_MODE:(parent.capabilities as {buildMode:string}).buildMode}).args))){"),
("leases-load","const {openRepositoryCachePartitionV1}=await import(\"../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts\"),control=phaseControl,leases=await openRepositoryCachePartitionV1(root,\"leases\",control);",
 "const {openRepositoryCachePartitionV1}=await profilePhaseV1(\"leases.load\",()=>import(\"../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts\")),control=phaseControl,leases=await profilePhaseV1(\"leases.open\",()=>openRepositoryCachePartitionV1(root,\"leases\",control));"),
("diagnostics-opened","async(diagnostics,diagnosticPhase)=>{const previous=process.env.SEMIO_DIAGNOSTIC_DIRECTORY;",
 "async(diagnostics,diagnosticPhase)=>{profileMarkV1(\"diagnostics.opened\");const previous=process.env.SEMIO_DIAGNOSTIC_DIRECTORY;"),
("body","try{await runOwnedBuildInLeasesV1(root,args,parent,diagnosticPhase);}finally{if(previous===undefined)delete process.env.SEMIO_DIAGNOSTIC_DIRECTORY;",
 "try{await profilePhaseV1(\"invocation.body\",()=>runOwnedBuildInLeasesV1(root,args,parent,diagnosticPhase));}finally{profileMarkV1(\"diagnostics.closing\");if(previous===undefined)delete process.env.SEMIO_DIAGNOSTIC_DIRECTORY;"),
("leases-close","finally{try{await leases.close();}","finally{try{await profilePhaseV1(\"leases.close\",()=>leases.close());}"),
("recovery-run","if(await(await import(\"./🚀️bootstrap/⌨️cli/🟦️.ts\")).runNativeRepoRecoveryV1(root,resolved.args,invocation,phaseControl))return;",
 "if(await profilePhaseV1(\"recovery.run\",async()=>(await import(\"./🚀️bootstrap/⌨️cli/🟦️.ts\")).runNativeRepoRecoveryV1(root,resolved.args,invocation,phaseControl)))return;"),
("validate","      validateBuildTaskArgumentsV1(resolved.args);\n","      profilePhaseV1(\"selection.validate\",()=>validateBuildTaskArgumentsV1(resolved.args));\n"),
("ensure","if(buildOperationRequiresV1(command)===\"dependencies\"){const {ensureBuildEnvironmentV1}=await import(\"./🚀️bootstrap/🟦️.ts\");await ensureBuildEnvironmentV1(root,invocation,phaseControl);}",
 "if(buildOperationRequiresV1(command)===\"dependencies\"){const {ensureBuildEnvironmentV1}=await profilePhaseV1(\"dependencies.ensure-load\",()=>import(\"./🚀️bootstrap/🟦️.ts\"));await profilePhaseV1(\"dependencies.ensure\",()=>ensureBuildEnvironmentV1(root,invocation,phaseControl));}"),
("dependencies-store","    await withRepositoryCacheStoreV1(root,\"dependencies\",parent,phaseControl,async(_dependencies,phaseControl)=>{\n",
 "    profileMarkV1(\"dependencies.opening\");await withRepositoryCacheStoreV1(root,\"dependencies\",parent,phaseControl,async(_dependencies,phaseControl)=>{profileMarkV1(\"dependencies.opened\");\n"),
("tool-storage","const {prepareBuildToolStorageV1}=await import(\"./🔧️tools/💾️storage/🟦️.ts\");await prepareBuildToolStorageV1(root,selected.tool,invocation,phaseControl,0);",
 "const {prepareBuildToolStorageV1}=await profilePhaseV1(\"tools.storage-load\",()=>import(\"./🔧️tools/💾️storage/🟦️.ts\"));await profilePhaseV1(\"tools.storage\",()=>prepareBuildToolStorageV1(root,selected.tool,invocation,phaseControl,0));"),
("graph-load","const baseline=await loadBuildInvocationGraphV1(root,resolved.args,parent,invocation,phaseControl,process.env.SEMIO_BUILD_REUSE_GRAPH===\"true\"),{withSelectedRustBuildGraphV1}=await import(\"./🕸️graph/🦀️rust/🟦️.ts\");await withSelectedRustBuildGraphV1(root,baseline,resolved.args,invocation,phaseControl,async(prepared,rustPhase)=>{const graph=prepared.graph;",
 "const baseline=await profilePhaseV1(\"graph.load\",()=>loadBuildInvocationGraphV1(root,resolved.args,parent,invocation,phaseControl,process.env.SEMIO_BUILD_REUSE_GRAPH===\"true\")),{withSelectedRustBuildGraphV1}=await profilePhaseV1(\"graph.rust-load\",()=>import(\"./🕸️graph/🦀️rust/🟦️.ts\"));await withSelectedRustBuildGraphV1(root,baseline,resolved.args,invocation,phaseControl,async(prepared,rustPhase)=>{profileMarkV1(\"graph.rust-ready\");const graph=prepared.graph;"),
("graph-pinned","    await withRepositoryBuildGraphArtifactV1(root,graph,invocation,rustPhase,async(pinned,artifactPhase)=>{\n",
 "    profileMarkV1(\"graph.pinning\");await withRepositoryBuildGraphArtifactV1(root,graph,invocation,rustPhase,async(pinned,artifactPhase)=>{profileMarkV1(\"graph.pinned\");\n"),
("nx","await withStoreReceiptsV1({root,tasks:await storeReceiptTasksV1(root,graph,resolved.args,invocation,readBuildMode(process.env.SEMIO_BUILD_MODE)),run:()=>nx(nxOperationArgumentsV1(resolved.args)!),",
 "await withStoreReceiptsV1({root,tasks:await profilePhaseV1(\"receipts.tasks\",()=>storeReceiptTasksV1(root,graph,resolved.args,invocation,readBuildMode(process.env.SEMIO_BUILD_MODE))),run:()=>profilePhaseV1(\"tool.nx\",()=>nx(nxOperationArgumentsV1(resolved.args)!)),"),
("bazel","await withStoreReceiptsV1({root,tasks:await storeReceiptTasksV1(root,graph,resolved.args,invocation,readBuildMode(process.env.SEMIO_BUILD_MODE)),run:async()=>{await adapter.runBazelTasksV1(root,graph,request,invocation,artifactPhase);},",
 "await withStoreReceiptsV1({root,tasks:await profilePhaseV1(\"receipts.tasks\",()=>storeReceiptTasksV1(root,graph,resolved.args,invocation,readBuildMode(process.env.SEMIO_BUILD_MODE))),run:async()=>{await profilePhaseV1(\"tool.bazel\",()=>adapter.runBazelTasksV1(root,graph,request,invocation,artifactPhase));},"),
("boot-loaded","const bazelTestContract=compileJsonSchemaSubset(bazelTestSchema);\n",
 "const bazelTestContract=compileJsonSchemaSubset(bazelTestSchema);\nprofileMarkV1(\"boot.loaded\");\n"),
("boot-main","if(import.meta.main)await runBuildCliV1(findWorkspaceRoot(import.meta.dir),process.argv.slice(2));",
 "if(import.meta.main){profileMarkV1(\"boot.main\");await profilePhaseV1(\"cli.run\",()=>runBuildCliV1(findWorkspaceRoot(import.meta.dir),process.argv.slice(2)));}"),
]
def main():
    ap=argparse.ArgumentParser();ap.add_argument("--write",action="store_true");ap.add_argument("--root",default=ROOT);ap.add_argument("--target");ap.add_argument("--dry-run",action="store_true")
    a=ap.parse_args()
    path=os.path.join(a.root,SCRIPT);text=open(path,encoding="utf8").read();out=text;todo=[];skipped=[];bad=[]
    for name,old,new in EDITS:
        co,cn=out.count(old),out.count(new)
        if cn>=1 and co==0: skipped.append(name);continue
        if co!=1: bad.append((name,co));continue
        out=out.replace(old,new);todo.append(name)
    print(f"{len(todo)} to apply, {len(skipped)} already applied, {len(bad)} refused; {SCRIPT}")
    for name,c in bad: print(f"REFUSED {name}: anchor occurs {c} times (drift)")
    if bad: sys.exit(2)
    if not a.write and not a.target: print("dry run ok (use --write)");return
    dest=a.target or path
    if len(dest.encode())>1000: raise SystemExit("path too long")
    tmp=dest+".tmp";open(tmp,"w",encoding="utf8").write(out);os.replace(tmp,dest);print("written",dest)
main()
