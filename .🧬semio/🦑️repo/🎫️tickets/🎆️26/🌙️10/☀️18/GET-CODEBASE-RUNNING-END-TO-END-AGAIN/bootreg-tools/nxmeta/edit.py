#!/usr/bin/env python3
"""♻ Anchored, idempotent edit of batch `nxmeta` (lane bootreg, apply AFTER planmemo): Cargo planning returns the resolved `cargo metadata` beside its graph, and the Nextest assertion step reads it through `--cargo-metadata` instead of running `cargo metadata` again; usage: python3 -I edit.py <root> check|apply."""
import os, sys
root, verb = sys.argv[1], sys.argv[2]
CAP = "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts"
TST = "🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts"
STF = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/🪜️steps/🔣️.json"
STL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/🪜️steps/🟦️.ts"
ORD = "🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🪜️order/🟦️.ts"
EDITS = {ORD: [
 ('captureCargoCompilationGraphV1:async()=>({}),', 'captureCargoCompilationPlanV1:async()=>({graph:{},metadata:{}}),'),
], CAP: [
 ('/** ⏺️ Plans the exact selected Cargo invocation without dispatching its compiler or test bodies. */\nexport async function captureCargoCompilationGraphV1(request:CargoCompilerPlanningRequestV1,control:ResourceLeaseControl,port:CargoCompilerPlanningPortV1):Promise<CargoCompilationGraphV1>{',
  '/** 🕸️ The compilation graph alone of {@link captureCargoCompilationPlanV1}. */\nexport async function captureCargoCompilationGraphV1(request:CargoCompilerPlanningRequestV1,control:ResourceLeaseControl,port:CargoCompilerPlanningPortV1):Promise<CargoCompilationGraphV1>{return(await captureCargoCompilationPlanV1(request,control,port)).graph;}\n/** ⏺️ Plans the exact selected Cargo invocation without dispatching its compiler or test bodies, and returns the resolved `cargo metadata` it planned from beside the graph, so a later step need not ask Cargo again. */\nexport async function captureCargoCompilationPlanV1(request:CargoCompilerPlanningRequestV1,control:ResourceLeaseControl,port:CargoCompilerPlanningPortV1):Promise<Readonly<{graph:CargoCompilationGraphV1;metadata:unknown}>>{'),
 ('if(validateJsonSchemaSubset(schema,graph).length)throw Error("Original complete Cargo compiler evidence refused");return graph;',
  'if(validateJsonSchemaSubset(schema,graph).length)throw Error("Original complete Cargo compiler evidence refused");return Object.freeze({graph,metadata:packageFacts});'),
], TST: [
 ('import {cargoPlanningOutputBytesV1,captureCargoCompilationGraphV1,actualCargoCompilerPlanningPortV1,type CargoCompilerPlanningPortV1} from',
  'import {cargoPlanningOutputBytesV1,captureCargoCompilationPlanV1,actualCargoCompilerPlanningPortV1,type CargoCompilerPlanningPortV1} from'),
 ('export function cargoTestPlanV1(request:CargoTestRequestV1, input:CargoTestPolicyV1, metadataPath:string):CargoTestStepV1[] {',
  'export function cargoTestPlanV1(request:CargoTestRequestV1, input:CargoTestPolicyV1, metadataPath:string, cargoMetadataPath?:string):CargoTestStepV1[] {'),
 ('||!isAbsolute(metadataPath)||',
  '||!isAbsolute(metadataPath)||cargoMetadataPath!==undefined&&!isAbsolute(cargoMetadataPath)||'),
 ('...threads,...profile,"--manifest-path",request.manifestPath,...split.executionArgs,"--",...split.libtestArgs,...skip])];\n  return [step("build",["test","--no-run"',
  '...threads,...profile,...(cargoMetadataPath===undefined?["--manifest-path",request.manifestPath]:["--cargo-metadata",cargoMetadataPath]),...split.executionArgs,"--",...split.libtestArgs,...skip])];\n  return [step("build",["test","--no-run"'),
 ('compilerArgs:readonly string[]):Promise<Readonly<Record<string,string|undefined>>>{',
  'compilerArgs:readonly string[]):Promise<Readonly<{environment:Readonly<Record<string,string|undefined>>;metadata:unknown}>>{'),
 (' const graph=await captureCargoCompilationGraphV1({workspaceRoot:admission.workspaceRoot,cwd:request.cwd,program:"cargo",prefix:[],args:compilerArgs,environment:env,maximumOutputBytes:cargoPlanningOutputBytesV1},control,admission.planningPort??actualCargoCompilerPlanningPortV1());',
  ' const{graph,metadata}=await captureCargoCompilationPlanV1({workspaceRoot:admission.workspaceRoot,cwd:request.cwd,program:"cargo",prefix:[],args:compilerArgs,environment:env,maximumOutputBytes:cargoPlanningOutputBytesV1},control,admission.planningPort??actualCargoCompilerPlanningPortV1());'),
 ('admission.sourceMemo);await storage.recheck();return env;\n}',
  'admission.sourceMemo);await storage.recheck();return{environment:env,metadata};\n}'),
 ('   request={...request,environment:actual.environment};admission={...admission,control:phase};const env=await admitCargoExecutionV1(request,policy,admission,actual.arguments);\n   mkdirSync(policy.artifactDirectory,{recursive:true});const directory=mkdtempSync(join(policy.artifactDirectory,"semio-nextest-")),metadata=join(directory,"binaries-metadata.json");requireNativeStoragePathV1(admission.storage.backing.root,directory);',
  '   request={...request,environment:actual.environment};admission={...admission,control:phase};const{environment:env,metadata:planned}=await admitCargoExecutionV1(request,policy,admission,actual.arguments);\n   mkdirSync(policy.artifactDirectory,{recursive:true});const directory=mkdtempSync(join(policy.artifactDirectory,"semio-nextest-")),metadata=join(directory,"binaries-metadata.json"),cargoMetadata=policy.nextest&&!policy.coverageEnabled?join(directory,"cargo-metadata.json"):undefined;requireNativeStoragePathV1(admission.storage.backing.root,directory);if(cargoMetadata)writeFileSync(cargoMetadata,JSON.stringify(planned));'),
 ('   try{const plan=cargoTestPlanV1(request,policy,metadata),steps:(readonly string[])[]=[];',
  '   try{const plan=cargoTestPlanV1(request,policy,metadata,cargoMetadata),steps:(readonly string[])[]=[];'),
 ('  try{request={...request,environment:actual.environment};admission={...admission,control:phase};const env=await admitCargoExecutionV1(request,policy,admission,actual.arguments);',
  '  try{request={...request,environment:actual.environment};admission={...admission,control:phase};const{environment:env}=await admitCargoExecutionV1(request,policy,admission,actual.arguments);'),
], STF: [
 ('  "metadata": "/owner/results/binaries-metadata.json",', '  "metadata": "/owner/results/binaries-metadata.json",\n  "cargoMetadata": "/owner/results/cargo-metadata.json",'),
], STL: [
 ('const steps=cargoTestPlanV1(request,policy,fixture.metadata).map(', 'const steps=[...cargoTestPlanV1(request,policy,fixture.metadata),...cargoTestPlanV1(request,policy,fixture.metadata,fixture.cargoMetadata)].map('),
]}
bad = 0
for rel, pairs in EDITS.items():
    path = os.path.join(root, rel); text = open(path, encoding="utf8").read(); out = text
    for old, new in pairs:
        if new in out: continue
        if out.count(old) != 1: print("ANCHOR", out.count(old), rel[-40:], old[:90].replace("\n", " ")); bad += 1; continue
        out = out.replace(old, new)
    if verb == "apply" and out != text:
        st = os.stat(path); open(path, "w", encoding="utf8").write(out); os.chmod(path, st.st_mode & 0o7777); print("edited", rel[-60:])
sys.exit(1 if bad else 0)
