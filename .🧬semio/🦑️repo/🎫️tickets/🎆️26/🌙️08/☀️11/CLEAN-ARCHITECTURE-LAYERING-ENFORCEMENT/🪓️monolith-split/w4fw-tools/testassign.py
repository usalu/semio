import json, re, os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
exec(open('plan.py').read().split("if __name__")[0])
from lex import mask
R = "/Users/ueli/Documents/semio/"
TOK = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
RANK = ['kernel-identity','kernel-ui','kernel-history','interaction','abi','source-projection','manifest-media','manifest-view-model','manifest-contribution','manifest-arg-schema','kernel-media-jobs','manifest-mutation-inputs','manifest-actions','action-bus','manifest-app-surface','manifest-plugin','kernel']
cl = {i: assign(i) for i in E['order']}
name2cl = collections.defaultdict(set)
for i, c in cl.items():
    if c and c not in ('RELOC',): name2cl[ents[i]['name']].add(c)
    elif c == 'RELOC': name2cl[ents[i]['name']].add('RELOC')
# test files from mount units
U = json.load(open('units.json'))
from graph import FILES
rows = []
for f, us in U.items():
    d = os.path.dirname(FILES[f])
    for u in us:
        if u.get('test') and u['kind'] == 'mod':
            m = re.search(r'#\[path\s*=\s*"([^"]+)"\]', u['attrs'])
            if not m: continue
            p = os.path.normpath(os.path.join(d, m.group(1)))
            if not os.path.exists(p): print("MISSING", p); continue
            t = mask(open(p, encoding='utf-8').read())
            toks = set(TOK.findall(t))
            cs = collections.Counter()
            for tk in toks:
                for c in name2cl.get(tk, ()): cs[c] += 1
            rows.append((f, u['name'], p.replace(R, '').replace('🧰️framework/🔨️modules/', ''), len(open(p, encoding='utf-8').read().split('\n')), dict(cs)))
for r in rows:
    print(r[0], r[1], r[2], r[3]); print('     ', r[4])
