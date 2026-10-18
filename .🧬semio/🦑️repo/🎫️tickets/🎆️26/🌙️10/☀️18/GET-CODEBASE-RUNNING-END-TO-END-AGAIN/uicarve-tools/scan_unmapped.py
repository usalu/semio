import sys, os, collections
sys.path.insert(0, 'lib')
import carve, rs
spec = carve.Spec(sys.argv[1])
roots = [carve.UI + '/🎯️targets/⌨️tui/🦀️.rs']
files = carve.reachable(roots[0])
print(len(files), 'files reachable from glue')
unm = collections.Counter(); tot = 0
for f in files:
    if not f.endswith('.rs'): continue
    s = carve.rd(f); toks = rs.lex(s); masked = rs.mask(s, toks)
    paths = []
    for u in rs.find_uses(s, toks):
        for p, a in rs.flatten_tree(u.tree):
            paths.append(p.split('::'))
    for a, b, segs in rs.path_occurrences(masked, {'crate'}): paths.append(segs)
    for segs in paths:
        if segs[0] != 'crate' or len(segs) < 2: continue
        tot += 1
        if segs[1] != 'tui': unm['crate::' + segs[1]] += 1; continue
        if not spec.rules.lookup(segs[1:]): unm['::'.join(segs[:4])] += 1
print(tot, 'paths')
for k, v in unm.most_common(60): print(v, k)
