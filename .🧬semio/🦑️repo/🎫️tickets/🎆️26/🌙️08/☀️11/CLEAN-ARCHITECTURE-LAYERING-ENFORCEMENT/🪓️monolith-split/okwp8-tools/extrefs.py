import sys, os, json, collections, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import unitgraph
from okrs import tokenize
R = '/Users/ueli/Documents/semio/'
U = R + '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs'
src, its = unitgraph.parse(U)
names = collections.defaultdict(list)
for r in its:
    if r['kind'] in ('fn', 'struct', 'enum', 'trait', 'const', 'static', 'macro_rules', 'type') and r['name']:
        names[r['name']].append(r)
d = json.load(open(os.path.dirname(os.path.abspath(__file__)) + '/../🗑️generated/okwp8-closure-before.json'))
t = d['test native']; lp = {r['path'] for r in d['lib native']}
files = [r['path'] for r in t if r['kind'] != 'embedded' and r['path'] not in lp and r['path'] != U[len(R):]]
for f in files:
    s = open(R + f, encoding='utf8').read()
    ids = {x[1] for x in tokenize(s) if x[0] == 'id'}
    hits = sorted(n for n in names if n in ids)
    if hits: print(f.split('🔨️modules/')[-1], '\n    ', hits)
