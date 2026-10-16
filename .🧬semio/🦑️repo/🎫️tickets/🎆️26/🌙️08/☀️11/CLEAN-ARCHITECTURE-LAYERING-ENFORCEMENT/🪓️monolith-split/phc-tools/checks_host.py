#!/usr/bin/env python3
"""🔎️ Definition scan, orphan scan, test inventory, lossless lines and region report for the plugin-host carve, from the persisted state."""
import os, sys, re, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm, plan_host, state, defscan, scan, verify_static
from carve import Model, inline_descendants, impl_self_ident
BASE = [('semio_framework_plugin_host', plan_host.P('📦️packages/🦀️rust/🦀️.rs'))]


def test_inventory(model):
    out = collections.Counter()
    for ident, root in model.roots.items():
        for fm in hcmod.walk(root):
            if fm.inline or getattr(fm, 'missing', False):
                continue
            toks = fm.toks
            for i, t in enumerate(toks):
                if t.x == 'fn' and t.k == 'id' and i + 1 < len(toks):
                    # look back over attributes for #[test] or async_test
                    j = i - 1
                    while j >= 0 and toks[j].x in ('async', 'pub', 'unsafe', 'const') :
                        j -= 1
                    k = j
                    attrs = ''
                    while k >= 0 and toks[k].x == ']':
                        d = 0
                        e = k
                        while e >= 0:
                            if toks[e].x == ']':
                                d += 1
                            elif toks[e].x == '[':
                                d -= 1
                                if d == 0:
                                    break
                            e -= 1
                        attrs += ''.join(x.x for x in toks[e:k + 1])
                        k = e - 2 if e >= 1 and toks[e - 1].x == '#' else e - 1
                    if '#[test]' in attrs.replace(' ', '') or 'async_test' in attrs:
                        out[toks[i + 1].x] += 1
    return out


def main():
    ws, model, roots = state.load()
    bws = wsm.WS()
    hcmod.FS = bws
    base = Model(bws, collections.OrderedDict(BASE), allow_dup=True)
    bt, nt = test_inventory(base), test_inventory(model)
    print('tests: base', sum(bt.values()), 'new', sum(nt.values()), 'lost', sum((bt - nt).values()), 'gained', sum((nt - bt).values()))
    for k, v in sorted((bt - nt).items())[:20]:
        print('  LOST TEST', v, k)
    hcmod.FS = ws
    lost, gained, un = defscan.scan('host', BASE, ws, list(roots.items()), defscan.explained)
    print('orphan scan:')
    hcmod.FS = ws
    for r in scan.orphan_scan(model):
        print('   ', r)
    ll, gg, of, nf = verify_static.lossless(ws, roots)
    print('lossless: lost lines', sum(ll.values()), 'distinct', len(ll), 'gained', sum(gg.values()), 'distinct', len(gg))
    for k, v in ll.most_common(15):
        print('   LOST LINE', v, k[:140])
    missing, unbalanced = verify_static.region_report(ws, of, nf, bws)
    print('regions: missing names', missing, 'unbalanced files', [os.path.relpath(f, plan_host.H) for f in unbalanced])


if __name__ == '__main__':
    main()
