from lane import *
import re
tree, mods = load_mods()
nodes = build(mods)
g, amb = edges(nodes, 'none')
GEN = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
ROOT = "/Users/ueli/Documents/semio/"
files = [l.rstrip('\n') for l in open(GEN + '/oinf-ext-users.txt', encoding='utf-8')]
calls = collections.Counter(); filesby = collections.defaultdict(set)
pat = re.compile(r'\.([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(')
for f in files:
    try: t = open(ROOT + f, encoding='utf-8').read()
    except Exception: continue
    for m in pat.finditer(t):
        calls[m.group(1)] += 1; filesby[m.group(1)].add(f)
for TY in ('BoardHost', 'DagHost'):
    ms = [n for n in nodes if n.parent == TY and n.kind == 'method']
    used = [n for n in ms if n.name in calls and n.name not in __import__('stdnames').STD]
    print(TY, len(ms), 'methods', sum(n.lines for n in ms), 'lines;', 'externally-called (non-std names):', len(used), [n.name for n in used][:60])
    # callee closure
    seen = set(); stack = [n.id for n in used]
    while stack:
        x = stack.pop()
        if x in seen: continue
        seen.add(x)
        for d in g[x]:
            if nodes[d].parent == TY and nodes[d].kind == 'method': stack.append(d)
    print('  closure methods', len(seen), 'lines', sum(nodes[i].lines for i in seen))
