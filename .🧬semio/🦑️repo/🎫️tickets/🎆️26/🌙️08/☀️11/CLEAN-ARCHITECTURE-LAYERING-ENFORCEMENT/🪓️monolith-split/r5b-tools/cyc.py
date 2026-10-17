#!/usr/bin/env python3
"""Condenses the explain edges of the last blocker of a plan log into the core groups and prints their adjacency with the reasons."""
import re, sys, collections
log = open(sys.argv[1], errors="replace").read().split("\n")
start = max(i for i, l in enumerate(log) if l.startswith("blocker:"))
if len(sys.argv) > 2: start = [i for i, l in enumerate(log) if l.startswith("blocker:")][int(sys.argv[2])]
edges = collections.defaultdict(list)
def norm(g):
    if re.search(r"(schema|binary|text)::mutations::[a-z_0-9]+$", g) and "+" not in g: return "LEAVES"
    return g
for l in log[start:]:
    if l.startswith("blocker:") and l is not log[start]: break
    m = re.match(r"^    \{(.*?)\} => \{(.*?)\} via (.*)$", l)
    if m:
        a, b = norm(m.group(1)), norm(m.group(2))
        if a == b: continue
        edges[(a, b)].append(m.group(3))
nodes = sorted({x for e in edges for x in e})
adj = collections.defaultdict(set)
for (a, b) in edges: adj[a].add(b)
for a in nodes:
    print(a[:100])
    for b in sorted(adj[a]):
        print("    ->", b[:90], "|", edges[(a, b)][0][:200].replace("standards::v1_7::subsets::", "").replace("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/", ""))

# SCC over condensed graph
sys.setrecursionlimit(10000)
index = {}; low = {}; onst = set(); st = []; comps = []; counter = [0]
def sc(v):
    index[v] = low[v] = counter[0]; counter[0] += 1; st.append(v); onst.add(v)
    for w in adj[v]:
        if w not in index: sc(w); low[v] = min(low[v], low[w])
        elif w in onst: low[v] = min(low[v], index[w])
    if low[v] == index[v]:
        comp = []
        while True:
            w = st.pop(); onst.discard(w); comp.append(w)
            if w == v: break
        comps.append(comp)
for v in nodes:
    if v not in index: sc(v)
print("\nSCCs (size>1):")
for c in comps:
    if len(c) > 1:
        print(len(c), [x[:60] for x in c if not x.startswith("sqlite") ][:40])
