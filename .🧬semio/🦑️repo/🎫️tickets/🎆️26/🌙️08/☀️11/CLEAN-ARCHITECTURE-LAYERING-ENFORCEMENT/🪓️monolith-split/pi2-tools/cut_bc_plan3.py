import sys, collections, re
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
THETA = 0.5
universal = {x for x, c in use.items() if c >= THETA * len(rest)}
for x in list(universal): universal |= M.closure([x])
ub = sum(its[x].bytes for x in universal)
B = int(sys.argv[1])
pieces = cluster(M, rest, universal, B - ub)
units = {}
for i, p in enumerate(pieces): units[f"L{i+1}"] = list(p["tests"])
for h, ts in bund.items():
    nm = "life" if "lifetime" in its[h].src else ("comp" if "composition/🦀️.rs" in its[h].src else f"x{h}")
    units[nm] = [h] + ts
extras = [i.idx for i in inc_items if i.idx not in bund]
# extras: pack by closure size
ex_units = []
for h in sorted(extras, key=lambda h: -its[h].bytes):
    cl = {x for x in M.closure([h]) if not its[x].has_tests}
    for u in ex_units:
        newc = u["cl"] | cl
        tot = sum(its[x].bytes for x in newc) + sum(its[t].bytes for t in u["items"] + [h])
        if tot <= B:
            u["items"].append(h); u["cl"] = newc; break
    else:
        ex_units.append(dict(items=[h], cl=cl))
for k, u in enumerate(ex_units): units[f"X{k+1}"] = u["items"]
sizes = {}
for n, ts in units.items():
    cl = {x for x in M.closure(ts) if not its[x].has_tests}
    sizes[n] = (sum(its[t].bytes for t in ts), sum(its[x].bytes for x in cl), sum(its[t].lines for t in ts) + sum(its[x].lines for x in cl))
    print(n, "tests", len(ts), "testbytes", sizes[n][0], "helper closure bytes", sizes[n][1], "total", sizes[n][0] + sizes[n][1], "lines", sizes[n][2])
