import sys; sys.path.insert(0,'.')
from usetree import *
import os, collections
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf/stage-w1/🌍️world"
allu = []
for dp, dn, fn in os.walk(S):
    if '🧪️tests' in dp or 'node_modules' in dp or '🎨️r3f' in dp: continue
    for f in fn:
        if f.endswith('.rs'):
            p = os.path.join(dp, f); t = open(p, encoding='utf-8').read()
            items, _ = chunk_file(t)
            for it in items:
                if it.kind == 'use':
                    attrs, vis, ents = parse_use_item(it.text)
                    for e in ents: allu.append((p[len(S)+1:], tuple(attrs), vis, e))
print(len(allu), 'leaf imports')
c = collections.Counter(); 
for f, a, v, e in allu:
    c[((f.split('/')[-2] if '/' in f else 'root'), bool(e['glob']), v)] += 1
print(c)
for f, a, v, e in allu:
    if e['glob']: print('GLOB', f, a, v, '::'.join(e['path']))
roots = collections.Counter(e['path'][0] for f,a,v,e in allu if not e['glob'])
print(roots)
