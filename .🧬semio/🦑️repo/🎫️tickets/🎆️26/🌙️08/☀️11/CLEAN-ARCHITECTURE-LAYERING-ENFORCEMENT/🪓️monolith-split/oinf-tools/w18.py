exec(open('w15.py').read().split("fam = collections")[0])
# type-definition reachability (struct/enum/type only: definition idents, not method bodies)
tydef = {}
for n in nodes:
    if n.id in ids and n.kind in ('struct','enum','union','type'):
        tydef.setdefault(n.name, []).append(n)
def refs_in_def(name):
    out = set()
    for n in tydef.get(name, ()):
        for idn in n.plain:
            if idn in tydef and idn != name: out.add(idn)
    return out
compty = {'World' + c.capitalize() + 'State': c for c in CORDER}
reach = {}
for tn, c in compty.items():
    seen = set(); st = [tn]
    while st:
        x = st.pop()
        for y in refs_in_def(x):
            if y not in seen and y not in compty: seen.add(y); st.append(y)
    reach[c] = seen
own = collections.defaultdict(set)
for c, s in reach.items():
    for t in s: own[t].add(c)
single = {t: next(iter(cs)) for t, cs in own.items() if len(cs) == 1}
multi = {t: cs for t, cs in own.items() if len(cs) > 1}
print('types reachable from comps:', len(own), 'single-owner', len(single), 'shared', len(multi))
for c in CORDER:
    ts = sorted(t for t, cc in single.items() if cc == c)
    print(c, len(ts), 'types', sum(n.lines for t in ts for n in tydef[t]), 'lines;', ts[:12])
print('shared:', sorted(multi)[:60])
