#!/usr/bin/env python3
"""🧷️ Trait-in-scope scan: names the monolith scope of a file imported through a glob that the carved file no longer binds, and that may be traits whose methods the file calls."""
import os, sys, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm, plan_host, state
from carve import Model, inline_descendants, Carver


def glob_names(m, seen=None, depth=0):
    """Every binding visible in module m through its globs (own uses and items excluded)."""
    seen = seen or set()
    out = {}
    for u in m.uses:
        if u.glob:
            r = hcmod.resolve(m, u.path, None, True)
            if r is not None and r.kind == 'mod' and r.mod is not None and id(r.mod) not in seen:
                tgt = r.mod
                seen2 = seen | {id(tgt)}
                for n, its in tgt.items.items():
                    out.setdefault(n, ('item', tgt, its[0]))
                for uu in tgt.uses:
                    if not uu.glob:
                        out.setdefault(uu.binding, ('use', tgt, uu))
                for n, v in glob_names(tgt, seen2, depth + 1).items():
                    out.setdefault(n, v)
    return out


def main():
    ws, model, roots = state.load()
    bws = wsm.WS()
    hcmod.FS = bws
    base = Model(bws, {'semio_framework_plugin_host': plan_host.P('📦️packages/🦀️rust/🦀️.rs')}, allow_dup=True)
    hcmod.CRATES.clear(); hcmod.CRATES.update(base.roots)
    report = collections.defaultdict(list)
    for file, bm in base.by_file.items():
        nm = model.by_file.get(file)
        if nm is None:
            continue
        old_names = glob_names(bm)
        if not old_names:
            continue
        ids = {t.x for t in nm.toks if t.k == 'id'}
        methods = {nm.toks[i].x for i in range(1, len(nm.toks)) if nm.toks[i - 1].x == '.' and nm.toks[i].k == 'id'}
        hcmod.CRATES.clear(); hcmod.CRATES.update(model.roots)
        for n, (kind, tgt, obj) in old_names.items():
            if not n[:1].isupper() or n in ids:
                continue
            bound = hcmod.lookup(nm, n) is not None
            if bound:
                continue
            if kind == 'item' and obj.kind == 'trait':
                ms = set()
                for i, t in enumerate(obj.toks):
                    if t.x == 'fn' and i + 1 < len(obj.toks):
                        ms.add(obj.toks[i + 1].x)
                hit = ms & methods
                if hit:
                    report[file].append((n, 'first-party-trait', sorted(hit)[:5]))
            elif kind == 'use':
                report[file].append((n, 'external-or-use', []))
        hcmod.CRATES.clear(); hcmod.CRATES.update(base.roots)
    for f, items in report.items():
        print(os.path.relpath(f, plan_host.H))
        for it in items:
            print('    ', it)


if __name__ == '__main__':
    main()
