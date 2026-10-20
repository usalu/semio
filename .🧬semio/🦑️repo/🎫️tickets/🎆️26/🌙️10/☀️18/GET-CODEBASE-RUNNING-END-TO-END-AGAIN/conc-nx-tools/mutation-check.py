#!/usr/bin/env python3
"""Mutation proof on the overlay (/private/tmp/conc-nx/ov): each mutation must make its suite fail; the file is restored byte-identical afterwards. usage: mutation-check.py"""
import os, subprocess, sys

OV = "/private/tmp/conc-nx/ov"
E = lambda c: chr(c) + "️"
LIB = os.path.join(OV, E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library", E(0x26A1) + "caching")
GEN = os.path.join(LIB, E(0x1F578) + "graph", E(0x1F50F) + "seal", E(0x1F4CC) + "generation")
NXG = os.path.join(LIB, E(0x1F4BE) + "storage", E(0x1F512) + "enforcement", E(0x1F50C) + "nx", E(0x1F4E6) + "graph")
TS = E(0x1F7E6) + ".ts"
NX = os.path.join(LIB, E(0x1F4BE) + "storage", E(0x1F512) + "enforcement", E(0x1F50C) + "nx")
STARTUP = os.path.join(LIB, E(0x1F578) + "graph", E(0x1F3C1) + "startup")
ADM = os.path.join(STARTUP, E(0x1F68F) + "admission")
PUT = os.path.join(NX, E(0x1F39F) + "put-slots")
suites = {
    "slots": os.path.join(PUT, E(0x1F9EA) + "tests", TS),
    "daemon": os.path.join(ADM, E(0x1F9EA) + "tests", TS),
    "gen": os.path.join(GEN, E(0x1F9EA) + "tests", TS),
    "pool": os.path.join(NXG, E(0x1F9EA) + "tests", E(0x1F513) + "pool-order", TS),
}
targets = {"gen": os.path.join(GEN, TS), "pool": os.path.join(NXG, TS), "slots": os.path.join(PUT, TS), "daemon": os.path.join(ADM, TS), "nx": os.path.join(NX, TS), "service": os.path.join(STARTUP, E(0x1F6A6) + "service", TS)}
suites["nx"] = suites["pool"]
suites["service"] = suites["daemon"]
suites["nx2"] = suites["slots"]
targets["nx2"] = targets["nx"]
mutations = [
    ("gen", "holds ignores the pinned digest", "receipt.claim?.graphDigest!==claim.graphDigest||", ""),
    ("gen", "holds ignores the identity digest", "||receipt.claim.identity!==claim.identity", ""),
    ("gen", "holds ignores graph stamps", "return current!==undefined&&NX_GENERATION_GRAPH_FILES.every(name=>Array.isArray(receipt.graph?.[name])&&same(receipt.graph[name],current[name]!));", "return current!==undefined;"),
    ("gen", "holds skips the live identity recheck", "if(await nxGenerationIdentityV1(root)!==claim.identity)return false;\n const current", "const current"),
    ("gen", "write accepts shared graph files", "||status.nlink!==1", ""),
    ("gen", "write skips the second stamp read", "if(!after||NX_GENERATION_GRAPH_FILES.some(name=>!same(before[name]!,after[name]!))||await nxGenerationIdentityV1(root)!==claim.identity)return false;", ""),
    ("pool", "pool pass before the namespace lease", "async function acquireNamespace(store:NxGraphCacheStoreV1,value:NxGraphNamespaceV1):Promise<NxGraphNamespaceLeaseV1>{", "async function acquireNamespace(store:NxGraphCacheStoreV1,value:NxGraphNamespaceV1):Promise<NxGraphNamespaceLeaseV1>{\n await poolWithRoom(store,async()=>undefined);"),
    ("pool", "eviction waits on victims", "timeoutMs:0});}catch(error){check(store);if(error instanceof Error&&error.message===\"Resource lease timed out: \"+resource)continue;throw error;}\n   try{\n    await store.recheck();let current", "});}catch(error){check(store);if(error instanceof Error&&error.message===\"Resource lease timed out: \"+resource)continue;throw error;}\n   try{\n    await store.recheck();let current"),
    ("pool", "attach takes the pool", "const child=await retainNxGraphChildV1(store,options);\n try{const backing", "const child=await pool(store,()=>retainNxGraphChildV1(store,options));\n try{const backing"),
    ("pool", "pool pass waits for the namespace lease", "timeoutMs:0});}catch(error){check(store);if(error instanceof Error&&error.message===\"Resource lease timed out: \"+resource)throw new NxGraphNamespaceBusyV1(resource);", "});}catch(error){check(store);if(error instanceof Error&&error.message===\"Resource lease timed out: \"+resource)throw new NxGraphNamespaceBusyV1(resource);"),
    ("pool", "attach ignores the committed phase", "||funded[0]!.phase!==\"committed\"", ""),
    ("slots", "probe counts its own slot", "if(index===slot.index)continue;", ""),
    ("slots", "slot kept after the body", "finally{slot.lease.release();}", "finally{}"),
    ("slots", "slot count not clamped", "Math.min(raw,policy.maximumSlots)", "raw"),
    ("nx", "image lease exclusive again", 'resource:"nx:fixed-cache:"+image.root,mode:"shared"}', 'resource:"nx:fixed-cache:"+image.root,mode}'),
    ("nx2", "headroom ignores the other slots", "await slot.others()*maximum", "0*maximum"),
    ("daemon", "live namespace not reused", "if(input.namespaceLive)return Object.freeze({enabled:true,reason:\"reuse\"});", ""),
    ("daemon", "watch daemons idle out", 'state.kind==="finite"&&', ""),
    ("daemon", "one slot too many", "index<Math.min(limit,policy.maximumSlots)", "index<Math.min(limit+1,policy.maximumSlots)"),
    ("daemon", "limits ignore memory", "Math.min(policy.finite,budget)", "policy.finite"),
    ("service", "slot never released", "finally{release();slot.release();}", "finally{release();}"),
    ("service", "idle timer not cleared", "finally{clearInterval(idleTimer);", "finally{"),
    ("pool", "old room walk returns", "async function evictIdleNamespacesV1", "async function room(){}\nasync function evictIdleNamespacesV1"),
]
failed = 0
for suite, name, old, new, *rest in mutations:
    path = targets[suite]
    original = open(path, encoding="utf8").read()
    if original.count(old) != 1:
        print("SKIP (anchor %d) %s" % (original.count(old), name))
        failed += 1
        continue
    open(path, "w", encoding="utf8").write(original.replace(old, new))
    try:
        result = subprocess.run(["bun", "test", suites[suite]], cwd=OV, capture_output=True, text=True, env={**os.environ, "SEMIO_TEST_ARTIFACT_DIR": "/private/tmp/conc-nx/work/art"})
        killed = result.returncode != 0
    finally:
        open(path, "w", encoding="utf8").write(original)
    print("%s %s" % ("killed  " if killed else "SURVIVED", name))
    failed += 0 if killed else 1
print("mutations %d, survivors/skips %d" % (len(mutations), failed))
sys.exit(1 if failed else 0)
