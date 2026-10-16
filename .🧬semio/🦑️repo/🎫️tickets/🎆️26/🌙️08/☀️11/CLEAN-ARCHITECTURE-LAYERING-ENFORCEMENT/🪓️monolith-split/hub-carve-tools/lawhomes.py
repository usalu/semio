#!/usr/bin/env python3
"""🗺️ Maps every test of the hub, old (monolith lib/bin) and new (carved crates), to `cargo test` law paths, so call sites can be re-pointed."""
import os, sys, re, collections, pickle, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm, plan_lib
from carve import Model, inline_descendants, chain_of
from helpers import P

TESTATTR = re.compile(r'^#\[\s*(tokio::test|test|semio_framework_async_macros::async_test|async_test)\b')


def is_test(it):
    return any(TESTATTR.match(a.replace(' ', '')) or a.replace(' ', '').startswith('#[tokio::test') or a.replace(' ', '').startswith('#[test]') for a in it.attrs)


def collect(model):
    """-> {(file, name): [(crate, law path)]}"""
    out = collections.defaultdict(list)
    for ident, root in model.roots.items():
        for fm in hcmod.walk(root):
            if getattr(fm, 'missing', False) or fm.inline:
                continue
            for S in [fm] + list(inline_descendants(fm)):
                for it in S.all_items:
                    if it.kind == 'fn' and is_test(it):
                        mp = model.modpath(S)
                        out[(fm.file, it.name)].append((ident, '::'.join(mp + [it.name])))
    return out


def old_new(state_path='/tmp/hubw/state-all.pkl'):
    st = pickle.load(open(state_path, 'rb'))
    base_ws = wsm.WS()
    hcmod.FS = base_ws
    base = Model(base_ws, collections.OrderedDict([('semio_hub', P('📦️packages/🦀️rust/🦀️.rs')), ('os_hub', P('🏗️bootstrap/🦀️.rs'))]), allow_dup=True)
    old = collect(base)
    ws, model, roots = plan_lib.load_state(state_path)
    new = collect(model)
    moved = st['moved']
    mapping = []
    for (f, name), olds in old.items():
        nf = moved.get((f, name), f)
        news = new.get((nf, name), [])
        if len(olds) > 1 and len(news) > 1:
            pairs = list(zip(olds, news))
        else:
            pairs = [(o, n) for o in olds[:1] for n in news[:1]]
        for (oc, op), (nc, np_) in pairs:
            mapping.append((oc, op, nc, np_))
    seen = {(m[0], m[1]) for m in mapping}
    bynew = collections.defaultdict(list)
    for (f, name), lst in new.items():
        for c, pth in lst:
            bynew[name].append((c, pth))
    for (f, name), olds in old.items():
        for oc, op in olds:
            if (oc, op) in seen:
                continue
            cand = bynew.get(name, [])
            if len(cand) == 1:
                mapping.append((oc, op, cand[0][0], cand[0][1]))
    return mapping, old, new


if __name__ == '__main__':
    mapping, old, new = old_new()
    miss = [(k, v) for k, v in old.items() if (k[0], k[1]) not in new and not any(True for _ in [0])]
    print('old tests', sum(len(v) for v in old.values()), 'mapped', len(mapping), 'new tests', sum(len(v) for v in new.values()))
    json.dump(mapping, open('/tmp/hubw/law-mapping.json', 'w'), ensure_ascii=False)
    c = collections.Counter(nc for _, _, nc, _ in mapping)
    for k, v in sorted(c.items()): print(f'{v:4d} {k}')
