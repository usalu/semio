from model import *
from modtree import walk, LIBROOT, INFROOT
tree = walk(LIBROOT)
files = [e['file'] for e in tree if e['file'] and e['file'].startswith(INFROOT) and '🧪️tests' not in e['file']]
mods = {}
for f in files:
    rel = f[len(INFROOT):]
    text = open(f, encoding='utf-8').read()
    items, _ = chunk_file(text)
    m = Mod(); m.rel = rel; m.text = text; m.items = items; m.short = rel.replace('/🦀️.rs','')
    mods[rel] = m
nodes, typenode, order = build_nodes(mods)
topdef = collections.defaultdict(set)
methdef = collections.defaultdict(set)
for nid, n in nodes.items():
    for d in n['defs']: topdef[d].add(nid)
    for d in n['methods']: methdef[d].add(nid)
for mode in ('top-only', 'top+unique-methods'):
    g = {}
    for nid, n in nodes.items():
        refs = set()
        for idn in n['idents']:
            for B in topdef.get(idn, ()):
                if B != nid: refs.add(B)
            if mode != 'top-only':
                ms = methdef.get(idn, ())
                if len(ms) == 1:
                    for B in ms:
                        if B != nid: refs.add(B)
        g[nid] = refs
    comps = scc(g)
    comps.sort(key=lambda c: -sum(nodes[x]['lines'] for x in c))
    print(mode, 'nodes', len(nodes), 'sccs', len(comps))
    for c in comps[:8]:
        ln = sum(nodes[x]['lines'] for x in c)
        mc = collections.Counter(nodes[x]['mod'].split('/')[-2] if '/' in nodes[x]['mod'] else nodes[x]['mod'] for x in c)
        print('  ', len(c), 'items', ln, 'lines', dict(mc.most_common(6)))
amb = {k: len(v) for k, v in methdef.items() if len(v) > 1}
print(len(methdef), 'method names;', len(amb), 'ambiguous')

g = {}
for nid, n in nodes.items():
    refs = set()
    for idn in n['idents']:
        for B in topdef.get(idn, ()):
            if B != nid: refs.add(B)
    g[nid] = refs
comps = scc(g)
comps.sort(key=lambda c: -sum(nodes[x]['lines'] for x in c))
big = comps[1]
S = set(big)
print('--- world SCC members')
for x in sorted(big, key=lambda x: (nodes[x]['mod'], x[2] or '')):
    outs = sorted(nodes[y]['name'] for y in g[x] if y in S)
    print(f"{nodes[x]['mod'].split('/')[-2]:14s} {nodes[x]['kind']:7s} {nodes[x]['name']:48s} {nodes[x]['lines']:5d} -> {', '.join(outs)[:150]}")
