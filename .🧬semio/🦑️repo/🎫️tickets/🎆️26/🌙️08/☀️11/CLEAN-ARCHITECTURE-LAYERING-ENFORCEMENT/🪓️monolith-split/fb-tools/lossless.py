"""🔬 Line-multiset losslessness of the FB carve: every non-blank line of the original engine tree must reappear in the new tree; lists what is new."""
import collections, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ORIG = HERE + "/orig/engine"
NEW = sys.argv[1]
def lines_of(base, only_rs=True):
    c = collections.Counter()
    where = {}
    for dp, dn, fn in os.walk(base):
        for name in fn:
            if not (name.endswith(".rs") or name.endswith(".json")) : continue
            p = os.path.join(dp, name)
            rel = os.path.relpath(p, base)
            if "/orig/" in rel: continue
            for ln in open(p, encoding="utf-8").read().split("\n"):
                s = ln.strip()
                if s:
                    c[s] += 1
                    where.setdefault(s, rel)
    return c, where
o, ow = lines_of(ORIG)
n, nw = lines_of(NEW)
gone = o - n
new = n - o
print("orig non-blank lines", sum(o.values()), "new", sum(n.values()))
print("LOST (in orig, not in new):", sum(gone.values()))
for l, k in sorted(gone.items(), key=lambda x: ow[x[0]]):
    print(f"  -{k} [{ow[l][:40]}] {l[:170]}")
print("ADDED:", sum(new.values()))
for l, k in sorted(new.items(), key=lambda x: nw[x[0]]):
    print(f"  +{k} [{nw[l][:50]}] {l[:170]}")
