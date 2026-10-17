"""Duplicate-mount scan over the renderer engine folder: Rust files reachable (path mounts and include!) from more than one package root.
usage: dupscan.py"""
import collections, re, sys
from lib import *
import modtree

INC = re.compile(r'\binclude!\(\s*"([^"]+)"\s*\)')


def manifests():
    for dp, dn, fn in os.walk(E):
        dn[:] = [d for d in dn if d not in ("node_modules", "target", "🗑️generated")]
        if "Cargo.toml" in fn:
            yield os.path.join(dp, "Cargo.toml")


def roots_of(m):
    t = read(m)
    d = os.path.dirname(m)
    out = []
    lp = re.search(r'^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"', t, re.M | re.S)
    if lp:
        out.append(os.path.normpath(os.path.join(d, lp.group(1))))
    elif os.path.exists(os.path.join(d, "src/lib.rs")):
        out.append(os.path.join(d, "src/lib.rs"))
    for sec in ("bin", "test"):
        for blk in re.finditer(r"^\[\[%s\]\](.*?)(?=^\[|\Z)" % sec, t, re.M | re.S):
            p = re.search(r'^path\s*=\s*"([^"]+)"', blk.group(1), re.M)
            if p:
                out.append(os.path.normpath(os.path.join(d, p.group(1))))
    return [x for x in out if os.path.exists(x)]


def reach(root):
    seen, queue = set(), [root]
    while queue:
        f = queue.pop()
        if f in seen or not os.path.exists(f):
            continue
        seen.add(f)
        for g in modtree.children(f):
            if g not in seen:
                queue.append(g)
    return seen


if __name__ == "__main__":
    owners = collections.defaultdict(set)
    pk = 0
    for m in manifests():
        mm = re.search(r'^name = "([^"]+)"', read(m), re.M)
        name = mm.group(1) if mm else m
        pk += 1
        for r in roots_of(m):
            for f in reach(r):
                if f.endswith(".rs"):
                    owners[f].add(name)
    multi = {f: sorted(p) for f, p in owners.items() if len(p) > 1}
    print(f"packages scanned: {pk}, rust files reached: {len(owners)}, files reached by more than one package: {len(multi)}")
    for f, p in sorted(multi.items()):
        print(os.path.relpath(f, E)[-120:], "|", ",".join(x.replace("semio-framework-os-renderer-wgpu-", "") for x in p))
