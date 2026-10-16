#!/usr/bin/env python3
"""🧬️ Item-body losslessness: every item of the monolith keeps its token skeleton after the carve (paths collapsed to their last segment, visibility and the testkit cfg opening normalized away)."""
import os, sys, re, collections, hashlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm, plan_host, state, defscan
from carve import Model, inline_descendants


def skeleton(it):
    toks = [t for t in it.toks_all if t.k != 'comment']
    out = []
    i = 0
    n = len(toks)
    while i < n:
        t = toks[i]
        # drop visibility
        if t.k == 'id' and t.x == 'pub':
            j = i + 1
            if j < n and toks[j].x == '(':
                while j < n and toks[j].x != ')':
                    j += 1
                i = j + 1
            else:
                i += 1
            continue
        # collapse a::b::c to c
        if t.k == 'id' and i + 2 < n and toks[i + 1].x == ':' and toks[i + 2].x == ':' and not (out and out[-1] == '<'):
            j = i
            while j + 2 < n and toks[j].k == 'id' and toks[j + 1].x == ':' and toks[j + 2].x == ':':
                j += 3
            i = j
            continue
        if t.x == ':' and i + 1 < n and toks[i + 1].x == ':':
            i += 2
            continue
        out.append(t.x)
        i += 1
    text = ' '.join(out)
    text = text.replace('any ( test , feature = "testkit" )', 'test').replace('any ( feature = "testkit" , test )', 'test')
    return text


def collect(model):
    out = collections.defaultdict(list)
    for ident, root in model.roots.items():
        for fm in hcmod.walk(root):
            if fm.inline or getattr(fm, 'missing', False):
                continue
            for S in [fm] + list(inline_descendants(fm)):
                for it in S.all_items:
                    if it.kind in ('use', 'none', 'extern_crate', 'mod', 'include'):
                        continue
                    key = (it.kind, defscan.impl_key(it) if it.kind == 'impl' else it.name)
                    out[key].append(skeleton(it))
    return out


def main():
    ws, model, roots = state.load()
    bws = wsm.WS()
    hcmod.FS = bws
    base = Model(bws, {'semio_framework_plugin_host': plan_host.P('📦️packages/🦀️rust/🦀️.rs')}, allow_dup=True)
    b = collect(base)
    hcmod.FS = ws
    n = collect(model)
    diff = []
    for k, v in b.items():
        nv = n.get(k, [])
        bs = collections.Counter(v)
        ns = collections.Counter(nv)
        if bs - ns:
            diff.append((k, len(bs - ns)))
    print('items', len(b), 'with body differences', len(diff))
    for k, c in diff[:40]:
        print('  ', k, c)
        bv = collections.Counter(b[k]) - collections.Counter(n.get(k, []))
        nv = collections.Counter(n.get(k, [])) - collections.Counter(b[k])
        import difflib
        for x in list(bv)[:1]:
            y = (list(nv) or [''])[0]
            sm = difflib.SequenceMatcher(None, x.split(' '), y.split(' '))
            for tag, i1, i2, j1, j2 in sm.get_opcodes():
                if tag != 'equal':
                    print('       ', tag, ' '.join(x.split(' ')[i1:i2])[:100], '=>', ' '.join(y.split(' ')[j1:j2])[:100])
                    break


if __name__ == '__main__':
    main()
