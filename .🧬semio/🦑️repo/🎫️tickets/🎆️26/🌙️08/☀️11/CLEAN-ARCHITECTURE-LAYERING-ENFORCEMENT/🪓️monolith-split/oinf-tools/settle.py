"""Layer settling for the os-infinite carve (monotone lifting)."""
from graph2 import *

TYPEKINDS = ('struct','enum','union','type','trait')

def settle(nodes, g, A, ids=None, maxiter=500):
    A = dict(A)
    moved = {}
    members = ids if ids is not None else set(n.id for n in nodes)
    for it in range(maxiter):
        changed = False
        for i in members:
            need = A[i]
            for d in g[i]:
                if d in members and A[d] > need: need = A[d]
            if need > A[i]:
                moved.setdefault(i, A[i]); A[i] = need; changed = True
        if not changed: break
    return A, moved

def ext_methods(nodes, A, ids):
    tyrank = {}
    for i in ids:
        n = nodes[i]
        if n.kind in TYPEKINDS: tyrank.setdefault(n.name, []).append(A[i])
    out = []
    for i in ids:
        n = nodes[i]
        if n.kind in ('method','assoc-const','assoc-type'):
            r = tyrank.get(n.parent)
            if r and A[i] > max(r): out.append(n)
    return out


def repair(nodes, g, A, ids, maxiter=20000, pinned=None, verbose=False, bias=1.0):
    """Min-closure repair: resolve each violating edge by moving the cheaper side."""
    A = dict(A)
    rev = collections.defaultdict(set)
    for i in ids:
        for d in g[i]:
            if d in ids: rev[d].add(i)
    moves = []
    pinned = pinned or set()
    for it in range(maxiter):
        viol = [(i, d) for i in ids for d in g[i] if d in ids and A[d] > A[i]]
        if not viol: break
        # choose violation with the smallest min-cost to keep movement small
        best = None
        for (n, d) in viol[:400]:
            target_up = A[d]
            seen = {n}; stack = [n]; up = 0
            ok = True
            while stack:
                x = stack.pop(); up += nodes[x].lines
                if x in pinned and x != n: ok = False
                for u in rev[x]:
                    if u not in seen and A[u] < target_up:
                        seen.add(u); stack.append(u)
            up_set = seen
            target_dn = A[n]
            seen = {d}; stack = [d]; dn = 0; ok2 = True
            while stack:
                x = stack.pop(); dn += nodes[x].lines
                if x in pinned and x != d: ok2 = False
                for v in g[x]:
                    if v in ids and v not in seen and A[v] > target_dn:
                        seen.add(v); stack.append(v)
            dn_set = seen
            cu = up if ok else 10**9
            cd = dn * bias if ok2 else 10**9
            if cu <= cd: cand = ('up', cu, up_set, target_up)
            else: cand = ('down', cd, dn_set, target_dn)
            if best is None or cand[1] < best[1]: best = cand
        kind, cost, sset, tgt = best
        for x in sset:
            moves.append((x, A[x], tgt)); A[x] = tgt
    else:
        print('repair: iteration limit')
    return A, moves
