#!/usr/bin/env python3
"""🧪️ Builds /private/tmp/ide-free/root: a throw-away copy of every file the batch touches plus the Go workspace (go.work and its modules), so the engine and the Go suites run on a copy.
Read only on the live tree; standard library only."""
import json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
ROOT = os.path.abspath(os.path.join(C, *[".."] * 7)); OUT = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/ide-free/root"
plan = json.load(open(os.path.join(HERE, "staged", "plan.json"), encoding="utf-8"))
shutil.rmtree(OUT, ignore_errors=True); os.makedirs(OUT)
def cp(rel):
    s = os.path.join(ROOT, rel)
    if not os.path.isfile(s): return
    d = os.path.join(OUT, rel); os.makedirs(os.path.dirname(d), exist_ok=True); shutil.copy2(s, d)
paths = {e["path"] for e in plan["edits"]} | {w["path"] for w in plan["writes"]} | {d["path"] for d in plan["deletes"]} | {m["src"] for m in plan["moves"]}
for p in sorted(paths): cp(p)
for rel in ("go.work", "go.work.sum", "AGENTS.md"): cp(rel)
SKIP = {"node_modules", "target", "dist", ".git", ".cache", "out", "__pycache__", ".next"}
mods = [l.strip()[2:] for l in open(os.path.join(ROOT, "go.work"), encoding="utf-8") if l.strip().startswith("./")]
n = 0
for m in mods:
    for dp, dn, fn in os.walk(os.path.join(ROOT, m)):
        dn[:] = [d for d in dn if d not in SKIP]
        for f in fn:
            rel = os.path.relpath(os.path.join(dp, f), ROOT)
            if not os.path.exists(os.path.join(OUT, rel)): cp(rel); n += 1
extra = set()
for m in mods:
    parts = m.split("/📦️packages/")
    if len(parts) == 2: extra.add(parts[0] + "/🧪️tests")
    elif m.endswith("/💻️client/🔌️mcp"): extra.add(m + "/🧪️tests")
extra |= {"🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp", "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧫️fixtures"}
for x in sorted(extra):
    for dp, dn, fn in os.walk(os.path.join(ROOT, x if x.startswith('🧰️') else x)):
        dn[:] = [d for d in dn if d not in SKIP]
        for f in fn:
            rel = os.path.relpath(os.path.join(dp, f), ROOT)
            if not os.path.exists(os.path.join(OUT, rel)): cp(rel); n += 1
print(f"overlay {OUT}: {len(paths)} batch files, {n} Go-module files")
