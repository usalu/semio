#!/usr/bin/env python3
"""Mutation proof of the graph-mem laws: every mutation changes one rule in the overlay copy of the patched files, runs the law that must notice it (one test process at a time, niced) and restores the file; a mutation that is not killed is a hole in the laws.

usage: mutation-check.py [OVERLAY_DIR] [--only NAME]"""
import os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
overlay = os.path.abspath(args[0]) if args and not args[0].startswith("--") else "/private/tmp/graph-mem/overlay"
only = args[args.index("--only") + 1] if "--only" in args else None
E = lambda code: chr(code) + "️"
L = os.path.join(overlay, E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
B = os.path.join(L, E(0x1F3D7) + "build")
G = os.path.join(B, E(0x1F578) + "graph")
MJS, TS, T = E(0x1F7E8) + ".mjs", E(0x1F7E6) + ".ts", E(0x1F9EA) + "tests"
MEMORY = os.path.join(B, E(0x1F9E0) + "memory")
OBS = os.path.join(B, E(0x1F441) + "observation")
NODES = os.path.join(G, E(0x1F4E5) + "inference", E(0x1F4E6) + "project-nodes")
LOAD = os.path.join(G, E(0x1F570) + "generations", E(0x1F50C) + "load")
ENGINE = os.path.join(G, E(0x1F570) + "generations")
suites = {
    "memory": os.path.join(MEMORY, T),
    "claims": os.path.join(B, T, E(0x1F578) + "graph", E(0x1F9F2) + "owner-claims"),
    "retention": os.path.join(OBS, T, E(0x1F9E0) + "retention"),
    "shared": os.path.join(ENGINE, T, E(0x1F9F6) + "shared-strings"),
    "wiring": os.path.join(NODES, T, E(0x1F6A6) + "governed-dispatch"),
}
mutations = [
    ("memory-boundary-exclusive", os.path.join(MEMORY, MJS), "if(used<=(blocked?budget*policy.resumeRatio:budget))", "if(used<(blocked?budget*policy.resumeRatio:budget))", "memory"),
    ("memory-no-worker-bound", os.path.join(MEMORY, MJS), 'if(running>=workers)return{admit:false,collect:false,reason:"workers"};', "", "memory"),
    ("memory-no-admission-spacing", os.path.join(MEMORY, MJS), "if(running>0&&spacing>0)", "if(false&&spacing>0)", "memory"),
    ("memory-forced-beside-running", os.path.join(MEMORY, MJS), 'return running===0?{admit:true', "return true?{admit:true", "memory"),
    ("memory-no-resume-hysteresis", os.path.join(MEMORY, MJS), "blocked?budget*policy.resumeRatio:budget", "budget", "memory"),
    ("memory-interner-returns-copy", os.path.join(MEMORY, MJS), "if(known!==undefined){hits++;return known;}", "if(known!==undefined){hits++;return text;}", "memory"),
    ("memory-interner-skips-nested-in-arrays", os.path.join(MEMORY, MJS), 'item[index]=interner.share(child);else if(child!==null&&typeof child==="object")pending.push(child);}', "item[index]=interner.share(child);}", "memory"),
    ("memory-interner-unbounded", os.path.join(MEMORY, MJS), "if(characters+text.length>maximumCharacters){table.clear();characters=0;clears++;}", "", "memory"),
    ("claims-scan-from-the-start", os.path.join(LOAD, MJS), "const found=[];for(;low<ordered.length", "const found=[];low=0;for(;low<ordered.length", "claims"),
    ("claims-arrival-order", os.path.join(LOAD, MJS), "[...new Set(list.flatMap(under))].sort((a,b)=>a-b)", "[...new Set(list.flatMap(under))]", "claims"),
    ("claims-no-key-dedupe", os.path.join(LOAD, MJS), "if(!keys.has(key)){keys.add(key);if(rows===base)", "if(true){keys.add(key);if(rows===base)", "claims"),
    ("observation-image-always-read", os.path.join(OBS, MJS), "previous.image!==null&&", "", "retention"),
    ("observation-images-unbounded", os.path.join(OBS, MJS), "while(retainedProjectImageBytes>GRAPH_MEMORY_POLICY_V1.retainedImageBytes&&", "while(false&&", "retention"),
    ("observation-no-interning", os.path.join(OBS, MJS), "return internJsonStringsV1(value,graphStringInternerV1);", "return value;", "retention"),
    ("engine-fragments-not-interned", os.path.join(ENGINE, MJS), "readGenerationFragmentV1(internJsonStringsV1(unpack(bytes,P.maximumFragmentBytes),graphStringInternerV1))", "readGenerationFragmentV1(unpack(bytes,P.maximumFragmentBytes))", "shared"),
    ("nodes-ungoverned-dispatch", os.path.join(NODES, MJS), "await governor.run(async()=>{", "await (async()=>{", "wiring"),
    ("nodes-statistics-dropped", os.path.join(NODES, MJS), "...(governor?{memory:governor.stats()}:{})", "", "wiring"),
]
survivors, killed = [], 0
env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=os.environ.get("SEMIO_TEST_ARTIFACT_DIR", "/private/tmp/graph-mem/artifacts"))
for name, path, old, new, suite in mutations:
    if only and name != only:
        continue
    text = open(path, encoding="utf8").read()
    if text.count(old) != 1:
        print("SKIP", name, "anchor occurs", text.count(old))
        survivors.append(name)
        continue
    try:
        open(path, "w", encoding="utf8").write(text.replace(old, new))
        done = subprocess.run(["nice", "-n", "10", "bun", "test", "--timeout", "120000", "./" + TS], cwd=suites[suite], env=env, capture_output=True, text=True)
        dead = done.returncode != 0
    finally:
        open(path, "w", encoding="utf8").write(text)
    print("KILLED" if dead else "SURVIVED", name)
    killed += dead
    if not dead:
        survivors.append(name)
print("%d killed, %d survivors %s" % (killed, len(survivors), survivors))
sys.exit(1 if survivors else 0)
