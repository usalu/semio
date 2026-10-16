"""Facade lib root of the infinite crate: explicit re-exports of every previously public board path, from the new crates."""
import sys, os, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bapi
from bcrates import *

def canonical_paths(mods, ex):
    paths = {('crate',): ()}
    q = [('crate',)]
    while q:
        p = q.pop(0)
        for n, o in sorted(ex[p].items()):
            if o[0] == 'mod' and o[1] not in paths and o[1] != ('crate', 'world') and mods[o[1]].path != ('crate',):
                paths[o[1]] = paths[p] + (n,)
                q.append(o[1])
    return paths

def attr_lines(variants):
    ks = sorted({tuple(v) for v in variants})
    return ks

def render(mods, ex, crates, defcfg, skip_ext_prefix=('semio_framework_os_infinite_world',)):
    wdef = {}
    for c in crates:
        for nm in c.defs: wdef[nm] = c.idx
    paths = canonical_paths(mods, ex)
    missing = []
    def gen(mp, depth):
        pad = '    ' * depth
        names = ex[mp]
        local = collections.defaultdict(list)
        ext = []; subs = []
        for n, o in sorted(names.items()):
            if o[0] == 'mod':
                tgt = o[1]
                if tgt == ('crate', 'world'): continue
                if paths.get(tgt) == paths[mp] + (n,): subs.append((n, tgt))
                elif tgt in paths: ext.append(('modalias', n, tgt))
            elif o[0] == 'item':
                if n not in wdef: missing.append(('::'.join(mp), n)); continue
                local[wdef[n]].append(n)
            elif o[0] == 'ext':
                if o[1][0].startswith(skip_ext_prefix): continue
                ext.append(('ext', n, o[1]))
        res = []
        for ci in sorted(local):
            ident = crate_ident(CRATES[ci][0])
            groups = collections.OrderedDict()
            for n in sorted(set(local[ci]), key=lambda s: s.lower()):
                vs = attr_lines(defcfg.get(n, [()]))
                if len(vs) > 1:
                    for v in vs: groups.setdefault(v, []).append(n)
                else: groups.setdefault(vs[0], []).append(n)
            for attrs, ns in groups.items():
                for a in attrs: res.append(pad + a)
                res.append(pad + (f"pub use {ident}::{{{', '.join(ns)}}};" if len(ns) > 1 else f"pub use {ident}::{ns[0]};"))
        grouped = collections.OrderedDict(); rest = []
        for kind, n, o in ext:
            if kind == 'ext' and len(o) > 1 and o[-1] == n: grouped.setdefault(tuple(o[:-1]), []).append(n)
            else: rest.append((kind, n, o))
        for parent, ns in grouped.items():
            ns = sorted(set(ns), key=lambda s: s.lower())
            res.append(pad + (f"pub use {'::'.join(parent)}::{{{', '.join(ns)}}};" if len(ns) > 1 else f"pub use {'::'.join(parent)}::{ns[0]};"))
        ext = rest
        for kind, n, o in ext:
            if kind == 'ext':
                path = list(o)
                last = path[-1]
                if len(path) == 1: res.append(pad + (f"pub use {path[0]};" if path[0] == n else f"pub use {path[0]} as {n};"))
                else: res.append(pad + (f"pub use {'::'.join(path)};" if last == n else f"pub use {'::'.join(path)} as {n};"))
            else:
                res.append(pad + f"pub use crate::{'::'.join(paths[o])} as {n};")
        for n, tgt in subs:
            res.append(pad + f"pub mod {n} {{")
            res += gen(tgt, depth + 1)
            res.append(pad + "}")
        return res
    return gen(('crate',), 0), missing
