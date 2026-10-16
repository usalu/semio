#!/usr/bin/env python3
"""🧭️ Law map: the cargo test path of every law in the monolith and the crate and path it has after the carve."""
import os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm, plan_host, state
from carve import Model, inline_descendants

H = plan_host.H
BASE = 'semio_framework_plugin_host'


def mod_key(m):
    chain = []
    x = m
    while x.inline:
        chain.append(x.name)
        x = x.parent
    return (x.file, tuple(reversed(chain)))


def relpath(m):
    parts = m.path.split('::')
    return '::'.join(parts[1:])


def build_map(new_model, bws=None):
    bws = bws or wsm.WS()
    hcmod.FS = bws
    base = Model(bws, {BASE: plan_host.P('📦️packages/🦀️rust/🦀️.rs')}, allow_dup=True)
    newmods = {}
    for ident, root in new_model.roots.items():
        for m in hcmod.walk(root):
            if getattr(m, 'missing', False):
                continue
            newmods[mod_key(m)] = (ident, relpath(m))
    out = []
    for m in hcmod.walk(base.roots[BASE]):
        if getattr(m, 'missing', False):
            continue
        k = mod_key(m)
        if k in newmods:
            out.append((relpath(m), newmods[k]))
    return sorted(out, key=lambda x: -len(x[0]))


def map_law(table, law):
    """(crate ident, new law path); the module path of a law is every segment but the last, matched exactly and then by its trailing segments."""
    segs = law.split('::')
    mod = '::'.join(segs[:-1])
    leaf = segs[-1]
    exact = [(ident, new) for old, (ident, new) in table if old == mod]
    if exact:
        ident, new = exact[0]
        return ident, (new + '::' + leaf) if new else leaf
    tail = segs[1:-1] if segs[0] == 'component' else segs[:-1]
    for k in range(len(tail), 0, -1):
        suffix = tail[-k:]
        hits = {(ident, new) for old, (ident, new) in table if old and old.split('::')[-k:] == suffix}
        if len(hits) == 1:
            ident, new = next(iter(hits))
            return ident, (new + '::' + leaf) if new else leaf
    return None


if __name__ == '__main__':
    ws, model, roots = state.load()
    table = build_map(model, state.base_ws(ws))
    for law in sys.argv[1:]:
        print(law, '->', map_law(table, law))
