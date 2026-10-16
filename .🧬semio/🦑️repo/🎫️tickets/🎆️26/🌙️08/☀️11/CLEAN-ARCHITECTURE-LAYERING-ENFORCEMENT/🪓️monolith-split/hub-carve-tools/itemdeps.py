#!/usr/bin/env python3
"""🔗️ Intra-file item dependency view: for each top-level item, the other items of the same file it names (impl blocks folded into their type)."""
import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import carve
from carve import parse_items, impl_self_ident

def analyse(path):
    src = open(path).read()
    its, toks = parse_items(src)
    names = {}
    groups = []  # (label, [items])
    byname = collections.OrderedDict()
    for it in its:
        if it.kind == 'use' or it.kind == 'none':
            continue
        if it.kind == 'impl':
            owner = impl_self_ident(it) or '?impl'
            byname.setdefault(owner, []).append(it)
        else:
            key = it.name or ('<%s>' % it.kind)
            byname.setdefault(key, []).append(it)
    idsets = {}
    for k, lst in byname.items():
        ids = set()
        for it in lst:
            for t in it.toks_all:
                if t.k == 'id':
                    ids.add(t.x)
        idsets[k] = ids
    deps = {}
    for k, ids in idsets.items():
        deps[k] = sorted(x for x in ids if x in byname and x != k)
    sizes = {k: sum(src[:it.b].count('\n') - src[:it.a].count('\n') + 1 for it in lst) for k, lst in byname.items()}
    return byname, deps, sizes

if __name__ == '__main__':
    byname, deps, sizes = analyse(sys.argv[1])
    for k in byname:
        print(f'{sizes[k]:5d} {k:48s} -> {", ".join(deps[k])}')
