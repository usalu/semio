"""🔍️ A compact difference of two asset documents: counts of changed leaves grouped by path prefix, plus the first few changes.

python3 diff.py <a.json> <b.json> [depth=4] [shown=12]"""
import collections
import json
import sys


def walk(x, y, path, out):
    """🚶️ Collects (path, a, b) of differing leaves."""
    if isinstance(x, dict) and isinstance(y, dict):
        for k in sorted(set(x) | set(y)):
            if k not in x or k not in y:
                out.append((f"{path}/{k}", "∅" if k not in x else "…", "∅" if k not in y else "…"))
            else:
                walk(x[k], y[k], f"{path}/{k}", out)
    elif isinstance(x, list) and isinstance(y, list):
        if len(x) != len(y):
            out.append((path, f"len {len(x)}", f"len {len(y)}"))
        for i, (u, v) in enumerate(zip(x, y)):
            walk(u, v, f"{path}/{i}", out)
    elif x != y:
        out.append((path, x, y))


a, b = (json.load(open(p, encoding="utf8")) for p in sys.argv[1:3])
depth, shown = int(sys.argv[3]) if len(sys.argv) > 3 else 4, int(sys.argv[4]) if len(sys.argv) > 4 else 12
out = []
walk(a, b, "", out)
groups = collections.Counter("/".join(p.split("/")[: depth + 1]) for p, _, _ in out)
print(len(out), "differences")
for g, n in groups.most_common(40):
    print(f"  {n:6d} {g}")
for p, u, v in out[:shown]:
    print("   ", p, str(u)[:70], "→", str(v)[:70])
