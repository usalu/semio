from model import *
mods = load()
top = collections.defaultdict(list)
for rel, m in mods.items():
    for it in m.items:
        if it.name and it.kind in ('fn','struct','enum','union','trait','type','const','static','macro_rules'):
            top[it.name].append((m.short, it.kind))
d = {k: v for k, v in top.items() if len(v) > 1}
print(len(top), 'top-level names;', len(d), 'duplicated')
for k, v in sorted(d.items())[:60]:
    print(k, v)
