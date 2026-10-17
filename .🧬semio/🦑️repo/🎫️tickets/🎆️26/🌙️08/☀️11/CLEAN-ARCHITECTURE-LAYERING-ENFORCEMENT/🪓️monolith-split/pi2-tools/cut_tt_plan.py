import sys, collections
sys.path.insert(0, ".")
from tsplit import *
path = PLUG + "/🧪️tests/🧪️time-travel/🦀️.rs"
M = Module.load(path)
its = M.items
tests = [i.idx for i in its if i.has_tests]
use = collections.Counter()
for t in tests:
    for x in M.closure([t]):
        if not its[x].has_tests: use[x] += 1
universal = {x for x, c in use.items() if c >= 0.5 * len(tests)}
for x in list(universal): universal |= M.closure([x])
universal = {x for x in universal if not its[x].has_tests}
ub = sum(its[x].bytes for x in universal)
B = int(sys.argv[1])
pieces = cluster(M, tests, universal, B - ub)
units = {f"P{i+1}": p["tests"] for i, p in enumerate(pieces)}
piece_dirs = {u: sibling(path, f"-part-{i+1}") for i, u in enumerate(units)}
docs = {u: f"Part {i+1} of {len(units)} of the history-edit session laws of the plugin runtime (cut from `🧪️time-travel`)." for i, u in enumerate(units)}
files, core, shared, private = emit(M, units, piece_dirs, "⏪️", docs)
for p, tx in files.items():
    print(rel(p)[-60:], len(tx.encode()), tx.count("\n"))
print("shared classes", {tuple(sorted(s)): len(v) for s, v in shared.items()})
import os
SB = os.environ.get("SB", "/tmp/none")
if SB != "/tmp/none":
    for p, tx in files.items():
        q = SB + "/" + rel(p)
        os.makedirs(os.path.dirname(q), exist_ok=True)
        open(q, "w", encoding="utf8").write(tx)
