import sys, os, collections, re
sys.path.insert(0, 'lib')
import carve, rs
files = carve.reachable(carve.UI + '/🎯️targets/⌨️tui/🦀️.rs')
std = {'std', 'core', 'alloc', 'crate', 'self', 'super', 'Self'}
byfile = {}
for f in files:
    if not f.endswith('.rs'): continue
    s = carve.rd(f); toks = rs.lex(s); masked = rs.mask(s, toks)
    ids = collections.Counter()
    for m in re.finditer(r'(?<![\w:.])([a-z_][a-z0-9_]*)::', masked):
        ids[m.group(1)] += 1
    byfile[f] = ids
agg = collections.defaultdict(set)
for f, ids in byfile.items():
    for i in ids: agg[i].add(os.path.relpath(f, carve.UI))
known = ['libc', 'wasm_bindgen', 'semio', 'ui_', 'unicode', 'serde', 'tiny_skia', 'pack_json', 'async']
for i, fs in sorted(agg.items()):
    if i in std: continue
    if any(k in i for k in known) or i.startswith('semio'):
        print(i, len(fs), sorted(fs)[:6])
