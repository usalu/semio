import sys, re, collections
from graph2 import *
tree, mods = load_mods()
nodes = build(mods)
ST = 'World3dState'
def find_state_params(text):
    """Return list of (name, mut, tok_index range) for params typed &World3dState / &mut World3dState in fn signature text."""
    toks = [t for t in lex(text) if t.k != 'comment']
    out = []
    # find first '(' after 'fn name' and its matching ')'
    i = 0
    while i < len(toks) and toks[i].s != 'fn': i += 1
    j = i
    while j < len(toks) and toks[j].s != '(': j += 1
    d = 0; k = j
    while k < len(toks):
        if toks[k].s == '(': d += 1
        elif toks[k].s == ')':
            d -= 1
            if d == 0: break
        k += 1
    sig = toks[j+1:k]
    # split params at depth-0 commas (angle brackets count)
    params = []; cur = []; dd = 0
    for t in sig:
        if t.s in '(<[{': dd += 1
        elif t.s in ')>]}': dd -= 1
        if t.s == ',' and dd == 0: params.append(cur); cur = []
        else: cur.append(t)
    if cur: params.append(cur)
    return params
res = collections.Counter(); names = collections.Counter(); ex = {}
forms = collections.Counter(); formex = {}
for n in nodes:
    if not n.mod.startswith('🌍️world') or n.kind not in ('fn','method'): continue
    mm = mods[n.mod]; it = mm.items[n.item_ref[0 if False else 1]]
    if n.kind == 'method':
        inner, sub = impl_members(mm.text, it)
        s = [x for x in sub if x.name == n.name and inner.count('\n', x.full_start, x.full_end) == n.lines]
        if not s: continue
        text = s[0].text
    else:
        text = it.text
    params = find_state_params(text)
    for p in params:
        ids = [t.s for t in p if t.k == 'id']
        if ST in ids:
            pn = ids[0] if ids[0] not in ('mut',) else ids[1]
            mut = 'mut' in ids[:ids.index(ST)] 
            res[(pn, 'mut' if mut else 'ref')] += 1
            ex.setdefault((pn, 'mut' if mut else 'ref'), n)
    if n.parent == ST: res[('self', n.kind)] += 1
print(res)
