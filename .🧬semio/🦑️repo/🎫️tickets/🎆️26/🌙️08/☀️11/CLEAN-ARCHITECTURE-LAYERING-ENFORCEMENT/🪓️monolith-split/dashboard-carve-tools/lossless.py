"""🧮 Line-multiset difference between the original dashboard tree and the carved one: everything removed or added must be an intended edit."""
import collections, os, sys
a, b = sys.argv[1], sys.argv[2]
def lines(root):
    c = collections.Counter()
    for dp, dn, fn in os.walk(root):
        if "node_modules" in dp or "🧭️journeys" in dp:
            continue
        for f in fn:
            if f.endswith(".rs"):
                for line in open(os.path.join(dp, f), encoding="utf-8").read().split("\n"):
                    if line.strip():
                        c[line] += 1
    return c
ca, cb = lines(a), lines(b)
removed = ca - cb
added = cb - ca
print("removed", sum(removed.values()), "added", sum(added.values()))
mode = sys.argv[3] if len(sys.argv) > 3 else "summary"
if mode == "full":
    print("---- removed"); [print(v, k[:230]) for k, v in sorted(removed.items())]
    print("---- added"); [print(v, k[:230]) for k, v in sorted(added.items())]
