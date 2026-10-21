#!/usr/bin/env python3
"""Cuts the per-backend code out of the live neutral CLI script into the two adapter modules (src/nx_backend.ts, src/bazel_backend.ts) and records the hashes of the slices in src/script.bases.json. The slices move verbatim; only relative import specifiers are re-based."""
import json,os,posixpath,re,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names,scriptslices as S
text=open(os.path.join(names.R,names.B,names.SCRIPT),encoding="utf8").read()
s=S.slices(text)
def rebase(code):
    def fix(m):
        spec=m.group(2)
        if not (spec.startswith("./") or spec.startswith("../")):return m.group(0)
        return m.group(1)+"@@UP@@"+(spec[2:] if spec.startswith("./") else spec)+m.group(3)
    return re.sub(r'(import\(")([^"]+)("\))',fix,code)
def dedent(code,n):
    return "\n".join(l[n:] if l.startswith(" "*n) else l for l in code.split("\n"))
def inner(name):
    a,b=s[name];lines=s["lines"][a+1:b-1];return dedent("\n".join(lines),4)
bases={k:S.digest(S.part(s,k)) for k in ("tools_bazel","tools_nx","reset","nx","bazel")}
bazel_setup=dedent(S.part(s,"tools_bazel"),6)
assert "selected" not in inner("nx") and "selected" not in S.part(s,"bazel"),"slice uses selected"
ctx="const {root,invocation,graph}=context,resolved={args:[...context.args],watch:context.watch},artifactPhase=context.phase,command=context.args[0]!;"
setupctx="const {root,invocation,phase:phaseControl}=context;"
def module(id_,setup,reset,storage,cache,remote,execute,extra="",imports="",probe=""):
    return f'''import {{join}} from "node:path";
import {{readBuildMode}} from "@@PROC@@/@@BUILDMOD@@/@@MODEMOD@@/@@TS@@";\nimport {{createIssuedResourceControlV1}} from "@@PROC@@/@@ROUTING@@/@@TS@@";{imports}
import type {{BackendContextV1,BackendExecutionContextV1,BuildBackendV1}} from "@@UP@@@@BACKENDS@@/@@TS@@";

/** 🔌️ {id_} behind the neutral interface: its setup, reset, storage, cache tools and execution, moved here from the neutral CLI so that neutral code never names it. */
export const backend:BuildBackendV1=Object.freeze({{
 id:"{id_}",{extra}
 async setup(context:BackendContextV1){{{setupctx}
{setup}
 }},
 async reset(context:BackendContextV1){{{setupctx}
{reset}
 }},
 async prepareStorage(context:BackendContextV1,snapshotBytes:number){{{setupctx}
{storage}
 }},
 async cache(){{{cache}}},{probe}
 async remoteCache(){{{remote}}},
 async execute(context:BackendExecutionContextV1){{{ctx}
{execute}
 }}
}});
'''
def ind(code,n=2):return "\n".join((" "*n+l if l.strip() else l) for l in code.split("\n"))
nx=module("nx",
 ind('await requireNxVersion(root,invocation,phaseControl);') ,
 ind('const{invalidateBuildGraphV1}=await import("@@UP@@🕸️graph/@@TS@@");await invalidateBuildGraphV1(root,invocation,phaseControl);const {runOwnedNxInvocationV1}=await import("@@UP@@../⚡️caching/🚀️bootstrap/@@SCRIPT@@");await runOwnedNxInvocationV1(root,["reset"],invocation,phaseControl);'),
 ind('const {prepareNxGraphCacheStorageV1}=await import("@@UP@@../⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/@@TS@@");await prepareNxGraphCacheStorageV1(root,invocation,phaseControl,snapshotBytes);'),
 'return (await import("@@UP@@../⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/@@TS@@")).nxBuildCacheToolV1();',
 'return (await import("@@UP@@../⚡️caching/🚀️bootstrap/🛠️tools/🌐️remote-cache/@@TS@@")).nxRemoteCacheToolV1();',
 ind(rebase(inner("nx"))),
 extra='\n bootstrapArguments(args:readonly string[]){const command=args[0];if(command==="tools")return ["--version"];if(command==="reset")return ["reset"];return command==="watch"||buildOperationHasNeutralBootstrapV1(command??"")?undefined:nxOperationArgumentsV1(args);},',
 probe='\n async cacheProbe(context){const {root,invocation}=context;await (await import("@@UP@@../⚡️caching/🚀️bootstrap/🛠️tools/@@TS@@")).prepareNxToolingStorageV1(root,invocation,createIssuedResourceControlV1(invocation,"cache-verify:nx",0));return (await import("@@UP@@../⚡️caching/🚀️bootstrap/🛠️tools/🔁️probe/@@TS@@")).nxCacheProbeV1(root);},',
 imports='\nimport {nxOperationArgumentsV1} from "@@UP@@../⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/@@TS@@";')
nx=nx.replace(ind('await requireNxVersion(root,invocation,phaseControl);'),ind('const adapter=await import("@@UP@@../⚡️caching/🚀️bootstrap/@@SCRIPT@@");await adapter.runOwnedNxInvocationV1(root,["--version"],invocation,phaseControl);'))
bz=module("bazel",
 ind(rebase(bazel_setup)),
 ind('const{resetBazelBuildStateV1}=await import("@@UP@@🔧️tools/🏗️bazel/💾️storage/🗑️reset/@@TS@@");await resetBazelBuildStateV1(root,invocation,phaseControl);'),
 ind('const {withBazelStorageV1}=await import("@@UP@@🔧️tools/🏗️bazel/💾️storage/@@TS@@");void snapshotBytes;await withBazelStorageV1(root,invocation,phaseControl,async storage=>{await storage.tools.recheck();await storage.dependencies.recheck();await storage.products.recheck();});'),
 'return (await import("@@UP@@🔧️tools/🏗️bazel/⚙️operations/@@TS@@")).bazelBuildCacheToolV1();',
 'return (await import("@@UP@@🔧️tools/🏗️bazel/🌐️remote-cache/@@TS@@")).bazelRemoteCacheToolV1();',
 ind(rebase(S.part(s,"bazel").replace("\n    ","\n    "))),
 probe='\n async cacheProbe(context){const {root,invocation}=context,[{withBazelSetupStorageV1},adapter,bootstrap,{bazelCacheProbeV1}]=await Promise.all([import("@@UP@@🔧️tools/🏗️bazel/💾️storage/@@TS@@"),import("@@UP@@🔧️tools/🏗️bazel/@@SCRIPT@@"),import("@@UP@@../⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/@@SCRIPT@@"),import("@@UP@@🔧️tools/🏗️bazel/🔁️probe/@@TS@@")]);return withBazelSetupStorageV1(root,invocation,createIssuedResourceControlV1(invocation,"cache-verify:bazel",0),async(storage,phase)=>{const executable=await bootstrap.prepareBazel(root,storage.tools,phase,()=>{},false),modules=await adapter.prepareBazelModulesV1(root,executable,invocation,storage,phase);return bazelCacheProbeV1({version:bootstrap.BAZEL_VERSION,executable,installBase:join(storage.paths.install,bootstrap.BAZEL_VERSION,`${process.platform}-${process.arch}`),module:adapter.BAZEL_MODULE,lock:modules.lock,repository:modules.repository});});},')
bz=bz.replace("    if(command===\"inputs\")","  if(command===\"inputs\")",1)
open(os.path.join(here,"src","nx_backend.ts"),"w",encoding="utf8").write(nx)
open(os.path.join(here,"src","bazel_backend.ts"),"w",encoding="utf8").write(bz)
json.dump(bases,open(os.path.join(here,"src","script.bases.json"),"w"),indent=1)
print("ok",bases)
