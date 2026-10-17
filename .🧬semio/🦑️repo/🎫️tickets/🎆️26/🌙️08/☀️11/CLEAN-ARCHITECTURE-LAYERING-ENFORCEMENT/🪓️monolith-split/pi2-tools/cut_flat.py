#!/usr/bin/env python3
"""✂️ Cuts one flat test module file into pieces plus shared fragments and re-mounts the pieces where the file was mounted; the piece budget is searched so that every piece's exact closure stays inside the source boundary.
usage: cut_flat.py <orig path rel to PLUG> <host file rel to PLUG> <mount text file> <module base name> <emoji> "<docs phrase>" [--apply] [--max-bytes N]"""
import sys, collections, os, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tsplit
from tsplit import *
import ed
a = [x for x in sys.argv[1:] if not x.startswith("--")]
path = PLUG + "/" + a[0]
HOST = PLUG + "/" + a[1]
MOUNT = open(a[2], encoding="utf8").read()
BASE, EMOJI, PHRASE = a[3], a[4], a[5]
args = sys.argv[1:]
LIMIT = int(args[args.index("--max-bytes") + 1]) if "--max-bytes" in args else 240000
LIMIT_LINES, LIMIT_FILES = 3600, 28


def attempt(M, budget):
    its = M.items
    tests = [i.idx for i in its if i.has_tests]
    use = collections.Counter()
    for t in tests:
        for x in M.closure([t]):
            if not its[x].has_tests:
                use[x] += 1
    universal = {x for x, c in use.items() if c >= 0.5 * len(tests)}
    for x in list(universal):
        universal |= M.closure([x])
    universal = {x for x in universal if not its[x].has_tests}
    ub = sum(its[x].bytes for x in universal)
    ue = {}
    for x in universal:
        ue.update(its[x].emb)
    pieces = cluster(M, tests, universal, budget - ub - sum(ue.values()), max_files=14)
    units = {f"P{i+1}": p["tests"] for i, p in enumerate(pieces)}
    dirs = {u: sibling(path, f"-part-{i+1}") for i, u in enumerate(units)}
    macros = [it.idx for it in its if it.kind == "macro_rules!"]
    docs = {u: f"Part {i+1} of {len(units)} of the {PHRASE} (cut from `{os.path.basename(os.path.dirname(path))}`; the shared fixtures stay in the original file)." for i, u in enumerate(units)}
    files, core, shared, private = emit2(M, units, dirs, EMOJI, docs, always_core=macros)
    sizes = {u: closure_size(dirs[u], files)[:3] for u in units}
    return units, dirs, files, sizes, (core, shared, private)


M = Module.load(path)
tsplit.MERGE_BELOW = 3000
res = None
for budget in range(300000, 80000, -10000):
    out = attempt(M, budget)
    sizes = out[3]
    ok = all(s[1] <= LIMIT and s[2] <= LIMIT_LINES and s[0] <= LIMIT_FILES for s in sizes.values())
    print("budget", budget, "pieces", len(out[0]), "sizes", [s[1] for s in sizes.values()], "OK" if ok else "")
    if ok:
        res = out
        break
units, dirs, files, sizes, (core, shared, private) = res
mounts = "".join(f'#[cfg(test)]\n#[path = "{os.path.relpath(dirs[u], os.path.dirname(HOST))}"]\nmod {BASE}_{i+1};\n' for i, u in enumerate(units))
host_text = read(HOST)
assert host_text.count(MOUNT) == 1, host_text.count(MOUNT)
probs = verify(M, files)
os.makedirs(SCRATCH, exist_ok=True)
perr = parse_check(files)
print("problems", probs[:5], len(probs), "parse errors", perr)
print("pieces", {u: len(t) for u, t in units.items()}, "files", {rel(p)[-50:]: len(t) for p, t in files.items()})
if "--apply" in args and not probs and not perr:
    for p, tx in files.items():
        ed.put(p, tx)
    ed.put(HOST, host_text.replace(MOUNT, mounts))
    print("applied")
