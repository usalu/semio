exec(open('w15.py').read().split("fam = collections")[0])
import sys
want = sys.argv[1:]
for u in sorted(units, key=lambda u: -u.lines):
    s = collections.Counter(seed(nodes[m]) for m in u.members).most_common(1)[0][0]
    if s in want or not want:
        print(f"{u.lines:5d} r{rank[u.id]:2d} {s:14s} {u.kind:6s} {u.name[:60]}  -> {', '.join(sorted(units[v].name for v in ug[u.id]))[:110]}")
