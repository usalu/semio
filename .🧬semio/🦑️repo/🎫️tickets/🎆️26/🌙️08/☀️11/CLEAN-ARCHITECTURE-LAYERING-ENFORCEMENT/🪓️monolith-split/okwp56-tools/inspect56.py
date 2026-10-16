# 🔍 Quick look at an A2 dump: outputs per node, line counts, new files, the header of every new lifted file.
import json, sys, os, collections
R = '/Users/ueli/Documents/semio'
d = json.load(open(sys.argv[1]))
M = R + '/🧰️framework/🛍️products/💻️os/🔨️modules/'
out = d['out']
print(len(out), 'outputs;', len(d['new']), 'new files; batch', d['batch'])
rows = []
for p, t in sorted(out.items()):
    rel = p.replace(M, 'M/') if p.startswith(M) else os.path.relpath(p, R)
    rows.append((len(t.split('\n')), rel[-110:], 'NEW' if p in set(d['new']) else ''))
for r in rows: print(*r)
if len(sys.argv) > 2:
    p = [q for q in out if sys.argv[2] in q][0]
    print('=' * 20, p[-80:]); print(out[p][:int(sys.argv[3]) if len(sys.argv) > 3 else 3000])
