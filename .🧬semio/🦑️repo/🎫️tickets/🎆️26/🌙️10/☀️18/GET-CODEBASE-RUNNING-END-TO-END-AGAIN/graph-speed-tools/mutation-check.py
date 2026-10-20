#!/usr/bin/env python3
"""Red/green proof on an overlay with the set applied: each mutation must turn the named test red (exit 1 means a mutation survived).

usage: mutation-check.py OVERLAY_DIR"""
import os, subprocess, sys

overlay = os.path.abspath(sys.argv[1])
E = lambda c: chr(c) + "️"
L = os.path.join(overlay, E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
BUILD = os.path.join(L, E(0x1F3D7) + "build")
TS, JS, TESTS = E(0x1F7E6) + ".ts", E(0x1F7E8) + ".mjs", E(0x1F9EA) + "tests"
RECORDS = os.path.join(L, E(0x26A1) + "caching", E(0x1F5C3) + "records")
checks = [
    ("records: eviction removed", os.path.join(RECORDS, TS), "while (openDescriptors.size >= RECORD_STORE_DESCRIPTOR_LIMIT)", "while (false)", os.path.join(RECORDS, TESTS, E(0x1F50C) + "descriptor-limit", TS)),
    ("records: close keeps descriptors", os.path.join(RECORDS, TS), "for (const pack of [...descriptors.keys()]) release(pack);", "", os.path.join(RECORDS, TESTS, E(0x1F50C) + "descriptor-limit", TS)),
    ("lean: sha256 not compared with the name", os.path.join(BUILD, E(0x1F4E5) + "inputs", E(0x1FAB6) + "lean-inventory", JS), 'if(sha256!==key)throw Error("Inventory record content differs from its name");', "", os.path.join(BUILD, E(0x1F4E5) + "inputs", E(0x1FAB6) + "lean-inventory", TESTS, TS)),
    ("lean: hard links accepted", os.path.join(BUILD, E(0x1F4E5) + "inputs", E(0x1FAB6) + "lean-inventory", JS), "before.nlink!==1n||", "", os.path.join(BUILD, E(0x1F4E5) + "inputs", E(0x1FAB6) + "lean-inventory", TESTS, TS)),
    ("lean: memo ignores the identity", os.path.join(BUILD, E(0x1F4E5) + "inputs", E(0x1FAB6) + "lean-inventory", JS), "stampOf(row)===stamp", "true", os.path.join(BUILD, E(0x1F4E5) + "inputs", E(0x1FAB6) + "lean-inventory", TESTS, TS)),
    ("phases: no throttle", os.path.join(BUILD, E(0x1F578) + "graph", E(0x23F1) + "phases", JS), "now()-state.lastAt<minimumMilliseconds", "false", os.path.join(BUILD, E(0x1F578) + "graph", E(0x23F1) + "phases", TESTS, TS)),
    ("phases: final state never published", os.path.join(BUILD, E(0x1F578) + "graph", E(0x23F1) + "phases", JS), "if(state.lastDone!==state.done)await emit(phase,state,state.done);", "", os.path.join(BUILD, E(0x1F578) + "graph", E(0x23F1) + "phases", TESTS, TS)),
    ("owner: shallowest root wins", os.path.join(BUILD, E(0x1F578) + "graph", E(0x1F9ED) + "file-owner", JS), "if(better(row,best))best=row;break;", "if(!best||best.root===\".\")best=row;", os.path.join(BUILD, E(0x1F578) + "graph", E(0x1F9ED) + "file-owner", TESTS, TS)),
    ("owner: root project ignored", os.path.join(BUILD, E(0x1F578) + "graph", E(0x1F9ED) + "file-owner", JS), 'let best=byRoot.get(".");', "let best;", os.path.join(BUILD, E(0x1F578) + "graph", E(0x1F9ED) + "file-owner", TESTS, TS)),
    ("sealed: the reader takes the writer", os.path.join(BUILD, E(0x1F578) + "graph", TS), 'return withRepositoryCacheStoreV1(root,"graph",invocation,phaseControl,', 'return withBuildGraphStateV1(root,invocation,phaseControl,', os.path.join(BUILD, E(0x1F578) + "graph", TESTS, E(0x1F513) + "sealed-reuse", TS)),
    ("sealed: graph published expanded", os.path.join(BUILD, E(0x1F578) + "graph", TS), "source=JSON.stringify(compact)", "source=JSON.stringify(graph)", os.path.join(BUILD, E(0x1F578) + "graph", TESTS, E(0x1F513) + "sealed-reuse", TS)),
]
survived = 0
for name, path, old, new, test in checks:
    text = open(path, encoding="utf8").read()
    if text.count(old) != 1:
        print("MISS  %s (anchor occurs %d times)" % (name, text.count(old)))
        survived += 1
        continue
    open(path, "w", encoding="utf8").write(text.replace(old, new))
    try:
        result = subprocess.run(["bun", "test", "./" + os.path.basename(test)], cwd=os.path.dirname(test), capture_output=True, text=True, env={**os.environ, "SEMIO_TEST_ARTIFACT_DIR": os.environ.get("SEMIO_TEST_ARTIFACT_DIR", "/tmp")})
    finally:
        open(path, "w", encoding="utf8").write(text)
    red = result.returncode != 0
    print("%-6s %s" % ("killed" if red else "ALIVE", name))
    survived += 0 if red else 1
sys.exit(1 if survived else 0)
