import sys, re, collections
from graph2 import *
tree, mods = load_mods()
nodes = build(mods)
st = [n for n in nodes if n.kind=='struct' and n.name=='World3dState'][0]
m = mods[st.mod]; it = m.items[st.item_ref[1]]
# parse fields with type (cfg(test) markers)
lines = it.text.split('\n')
fields = []; pending_cfg = None
i = 0
body = it.text
toks = lex(body)
# naive: iterate lines at 4-space indent
cfg = None
for ln in lines:
    mm = re.match(r'^    (#\[cfg\(test\)\])\s*$', ln)
    if mm: cfg = 'test'; continue
    mm = re.match(r'^    (?:pub\s+)?([a-z_][a-z0-9_]*)\s*:\s*(.*?),?\s*$', ln)
    if mm and not ln.strip().startswith('//'):
        fields.append((mm.group(1), mm.group(2), cfg, ln.strip().startswith('pub'))); cfg = None
fset = {f[0] for f in fields}
use = collections.defaultdict(collections.Counter)
for n in nodes:
    if not n.mod.startswith('🌍️world'): continue
    for f in n.fields & fset:
        use[f][n.mod.split('/')[-2]] += 1
for f, ty, cfg, pub in fields:
    mods_ = ','.join(f"{k[:4]}{v}" for k, v in use[f].most_common(5))
    print(f"{'T' if cfg else ' '}{'P' if pub else ' '} {f:38s} {ty[:60]:60s} {mods_}")
