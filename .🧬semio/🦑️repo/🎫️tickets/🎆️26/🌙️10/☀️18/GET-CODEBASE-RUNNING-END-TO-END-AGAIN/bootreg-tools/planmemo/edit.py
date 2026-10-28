#!/usr/bin/env python3
"""🧠 Anchored, idempotent edit of batch `planmemo` (lane bootreg): Cargo planning serves `cargo metadata` + `--unit-graph` from a memo keyed by request, configuration, toolchain and Cargo-read environment and vouched by every manifest, lock and target-discovery path Cargo read; usage: python3 -I edit.py <root> check|apply."""
import os, sys
root, verb = sys.argv[1], sys.argv[2]
CAP = "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts"
LAU = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts"
FAC = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts"
UCS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json"
EDITS = {CAP: [
 ('import {ensureCargoLockCurrentV1,cargoLockStaleFailureV1} from "../../../../🦀️cargo-workspace/🟦️.ts";',
  'import {ensureCargoLockCurrentV1,cargoLockStaleFailureV1} from "../../../../🦀️cargo-workspace/🟦️.ts";\nimport {cargoPlanKeyV1,cargoToolIdentityV1,type CargoPlanMemoV1} from "./🧠️memo/🟦️.ts";'),
 (',control:ResourceLeaseControl,maximumBytes:number):Promise<unknown>}\n',
  ',control:ResourceLeaseControl,maximumBytes:number):Promise<unknown>;readonly plans?:CargoPlanMemoV1}\n'),
 ('export function actualCargoCompilerPlanningPortV1(options:Readonly<{digests?:FileDigestMemoV1}>={}):CargoCompilerPlanningPortV1{return{',
  'export function actualCargoCompilerPlanningPortV1(options:Readonly<{digests?:FileDigestMemoV1;plans?:CargoPlanMemoV1}>={}):CargoCompilerPlanningPortV1{return{...(options.plans?{plans:options.plans}:{}),'),
 ('await lock(false);\n const packageFacts=await locked(()=>port.capture(request.program,[...request.prefix,...metadata],request.cwd,request.environment,control,request.maximumOutputBytes));',
  'await lock(false);\n const startedMs=Date.now(),early=port.plans?await port.compiler(request.environment,request.cwd,control):undefined,planKey=early?cargoPlanKeyV1({program:request.program,prefix:request.prefix,args:request.args,cwd:request.cwd,workspaceRoot:request.workspaceRoot,configurationDigest,compiler:early,cargo:await cargoToolIdentityV1(early.cargoProgram),environment:request.environment}):undefined,recalled=planKey?await port.plans!.recall(planKey,control):undefined;\n const packageFacts=recalled?recalled.metadata:await locked(()=>port.capture(request.program,[...request.prefix,...metadata],request.cwd,request.environment,control,request.maximumOutputBytes));'),
 (' const units=await locked(()=>port.capture(request.program,[...request.prefix,...compile],request.cwd,request.environment,control,request.maximumOutputBytes));checkResourceLeaseControl(control);',
  ' const units=recalled?recalled.units:await locked(()=>port.capture(request.program,[...request.prefix,...compile],request.cwd,request.environment,control,request.maximumOutputBytes));checkResourceLeaseControl(control);if(planKey&&!recalled)await port.plans!.remember(planKey,{metadata:packageFacts,units},startedMs,control);checkResourceLeaseControl(control);'),
 ('const graph=readCargoCompilationGraphV1(units,packageFacts,request.workspaceRoot),compiler=await port.compiler(request.environment,request.cwd,control),',
  'const graph=readCargoCompilationGraphV1(units,packageFacts,request.workspaceRoot),compiler=early??await port.compiler(request.environment,request.cwd,control),'),
], LAU: [
 ('import {captureCargoCompilationGraphV1,actualCargoCompilerPlanningPortV1,cargoPlanningOutputBytesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts";',
  'import {captureCargoCompilationGraphV1,actualCargoCompilerPlanningPortV1,cargoPlanMemoV1,cargoPlanningOutputBytesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts";\nimport {UNIT_CACHE_POLICY_V1} from "../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";'),
 ('export function repositoryCargoPlanningPortV1(store:Readonly<{root:string}>){return actualCargoCompilerPlanningPortV1({digests:foreignDigestMemoV1(unitCacheRootV1(store.root))});}',
  'export function repositoryCargoPlanningPortV1(store:Readonly<{root:string}>){return actualCargoCompilerPlanningPortV1({digests:foreignDigestMemoV1(unitCacheRootV1(store.root)),plans:cargoPlanMemoV1(join(unitCacheRootV1(store.root),UNIT_CACHE_POLICY_V1.layout.plans))});}'),
], FAC: [
 ('export type { CargoCompilerPlanningPortV1 } from "../../../../../../📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts";',
  'export type { CargoCompilerPlanningPortV1 } from "../../../../../../📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts";\nexport { cargoPlanMemoV1 } from "../../../../../../📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo/🟦️.ts";\nexport type { CargoPlanMemoV1 } from "../../../../../../📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo/🟦️.ts";'),
], UCS: [
 ('"foreign": "f", ', '"foreign": "f", "plans": "p", '),
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
