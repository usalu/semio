import em, collections, re
t = em.read(); tab = em.method_table(t)
info, home = em.homes(t, tab)
bare = collections.OrderedDict()
for n, m in tab.items():
    if n.startswith('<'): continue
    toks = em.code_tokens(m['text'])
    for i, tk in enumerate(toks):
        if tk[0] == 'id' and tk[1] == 'self':
            nx = toks[i + 1][1] if i + 1 < len(toks) else ''
            pv = toks[i - 1][1] if i > 0 else ''
            if nx != '.' and not (pv == '&' and False):
                # skip the receiver param
                bare.setdefault(n, []).append(' '.join(x[1] for x in toks[max(0, i - 3): i + 4]))
for n, v in bare.items():
    if len(v) == 1 and re.match(r'.*\(\s*&\s*(mut\s+)?self', v[0]) : continue
    print(n, home[n], v[:4])
