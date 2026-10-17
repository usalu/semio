"""🔬 OSD: line-multiset losslessness of the carve: every non-blank line of the snapshot (.rs files) must reappear in the new tree of the directory module (excluding the client); prints lost and added lines."""
import collections, os, sys
M = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory"
ORIG = os.path.dirname(os.path.abspath(__file__)) + "/orig"
def lines(base):
    c, where = collections.Counter(), {}
    for dp, dn, fn in os.walk(base):
        rel = os.path.relpath(dp, base)
        if rel.startswith("🔌️client") or "/orig" in dp[len(base):] or "node_modules" in rel:
            dn[:] = []; continue
        for name in fn:
            if not name.endswith(".rs"): continue
            p = os.path.join(dp, name); r = os.path.relpath(p, base)
            for ln in open(p, encoding="utf-8").read().split("\n"):
                s = ln.strip()
                if s: c[s] += 1; where.setdefault(s, r)
    return c, where
o, ow = lines(ORIG); n, nw = lines(M)
gone, new = o - n, n - o
print("orig non-blank", sum(o.values()), "new", sum(n.values()))
print("LOST", sum(gone.values()))
for l, k in sorted(gone.items(), key=lambda x: ow[x[0]]): print(f"  -{k} [{ow[l][:60]}] {l[:200]}")
print("ADDED", sum(new.values()))
for l, k in sorted(new.items(), key=lambda x: nw[x[0]]): print(f"  +{k} [{nw[l][:60]}] {l[:200]}")
