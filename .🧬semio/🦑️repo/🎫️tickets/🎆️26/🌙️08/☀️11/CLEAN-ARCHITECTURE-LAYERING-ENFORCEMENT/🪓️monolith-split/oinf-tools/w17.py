exec(open('w15.py').read().split("fam = collections")[0])
import sys
BUDGET = int(sys.argv[1]) if len(sys.argv) > 1 else 3000
maxr = max(rank.values())
users = collections.defaultdict(set)
for u in ug:
    for v in ug[u]: users[v].add(u)
alap = {}
def AL(u):
    if u in alap: return alap[u]
    alap[u] = maxr
    if users[u]:
        alap[u] = min(AL(w) for w in users[u]) - 1
    return alap[u]
for u in ug: AL(u)
assert all(alap[u] >= rank[u] for u in ug)
lvl = {u: alap[u] for u in ug}
L = maxr + 1
size = [0] * L
for u in units: size[lvl[u.id]] += u.lines
E = collections.Counter()
for u in ug:
    for v in ug[u]: E[(lvl[u], lvl[v])] += 1
print('sizes by alap level', size)
# DP: segments [i,j] consecutive levels; cost = number of edges (u level a, v level b) crossing different segments
import functools
INF = 10**9
# cross cost between segmentation: edges (a,b) a>=b with seg(a) != seg(b)
best = {}
def segcost_cut(cuts):
    pass
# simple DP over end positions with cost = edges crossing cut (compute cross edges as sum over cut positions c of edges with b < c <= a ... approximate with cut-based count counted once per cut crossed)
cross_at = [0] * (L + 1)   # edges crossing boundary before level c
for (a, b), n in E.items():
    for c in range(b + 1, a + 1): cross_at[c] += n
dp = [INF] * (L + 1); prv = [None] * (L + 1); dp[0] = 0
pre = [0]
for s in size: pre.append(pre[-1] + s)
for j in range(1, L + 1):
    for i in range(j):
        if pre[j] - pre[i] > BUDGET and j - i > 1: continue
        c = dp[i] + (cross_at[i] if i > 0 else 0)
        if c < dp[j]: dp[j] = c; prv[j] = i
cuts = []; j = L
while j > 0: cuts.append((prv[j], j)); j = prv[j]
cuts.reverse()
print('segments', cuts, 'cost', dp[L])
crate_of_level = {}
for k, (i, j) in enumerate(cuts):
    for l in range(i, j): crate_of_level[l] = k
for k, (i, j) in enumerate(cuts):
    members = [u for u in units if crate_of_level[lvl[u.id]] == k]
    fam = collections.Counter()
    for u in members:
        s = collections.Counter(seed(nodes[m]) for m in u.members).most_common(1)[0][0]
        fam[s] += u.lines
    print(f"crate {k}: levels {i}-{j-1} lines {sum(u.lines for u in members)}  {fam.most_common(7)}")
