"""🗺️ Writes the census input of the staged plugin tree: every .rs file of the gate-counter closures assigned to its crate (directory rule, lift files to their destination crate). usage: CLOSURE=<closure.json> python3 census_input.py"""
import json, os, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import graph3 as g
pt = g.pt
PL = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
CL = pt.CLOSURE
lib_rows = ['lib native', 'lib wasm32-wasip2', 'lib wasm32-unknown-unknown', 'lib native testing-features']
test_rows = ['test native', 'test wasm32-wasip2', 'test native all-features']
owner = {}; lib = collections.defaultdict(set); test = collections.defaultdict(set)
for rows, bucket in ((lib_rows, lib), (test_rows, test)):
    for r in rows:
        for x in CL[r]:
            p = x['path']
            if x.get('kind') == 'embedded' or not p.endswith('.rs') or not p.startswith(PL): continue
            c = pt.file_crate(p[len(PL):])
            owner[p] = c; bucket[c].add(p)
out = dict(owner=owner, lib={c: sorted(v) for c, v in lib.items()}, test={c: sorted(v - lib.get(c, set())) for c, v in test.items()})
json.dump(out, open(HERE + '/../🗑️generated/' + os.environ.get('OUT', 'pwp5-census-input.json'), 'w'), ensure_ascii=False, indent=0)
print({c: (len(out['lib'].get(c, [])), len(out['test'].get(c, []))) for c in sorted(set(out['lib']) | set(out['test'])) if not c.startswith(('UNMAPPED',))})
