"""🪞 Public surface of the pre-split replication crate: every public module and the names it exposes, resolved in the old world."""
import collections, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *

def candidate_names(W):
    names = set()
    for node in W.nodes.values():
        names |= set(node.defs)
        names |= set(node.mods)
        for pub, leaves, it in node.uses:
            for lf in leaves:
                n = usetree.local_name(lf)
                if n: names.add(n)
    for exported in EXT_EXPORTS.values(): names |= set(exported)
    return names

def public_modules(W):
    """{old path tuple: node} of every module reachable through pub mod / pub use chains from the root."""
    out = {(): W.root}
    stack = [((), W.root)]
    while stack:
        path, node = stack.pop()
        for mname in list(node.mods):
            r = W.lookup(node, mname, mode=2)
            if r and r[0] == "mod" and (path + (mname,)) not in out:
                out[path + (mname,)] = r[1]; stack.append((path + (mname,), r[1]))
        for pub, leaves, it in node.uses:
            if it["vis"] != "pub": continue
            for lf in leaves:
                n = usetree.local_name(lf)
                if not n or lf[2]: continue
                r = W.lookup(node, n, mode=2)
                if r and r[0] == "mod" and (path + (n,)) not in out:
                    out[path + (n,)] = r[1]; stack.append((path + (n,), r[1]))
    return out

def surface(W):
    cands = sorted(candidate_names(W))
    mods = public_modules(W)
    tab = {}
    for path, node in sorted(mods.items()):
        ent = {}
        for n in cands:
            r = W.lookup(node, n, mode=2)
            if r is None: continue
            ent[n] = r
        tab[path] = ent
    return tab

if __name__ == "__main__":
    W = World(FACADE_ROOT, staged=False)
    tab = surface(W)
    for path, ent in tab.items():
        kinds = collections.Counter(r[0] for r in ent.values())
        print("::".join(path) or "<root>", len(ent), dict(kinds))
