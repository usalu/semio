#!/usr/bin/env python3
"""🧅️ Item-level layering check: given an ordered list of groups and an item→group assignment, report upward item edges and propose the downward moves that repair them."""
import sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

def load(path='/tmp/hubw/ig-lib.json'):
    g = json.load(open(path))
    return g['nodes'], g['edges']

def check(nodes, edges, group_of, order, ignore=lambda k: False):
    rank = {g: i for i, g in enumerate(order)}
    grp = {k: group_of(k) for k in nodes}
    viol = []
    for a, bs in edges.items():
        if grp.get(a) is None or ignore(a):
            continue
        for b in bs:
            if grp.get(b) is None or ignore(b):
                continue
            if rank[grp[b]] > rank[grp[a]]:
                viol.append((a, b))
    return grp, viol

def propose(nodes, edges, group_of, order, ignore=lambda k: False, frozen=lambda k: False):
    """Downward propagation: an item moves to the lowest group among the groups of items that depend on it."""
    rank = {g: i for i, g in enumerate(order)}
    grp = {k: group_of(k) for k in nodes}
    changed = True
    moves = {}
    rev = collections.defaultdict(set)
    for a, bs in edges.items():
        for b in bs:
            rev[b].add(a)
    while changed:
        changed = False
        for b in list(nodes):
            if grp.get(b) is None or ignore(b) or frozen(b):
                continue
            low = min((rank[grp[a]] for a in rev.get(b, ()) if grp.get(a) is not None and not ignore(a)), default=None)
            if low is not None and low < rank[grp[b]]:
                grp[b] = order[low]
                moves[b] = order[low]
                changed = True
    return grp, moves

def key_file(k):
    return k.split('|')[0]

def key_name(k):
    return k.split('|')[2]
