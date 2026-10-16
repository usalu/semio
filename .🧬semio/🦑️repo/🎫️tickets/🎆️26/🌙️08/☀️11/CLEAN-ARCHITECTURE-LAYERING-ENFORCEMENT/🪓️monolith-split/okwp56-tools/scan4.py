import sys, os, re, collections, json
sys.path.insert(0, '../okwp0-tools')
import em
t = em.read(); tab = em.method_table(t)
fns = {n: m for n, m in tab.items() if not n.startswith('<')}
files = json.load(open('../🗑️generated/okwp56-kernel-files.json'))
ext = collections.defaultdict(lambda: collections.Counter())
rootsrc = em.code_tokens(t)
# root usages outside the 4 impl blocks (incl. other impls like SpaceMember, Drop)
blocks = em.impl_blocks(t)
L = t.split('\n')
mask = [True] * len(L)
for b in blocks:
    for i in range(b['s'] - 1, b['e']): mask[i] = False
rootout = '\n'.join(l if mask[i] else '' for i, l in enumerate(L))
srcs = {em.ROOT + ' [outside impls]': rootout}
for f in files:
    if f != em.ROOT: srcs[f] = open(f, encoding='utf8').read()
for f, s in srcs.items():
    toks = em.code_tokens(s)
    for i in range(1, len(toks) - 1):
        if toks[i][0] == 'id' and toks[i][1] in fns and toks[i + 1][1] in ('(', ':') and toks[i - 1][1] in ('.', ':'):
            if toks[i + 1][1] == ':' and not (i + 2 < len(toks) and toks[i + 2][1] == ':'): continue
            ext[toks[i][1]][os.path.relpath(f, em.S) if f.startswith(em.S) else f[-50:]] += 1
vis = {n: m['vis'] for n, m in fns.items()}
print('methods used outside the four impls:', len(ext), 'of which pub', sum(1 for n in ext if vis[n]), 'non-pub', sum(1 for n in ext if not vis[n]))
for n, c in sorted(ext.items(), key=lambda kv: -sum(kv[1].values())):
    if not vis[n]: print('NONPUB', n, dict(c.most_common(6)))
print('PUB used outside impls count', sum(1 for n in ext if vis[n]))
json.dump({n: dict(c) for n, c in ext.items()}, open('../🗑️generated/okwp56-ext-method-use.json', 'w'), ensure_ascii=False)
