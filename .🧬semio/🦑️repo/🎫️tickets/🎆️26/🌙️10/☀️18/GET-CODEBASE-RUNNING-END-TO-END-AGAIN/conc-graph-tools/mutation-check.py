#!/usr/bin/env python3
"""Red/green proof on an overlay with the set applied: each mutation must turn its named test red (exit 1 means a mutation survived). One test process at a time, niced.

usage: mutation-check.py OVERLAY_DIR [NAME_FILTER]"""
import os, subprocess, sys

overlay = os.path.abspath(sys.argv[1])
only = sys.argv[2] if len(sys.argv) > 2 else ""
E = lambda c: chr(c) + "️"
L = os.path.join(overlay, E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
G = os.path.join(L, E(0x1F3D7) + "build", E(0x1F578) + "graph")
GEN = os.path.join(G, E(0x1F570) + "generations")
TS, MJS, TESTS = E(0x1F7E6) + ".ts", E(0x1F7E8) + ".mjs", E(0x1F9EA) + "tests"
ENGINE, LOAD, ENV = os.path.join(GEN, MJS), os.path.join(GEN, E(0x1F50C) + "load", MJS), os.path.join(GEN, E(0x1F30D) + "environment", MJS)
T_GEN, T_LOAD, T_ENV, T_PROTOCOL = os.path.join(GEN, TESTS, TS), os.path.join(GEN, E(0x1F50C) + "load", TESTS, TS), os.path.join(GEN, E(0x1F30D) + "environment", TESTS, TS), os.path.join(G, TESTS, E(0x1F570) + "generation-protocol", TS)
GRAPH = os.path.join(G, TS)
INF = os.path.join(G, E(0x1F4E5) + "inference")
SCRIPT_INPUTS, ENTRY = os.path.join(INF, E(0x1F9ED) + "script-inputs", MJS), os.path.join(INF, E(0x1F6A6) + "invocation", MJS)
T_FACTS = os.path.join(INF, TESTS, E(0x1F9F7) + "closure-facts", TS)
OBSERVATION = os.path.join(L, E(0x1F3D7) + "build", E(0x1F441) + "observation", MJS)
checks = [
    ("engine: compare-and-swap ignores the generation", ENGINE, "if((pointer?.generation??-1)!==expectedGeneration){", "if(false){", T_GEN, "compare-and-swap"),
    ("engine: stale classes name no owner", ENGINE, "else if(manifest.owners[name])dirty.add(name);}", "else if(manifest.owners[name]){}}", T_GEN, "scenario ring"),
    ("engine: bisection stops at one row", ENGINE, "if(list.length===1){stale.add(list[0]);return;}", "if(list.length===1){return;}", T_GEN, "stale rows are found exactly"),
    ("engine: upserted owners keep their old classes", ENGINE, "for(const name of upserts.keys())touched.add(idOf(name));", "for(const name of upserts.keys())idOf(name);", T_GEN, "fact index names the consumers"),
    ("engine: the manifest digest is not verified", ENGINE, "sha256(bytes)!==pointer.manifest", "false", T_GEN, "missing fragment is rebuilt"),
    ("engine: current serves any identity", ENGINE, "usable=first?.manifest&&first.manifest.identity===identity;", "usable=Boolean(first?.manifest);", T_GEN, "graph policies"),
    ("engine: a closure verifies every owner", ENGINE, "selectFactClassesV1(index,only?new Set([...only,P.envOwner]):undefined)", "selectFactClassesV1(index,undefined)", T_GEN, "outside a closure"),
    ("engine: the pointer swap takes no lease", ENGINE, "return lease?lease.exclusive(resources.facts,body,{signal,timeoutMs:P.publishWaitMilliseconds}):body();", "return body();", T_GEN, "exclusive hold"),
    ("engine: virtual groups are never replaced", ENGINE, "row.tag&&result.replaceTags?.includes(row.tag)&&!returned.has(name)", "false", T_GEN, "virtual groups"),
    ("engine: a missing fragment is not noticed", ENGINE, "&&!present.has(row.fragment+\".json.br\"))dirty.add(name);", "&&false)dirty.add(name);", T_GEN, "missing fragment is rebuilt"),
    ("load: a shared reader builds", LOAD, "builder:builder?body=>body():needsBuild", "builder:body=>body()", T_LOAD, "shared reader never enters the builder"),
    ("load: unclaimed facts are dropped", LOAD, "...part.rows.filter(row=>!claimed.has(rowKeyV1(row)))", "", T_LOAD, "shaping routes"),
    ("load: the graph policy is not read", LOAD, "if(argument.startsWith(\"--graph-policy=\")){policy=argument.slice(\"--graph-policy=\".length);explicit=true;continue;}", "", T_LOAD, "policy flag"),
    ("load: a closure expands every project", LOAD, "const projects=await host.expand(assembled.projects,control);", "const projects=await host.expand(assembled.projects,control);host.expand({a:1,b:1,c:1,d:1,e:1},control);", T_LOAD, "closure reads and expands"),
    ("environment: the lock is not part of the digest", ENV, "lock:lockDigest,", "lock:null,", T_ENV, "digest equals"),
    ("glue: the builder queues on the unscoped writer", GRAPH, "withBuildGraphStateV1(root,invocation,phaseControl,body,generationScopeV1)", "withBuildGraphStateV1(root,invocation,phaseControl,body)", T_PROTOCOL, "builder queues"),
    ("glue: the part infers every owner", GRAPH, "...(part.cold?{}:{ownerRoots:part.roots})", "", T_PROTOCOL, "infers only the named roots"),
    ("glue: the shared reader takes the facts writer", GRAPH, "withRepositoryCacheStoreV1(root,\"graph\",invocation,phaseControl,async(store,control)=>{await store.recheck();checkResourceLeaseControl(control);return body(store,control);})", "withRepositoryBuildGraphFactStoreV1(root,invocation,phaseControl,async(store,control)=>{await store.recheck();checkResourceLeaseControl(control);return body(store,control);})", T_PROTOCOL, "shared reader"),
 ("facts: a closure member is read for its bytes again", SCRIPT_INPUTS, "if(!seen)requirePhysicalSourceV1(path);return;", "if(!seen)readPhysicalSource(path);return;", T_FACTS, "present: a data or asset leaf"),
    ("facts: the partial pass settles in rounds", ENTRY, "args[1]?.snapshot ? observeInferenceSnapshotV1(() => emojiProjectJsonNodes(...args)) : ", "", T_FACTS, "partial branch"),
    ("facts: record headers repeat their rows", OBSERVATION, "if(persistInferenceBlockV1(directory,block))blocks.push(block.digest);else for(const row of block.rows)inline.push(row);", "for(const row of block.rows)inline.push(row);", T_FACTS, "share its fact blocks"),
    ("facts: a bad block is trusted", OBSERVATION, 'if(createHash("sha256").update(text).digest("hex")!==digest)throw Error("Inference block content differs from its name");', "", T_FACTS, "share its fact blocks"),
]
survived = 0
for name, path, old, new, test, selector in checks:
    if only and only not in name:
        continue
    text = open(path, encoding="utf8").read()
    if text.count(old) != 1:
        print("MISS   %s (anchor occurs %d times)" % (name, text.count(old)))
        survived += 1
        continue
    open(path, "w", encoding="utf8").write(text.replace(old, new))
    try:
        result = subprocess.run(["nice", "-n", "10", "bun", "test", "./" + os.path.basename(test), "-t", selector], cwd=os.path.dirname(test), capture_output=True, text=True, env={**os.environ, "SEMIO_TEST_ARTIFACT_DIR": os.environ.get("SEMIO_TEST_ARTIFACT_DIR", "/private/tmp/conc-graph/artifacts")})
    finally:
        open(path, "w", encoding="utf8").write(text)
    red = result.returncode != 0
    print("%-6s %s" % ("killed" if red else "ALIVE", name), flush=True)
    survived += 0 if red else 1
sys.exit(1 if survived else 0)
