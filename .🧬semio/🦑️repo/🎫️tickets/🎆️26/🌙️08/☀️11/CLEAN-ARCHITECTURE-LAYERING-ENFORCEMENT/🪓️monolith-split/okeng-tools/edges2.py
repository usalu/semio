# 🕸️ Item-level cross-crate edges of the live kernel tree under the A1 re-anchored assignment (okwp56 mycof), kernel-owned items as '<kernel>' (rank top). Prints every back edge with item names. usage: edges2.py [--crates a,b] [--limit N]
import os, sys, collections, json
HERE = os.path.dirname(os.path.abspath(__file__))
P56 = HERE + '/../okwp56-tools'
sys.path.insert(0, P56); sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools'); sys.path.insert(0, HERE)
import a2lib
from a2lib import *
import a2, edges as E
from okrs import tokenize
RANK = C.RANK + ['<kernel>']
ROOT = ns.S + '/🦀️.rs'
def rk(c): return RANK.index(c) if c in RANK else 98

def crate_of(res, root_file_override=True):
    if res.kind != 'def': return None
    f = res.mod.file
    if f not in model.cof()['items']:
        c = model.crate_of_file(f)
        return c or '<kernel>'
    fm = model.file_model(f)
    for x in fm.items:
        if x['start'] == res.item['start']:
            if f == ROOT: return x['crate'] or '<kernel>'
            return x['crate'] or fm.main_crate() or '<kernel>'
    return fm.main_crate() or '<kernel>'

def main():
    a2.prepare()
    for f in list(model.FILE_OVERRIDE):
        pass
    sel = None; limit = 8
    if '--crates' in sys.argv: sel = set(sys.argv[sys.argv.index('--crates') + 1].split(','))
    if '--limit' in sys.argv: limit = int(sys.argv[sys.argv.index('--limit') + 1])
    w = ns.world()
    edges = collections.defaultdict(collections.Counter)
    ex = collections.defaultdict(lambda: collections.defaultdict(list))
    def is_test(m):
        while m is not None:
            if any('test' in c for c in m.cfg): return True
            m = m.parent
        return False
    for f, mod in sorted(w.files.items()):
        if '--notests' in sys.argv and is_test(mod): continue
        planner = f in model.cof()['items']
        if not planner and f not in model.FILE_OVERRIDE and not model.crate_of_file(f, w): 
            fc = None
        fc = model.crate_of_file(f, w)
        fm = model.file_model(f) if planner else None
        if planner:
            its = fm.items
        else:
            from okrs import items as _items
            src = open(f, encoding='utf8').read()
            its = _items(src)
            for it in its: it['crate'] = fc or '<kernel>'; it['sl'] = 0
        for it in its:
            if it['kind'] in ('use', 'mod', None, 'extern'): continue
            if planner: c = (it['crate'] or ('<kernel>' if f == ROOT else fm.main_crate())) or '<kernel>'
            else: c = it['crate']
            toks = E.item_tokens(fm, it) if planner else [t for t in tokenize(it['text']) if t[0] not in ('ws', 'comment')]
            seen = set()
            for segs, pos in E.name_uses(toks):
                key = tuple(segs[:6])
                if key in seen: continue
                seen.add(key)
                r = ns.resolve(mod, segs)
                if r.kind == 'def':
                    d = crate_of(r)
                    if d and d != c:
                        edges[c][d] += 1
                        if len(ex[c][d]) < limit: ex[c][d].append((model.short(f)[-48:], (it['name'] or it['kind'])[:40], '::'.join(segs[:3]) + ' -> ' + r.name))
    batch = open(HERE + '/crates.txt').read().split() if os.path.exists(HERE + '/crates.txt') else open(P56 + '/crates56.txt').read().split()
    for c in RANK:
        if sel and c not in sel: continue
        if c not in edges: continue
        print('##', c, '(batch)' if c in batch else '')
        for d, n in sorted(edges[c].items(), key=lambda kv: rk(kv[0])):
            flag = 'BACK' if rk(d) >= rk(c) and d != c else 'ok'
            if flag == 'BACK' or '--all' in sys.argv:
                print(f'   -> {d} x{n} {flag}')
                if flag == 'BACK':
                    for e in ex[c][d]: print('        ', e)
main()
