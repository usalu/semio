import sys, collections
sys.path.insert(0, ".")
from tsplit import *
M = Module.load(PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs", "plugin_builder_contract_tests")
its = M.items
comp = next(i for i in its if i.src.endswith("⏳️completion/🦀️.rs")); comp.has_tests = False
laws = [i.idx for i in its if i.is_test]
inc_items = [i for i in its if i.has_tests and not i.is_test]
bund = {}
for t in laws:
    cl = M.closure([t])
    for h in inc_items:
        if h.idx in cl: bund.setdefault(h.idx, []).append(t)
bundled = {t for v in bund.values() for t in v}
rest = [t for t in laws if t not in bundled]
use = collections.Counter()
for t in rest:
    for x in M.closure([t]):
        if not its[x].has_tests: use[x] += 1
THETA = float(sys.argv[2]) if len(sys.argv) > 2 else 0.5
universal = {x for x, c in use.items() if c >= THETA * len(rest)}
for x in list(universal): universal |= M.closure([x])
ub = sum(its[x].bytes for x in universal)
print("universal", len(universal), ub, sum(its[x].lines for x in universal))
budget = int(sys.argv[1])
pieces = cluster(M, rest, universal, budget)
print("pieces", len(pieces))
allh = collections.Counter()
for p in pieces:
    for x in p["helpers"]: allh[x] += 1
mid = {x for x, c in allh.items() if c >= 2}
print("mid-shared", len(mid), sum(its[x].bytes for x in mid))
for i, p in enumerate(pieces):
    priv = p["helpers"] - mid
    tb = sum(its[t].bytes for t in p["tests"])
    print(i + 1, "tests", len(p["tests"]), tb, "private", len(priv), sum(its[x].bytes for x in priv), "total", tb + sum(its[x].bytes for x in priv) + sum(its[x].bytes for x in mid) + ub)
import re
A = re.compile(r"\bassert(?:_eq|_ne)?!")
print("asserts in universal", sum(len(A.findall(its[x].body)) for x in universal), "mid", sum(len(A.findall(its[x].body)) for x in mid))
print("helper asserts total", sum(len(A.findall(i.body)) for i in its if not i.has_tests and not i.is_test), "tests", sum(len(A.findall(i.body)) for i in its if i.is_test))
sig = collections.defaultdict(list)
for x in mid:
    s = tuple(i + 1 for i, p in enumerate(pieces) if x in p["helpers"])
    sig[s].append(x)
print("signature classes", len(sig))
for s, xs in sorted(sig.items(), key=lambda kv: -sum(its[x].bytes for x in kv[1])): print(s, len(xs), sum(its[x].bytes for x in xs))
