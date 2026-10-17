"""🧭 POS2 family helper: resolves repo files and directories to owning crates (by lib path ancestry), lists the dependencies of a crate and writes family specs.
usage: pos2fam.py of <repo-relative file or dir>...   |   pos2fam.py deps <pkg>   |   pos2fam.py rdeps <pkg>
Read only (manifests are read from the working tree or a git revision)."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from poslib import *


def load(rev=None):
    src = Source(rev)
    m = Manifests(src)
    m.build_index()
    return src, m


def libdirs(m, rev_src):
    out = {}
    for pkg in m.by_name:
        man = m.manifest(pkg, rev_src)
        if not man:
            continue
        d = man["dir"]
        root = os.path.dirname(os.path.dirname(d)) if d.endswith("📦️packages/🦀️rust") else d
        out[pkg] = (os.path.join(root, "x"), man["dir"])
    return out


def owners(m, src, rels):
    ld = libdirs(m, src)
    res = {}
    for rel in rels:
        rel = rel.rstrip("/")
        best = []
        for pkg, (lib, mdir) in ld.items():
            ldir = os.path.dirname(lib)
            if rel == lib or rel.startswith(ldir + "/") or ldir.startswith(rel + "/") or ldir == rel:
                best.append((len(ldir), pkg))
        best.sort(reverse=True)
        res[rel] = [p for _, p in best[:6]]
    return res


if __name__ == "__main__":
    a = sys.argv[1:]
    src, m = load(None)
    if a[0] == "of":
        for k, v in owners(m, src, a[1:]).items():
            print(k, "->", v)
    elif a[0] == "deps":
        man = m.manifest(a[1], src)
        for kind, name, pkg, feats, opt, ren in m.deps(man):
            print(kind, pkg)
    elif a[0] == "rdeps":
        for pkg in m.by_name:
            man = m.manifest(pkg, src)
            if man and any(d[2] == a[1] for d in m.deps(man)):
                print(pkg)
