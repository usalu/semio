import sys; sys.path.insert(0,'.')
from wengine import *
allfns, files = load_world()
defs, unres = resolve_calls(allfns)
amb = collections.defaultdict(list)
for f in allfns:
    for c in f.calls:
        if c['kind'] == 'method' and len(c['targets']) > 1:
            amb[c['name']].append((f, c))
for nm, lst in amb.items():
    print('==', nm, 'defs:', [str(t.parent) for t in lst[0][1]['targets']])
    for f, c in lst[:12]:
        toks = files[f.file][1]; text = files[f.file][0]
        ci = c['callee_tok']
        # receiver tokens: go back from '.' until statement boundary
        k = ci - 2
        parts = []
        while k >= 0 and toks[k].s not in (';', '{', '}', '=', '(', ',', '=>', '|') and len(parts) < 8:
            parts.append(toks[k].s); k -= 1
        print('   in', f, '  recv:', ''.join(reversed(parts)))
print('=========== context')
for nm, lst in amb.items():
    for f, c in lst:
        text = files[f.file][0]; toks = files[f.file][1]
        pos = toks[c['callee_tok']].a
        line = text.count('\n', 0, pos) + 1
        ls = text.rfind('\n', 0, pos) + 1; le = text.find('\n', pos)
        print(f"{f.file.split('/')[-2]}:{line} {f.parent}::{f.name}: {text[ls:le].strip()[:170]}")
