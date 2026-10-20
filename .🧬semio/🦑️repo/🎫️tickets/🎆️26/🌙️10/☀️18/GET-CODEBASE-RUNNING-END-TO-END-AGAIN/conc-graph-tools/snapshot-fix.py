#!/usr/bin/env python3
"""Cold builds must also skip the all-or-nothing settle pass: the partial-pass switch becomes an explicit `snapshot` option (a cold build has no owner roots). Anchored, dry run by default.

usage: snapshot-fix.py [--root REPO_ROOT] [--write]
WARNING: edits graph-module authority files (graph 🟦️.ts, inference entry): a running graph build fails its authority check and must be restarted."""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
if "--root" in sys.argv:
    ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
E = lambda c: chr(c) + "️"
LIB = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
GRAPH = os.path.join(LIB, E(0x1F3D7) + "build", E(0x1F578) + "graph")
EDITS = [
    (os.path.join(GRAPH, E(0x1F4E5) + "inference", E(0x1F6A6) + "invocation", E(0x1F7E8) + ".mjs"), "args[1]?.ownerRoots ? observeInferenceSnapshotV1(", "args[1]?.snapshot ? observeInferenceSnapshotV1("),
    (os.path.join(GRAPH, E(0x1F7E6) + ".ts"), "...(part.cold?{}:{ownerRoots:part.roots}),onOwnerObservations", "snapshot:true,...(part.cold?{}:{ownerRoots:part.roots}),onOwnerObservations"),
    (os.path.join(GRAPH, E(0x1F4E5) + "inference", E(0x1F9EA) + "tests", E(0x1F9F7) + "closure-facts", E(0x1F523) + ".json"), "args[1]?.ownerRoots ? observeInferenceSnapshotV1(", "args[1]?.snapshot ? observeInferenceSnapshotV1("),
]
state = []
for path, old, new in EDITS:
    text = open(os.path.join(ROOT, path), encoding="utf8").read()
    if new in text and old not in text.replace(new, ""):
        state.append((path, "applied", text))
    elif text.count(old) == 1:
        state.append((path, "pending", text.replace(old, new)))
    else:
        raise SystemExit("DRIFT %s (%d)" % (path[-60:], text.count(old)))
for path, s, _ in state:
    print(s, path[-70:])
if "--write" in sys.argv:
    for path, s, text in state:
        if s == "pending":
            open(os.path.join(ROOT, path), "w", encoding="utf8", newline="").write(text)
    print("WRITTEN")
