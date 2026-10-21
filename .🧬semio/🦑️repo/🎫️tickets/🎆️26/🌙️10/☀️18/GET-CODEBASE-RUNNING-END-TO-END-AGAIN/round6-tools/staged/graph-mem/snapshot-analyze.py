#!/usr/bin/env python3
"""🧠️ Attributes a V8-format heap snapshot (Bun.generateHeapSnapshot) to what retains it: self size by type, string kinds, dominator-tree retained sizes with the reference path of the biggest subtrees."""
import json, sys, collections

path = sys.argv[1]
top = int(sys.argv[2]) if len(sys.argv) > 2 else 25
d = json.load(open(path))
meta = d["snapshot"]["meta"]
NF = len(meta["node_fields"]); EF = len(meta["edge_fields"])
ntypes = meta["node_types"][0]; etypes = meta["edge_types"][0]
nodes = d["nodes"]; edges = d["edges"]; strings = d["strings"]
count = d["snapshot"]["node_count"]
T = lambda i: ntypes[nodes[i * NF]]
NAME = lambda i: strings[nodes[i * NF + 1]]
SELF = lambda i: nodes[i * NF + 3]
first = [0] * (count + 1)
for i in range(count): first[i + 1] = first[i] + nodes[i * NF + 4]
succ = [[] for _ in range(count)]
pred = [[] for _ in range(count)]
ename = {}
for i in range(count):
    for e in range(first[i], first[i + 1]):
        et = etypes[edges[e * EF]]
        if et == "weak": continue
        to = edges[e * EF + 2] // NF
        succ[i].append(to); pred[to].append(i)
        ename[(i, to)] = (et, edges[e * EF + 1])
bytype = collections.Counter(); cnt = collections.Counter()
for i in range(count):
    bytype[T(i)] += SELF(i); cnt[T(i)] += 1
total = sum(bytype.values())
print("total self MB", round(total / 1e6, 1), "nodes", count)
for k, v in bytype.most_common(8): print("  type", k, round(v / 1e6, 1), "MB", cnt[k])
kinds = collections.Counter(); kcount = collections.Counter()
for i in range(count):
    if T(i) not in ("string", "concatenated string", "sliced string"): continue
    s = NAME(i); n = SELF(i)
    kind = "path" if s.startswith("/") else "json" if s[:1] in "{[" else "hex64" if len(s) == 64 and all(c in "0123456789abcdef" for c in s) else "key(kind\\0path)" if "\0" in s else "other-short" if len(s) < 64 else "other-long"
    kinds[kind] += n; kcount[kind] += 1
for k, v in kinds.most_common(): print("  string", k, round(v / 1e6, 1), "MB", kcount[k])
# duplicates of strings
dup = collections.defaultdict(lambda: [0, 0])
for i in range(count):
    if T(i) == "string":
        e = dup[NAME(i)]; e[0] += 1; e[1] += SELF(i)
wasted = sum(v[1] - v[1] // v[0] for v in dup.values() if v[0] > 1)
print("duplicate string waste MB", round(wasted / 1e6, 1))
for s, v in sorted(dup.items(), key=lambda kv: -(kv[1][1] - kv[1][1] // kv[1][0]))[:6]:
    print("   dup x%d %.2f MB %r" % (v[0], v[1] / 1e6, s[:90]))
# dominators (Cooper-Harvey-Kennedy) from root 0
sys.setrecursionlimit(10000)
order = []; seen = [False] * count; stack = [(0, 0)]; seen[0] = True
post = [-1] * count
it = [0] * count
st = [0]
while st:
    v = st[-1]
    if it[v] < len(succ[v]):
        w = succ[v][it[v]]; it[v] += 1
        if not seen[w]: seen[w] = True; st.append(w)
    else:
        post[v] = len(order); order.append(v); st.pop()
idom = [-1] * count; idom[0] = 0
rpo = order[::-1]
changed = True
def intersect(a, b):
    while a != b:
        while post[a] < post[b]: a = idom[a]
        while post[b] < post[a]: b = idom[b]
    return a
while changed:
    changed = False
    for v in rpo:
        if v == 0: continue
        new = -1
        for p in pred[v]:
            if idom[p] == -1 or not seen[p]: continue
            new = p if new == -1 else intersect(p, new)
        if new != idom[v]: idom[v] = new; changed = True
ret = [SELF(i) if seen[i] else 0 for i in range(count)]
for v in order:  # post-order: children before parents
    if v != 0 and idom[v] >= 0: ret[idom[v]] += ret[v]
rows = sorted(range(count), key=lambda i: -ret[i])
print("top retained subtrees (excluding root chain)")
shown = 0
for i in rows:
    if i == 0: continue
    if T(i) in ("synthetic",): continue
    parent = idom[i]; et = ename.get((parent, i), ("?", "?"))
    pn = NAME(parent)[:40] if parent >= 0 else "?"
    en = et[1] if isinstance(et[1], int) is False else et[1]
    en = strings[en] if et[0] in ("property", "context", "internal", "shortcut") and isinstance(en, int) and en < len(strings) else en
    print("  %8.2f MB  %-9s %-28s <- %-9s %-30s via %s:%s" % (ret[i] / 1e6, T(i), NAME(i)[:28].replace("\n", " "), T(parent), pn.replace("\n", " "), et[0], str(en)[:30]))
    shown += 1
    if shown >= top: break
