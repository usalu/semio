"""🧬️ Which digest entries differ between the fixture on disk, the digests with the old chord and those with the monotone chord: dgdiff.py <fixture> <old> <new>"""
import json, sys
fx, old, new = (json.load(open(p)) for p in sys.argv[1:4])
def diff(a, b):
    out = []
    for k in sorted(set(a) | set(b)):
        if a.get(k) != b.get(k): out.append(k)
    return out
for kind in ("glb", "fbx"):
    keys = sorted(set(fx[kind]) | set(new[kind]))
    fo = [k for k in keys if fx[kind].get(k) != old[kind].get(k)]
    on = [k for k in keys if old[kind].get(k) != new[kind].get(k)]
    print(kind, "entries", len(keys), "| fixture != old (not mine):", len(fo), "| old != new (mine):", len(on))
    for k in on[:50]: print("  mine", k, diff(old[kind][k], new[kind][k]))
    for k in fo[:10]: print("  other", k, diff(fx[kind].get(k, {}), old[kind].get(k, {})))
