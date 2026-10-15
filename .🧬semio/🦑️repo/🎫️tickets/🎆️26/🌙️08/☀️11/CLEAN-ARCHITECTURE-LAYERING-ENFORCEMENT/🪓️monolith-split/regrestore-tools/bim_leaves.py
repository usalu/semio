import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from run import *

spec = SPECS['bim']
O, files, Ts = load(spec)
snapshot(spec, Ts)
OPEN = '//#region 🔖️Leaves'
CLOSE = '//#endregion 🔖️Leaves'
n = 0
for T in Ts:
    p = T.path
    if '/🧬️mutations/' not in p:
        continue
    rel = p.split('/🧬️mutations/')[1]
    cur = read(p)
    assert cur == T.text
    if 'Leaves' in cur:
        continue
    lines = cur.split('\n')
    if rel == '🦀️.rs':
        h = next(i for i, l in enumerate(lines) if l == 'pub mod horizontal_rules;')
        e = next(i for i, l in enumerate(lines) if l == '//#endregion 🧩️Modules')
        assert lines[h + 1].startswith('#[path = "') and 'create-site' in lines[h + 1]
        assert lines[e - 1].startswith('pub mod ')
        new = lines[:h + 1] + [OPEN] + lines[h + 1:e] + [CLOSE] + lines[e:]
    elif rel.count('/') == 1 and len(lines) > 3 and lines[2].startswith('#[path = "🦠️mutation'):
        last = len(lines) - 1
        while lines[last].strip() == '':
            last -= 1
        new = lines[:2] + [OPEN] + lines[2:last + 1] + [CLOSE] + lines[last + 1:]
    else:
        continue
    out = '\n'.join(new)
    assert strip_markers(out) == strip_markers(cur)
    open(p, 'w', encoding='utf-8', newline='').write(out)
    n += 1
print('bim leaves written', n)
