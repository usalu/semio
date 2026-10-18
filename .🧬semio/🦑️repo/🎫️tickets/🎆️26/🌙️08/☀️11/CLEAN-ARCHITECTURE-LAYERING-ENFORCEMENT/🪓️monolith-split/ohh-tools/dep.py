"""🕸️ OHH dependency probe (read only): does package A reach package B through normal dependencies? usage: dep.py reach <A> <B> | dep.py deps <A> [--dev]"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/pos")
from poslib import *
m = Manifests(Source(None))
m.build_index()
def deps(pkg, dev=False):
    man = m.manifest(pkg)
    if not man:
        return []
    return sorted({d[2] for d in m.deps(man) if (dev or d[0] in ("normal", "dependencies", "dep"))})
if __name__ == "__main__":
    a = sys.argv[1:]
    if a[0] == "deps":
        man = m.manifest(a[1])
        for d in m.deps(man):
            print(d[0], d[2])
    elif a[0] == "reach":
        src, dst = a[1], a[2]
        seen, stack, par = set(), [src], {}
        while stack:
            x = stack.pop()
            if x in seen:
                continue
            seen.add(x)
            man = m.manifest(x)
            if not man:
                continue
            for d in m.deps(man):
                if d[0].startswith("dev"):
                    continue
                if d[2] not in seen:
                    par.setdefault(d[2], x)
                    stack.append(d[2])
        if dst in seen:
            p, path = dst, [dst]
            while p != src:
                p = par[p]
                path.append(p)
            print(" <- ".join(path))
        else:
            print("no path", len(seen))
