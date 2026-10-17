import sys, collections, json
sys.path.insert(0, ".")
from tsplit import *
M = Module.load(PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs", "plugin_builder_contract_tests")
its = M.items
# completion include: its single test is extracted later; treat as helper here
comp = next(i for i in its if i.src.endswith("⏳️completion/🦀️.rs"))
comp.has_tests = False
laws = [i for i in its if i.is_test]
bund = {}  # include item idx -> list of law tests depending on it
inc_items = [i for i in its if i.has_tests and not i.is_test]
for t in laws:
    cl = M.closure([t.idx])
    for h in inc_items:
        if h.idx in cl:
            bund.setdefault(h.idx, []).append(t.idx)
print("bundles", {its[k].src[-40:]: len(v) for k, v in bund.items()})
# pieces: lifetime (and its dependants), composition (and its dependants), extras
assign = {}
extra_atomic = []
for h in inc_items:
    if h.idx in bund:
        name = "life" if "lifetime" in h.src else ("comp" if "composition/🦀️.rs" in h.src else "x" + str(h.idx))
        assign[h.idx] = name
        for t in bund[h.idx]:
            assign[t] = name
    else:
        extra_atomic.append(h.idx)
rest = [t for t in laws if t.idx not in assign]
K = int(sys.argv[1]) if len(sys.argv) > 1 else 5
tot = sum(t.bytes for t in rest)
acc, k = 0, 1
for t in rest:
    assign[t.idx] = f"L{k}"
    acc += t.bytes
    if acc >= tot * k / K and k < K:
        k += 1
# extras
ex_bytes = sum(its[i].bytes for i in extra_atomic)
print("extras", [(its[i].src[-45:], its[i].bytes) for i in extra_atomic], ex_bytes)
for i in extra_atomic:
    assign[i] = "X1"
core, private, unused = plan(M, assign)
cb = sum(its[i].bytes for i in core); cl_ = sum(its[i].lines for i in core)
print("core items", len(core), "bytes", cb, "lines", cl_, "unused", len(unused), sum(its[i].bytes for i in unused))
byp = collections.defaultdict(list)
for i, p in assign.items(): byp[p].append(i)
for p, ts in sorted(byp.items()):
    tb = sum(its[i].bytes for i in ts); tl = sum(its[i].lines for i in ts)
    pb = sum(its[i].bytes for i in private[p]); pl = sum(its[i].lines for i in private[p])
    print(p, "tests", len(ts), tb, tl, "private", len(private[p]), pb, pl, "total with core", tb + pb + cb, tl + pl + cl_)
print("--- core items by size")
reach = {}
for p, ts in byp.items():
    reach[p] = M.closure(ts) | set(ts)
rows = []
for i in core:
    ps = sorted(p for p in reach if i in reach[p])
    rows.append((its[i].bytes, its[i].lines, its[i].kind, its[i].name, its[i].src[-30:], ps))
for r in sorted(rows, reverse=True)[:40]: print(r)
