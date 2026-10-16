"""♻️ Losslessness of the WP5 cuts: the multiset of non-blank lines of the Rust files the work package touched (pre-edit snapshot, taken by lib5 before each first write) against the same files plus the files it created, now. Prints what was lost and what is new. usage: python3 lossless5.py [-v]"""
import collections, os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import lib5 as L
snapped = []
for dp, dn, fn in os.walk(L.SNAP):
    for f in fn:
        if f.endswith('.rs'): snapped.append(os.path.relpath(os.path.join(dp, f), L.SNAP))
created = [l for l in L.rd(L.CREATED).split('\n') if l.endswith('.rs')] if os.path.exists(L.CREATED) else []
deleted = [l for l in L.rd(L.DELETED).split('\n') if l] if os.path.exists(L.DELETED) else []
def lines(paths, base):
    c = collections.Counter()
    for p in paths:
        q = os.path.join(base, p)
        if not os.path.exists(q): continue
        for l in open(q, encoding='utf8').read().split('\n'):
            if l.strip(): c[l.rstrip()] += 1
    return c
B = lines(snapped, L.SNAP); A = lines(sorted(set(snapped) | set(created)), L.PLUG)
lost = B - A; new = A - B
print('snapshotted .rs files', len(snapped), '| created .rs files', len(created), '| deleted', len(deleted), '| non-blank lines before', sum(B.values()), 'after', sum(A.values()), '| lost', sum(lost.values()), 'new', sum(new.values()))
if '-v' in sys.argv:
    print('--- lost'); [print(v, k[:200]) for k, v in sorted(lost.items(), key=lambda x: -x[1])[:70]]
    print('--- new'); [print(v, k[:200]) for k, v in sorted(new.items(), key=lambda x: -x[1])[:70]]
