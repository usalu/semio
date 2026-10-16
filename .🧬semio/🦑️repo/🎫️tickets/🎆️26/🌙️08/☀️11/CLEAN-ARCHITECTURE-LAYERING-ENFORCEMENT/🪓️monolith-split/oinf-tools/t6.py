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
topdef = collections.defaultdict(set)   # name -> modules
methdef = collections.defaultdict(set)
for nid, n in nodes.items():
    for d in n['defs']: topdef[d].add(n['mod'])
    for d in n['methods']: methdef[d].add(n['mod'])
# also trait impl methods (not in 'methods'): skip
modrefs = collections.defaultdict(lambda: collections.defaultdict(set))
for nid, n in nodes.items():
    A = n['mod']
    for idn in n['idents']:
        for B in topdef.get(idn, ()):
            if B != A: modrefs[A][B].add(idn)
        ms = methdef.get(idn, ())
        if len(ms) == 1:
            for B in ms:
                if B != A: modrefs[A][B].add('.'+idn)
short = {r: m.short.replace('🎲️board/🔌️ports/➡️directed/','D/').replace('🌍️world/','W/') for r, m in mods.items()}
lines = {r: sum(n['lines'] for n in nodes.values() if n['mod']==r) for r in mods}
for A in sorted(mods):
    outs = sorted(((len(v), B) for B, v in modrefs[A].items()), reverse=True)
    print(f'{short[A]} [{lines[A]}] ->', ', '.join(f'{short[B].split("/")[-1]}:{c}' for c, B in outs))
