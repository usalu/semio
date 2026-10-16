#!/usr/bin/env python3
"""🔬️ Definition scan: every item of the monolith (kind, name, impl header) exists exactly once after the carve; the extras are the explained new items. A seeded sensitivity check proves the scan sees a deleted item."""
import os, sys, re, collections, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm
from carve import Model, inline_descendants, impl_self_ident


def impl_key(it):
    toks = [t.x for t in it.toks if t.k != 'comment']
    s = ' '.join(toks)
    s = s.split('{')[0]
    s = re.sub(r'(?:\b\w+ : : )+', '', s)
    s = re.sub(r'(?<![\w>]) : : ', ' ', s)
    return re.sub(r'\s+', ' ', s).strip()


def collect(model):
    out = collections.Counter()
    for ident, root in model.roots.items():
        for fm in hcmod.walk(root):
            if getattr(fm, 'missing', False) or fm.inline:
                continue
            for S in [fm] + list(inline_descendants(fm)):
                for it in S.all_items:
                    if it.kind in ('use', 'none', 'extern_crate', 'mod', 'include'):
                        continue
                    key = (it.kind, impl_key(it) if it.kind == 'impl' else it.name)
                    out[key] += 1
    return out


def base_model(roots):
    base_ws = wsm.WS()
    hcmod.FS = base_ws
    return Model(base_ws, collections.OrderedDict(roots), allow_dup=True)


def scan(label, base_roots, ws, new_roots, explain):
    b = collect(base_model(base_roots))
    hcmod.FS = ws
    n = collect(Model(ws, collections.OrderedDict(new_roots), allow_dup=True))
    lost, gained = b - n, n - b
    print(f'[{label}] base items {sum(b.values())}, new items {sum(n.values())}, lost {sum(lost.values())}, gained {sum(gained.values())}')
    for k, v in sorted(lost.items()):
        print('   LOST', v, k)
    unexplained = {k: v for k, v in gained.items() if not explain(k)}
    for k, v in sorted(unexplained.items()):
        print('   GAINED', v, k)
    return lost, gained, unexplained


def seeded(label, ws, new_roots, base_roots, victim_file, victim_name):
    """Deletes one real item from the in-memory state and shows the scan reports it as lost."""
    import helpers
    src = ws.read(victim_file)
    keep = src
    helpers.remove_items(ws, victim_file, lambda it, toks: it.name == victim_name and it.kind in ('fn', 'struct', 'const', 'enum'))
    try:
        lost, gained, un = scan(label + ' (seeded)', base_roots, ws, new_roots, lambda k: True)
        return sum(lost.values())
    finally:
        ws.write(victim_file, keep)


EXPLAINED_NAMES = set()


def explained(key):
    kind, name = key
    return name in EXPLAINED_NAMES
