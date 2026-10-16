"""🧱 Rewrites the moved test files (non-unit-file) for their new crate roots; reports unresolved leftovers."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, pathrw, units, mounts, packing as P
from okrs import tokenize
FE = '️'
d = json.load(open(L.GEN + '/okwp8-closure-before.json'))
PATHS = [r['path'] for r in d['test native'] if r['kind'] != 'embedded']; PS = set(PATHS)
S = L.S_REL + '/'
def tree_with_depth(root, depth, out=None):
    out = out if out is not None else {}
    if root in out: return out
    out[root] = depth
    for name, tgt, cfg in mounts.children(root):
        if tgt in PS: tree_with_depth(tgt, depth + 1, out)
    return out
def unit_roots():
    res = collections.OrderedDict()
    for name, tgt, cfg in mounts.children(S + '🦀' + FE + '.rs'):
        if tgt in PS and tgt != P.UNIT: res[name] = (tgt, 1)
    for name, tgt, cfg in mounts.children(P.UNIT):
        if tgt in PS: res[name] = (tgt, 2)
    return res
def rewrite_file(f, depth):
    t = L.read(L.R + '/' + f)
    rw = pathrw.Rewriter('bin', depth=depth)
    t2 = rw.rewrite(t)
    return t2, rw.unresolved
def leftovers(text):
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    out = collections.Counter()
    for i, t in enumerate(toks):
        if t[0] == 'id' and t[1] in ('crate', 'store', 'dsl', 'spr', 'vcs', 'semio_format', 'dsl_grammar', 'semio_framework_os_kernel') and i + 2 < len(toks) and toks[i + 1][1] == ':' and toks[i + 2][1] == ':':
            if i > 0 and toks[i - 1][1] in ('.', ):
                continue
            segs = [t[1]]; j = i
            while j + 3 < len(toks) and toks[j + 1][1] == ':' and toks[j + 2][1] == ':' and toks[j + 3][0] == 'id' and len(segs) < 3:
                segs.append(toks[j + 3][1]); j += 3
            out['::'.join(segs)] += 1
    return out
if __name__ == '__main__':
    roots = unit_roots()
    tot = collections.Counter()
    for name, (root, depth) in roots.items():
        if name in ('fixture_mutations',) or name.startswith('retained_codec'): continue
        for f, dp in tree_with_depth(root, depth).items():
            t2, un = rewrite_file(f, dp)
            lo = leftovers(t2)
            if un or lo:
                print('###', f.split('🔨️modules/')[-1], 'depth', dp)
                if un: print('   unresolved', un[:8])
                if lo: print('   leftovers', dict(lo))
            tot.update(lo)
    print(tot)
