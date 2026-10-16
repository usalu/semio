# 🔗️ First-party edges the WP4 crates need (item-level token evidence on the S0 layout) against the declared dependency sets of Appendix H.
import collections, json
exec(open('dag4.py').read().split("bad = []")[0])
need = collections.defaultdict(collections.Counter)
for u, vs in adj.items():
    cu = crate[u]
    if cu not in WP4_CRATES: continue
    for v in vs:
        cv = crate[v]
        if cv != cu and not cv.startswith('TOP') and cv in CR: need[cu][cv] += 1
decl = {k: set(v['deps']) for k, v in CR.items()}
for c in sorted(WP4_CRATES, key=lambda k: RANK[k]):
    have = decl[c]; n = set(need[c])
    miss = sorted(n - have, key=lambda k: RANK[k]); extra = sorted(have - n, key=lambda k: RANK[k])
    up = [m for m in miss if RANK[m] > RANK[c]]
    print(f"{c:20s} rank {RANK[c]:2d} declared {len(have):2d} needed {len(n):2d}  MISSING {[(m, need[c][m]) for m in miss if m not in up]}  UPWARD {[(m, need[c][m]) for m in up]}")
    print(f"{'':24s} declared but not evidenced: {extra}")
