"""Plan evaluation: fine-band seed -> settle -> greedy merge -> crate stats."""
from settle import *
import itertools, random, functools

def short(rel): return rel.replace('/🦀️.rs','').split('/')[-1]

def evaluate(nodes, g, ids, seed, order, budget=3200, verbose=False, tyfix=None):
    pos = {b: i for i, b in enumerate(order)}
    A0 = {n.id: pos[seed(n)] for n in nodes if n.id in ids}
    A, moved = settle(nodes, g, A0, ids)
    # band sizes
    sizes = collections.Counter()
    for i in ids: sizes[A[i]] += nodes[i].lines
    # greedy merge of adjacent non-empty bands
    groups = []; cur = []; cs = 0
    for b in range(len(order)):
        s = sizes.get(b, 0)
        if s == 0: continue
        if cur and cs + s > budget:
            groups.append(cur); cur = []; cs = 0
        cur.append(b); cs += s
    if cur: groups.append(cur)
    crate_of_band = {}
    for gi, grp in enumerate(groups):
        for b in grp: crate_of_band[b] = gi
    C = {i: crate_of_band[A[i]] for i in ids}
    ext = ext_methods(nodes, C, ids)
    return dict(A=A, A0=A0, moved=moved, groups=groups, C=C, ext=ext, sizes=sizes, order=order,
                score=(len(groups), sum(n.lines for n in ext)))

def report(res, nodes, ids):
    order = res['order']
    for gi, grp in enumerate(res['groups']):
        sz = sum(res['sizes'][b] for b in grp)
        print(f"crate {gi}: {sz} lines  bands={[order[b] for b in grp]}")
    print('moved nodes:', len(res['moved']), 'lines', sum(nodes[i].lines for i in res['moved']))
    ext = res['ext']
    print('extension methods:', len(ext), 'lines', sum(n.lines for n in ext))
    print(collections.Counter(n.parent for n in ext).most_common(30))
