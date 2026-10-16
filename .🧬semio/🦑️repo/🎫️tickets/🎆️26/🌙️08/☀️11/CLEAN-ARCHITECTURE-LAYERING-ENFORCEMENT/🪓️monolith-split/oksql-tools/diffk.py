# 🔍 Keyed error diff of two oracle runs (new tag against base tag): prints internal onlyNew / onlyOld entries.
# usage: diffk.py <newTag> <baseTag> <config> [maxRows]
import os, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools')
import okoracle as O
new, base, cfg = sys.argv[1:4]
rows = int(sys.argv[4]) if len(sys.argv) > 4 else 60
def load(tag):
    sb = O.SCR + '/ok-union-' + tag + '/A'
    err = open(f'{O.GEN}/okwp0-oracle-{tag}-A-{cfg}.jsonl').read()
    return O.parse_keyed(err, sb, lambda f: f)
a, ae, aex = load(new); b, be, bex = load(base)
on, oo = a - b, b - a
print(f'{cfg}: new internal={sum(a.values())} external={sum(ae.values())}; base internal={sum(b.values())} external={sum(be.values())}; onlyNew={sum(on.values())} onlyOld={sum(oo.values())}')
for k, v in sorted(on.items())[:rows]: print('  +new', v, tuple(str(x)[-90:] for x in k), '|', aex.get(k, '')[:100])
for k, v in sorted(oo.items())[:rows]: print('  -old', v, tuple(str(x)[-90:] for x in k), '|', bex.get(k, '')[:100])
