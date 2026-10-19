import sys, os, collections
sys.path.insert(0, 'lib')
import carve, rs
spec = carve.Spec('specs/wgpu.json')
W = carve.UI + '/🎯️targets/🧊️wgpu'
files = carve.reachable(W + '/🦀️.rs')
for extra in (W + '/🪀️widgets/🧰️kit/🦀️.rs', W + '/⚙️engine/🪟️surfaces/🦀️.rs', W + '/📌️mounted_layout/🎯️hit/🦀️.rs'):
    carve.reachable(extra, files)
print(len(files), 'files')
unm = collections.Counter(); tot = 0; other = collections.Counter()
for f in files:
    if not f.endswith('.rs'): continue
    s = carve.rd(f); toks = rs.lex(s); masked = rs.mask(s, toks)
    paths = []
    for u in rs.find_uses(s, toks):
        for p, a in rs.flatten_tree(u.tree): paths.append(p.split('::'))
    for a, b, segs in rs.path_occurrences(masked, {'crate'}): paths.append(segs)
    for segs in paths:
        if segs[0] != 'crate' or len(segs) < 2: continue
        tot += 1
        if segs[1] != 'wgpu': other['::'.join(segs[:3])] += 1; continue
        if len(segs) == 2: continue
        if not spec.rules.lookup(segs[1:]): unm['::'.join(segs[:4])] += 1
print(tot, 'crate:: paths;', 'non-wgpu crate paths:', dict(other.most_common(10)))
for k, v in unm.most_common(60): print(v, k)
