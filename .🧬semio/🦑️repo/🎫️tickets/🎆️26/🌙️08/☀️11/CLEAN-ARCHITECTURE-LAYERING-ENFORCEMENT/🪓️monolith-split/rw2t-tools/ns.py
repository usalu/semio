"""🧭 rw2t: namespace model of a Rust file module (own items, imports, glob chains) for the explicit-import rewrite of the law files."""
import os, re, sys, collections
from idx import *
import items as itm

PARENT = {}
CRATE_OF = {}
ROOTS = {}

def register_crate(suffix):
    root = os.path.join(T, CRATES[suffix], "🦀️.rs")
    ROOTS[suffix] = root
    def walk(file, parent, mod):
        if file in PARENT and PARENT[file][0] is not None and parent is not None and PARENT[file][0] != parent:
            pass
        PARENT.setdefault((suffix, file), (parent, mod))
        CRATE_OF[(suffix, file)] = suffix
        for name, is_pub, f, cfg in EX.child_mounts(file):
            if (suffix, f) not in PARENT:
                walk(f, file, name)
    walk(root, None, None)

def file_info(file):
    text = read(file)
    clean = blank(text)
    tab = itm.table(text)
    defs = {}
    for it in tab:
        if it["kind"] in ("impl", "use"):
            continue
        defs.setdefault(it["name"], it)
    mods = {n: (p, f) for n, p, f, c in [(a, b, c, d) for a, b, c, d in EX.child_mounts(file)]}
    us = uses(text, top_only=True)
    return dict(text=text, defs=defs, mods=mods, uses=us)

_INFO = {}
def info(file):
    if file not in _INFO:
        _INFO[file] = file_info(file)
    return _INFO[file]

def ns(suffix, file, stack=()):
    """name -> origin: ('def', file) | ('import', 'path::string') | ('mod', file)"""
    key = (suffix, file)
    if key in stack:
        return {}
    inf = info(file)
    out = {}
    parent = PARENT.get(key, (None, None))[0]
    for cfg, vis, p, local, glob in inf["uses"]:
        if glob:
            if p == ["super"] and parent:
                for n, o in ns(suffix, parent, stack + (key,)).items():
                    out.setdefault(n, o)
            elif len(p) == 1 and p[0] in inf["mods"]:
                cf = inf["mods"][p[0]][1]
                cinf = info(cf)
                for n, it in cinf["defs"].items():
                    out.setdefault(n, ("def", cf))
                for n, o in ns(suffix, cf, stack + (key,)).items():
                    if o[0] in ("def", "mod") and (n in cinf["defs"] or n in cinf["mods"]):
                        out.setdefault(n, o)
                for cfg2, vis2, p2, local2, glob2 in cinf["uses"]:
                    if vis2.startswith("pub") and not glob2:
                        out.setdefault(local2, ("import", "::".join(p2)))
                    elif vis2.startswith("pub") and glob2 and len(p2) == 1 and p2[0] in cinf["mods"]:
                        gf = cinf["mods"][p2[0]][1]
                        for n, it in info(gf)["defs"].items():
                            out.setdefault(n, ("def", gf))
        else:
            out.setdefault(local, ("import", "::".join(p)))
    for n, it in inf["defs"].items():
        out[n] = ("def", file)
    for n, (p, f) in inf["mods"].items():
        out[n] = ("mod", f)
    for m in re.finditer(r"(?m)^\s*macro_rules!\s+(\w+)", blank(inf["text"])):
        out[m.group(1)] = ("def", file)
    return out
