# 🔍️ WP4 losslessness: line multiset of the original Shell tree against the new tree (visibility widening normalised), plus region marker census.
import sys, os, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import *
new = sys.argv[1]
def walk(root):
    out = {}
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d != 'node_modules']
        for f in fn:
            if f.endswith('.rs'):
                out[os.path.join(dp, f)] = open(os.path.join(dp, f), encoding='utf-8').read()
    return out
def norm(l):
    s = l.strip()
    s = re.sub(r'\bpub\(super\) ', '', s)
    return s
def multiset(files, skip=lambda p: False):
    c = collections.Counter(); regs = collections.Counter()
    for p, t in files.items():
        if skip(p): continue
        for l in t.split('\n'):
            s = norm(l)
            if not s: continue
            if s.startswith('//#region') or s.startswith('//#endregion'): regs[s] += 1; continue
            c[s] += 1
    return c, regs
old = walk(SH); nw = walk(new)
oc, orr = multiset(old); nc, nrr = multiset({p: t for p, t in nw.items() if '/📥️staging/' not in p} | {p: t for p, t in nw.items() if '/📥️staging/' in p})
lost = oc - nc
print('lost line kinds:', len(lost), 'total lines', sum(lost.values()))
for k, v in sorted(lost.items(), key=lambda kv: -kv[1])[:60]: print(v, repr(k[:150]))
print('regions: old distinct', len(orr), 'new distinct', len(nrr))
print('region lines missing in new:', [k for k in orr if k not in nrr])
extra = nc - oc
print('added line kinds', len(extra), sum(extra.values()))
for k, v in sorted(extra.items(), key=lambda kv: -kv[1])[:25]: print(v, repr(k[:140]))
