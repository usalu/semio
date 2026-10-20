#!/usr/bin/env python3
"""Builds edits.json: the anchored edits of existing files (exact old text, exact new text, expected occurrence count), computed from the live tree. usage: build_edits.py"""
import json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0,HERE)
import manifest as m
REAL="/Users/ueli/Documents/semio"
SB=m.SB_DIR
V=m.V
def read(rel): return open(REAL+"/"+rel,encoding="utf8").read()
def rel_import(frm,to):
    r=os.path.relpath(REAL+"/"+to,os.path.dirname(REAL+"/"+frm))
    return r if r.startswith(".") else "./"+r
edits=[]
def add(target,pairs): edits.append({"target":target,"pairs":[{"old":o,"new":n,"count":c} for o,n,c in pairs]})

EXEC=f"{m.FW}/{m.MODS}/{m.PROC}/{m.TESTING}/{m.e(0x1F39B,'execution')}/{m.IDX}"
t=read(EXEC)
start=t.index("/** 🧲️ Lists the attached macOS disk images"); end=t.index("/** 🗑️ Detaches every disk image under a scratch root created by this module")
block=t[start:end]
add(EXEC,[
 ('import { spawn, spawnSync } from "node:child_process";','import { spawn } from "node:child_process";',1),
 ('import { join, resolve, sep } from "node:path";\nimport { terminateOwnedProcessTree } from "../../🪓️termination/🟦️.ts";',
  'import { join, resolve } from "node:path";\nimport { terminateOwnedProcessTree } from "../../🪓️termination/🟦️.ts";\nimport { detachDiskImagesUnderV1 } from "../'+m.e(0x1F3D6,"sandbox")+'/'+m.IMAGES+'/'+m.IDX+'";\nimport { guardArgumentsV1, testCheckoutRootV1, testSandboxEnvironmentV1 } from "../'+m.e(0x1F3D6,"sandbox")+'/'+m.IDX+'";\n\nexport { attachedDiskImagesV1, detachDiskImagesUnderV1 } from "../'+m.e(0x1F3D6,"sandbox")+'/'+m.IMAGES+'/'+m.IDX+'";',1),
 (block,"",1),
 ('  const child = spawn(command, [...args], { stdio: options.captureStdout ? ["inherit", "pipe", "inherit"] : "inherit", cwd: options.cwd, env: options.env ?? process.env, detached: process.platform !== "win32", windowsHide: true });',
  '  const sandbox = testSandboxEnvironmentV1(options.env ?? process.env, { route: "test-command", checkout: testCheckoutRootV1(options.cwd, options.env ?? process.env) });\n  const child = spawn(command, [...guardArgumentsV1(command, args, sandbox.environment)], { stdio: options.captureStdout ? ["inherit", "pipe", "inherit"] : "inherit", cwd: options.cwd, env: sandbox.environment, detached: process.platform !== "win32", windowsHide: true });',1),
 ('    options.signal?.removeEventListener("abort", abort);\n    process.off("SIGINT", interrupt);\n    process.off("SIGTERM", interrupt);\n  }\n}',
  '    options.signal?.removeEventListener("abort", abort);\n    process.off("SIGINT", interrupt);\n    process.off("SIGTERM", interrupt);\n    sandbox.release();\n  }\n}',1),
])

FACADE=f"{m.FW}/{m.MODS}/{m.PROC}/{m.e(0x1F4E6,'packages')}/{m.e(0x1F7E6,'typescript')}/{m.TESTING}/{m.e(0x1F39B,'execution')}/{m.IDX}"
ft=read(FACADE); first=ft.split("\n")[0]
add(FACADE,[(first,first+'\nexport { createTestSandboxV1, withTestSandboxV1, testSandboxEnvironmentV1, testScratchRootV1, reserveTestPortV1, reclaimAbandonedTestSandboxesV1, compileLawV1, scanTestsV1, guardArgumentsV1, summarizeGuardReportV1 } from "../../../../'+m.TESTING+'/'+m.e(0x1F3D6,"sandbox")+'/'+m.IDX+'";\nexport type { TestSandboxV1, TestSandboxOptionsV1, TestGuardModeV1, ScanFindingV1 } from "../../../../'+m.TESTING+'/'+m.e(0x1F3D6,"sandbox")+'/'+m.IDX+'";',1)])

LIB=f"{m.FW}/{m.e(0x1F6CD,'products')}/{m.e(0x1F991,'repo')}/{m.MODS}/{m.e(0x1F4DA,'library')}"
OUT=f"{LIB}/{m.PROC}/{m.e(0x1F33F,'environment')}/{m.e(0x1F9EA,'test-output')}/{m.IDX}"
ot=read(OUT)
sb_facade=f"{m.FW}/{m.MODS}/{m.PROC}/{m.e(0x1F4E6,'packages')}/{m.e(0x1F7E6,'typescript')}/{m.TESTING}/{m.e(0x1F39B,'execution')}/{m.IDX}"
add(OUT,[
 ('import { resolve } from "node:path";\n','import { resolve } from "node:path";\nimport { testSandboxEnvironmentV1 } from "'+rel_import(OUT,sb_facade)+'";\n',1),
 ('/** 🧾️ Resolves caller-owned test output or a neutral cache route without naming a ticket. */','/** 🧾️ Resolves caller-owned test output, or the artifact directory of the per-run test sandbox (private temp root, image root, port leases, reclaimed when its owner dies), without naming a ticket or a shared cache route. */',1),
 ('  const artifactRoot = resolve(repoRoot, configured || `.🧬semio/🦑️repo/⚡️cache/🧪️tests/${route}`);\n  mkdirSync(artifactRoot, { recursive: true });\n  return { ...env, SEMIO_TEST_ARTIFACT_DIR: artifactRoot };',
  '  if (!configured) return testSandboxEnvironmentV1(env, { route, checkout: repoRoot }).environment;\n  const artifactRoot = resolve(repoRoot, configured);\n  mkdirSync(artifactRoot, { recursive: true });\n  return { ...env, SEMIO_TEST_ARTIFACT_DIR: artifactRoot };',1),
])

GB=f"{LIB}/{m.TESTING}/{m.e(0x1F331,'genuine-birth')}/{m.IDX}"
add(GB,[
 ('import { repoTestArtifactEnvironment } from "../../🏃️process/🌿️environment/🧪️test-output/🟦️.ts";','import { repoTestArtifactEnvironment } from "../../🏃️process/🌿️environment/🧪️test-output/🟦️.ts";\nimport { guardArgumentsV1 } from "'+rel_import(GB,sb_facade)+'";',1),
 ('["test", "--timeout", String(declared.childTimeoutMilliseconds), "./" + local, ...(request.pattern === undefined ? [] : ["--test-name-pattern", request.pattern])]','[...guardArgumentsV1(process.execPath, ["test", "--timeout", String(declared.childTimeoutMilliseconds), "./" + local, ...(request.pattern === undefined ? [] : ["--test-name-pattern", request.pattern])], environment)]',1),
])

SBX=f"{m.FW}/{m.MODS}/{m.PROC}/{m.TESTING}/{m.e(0x1F3D6,'sandbox')}/{m.IDX}"
STORE=f"{m.FW}/{m.MODS}/{m.PROC}/{m.e(0x1F4E6,'artifacts')}/{m.e(0x1F3D7,'native-build')}/{m.e(0x1F4BE,'storage')}"
CU=m.e(0x1F9F9,"cleanup")
TICKET='resolve(process.cwd(),".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated")'
def scratch(target,count,slug):
    text=read(target); first=text.split("\n")[0]
    assert text.count(TICKET)==count,(target,text.count(TICKET))
    imp='import {testScratchRootV1} from "'+rel_import(target,SBX)+'";'
    add(target,[(first,imp+"\n"+first,1),(TICKET,f'testScratchRootV1("{slug}")',count)])
T_=m.TESTS
scratch(f"{STORE}/{CU}/{m.e(0x1F39B,'command')}/{T_}/{m.IDX}",1,"cleanup-command")
scratch(f"{STORE}/{CU}/{m.e(0x1F4E5,'operation')}/{T_}/{m.IDX}",6,"cleanup-operation")
scratch(f"{STORE}/{CU}/{m.e(0x1F4E5,'receiver')}/{T_}/{m.IDX}",1,"cleanup-receiver")
scratch(f"{m.FW}/{m.MODS}/{m.PROC}/{m.e(0x1F4E6,'artifacts')}/{m.e(0x1F4E4,'publication')}/{m.e(0x1F4E5,'sources')}/{T_}/{m.IDX}",1,"publication-sources")
scratch(f"{LIB}/{m.e(0x26A1,'caching')}/{m.e(0x1F4BE,'storage')}/{m.e(0x1F512,'enforcement')}/{m.e(0x1F3D7,'compiler')}/{m.e(0x23F3,'phase')}/{T_}/{m.IDX}",1,"compiler-phase")

INV=f"{LIB}/{T_}/{m.e(0x1F4A0,'inventory-artifact-shards')}/{m.IDX}"
it=read(INV)
OLD='const ticketRoot = process.env.SEMIO_TEST_ARTIFACT_DIR ?? join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/sol-root-taxonomy-workflow-extraction/inventory-artifact-shards");'
assert it.count(OLD)==1
first=it.split("\n")[0]
add(INV,[(first,'import { testScratchRootV1 } from "'+rel_import(INV,SBX)+'";\n'+first,1),(OLD,'const ticketRoot = testScratchRootV1("inventory-artifact-shards");',1)])
json.dump(edits,open(HERE+"/edits.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
print(len(edits),"targets",sum(len(e["pairs"]) for e in edits),"pairs")
